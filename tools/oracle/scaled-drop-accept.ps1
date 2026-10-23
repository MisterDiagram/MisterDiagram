# Brief31c: one native invocation, all 13 drawings and a fresh blank window control.
# No historical comparison rule is changed by this capture.
# https://learn.microsoft.com/en-us/office/vba/api/visio.documents.openex
# https://learn.microsoft.com/en-us/office/vba/api/visio.document.saveas
$out = Join-Path $PSScriptRoot 'out/scaled-drop-accept'
New-Item -ItemType Directory -Force $out | Out-Null
$log = Join-Path $out 'probe-log.txt'
"START $(Get-Date -Format s)" | Out-File $log
$app = $null
$runId = [guid]::NewGuid().ToString('N')
[IO.File]::WriteAllText((Join-Path $out 'run.json'),(@{run_id=$runId;completed=$false} | ConvertTo-Json),[Text.UTF8Encoding]::new($false))
try {
    . (Join-Path $PSScriptRoot 'common.ps1')
    $root = '\\Mac\Home\Projects\diagramcraft\plan\brief31c\final-inputs'
    $names = @('blank-window-control','brief31-noscale-control-1-0','brief31-noscale-control-1-3','brief31-noscale-control-10-3','brief31-noscale-control-100-3','brief31-noscale-control-4-3','brief31-scale-boundary-1-0','brief31-scale-boundary-1-3','brief31-scale-matrix-1-0','brief31-scale-matrix-1-3','brief31-scale-matrix-10-3','brief31-scale-matrix-100-3','brief31-scale-matrix-4-3','issue70')
    $scriptHash = (Get-FileHash -Algorithm SHA256 $PSCommandPath).Hash.ToLowerInvariant()
    $commonPath = Join-Path $PSScriptRoot 'common.ps1'
    $commonHash = (Get-FileHash -Algorithm SHA256 $commonPath).Hash.ToLowerInvariant()
    Copy-Item -LiteralPath $PSCommandPath -Destination (Join-Path $out 'capture.ps1')
    Copy-Item -LiteralPath $commonPath -Destination (Join-Path $out 'common.ps1')
    $started = [DateTime]::UtcNow.ToString('o')
    $app = New-Visio
    $run = @{run_id=$runId;names=$names;script_sha256=$scriptHash;common_sha256=$commonHash;started_utc=$started;visio_version=[string]$app.Version;visio_build=[string]$app.Build;user=$env:USERNAME;machine=$env:COMPUTERNAME;completed=$false}
    $rows = @()
    foreach ($name in $names) {
        $path = Join-Path $root ($name + '.vsdx')
        Begin-File $path
        $dir = Join-Path $out $name
        New-Item -ItemType Directory -Force $dir | Out-Null
        Copy-Item -LiteralPath $path -Destination (Join-Path $dir 'input.vsdx')
        $local = Join-Path $env:TEMP ($runId + '-' + $name + '.vsdx')
        $inputHash = (Get-FileHash -Algorithm SHA256 (Join-Path $dir 'input.vsdx')).Hash.ToLowerInvariant()
        Copy-Item -LiteralPath (Join-Path $dir 'input.vsdx') -Destination $local -Force
        if ((Get-FileHash -Algorithm SHA256 $local).Hash.ToLowerInvariant() -ne $inputHash) { throw 'local input copy hash mismatch' }
        $doc = $null
        $status = @{name=$name;run_id=$runId;script_sha256=$scriptHash;common_sha256=$commonHash;input_sha256=$inputHash;opened=$false;saved=$false;error=$null;artifacts=@()}
        try {
            $doc = $app.Documents.OpenEx($local,130)
            $status.opened = $true
            Dump-Document $doc (Join-Path $dir 'observed.sheet.tsv')
            Dump-Connects $doc (Join-Path $dir 'connects.tsv')
            $doc.SaveAs((Join-Path $dir 'saved.vsdx')) | Out-Null
            $status.saved = $true
            Log "ACCEPT $name opened=True saved=True run=$runId"
        } catch { $status.error = "$_"; Log "FAIL $name $_" }
        finally {
            if ($doc) { Close-Document $doc }
            Remove-Item -LiteralPath $local -ErrorAction SilentlyContinue
            foreach ($file in @('observed.sheet.tsv','connects.tsv','saved.vsdx')) {
                $artifact = Join-Path $dir $file
                if (Test-Path -LiteralPath $artifact) {
                    if ($file.EndsWith('.tsv')) {
                        $text = [IO.File]::ReadAllText($artifact)
                        [IO.File]::WriteAllText($artifact,$text,[Text.UTF8Encoding]::new($false))
                    }
                    $status.artifacts += @{file=$file;sha256=(Get-FileHash -Algorithm SHA256 $artifact).Hash.ToLowerInvariant()}
                }
            }
            $rows += $status
            End-File
        }
    }
    $run.finished_utc = [DateTime]::UtcNow.ToString('o')
    $run.completed = ($rows.Count -eq 14 -and @($rows | Where-Object { -not $_.opened -or -not $_.saved -or $_.error }).Count -eq 0)
    [IO.File]::WriteAllText((Join-Path $out 'status.json'),($rows | ConvertTo-Json -Depth 6),[Text.UTF8Encoding]::new($false))
    [IO.File]::WriteAllText((Join-Path $out 'run.json'),($run | ConvertTo-Json -Depth 6),[Text.UTF8Encoding]::new($false))
    Log "SUMMARY completed=$($run.completed) run=$runId"
} catch { "FAIL $($_.Exception.Message) $($_.ScriptStackTrace)" | Out-File $log -Append }
finally {
    if ($app) { try { $app.Quit() } catch {} }
    'DONE' | Out-File (Join-Path $out 'DONE')
}
