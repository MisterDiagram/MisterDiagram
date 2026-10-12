$out = '\\Mac\Home\Projects\diagramcraft\corpus\_vm\out\accept'
New-Item -ItemType Directory -Force $out | Out-Null
$log = Join-Path $out 'probe-log.txt'
"START $(Get-Date -Format s)" | Out-File $log
$app = $null
try {
    . (Join-Path $PSScriptRoot 'common.ps1')
    Add-Type -AssemblyName System.IO.Compression.FileSystem
    $app = New-Visio
    $ok = 0; $failed = 0
    $used = @{}
    $tempRoot = Join-Path $env:TEMP 'diagramcraft-accept'
    New-Item -ItemType Directory -Force $tempRoot | Out-Null
    foreach ($path in (Get-Content (Join-Path $PSScriptRoot 'accept-list.txt') -Encoding UTF8)) {
        if ([string]::IsNullOrWhiteSpace($path)) { continue }
        $name = [IO.Path]::GetFileNameWithoutExtension($path)
        if ($used.ContainsKey($name)) { throw "duplicate acceptance basename: $name" }
        $used[$name] = $true
        $doc = $null
        $local = Join-Path $tempRoot ([IO.Path]::GetFileName($path))
        Begin-File $path
        try {
            Copy-Item -LiteralPath $path -Destination $local -Force
            Log "LOCAL COPY $path -> $local"
            $doc = $app.Documents.OpenEx($local, 2)
            Log "OPEN OK $path"
            Dump-Document $doc (Join-Path $out "$name.accept.sheet.tsv")
            Export-Png $app $doc.Pages.Item(1) (Join-Path $out "$name.accept.png")
            $zip = [IO.Compression.ZipFile]::OpenRead($local)
            try { $zip.Entries | Sort-Object FullName | ForEach-Object { "$($_.FullName)`t$($_.Length)" } | Out-File (Join-Path $out "$name.accept.parts.txt") }
            finally { $zip.Dispose() }
            $ok++
            Log "ACCEPT OK $path"
        } catch { $failed++; Log "ACCEPT FAIL $path $($_.Exception.Message) $($_.ScriptStackTrace)" }
        finally {
            if($doc) { try { Close-Document $doc } catch { Log "CLOSE FAIL $path $_" } }
            if (Test-Path -LiteralPath $local) { Remove-Item -LiteralPath $local -Force }
            End-File
        }
    }
    Log "SUMMARY accepted=$ok failed=$failed cells=$script:CellCount cell_errors=$script:CellErrors primary_cells=$($script:CellCount-$script:SupplementCells) supplemental_cells=$script:SupplementCells"
} catch { "FAIL $($_.Exception.Message) $($_.ScriptStackTrace)" | Out-File $log -Append
} finally {
    if ($app) { try { $app.Quit() } catch { "QUIT FAIL $_" | Out-File $log -Append } }
    'DONE' | Out-File (Join-Path $out 'DONE')
}
