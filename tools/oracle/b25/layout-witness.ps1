# Contributor-original topology and line-weight observations, MIT OR Apache-2.0.
# https://learn.microsoft.com/en-us/office/client-developer/visio/shaperoutestyle-cell-shape-layout-section
# https://learn.microsoft.com/en-us/office/client-developer/visio/lineweight-cell-line-format-section
$out = '\\Mac\Home\Projects\diagramcraft\corpus\_vm\out\layout-witness'
New-Item -ItemType Directory -Force $out | Out-Null
$log = Join-Path $out 'probe-log.txt'
"START $(Get-Date -Format s)" | Out-File $log
$app=$null; $doc=$null; $stencil=$null
try {
 . (Join-Path $PSScriptRoot 'common.ps1')
 $app=New-Visio
 $stencil=$app.Documents.OpenEx('\\Mac\Home\Projects\diagramcraft\corpus\_state\brief25\accept-inputs\b25-network.vssx',66)
 $names=@('Sky Internet','Alder Firewall','Birch Router','Cedar Core','Elm Access','Fir Access','Grove Server','Hazel Wi-Fi','Iris PC','Juniper PC')
 $masters=@('network.cloud','network.firewall','network.router','network.switch.core','network.switch.access','network.switch.access','compute.server','network.wireless_ap','compute.desktop','compute.desktop')
 $xs=@(6.375,6.375,6.375,6.375,5.5,10.75,2,12.5,5.5,9)
 $ys=@(10.5,8.75,7,5.25,3.5,3.5,3.5,1.75,1.75,1.75)
 $links=@(@(0,1),@(1,2),@(2,3),@(3,6),@(3,4),@(3,5),@(4,8),@(5,9),@(5,7))
 foreach($style in @(3,7,17)) {
  $doc=$app.Documents.Add('');$page=$doc.Pages.Item(1)
  Cell $page.PageSheet 'PageWidth' '14 in';Cell $page.PageSheet 'PageHeight' '12 in'
  $shapes=@();$connectors=@()
  for($i=0;$i -lt 10;$i++) {
   $s=$page.Drop($stencil.Masters.ItemU($masters[$i]),$xs[$i],$ys[$i]);$s.Text=$names[$i];$shapes+=,$s
  }
  foreach($pair in $links) {
   $c=$page.Drop($app.ConnectorToolDataObject,0,0)
   Cell $c 'ShapeRouteStyle' ([string]$style)
   $c.CellsU('BeginX').GlueTo($shapes[$pair[0]].CellsU('Connections.X9'))
   $c.CellsU('EndX').GlueTo($shapes[$pair[1]].CellsU('Connections.X10'))
   $connectors+=,$c
  }
  Route-Connectors $page
  $base=Join-Path $out "office-style-$style"
  $doc.SaveAs("$base.vsdx");Export-Png $app $page "$base.png"
  $w=Writer "$base.routes.tsv"
  try {
   $w.WriteLine("id`tname`tformula`tvalue")
   foreach($c in $connectors) {
    foreach($n in @('BeginX','BeginY','EndX','EndY','Width','Height','PinX','PinY','LocPinX','LocPinY','ShapeRouteStyle')) {
     $v=$c.CellsU($n);$w.WriteLine("$($c.ID)`t$n`t$($v.FormulaU)`t$($v.ResultIU.ToString('R',[Globalization.CultureInfo]::InvariantCulture))")
    }
    for($r=1;$r -lt $c.RowCount(10);$r++) {foreach($col in @('X','Y')) {
     $n="Geometry1.$col$r";$v=$c.CellsU($n);$w.WriteLine("$($c.ID)`t$n`t$($v.FormulaU)`t$($v.ResultIU.ToString('R',[Globalization.CultureInfo]::InvariantCulture))")
    }}
   }
  } finally {$w.Dispose()}
  Dump-Connects $doc "$base.connects.tsv"
  Close-Document $doc;$doc=$null
  Log "OFFICE OK style=$style"
 }
 $doc=$app.Documents.Add('');$page=$doc.Pages.Item(1)
 $a=$page.DrawRectangle(1,4,2.5,5.25);$b=$page.DrawRectangle(5,1,6.5,2.25)
 foreach($s in @($a,$b)) {$s.AddSection(7)|Out-Null;for($i=0;$i -lt 2;$i++) {$s.AddRow(7,$i,153)|Out-Null;Cell $s "Connections.X$($i+1)" "Width*$(($i+1)/4)";Cell $s "Connections.Y$($i+1)" $(if($s.ID -eq $a.ID){'0'}else{'Height'})}}
 for($i=1;$i -le 2;$i++) {
  $c=$page.Drop($app.ConnectorToolDataObject,0,0);Cell $c 'ShapeRouteStyle' '2'
  $c.CellsU('BeginX').GlueTo($a.CellsU("Connections.X$i"));$c.CellsU('EndX').GlueTo($b.CellsU("Connections.X$i"))
 }
 Cell $a 'LineWeight' '1.5 pt'
 $v=$a.CellsU('LineWeight');"formula`tvalue`tunits`n$($v.FormulaU)`t$($v.ResultIU.ToString('R',[Globalization.CultureInfo]::InvariantCulture))`t$($v.Units)"|Out-File (Join-Path $out 'line-weight.tsv')
 Route-Connectors $page;$doc.SaveAs((Join-Path $out 'straight-and-weight.vsdx'))
 Dump-Document $doc (Join-Path $out 'straight-and-weight.sheet.tsv')
 Dump-Connects $doc (Join-Path $out 'straight-and-weight.connects.tsv')
 Export-Png $app $page (Join-Path $out 'straight-and-weight.png')
 Log 'SUMMARY office=3 straight=2 weight=1 errors=0'
} catch {"FAIL $($_.Exception.Message) $($_.ScriptStackTrace)"|Out-File $log -Append}
finally {
 if($doc){try{Close-Document $doc}catch{}}
 if($stencil){try{Close-Document $stencil}catch{}}
 if($app){try{$app.Quit()}catch{}}
 'DONE'|Out-File (Join-Path $out 'DONE')
}
