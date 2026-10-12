$out = '\\Mac\Home\Projects\diagramcraft\corpus\_vm\out\dump-blank'
New-Item -ItemType Directory -Force $out | Out-Null
$log = Join-Path $out 'probe-log.txt'
"START $(Get-Date -Format s)" | Out-File $log
$app = $null; $doc = $null
try {
    . (Join-Path $PSScriptRoot 'common.ps1')
    $app = New-Visio
    $doc = $app.Documents.Add('')
    $page = $doc.Pages.Item(1)
    $doc.SaveAs((Join-Path $out 'blank.vsdx'))
    foreach ($entry in @(@('document', $doc.DocumentSheet), @('page', $page.PageSheet))) {
        $w = Writer (Join-Path $out "$($entry[0]).tsv")
        try { Sheet-Header $w; Dump-Sheet $entry[1] $w $entry[0] } finally { $w.Dispose() }
    }
    $names = @('Normal','None','Text Only','Guide','No Style','Connector')
    foreach ($style in $doc.Styles) { $names += $style.NameU }
    foreach ($name in ($names | Select-Object -Unique)) {
        try { $style = $doc.Styles.ItemU($name) } catch { Log "STYLE ABSENT $name $($_.Exception.Message)"; continue }
        $safe = $name -replace '[^a-zA-Z0-9_-]', '_'
        $w = Writer (Join-Path $out "style-$($style.ID)-$safe.tsv")
        try { Sheet-Header $w; Dump-Sheet $style $w 'style' -Style } finally { $w.Dispose() }
        Log "STYLE $name id=$($style.ID)"
    }
    $rect = $page.DrawRectangle(1, 1, 3, 2)
    $w = Writer (Join-Path $out 'rectangle.tsv')
    try { Sheet-Header $w; Dump-Sheet $rect $w 'page:1' } finally { $w.Dispose() }
    $doc.SaveAs((Join-Path $out 'blank-rect.vsdx'))
    Log "SUMMARY cells=$script:CellCount cell_errors=$script:CellErrors primary_cells=$($script:CellCount-$script:SupplementCells) supplemental_cells=$script:SupplementCells"
} catch { "FAIL $($_.Exception.Message) $($_.ScriptStackTrace)" | Out-File $log -Append
} finally {
    if ($doc) { try { $doc.Saved = $true; $doc.Close() } catch {} }
    if ($app) { try { $app.Quit() } catch { "QUIT FAIL $_" | Out-File $log -Append } }
    'DONE' | Out-File (Join-Path $out 'DONE')
}
