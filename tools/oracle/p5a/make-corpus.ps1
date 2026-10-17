# P5a original witness only. Existing oracle outputs are never regenerated here.
# https://learn.microsoft.com/en-us/office/vba/api/visio.cell.glueto
# https://learn.microsoft.com/en-us/office/client-developer/visio/connection-points-section
$out = '\\Mac\Home\Projects\diagramcraft\corpus\_vm\out\make-corpus'
New-Item -ItemType Directory -Force (Join-Path $out 'route') | Out-Null
$log = Join-Path $out 'probe-log.txt'
"START $(Get-Date -Format s)" | Out-File $log
$app = $null; $doc = $null
try {
    . (Join-Path $PSScriptRoot 'common.ps1')
    $app = New-Visio
    Begin-File 'route/side-glue'
    $doc = $app.Documents.Add(''); $page = $doc.Pages.Item(1)
    $a = $page.DrawRectangle(1,6,2.5,7.5); $a.Text = 'A'
    $b = $page.DrawRectangle(5,6,6.5,7.5); $b.Text = 'B'
    foreach ($shape in @($a,$b)) {
        if (-not $shape.SectionExists(7,0)) { $shape.AddSection(7) | Out-Null }
        $shape.AddRow(7,0,153) | Out-Null
        Cell $shape 'Connections.Y1' 'Height*0.5'
    }
    Cell $a 'Connections.X1' 'Width'
    Cell $b 'Connections.X1' '0'
    $c = $page.Drop($app.ConnectorToolDataObject,0,0)
    $c.CellsU('BeginX').GlueTo($a.CellsU('Connections.X1'))
    $c.CellsU('EndX').GlueTo($b.CellsU('Connections.X1'))
    Route-Connectors $page
    $base = Join-Path $out 'route/side-glue'
    $doc.SaveAs("$base.vsdx")
    Export-Png $app $page "$base.png"
    $page.Export("$base.svg")
    $doc.ExportAsFixedFormat(1,"$base.pdf",1,0)
    Dump-Document $doc "$base.sheet.tsv"
    Dump-Connects $doc "$base.connects.tsv"
    Log "DRAWING OK route/side-glue"
    Log "SUMMARY made=1 failed=0 cells=$script:CellCount cell_errors=$script:CellErrors"
} catch { "FAIL $($_.Exception.Message) $($_.ScriptStackTrace)" | Out-File $log -Append
} finally {
    if ($doc) { try { Close-Document $doc } catch { "CLOSE FAIL $_" | Out-File $log -Append } }
    End-File
    if ($app) { try { $app.Quit() } catch { "QUIT FAIL $_" | Out-File $log -Append } }
    'DONE' | Out-File (Join-Path $out 'DONE')
}
