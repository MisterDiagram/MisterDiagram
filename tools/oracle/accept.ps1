# P3 acceptance: retain the actual post-open save, COM observation and every page/master image.
# https://learn.microsoft.com/en-us/office/vba/api/visio.document.saveas
# https://learn.microsoft.com/en-us/office/vba/api/visio.documents.openex
$outputName = 'accept'
$outputConfig = Join-Path $PSScriptRoot 'accept-output.txt'
if (Test-Path -LiteralPath $outputConfig) { $outputName = (Get-Content -LiteralPath $outputConfig -Raw).Trim() }
if ($outputName -notin @('accept','baseline')) { throw 'invalid acceptance output tree' }
$out = Join-Path (Join-Path $PSScriptRoot 'out') $outputName
New-Item -ItemType Directory -Force $out | Out-Null
$log = Join-Path $out 'probe-log.txt'
if (Test-Path -LiteralPath $log) {
    $history = Join-Path $out ('history-run-' + [guid]::NewGuid().ToString('N'))
    New-Item -ItemType Directory -Force $history | Out-Null
    Copy-Item -LiteralPath $log -Destination (Join-Path $history 'probe-log.txt')
}
"START $(Get-Date -Format s)" | Out-File $log
$app = $null
try {
    . (Join-Path $PSScriptRoot 'common.ps1')
    Add-Type -AssemblyName System.IO.Compression.FileSystem
    $app = New-Visio
    $ok = 0; $failed = 0; $resumed = 0
    $used = @{}
    $scriptHash = (Get-FileHash -Algorithm SHA256 $PSCommandPath).Hash.ToLowerInvariant()
    $commonHash = (Get-FileHash -Algorithm SHA256 (Join-Path $PSScriptRoot 'common.ps1')).Hash.ToLowerInvariant()
    $tempRoot = Join-Path $env:TEMP 'diagramcraft-accept'
    New-Item -ItemType Directory -Force $tempRoot | Out-Null
    foreach ($path in (Get-Content (Join-Path $PSScriptRoot 'accept-list.txt') -Encoding UTF8)) {
        if ([string]::IsNullOrWhiteSpace($path)) { continue }
        $name = [IO.Path]::GetFileNameWithoutExtension($path)
        if ($used.ContainsKey($name)) { throw "duplicate acceptance basename: $name" }
        $used[$name] = $true
        $hash = (Get-FileHash -Algorithm SHA256 -LiteralPath $path).Hash.ToLowerInvariant()
        $directory = Join-Path $out $name
        $complete = Join-Path $directory 'status.json'
        if (Test-Path -LiteralPath $directory) {
            $valid = $false
            try {
                $previous = Get-Content -LiteralPath $complete -Raw -Encoding UTF8 | ConvertFrom-Json
                $valid = $previous.opened -and $previous.saved -and (-not $previous.error) -and $previous.input_sha256 -eq $hash -and $previous.script_sha256 -eq $scriptHash -and $previous.common_sha256 -eq $commonHash
                $required = @('observed.sheet.tsv','connects.tsv','parts.txt',('saved' + [IO.Path]::GetExtension($path)))
                foreach ($requiredName in $required) {
                    if ($requiredName -notin @($previous.artifacts | ForEach-Object { $_.file })) { $valid = $false }
                }
                foreach ($artifact in $previous.artifacts) {
                    if ([IO.Path]::GetFileName($artifact.file) -ne $artifact.file) { $valid = $false; break }
                    $p = Join-Path $directory $artifact.file
                    if (-not (Test-Path -LiteralPath $p)) { $valid = $false; break }
                    if ((Get-FileHash -Algorithm SHA256 -LiteralPath $p).Hash.ToLowerInvariant() -ne $artifact.sha256) { $valid = $false; break }
                }
            } catch { $valid = $false }
            if ($valid) { $resumed++; Log "RESUME OK $path"; continue }
            # Preserve interrupted, failed or stale evidence, including a missing status file.
            $archive = Join-Path $out ('history-' + $name + '-' + [guid]::NewGuid().ToString('N'))
            Move-Item -LiteralPath $directory -Destination $archive
        }
        New-Item -ItemType Directory -Force $directory | Out-Null
        $doc = $null
        $local = Join-Path $tempRoot ([IO.Path]::GetFileName($path))
        $status = @{ input_sha256=$hash; script_sha256=$scriptHash; common_sha256=$commonHash; opened=$false; saved=$false; error=$null; artifacts=@() }
        Begin-File $path
        try {
            Copy-Item -LiteralPath $path -Destination $local -Force
            $doc = $app.Documents.OpenEx($local, 130)
            $status.opened = $true
            Log "OPEN OK $path"
            Dump-Document $doc (Join-Path $directory 'observed.sheet.tsv')
            Dump-Connects $doc (Join-Path $directory 'connects.tsv')
            foreach ($page in $doc.Pages) { Export-Png $app $page (Join-Path $directory "page-$($page.ID).png") }
            foreach ($master in $doc.Masters) { Export-Png $app $master (Join-Path $directory "master-$($master.ID).png") }
            # SaveAs is essential: inspecting the untouched input cannot reveal silent repair.
            $saved = Join-Path $directory ('saved' + [IO.Path]::GetExtension($path))
            $doc.SaveAs($saved) | Out-Null
            $status.saved = $true
            Close-Document $doc
            $doc = $null
            $zip = [IO.Compression.ZipFile]::OpenRead($saved)
            try { $zip.Entries | Sort-Object FullName | ForEach-Object { $_.FullName } | Out-File (Join-Path $directory 'parts.txt') }
            finally { $zip.Dispose() }
            $ok++
            Log "ACCEPT OBSERVED $path"
        } catch { $failed++; $status.error = "$($_.Exception.Message) $($_.ScriptStackTrace)"; Log "ACCEPT FAIL $path $($status.error)" }
        finally {
            if ($doc) { try { Close-Document $doc; $doc = $null } catch { $status.error = "close failed: $_"; Log "CLOSE FAIL $path $_" } }
            if (Test-Path -LiteralPath $local) { Remove-Item -LiteralPath $local -Force }
            # Write UTF8 here so the host conversion does not invalidate completion hashes.
            # A failed close must not cause even a hash read of the locked save.
            foreach ($f in $(if (-not $doc) { Get-ChildItem -LiteralPath $directory -File })) {
                if ($f.Extension -in @('.tsv','.txt')) {
                    $text = [IO.File]::ReadAllText($f.FullName)
                    [IO.File]::WriteAllText($f.FullName,$text,[Text.UTF8Encoding]::new($false))
                }
                $status.artifacts += @{file=$f.Name;sha256=(Get-FileHash -Algorithm SHA256 -LiteralPath $f.FullName).Hash.ToLowerInvariant()}
            }
            [IO.File]::WriteAllText($complete,($status | ConvertTo-Json -Depth 5),[Text.UTF8Encoding]::new($false))
            End-File
        }
    }
    Log "SUMMARY observed=$ok resumed=$resumed failed=$failed cells=$script:CellCount cell_errors=$script:CellErrors"
} catch { "FAIL $($_.Exception.Message) $($_.ScriptStackTrace)" | Out-File $log -Append
} finally {
    if ($app) { try { $app.Quit() } catch { "QUIT FAIL $_" | Out-File $log -Append } }
    'DONE' | Out-File (Join-Path $out 'DONE')
}
