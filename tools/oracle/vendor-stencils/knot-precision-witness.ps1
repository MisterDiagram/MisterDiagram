# Contributor-original mixed numeric/constructor knot serialization witness.
# https://learn.microsoft.com/en-us/office/client-developer/visio/nurbs-function
# https://learn.microsoft.com/en-us/office/client-developer/visio/nurbsto-row-geometry-section
# https://learn.microsoft.com/en-us/office/vba/api/visio.cell.resultstru
$out='\\Mac\Home\Projects\diagramcraft\corpus\_vm\out\knot-precision-witness'
New-Item -ItemType Directory -Force $out | Out-Null
$log=Join-Path $out 'probe-log.txt'; "START $(Get-Date -Format s)" | Out-File $log
$app=$null; $doc=$null
try {
    . (Join-Path $PSScriptRoot 'common.ps1')
    Add-Type -AssemblyName System.IO.Compression
    Add-Type -AssemblyName System.IO.Compression.FileSystem
    $numbers=@('0.12345678901234567','0.7138452034029296','0.9876543210987654','12.345678901234567','12345.678901234567','0.000012345678901234567','1.2345678901234567E14','1.2345678901234567E-14')
    $app=New-Visio; $doc=$app.Documents.AddEx('',0,0,2)
    for ($i=0;$i -lt $numbers.Count;$i++) {
        $master=$doc.Masters.Add(); $master.NameU="Knot$i"
        $shape=$master.DrawRectangle(0,0,3,2); $shape.NameU="Knot$i"
        $shape.AddNamedRow(242,'Knot',0) | Out-Null
        $shape.CellsU('User.Knot').ResultIU=[double]::Parse($numbers[$i],$inv)
        Cell $shape 'FillPattern' '0'; Cell $shape 'LineWeight' '0.02 in'; Cell $shape 'LineColor' 'RGB(220,0,0)'
    }
    $base=Join-Path $out 'base.vssx';$doc.SaveAs($base);Close-Document $doc;$doc=$null
    $input=Join-Path $out 'input.vssx';Copy-Item $base $input
    $zip=[IO.Compression.ZipFile]::Open($input,[IO.Compression.ZipArchiveMode]::Update)
    try {
        foreach ($entry in @($zip.Entries | Where-Object {$_.FullName -match '^visio/masters/master[0-9]+\.xml$'})) {
            $part=$entry.FullName;$r=[IO.StreamReader]::new($entry.Open());$xml=[xml]$r.ReadToEnd();$r.Dispose()
            $uri='http://schemas.microsoft.com/office/visio/2012/main';$ns=[Xml.XmlNamespaceManager]::new($xml.NameTable);$ns.AddNamespace('v',$uri)
            foreach ($shape in $xml.SelectNodes('//v:Shape',$ns)) {
                $name=$shape.GetAttribute('NameU');if ($name -notmatch '^Knot([0-7])$') {continue}
                $q=$numbers[[int]$Matches[1]];$last=([double]::Parse($q,$inv)*2).ToString('R',$inv)
                foreach ($old in @($shape.SelectNodes('v:Section[@N="Geometry"]',$ns))) {$shape.RemoveChild($old)|Out-Null}
                $geom=$xml.CreateElement('Section',$uri);$geom.SetAttribute('N','Geometry');$geom.SetAttribute('IX','0')
                function Add-Cell($parent,[string]$n,[string]$v,[string]$f) {
                    $c=$xml.CreateElement('Cell',$uri);$c.SetAttribute('N',$n);$c.SetAttribute('V',$v)
                    if ($f) {$c.SetAttribute('F',$f)};$parent.AppendChild($c)|Out-Null
                }
                Add-Cell $geom 'NoFill' '1' '';Add-Cell $geom 'NoLine' '0' '';Add-Cell $geom 'NoShow' '0' ''
                $move=$xml.CreateElement('Row',$uri);$move.SetAttribute('IX','1');$move.SetAttribute('T','MoveTo')
                Add-Cell $move 'X' '0' '';Add-Cell $move 'Y' '0' '';$geom.AppendChild($move)|Out-Null
                $curve=$xml.CreateElement('Row',$uri);$curve.SetAttribute('IX','2');$curve.SetAttribute('T','NURBSTo')
                Add-Cell $curve 'X' '3' '';Add-Cell $curve 'Y' '1' '';Add-Cell $curve 'A' $q 'User.Knot';Add-Cell $curve 'B' '1' '';Add-Cell $curve 'C' '0' '';Add-Cell $curve 'D' '1' ''
                Add-Cell $curve 'E' "NURBS($last,2,1,1,1,1,0,1,2,0,$q,1)" 'NURBS(User.Knot*2,2,1,1,1,1,0,1,2,0,User.Knot,1)'
                $geom.AppendChild($curve)|Out-Null;$shape.AppendChild($geom)|Out-Null
                Log "INPUT $name q=$q"
            }
            $entry.Delete();$replacement=$zip.CreateEntry($part);$w=[IO.StreamWriter]::new($replacement.Open(),[Text.UTF8Encoding]::new($false));$w.Write($xml.OuterXml);$w.Dispose()
        }
    } finally {$zip.Dispose()}
    function Snapshot($document,[string]$phase) {
        foreach ($master in $document.Masters) {
            $s=$master.Shapes.Item(1);$a=$s.CellsU('Geometry1.A2');$e=$s.CellsU('Geometry1.E2')
            Log "VALUE $phase $($master.NameU) A=$(([double]$a.ResultIU).ToString('R',$inv)) E=$($e.ResultStrU(0))"
            Log "FORMULA $phase $($master.NameU) A=$($a.FormulaU) E=$($e.FormulaU)"
        }
    }
    $doc=$app.Documents.OpenEx($input,0);Snapshot $doc 'open'
    $doc.Clean(511,8,0,8,$true,'',0)
    foreach ($master in $doc.Masters) {
        $name=$master.NameU;if ($name -notmatch '^Knot([0-7])$') {throw 'Unknown case'}
        $q=[double]::Parse($numbers[[int]$Matches[1]],$inv);$s=$master.Shapes.Item(1)
        $s.CellsU('User.Knot').ResultIU=$q/2;$s.CellsU('User.Knot').ResultIU=$q
    }
    Snapshot $doc 'forced'
    $saved=Join-Path $out 'saved.vssx';$doc.SaveAs($saved);Snapshot $doc 'save'
    Close-Document $doc;$doc=$app.Documents.OpenEx($saved,0);Snapshot $doc 'reopen'
    foreach ($master in $doc.Masters) {Export-Png $app $master (Join-Path $out "$($master.NameU).png")}
    'OK'|Out-File (Join-Path $out 'SUCCESS')
} catch {"FAIL $($_.Exception.Message) $($_.ScriptStackTrace)"|Out-File $log -Append
} finally {
    if ($doc) {try {Close-Document $doc} catch {"CLOSE FAIL $_"|Out-File $log -Append}}
    if ($app) {try {$app.Quit()} catch {"QUIT FAIL $_"|Out-File $log -Append}}
    'DONE'|Out-File (Join-Path $out 'DONE')
}
