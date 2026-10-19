# Read-only vendor diagnosis authorized by Brief25 addendum1. Outputs stay local.
# https://learn.microsoft.com/en-us/office/vba/api/visio.page.drop
# https://learn.microsoft.com/en-us/office/vba/api/visio.cell.resultiu
# https://learn.microsoft.com/en-us/office/vba/api/visio.shape.mastershape
$out='\\Mac\Home\Projects\diagramcraft\corpus\_vm\out\dell-instance-witness'
New-Item -ItemType Directory -Force $out|Out-Null
$log=Join-Path $out 'probe-log.txt';"START $(Get-Date -Format s)"|Out-File $log
$app=$null;$doc=$null;$stencil=$null
try {
 . (Join-Path $PSScriptRoot 'common.ps1')
 $app=New-Visio
 $app.Settings.SetRasterExportResolution(3,384,384,0)
 $app.Settings.SetRasterExportSize(2,0,0,0)
 $vendorRoot='\\Mac\Home\Projects\craft-apps\scope\visio\stencil-survey\extracted\dell\Dell-Networking'
 foreach($case in @(@('Z9264F-ON','Dell-Networking-FixedPort.vssx',20),@('VEP4600','Dell-Networking-AccessPlatforms.vssx',1),@('Z9100-ON','Dell-Networking-FixedPort.vssx',150))) {
  $path=Join-Path $vendorRoot $case[1];$before=(Get-FileHash -Algorithm SHA256 $path).Hash
  $stencil=$app.Documents.OpenEx($path,2+8+128+256)
  $master=$stencil.Masters.ItemFromID([int]$case[2])
  $doc=$app.Documents.Add('');$page=$doc.Pages.Item(1)
  $instance=$page.Drop($master,10,5)
  $base=Join-Path $out $case[0]
  $doc.SaveAs("$base.vsdx")
  # The full dump includes every geometry, layer, User, Prop and Actions cell,
  # rather than guessing which formula dependencies determine visibility.
  Dump-Document $doc "$base.cells.tsv"
  $map=Writer "$base.shapes.tsv";$map.WriteLine("shape`tmaster_shape`tname")
  function Map-Shapes($shapes,$writer) {
   foreach($shape in $shapes) {
    $masterId='';try{$masterId=$shape.MasterShape.ID}catch{}
    $writer.WriteLine("$($shape.ID)`t$masterId`t$(Tsv $shape.NameU)")
    if($shape.Type -eq 2){Map-Shapes $shape.Shapes $writer}
   }
  }
  try{Map-Shapes $page.Shapes $map}finally{$map.Dispose()}
  $instance.Export("$base.instance.png");$master.Export("$base.master.png")
  Close-Document $doc;$doc=$null;Close-Document $stencil;$stencil=$null
  if($before -ne (Get-FileHash -Algorithm SHA256 $path).Hash){throw "Vendor input changed: $path"}
  Log "VENDOR UNCHANGED $($case[0]) $before"
 }
 Log "SUMMARY vendor=3 cells=$script:CellCount errors=$script:CellErrors"
 'OK'|Out-File (Join-Path $out 'SUCCESS')
} catch {"FAIL $($_.Exception.Message) $($_.ScriptStackTrace)"|Out-File $log -Append}
finally {if($doc){try{Close-Document $doc}catch{}};if($stencil){try{Close-Document $stencil}catch{}};if($app){try{$app.Quit()}catch{}};'DONE'|Out-File (Join-Path $out 'DONE')}
