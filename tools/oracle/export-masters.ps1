# Private black-box master exports. The host runner performs the health probe first.
# https://learn.microsoft.com/en-us/office/vba/api/visio.master.export
$out = '\\Mac\Home\Projects\diagramcraft\corpus\_vm\out\export-masters'
New-Item -ItemType Directory -Force $out | Out-Null
$log = Join-Path $out 'probe-log.txt'
"START $(Get-Date -Format s)" | Out-File $log
$app = $null; $doc = $null; $writer = $null
try {
    . (Join-Path $PSScriptRoot 'common.ps1')
    $app = New-Visio
    Raster-96 $app
    $writer = Writer (Join-Path $out 'masters.tsv')
    $writer.WriteLine("stencil`tmaster_id`tmaster_name`tfile")
    $ok = 0; $failed = 0
    $tempRoot = Join-Path $env:TEMP 'diagramcraft-export-masters'
    New-Item -ItemType Directory -Force $tempRoot | Out-Null
    foreach ($path in (Get-Content (Join-Path $PSScriptRoot 'export-masters-list.txt') -Encoding UTF8)) {
        if ([string]::IsNullOrWhiteSpace($path)) { continue }
        $name = [IO.Path]::GetFileNameWithoutExtension($path)
        $local = Join-Path $tempRoot ([IO.Path]::GetFileName($path))
        Begin-File $path
        try {
            Copy-Item -LiteralPath $path -Destination $local -Force
            $doc = $app.Documents.OpenEx($local, 2)
            $directory = Join-Path $out $name
            New-Item -ItemType Directory -Force $directory | Out-Null
            foreach ($master in $doc.Masters) {
                try {
                    $file = "$name/$($master.ID).png"
                    $master.Export((Join-Path $directory "$($master.ID).png"))
                    $writer.WriteLine("$name`t$($master.ID)`t$(Tsv $master.NameU)`t$file")
                    $writer.Flush()
                    $ok++
                } catch { $failed++; Log "MASTER FAIL $name/$($master.ID) $($_.Exception.Message)" }
            }
            Log "STENCIL OK $name masters=$($doc.Masters.Count)"
        } catch { $failed++; Log "STENCIL FAIL $name $($_.Exception.Message)" }
        finally {
            if ($doc) { try { Close-Document $doc } catch { Log "CLOSE FAIL $_" }; $doc = $null }
            if (Test-Path -LiteralPath $local) { Remove-Item -LiteralPath $local -Force }
            End-File
        }
    }
    Log "SUMMARY exported=$ok failed=$failed"
} catch { "FAIL $($_.Exception.Message) $($_.ScriptStackTrace)" | Out-File $log -Append
} finally {
    if ($writer) { $writer.Dispose() }
    if ($app) { try { $app.Quit() } catch { "QUIT FAIL $_" | Out-File $log -Append } }
    'DONE' | Out-File (Join-Path $out 'DONE')
}
