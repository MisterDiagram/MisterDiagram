# Original picture-cell experiment; vendor observations remain local and read-only.
# https://learn.microsoft.com/en-us/office/client-developer/visio/imgwidth-cell-foreign-image-info-section
# https://learn.microsoft.com/en-us/office/client-developer/visio/imgoffsetx-cell-foreign-image-info-section
$out='\\Mac\Home\Projects\diagramcraft\corpus\_vm\out\picture-witness'
New-Item -ItemType Directory -Force $out|Out-Null
$log=Join-Path $out 'probe-log.txt';"START $(Get-Date -Format s)"|Out-File $log
$app=$null;$doc=$null
try {
 . (Join-Path $PSScriptRoot 'common.ps1')
 $app=New-Visio
 function Export-Large($object,$path) {
  $app.Settings.SetRasterExportResolution(3,384,384,0)
  $app.Settings.SetRasterExportSize(2,0,0,0)
  $object.Export($path)
 }
 $inputRoot='\\Mac\Home\Projects\diagramcraft\corpus\_state\brief25\picture-inputs'
 $cases=@(@('zero',0,2,0,0),@('zero-height',3,0,0,0),@('normal',3,2,0,0),@('small',1.5,1,0,0),@('large',6,4,0,0),@('negative-x',3,2,-0.5,0),@('positive-x',3,2,0.5,0),@('negative-y',3,2,0,-0.5),@('positive-y',3,2,0,0.5),@('offset-small',1.5,1,0.5,0.5),@('offset-large',6,4,-1,-1))
 $writer=Writer (Join-Path $out 'picture-cells.tsv');$writer.WriteLine("format`tcase`tshape`tname`tformula`tvalue")
 try {
  foreach($format in @('png','emf')) {foreach($case in $cases) {
   $doc=$app.Documents.Add('');$page=$doc.Pages.Item(1)
   $frame=$page.DrawRectangle(0.75,0.75,4.25,3.25);Cell $frame 'FillPattern' '0';Cell $frame 'LineWeight' '0.015625 in';Cell $frame 'LineColor' 'RGB(0,0,0)'
   $picture=$page.Import((Join-Path $inputRoot "panels.$format"))
   Cell $picture 'Width' '3 in';Cell $picture 'Height' '2 in';Cell $picture 'PinX' '2.5 in';Cell $picture 'PinY' '2 in'
   Cell $picture 'ImgWidth' "$($case[1]) in";Cell $picture 'ImgHeight' "$($case[2]) in"
   Cell $picture 'ImgOffsetX' "$($case[3]) in";Cell $picture 'ImgOffsetY' "$($case[4]) in"
   $base=Join-Path $out "$format-$($case[0])"
   foreach($n in @('Width','Height','PinX','PinY','ImgWidth','ImgHeight','ImgOffsetX','ImgOffsetY')) {$c=$picture.CellsU($n);$writer.WriteLine("$format`t$($case[0])`t$($picture.ID)`t$n`t$($c.FormulaU)`t$($c.ResultIU.ToString('R',[Globalization.CultureInfo]::InvariantCulture))")}
   $doc.SaveAs("$base.vsdx");Export-Large $page "$base.png"
   Close-Document $doc;$doc=$null;Log "PICTURE OK $format-$($case[0])"
  }}
 } finally {$writer.Dispose()}
 $vendorRoot='\\Mac\Home\Projects\craft-apps\scope\visio\stencil-survey\extracted\dell\Dell-Networking'
 foreach($case in @(@('Z9264F-ON','Dell-Networking-FixedPort.vssx',20),@('VEP4600','Dell-Networking-AccessPlatforms.vssx',1),@('Z9100-ON','Dell-Networking-FixedPort.vssx',150))) {
  $path=Join-Path $vendorRoot $case[1];$before=(Get-FileHash -Algorithm SHA256 $path).Hash
  $doc=$app.Documents.OpenEx($path,2+8+128+256);$master=$doc.Masters.ItemFromID([int]$case[2])
  Export-Large $master (Join-Path $out "$($case[0]).png")
  $w=Writer (Join-Path $out "$($case[0]).cells.tsv");$w.WriteLine("shape`tname`tformula`tvalue")
  function Observe-Shapes($shapes,$writer) {
   foreach($shape in $shapes) {
    foreach($n in @('Width','Height','PinX','PinY','LocPinX','LocPinY','ImgWidth','ImgHeight','ImgOffsetX','ImgOffsetY','Geometry1.NoShow')) {
     if($shape.CellExistsU($n,0)) {$c=$shape.CellsU($n);$writer.WriteLine("$($shape.ID)`t$n`t$($c.FormulaU)`t$($c.ResultIU.ToString('R',[Globalization.CultureInfo]::InvariantCulture))")}
    }
    if($shape.Type -eq 2){Observe-Shapes $shape.Shapes $writer}
   }
  }
  try{Observe-Shapes $master.Shapes $w}finally{$w.Dispose()}
  Close-Document $doc;$doc=$null
  if($before -ne (Get-FileHash -Algorithm SHA256 $path).Hash){throw "Vendor input changed: $path"}
  Log "VENDOR UNCHANGED $($case[0]) $before"
 }
 'OK'|Out-File (Join-Path $out 'SUCCESS')
 Log 'SUMMARY synthetic=22 vendor=3'
} catch {"FAIL $($_.Exception.Message) $($_.ScriptStackTrace)"|Out-File $log -Append}
finally {if($doc){try{Close-Document $doc}catch{}};if($app){try{$app.Quit()}catch{}};'DONE'|Out-File (Join-Path $out 'DONE')}
