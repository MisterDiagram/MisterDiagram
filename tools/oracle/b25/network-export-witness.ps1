# Observe native library export coordinates without changing the accepted drawing.
$out='\\Mac\Home\Projects\diagramcraft\corpus\_vm\out\network-export-witness'
New-Item -ItemType Directory -Force $out|Out-Null
$log=Join-Path $out 'probe-log.txt';'START'|Out-File $log
$app=$null;$doc=$null
try {
 . (Join-Path $PSScriptRoot 'common.ps1')
 $app=New-Visio
 $doc=$app.Documents.OpenEx('\\Mac\Home\Projects\diagramcraft\corpus\_state\brief25\accept-inputs\b25-network-dropped.vsdx',2+8+128+256)
 $page=$doc.Pages.Item(1)
 $page.Export((Join-Path $out 'page.svg'))
 Export-Png $app $page (Join-Path $out 'page.png')
 $w=Writer (Join-Path $out 'bounds.tsv');$w.WriteLine("shape`tflags`tleft`tbottom`tright`ttop")
 try {foreach($s in $page.Shapes){foreach($flag in @(0,1,4)) {
  [double]$l=0;[double]$b=0;[double]$r=0;[double]$t=0
  $s.BoundingBox($flag,[ref]$l,[ref]$b,[ref]$r,[ref]$t)
  $w.WriteLine("$($s.ID)`t$flag`t$l`t$b`t$r`t$t")
 }}}finally{$w.Dispose()}
 Log 'OK'
}catch{$_|Out-File $log -Append}
finally{if($doc){try{Close-Document $doc}catch{}};if($app){try{$app.Quit()}catch{}};'DONE'|Out-File (Join-Path $out 'DONE')}
