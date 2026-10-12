$out = '\\Mac\Home\Projects\diagramcraft\corpus\_vm\out\dump-shipped'
New-Item -ItemType Directory -Force $out | Out-Null
$log = Join-Path $out 'probe-log.txt'
"START $(Get-Date -Format s)" | Out-File $log -Encoding utf8 -Append
$app = $null
try {
    . (Join-Path $PSScriptRoot 'common.ps1')
    # The runner converts logs to UTF-8. Append in that encoding on subsequent nights.
    function Log([string]$message) { "$(Get-Date -Format s) $message" | Out-File $log -Encoding utf8 -Append }
    $app = New-Visio
    $office = Office-Folder
    $content = Join-Path $office 'Visio Content\1033'
    $copies = Join-Path (Split-Path $out) 'dump-shipped-files'
    New-Item -ItemType Directory -Force $copies | Out-Null
    $files = @(Get-ChildItem $content -Recurse -File | Where-Object { $_.Extension -in '.vssx','.vstx' } | Sort-Object FullName)
    # Copy everything before the slow COM traversal, even when a later dump times out.
    foreach ($file in $files) {
        $relative = '1033\' + $file.FullName.Substring($content.Length + 1)
        $copy = Join-Path $copies $relative
        New-Item -ItemType Directory -Force (Split-Path $copy) | Out-Null
        Copy-Item -LiteralPath $file.FullName -Destination $copy -Force
    }
    Log "COPIED files=$($files.Count)"
    $ok = 0; $failed = 0; $skipped = 0; $totalMasters = 0
    foreach ($file in $files) {
        $doc = $null; $w = $null; $masters = 0
        # Keep 1033 in relative paths, matching corpus/ms-shipped/*/*.vssx.
        $relative = '1033\' + $file.FullName.Substring($content.Length + 1)
        $target = Join-Path $out "$relative.tsv"
        if (Test-Path -LiteralPath $target) { $skipped++; Log "FILE SKIP $relative existing_tsv"; continue }
        $partial = "$target.partial"
        if (Test-Path -LiteralPath $partial) { Move-Item -LiteralPath $partial -Destination "$partial.$(Get-Date -Format yyyyMMddHHmmss)" }
        Begin-File $relative
        try {
            $doc = $app.Documents.OpenEx($file.FullName, 2)
            $w = Writer $partial
            Sheet-Header $w
            $errorsBefore = $script:CellErrors
            $masters = $doc.Masters.Count
            foreach ($master in $doc.Masters) { Dump-Shapes $master.Shapes $w "master:$($master.ID):$($master.NameU)" }
            $w.Dispose(); $w = $null
            if ($script:CellErrors -ne $errorsBefore) { throw 'cell errors: incomplete dump retained as .partial' }
            Move-Item -LiteralPath $partial -Destination $target
            $totalMasters += $masters
            $ok++
            Log "FILE OK $relative masters=$masters"
        } catch { $failed++; Log "FILE FAIL $relative masters=$masters error=$($_.Exception.Message)" }
        finally {
            if ($w) { $w.Dispose() }
            if ($doc) { try { Close-Document $doc } catch { Log "CLOSE FAIL $relative $_" } }
            End-File
        }
        if (($ok + $failed) % 25 -eq 0) { Log "PROGRESS files=$($ok+$failed)/$($files.Count) ok=$ok failed=$failed cells=$script:CellCount" }
    }
    Log "SUMMARY files=$($files.Count) dumped=$ok skipped=$skipped coverage=$($ok+$skipped) failed=$failed masters=$totalMasters cells=$script:CellCount cell_errors=$script:CellErrors primary_cells=$($script:CellCount-$script:SupplementCells) supplemental_cells=$script:SupplementCells"
} catch { "FAIL $($_.Exception.Message) $($_.ScriptStackTrace)" | Out-File $log -Encoding utf8 -Append
} finally {
    if ($app) { try { $app.Quit() } catch { "QUIT FAIL $_" | Out-File $log -Encoding utf8 -Append } }
    'DONE' | Out-File (Join-Path $out 'DONE')
}
