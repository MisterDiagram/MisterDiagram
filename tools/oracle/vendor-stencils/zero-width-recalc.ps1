# Contributor-original synthetic Action-scaled geometry observation, Brief 26c.
# https://learn.microsoft.com/en-us/office/client-developer/visio/ellipticalarcto-row-geometry-section
# https://learn.microsoft.com/en-us/office/client-developer/visio/about-functions
# https://learn.microsoft.com/en-us/office/client-developer/visio/action-cell-actions-section
$out = '\\Mac\Home\Projects\diagramcraft\corpus\_vm\out\zero-width-recalc'
New-Item -ItemType Directory -Force $out | Out-Null
$log = Join-Path $out 'probe-log.txt'
"START $(Get-Date -Format s)" | Out-File $log
$app = $null; $doc = $null; $picture = $null
try {
    . (Join-Path $PSScriptRoot 'common.ps1')
    Add-Type -AssemblyName System.IO.Compression
    Add-Type -AssemblyName System.IO.Compression.FileSystem
    $app = New-Visio
    $doc = $app.Documents.AddEx('',0,0,2)
    foreach ($name in @('RatioSide','RatioTop','TinyAngle','ThetaNear','ThetaCircle','RatioSideReference','RatioTopReference','TinyAngleReference','ThetaNearReference','ThetaCircleReference')) {
        $master = $doc.Masters.Add(); $master.NameU = $name
        $shape = $master.DrawRectangle(0,0,2,2); $shape.NameU = $name
        $shape.AddNamedRow(240,'Visible',0) | Out-Null
        Cell $shape 'Actions.Visible.Action' '1'
        Cell $shape 'Width' 'GUARD(2 in*Actions.Visible.Action)'
        Cell $shape 'Height' '2 in'
        Cell $shape 'LineColor' 'RGB(255,0,0)'
        Cell $shape 'LineWeight' '0.05 in'
        Cell $shape 'LinePattern' '1'
        Cell $shape 'FillPattern' '0'
    }
    $base = Join-Path $out 'base.vssx'
    $doc.SaveAs($base); Close-Document $doc; $doc = $null
    function Patch-Input([string]$path, [int]$visible) {
        $zip = [IO.Compression.ZipFile]::Open($path,[IO.Compression.ZipArchiveMode]::Update)
        $patched = 0
        try {
            foreach ($entry in @($zip.Entries | Where-Object { $_.FullName -match '^visio/masters/master[0-9]+\.xml$' })) {
                $part = $entry.FullName
                $r = [IO.StreamReader]::new($entry.Open()); $xml = [xml]$r.ReadToEnd(); $r.Dispose()
                $ns = [Xml.XmlNamespaceManager]::new($xml.NameTable)
                $uri = 'http://schemas.microsoft.com/office/visio/2012/main'; $ns.AddNamespace('v',$uri)
                foreach ($shape in $xml.SelectNodes('//v:Shape',$ns)) {
                    $name = $shape.GetAttribute('NameU')
                    if ($name -notin @('RatioSide','RatioTop','TinyAngle','ThetaNear','ThetaCircle','RatioSideReference','RatioTopReference','TinyAngleReference','ThetaNearReference','ThetaCircleReference')) { continue }
                    $patched++
                    $reference = $name.EndsWith('Reference'); $name = $name.Replace('Reference','')
                    $action = $shape.SelectSingleNode('v:Section[@N="Actions"]/v:Row[@N="Visible"]/v:Cell[@N="Action"]',$ns)
                    if (-not $action) { throw "Missing Action for $name" }
                    $action.SetAttribute('F','NOT("Actions.Visible.Action")')
                    $action.SetAttribute('V',[string]$visible); $action.SetAttribute('U','BOOL')
                    $width = 2*$visible
                    $shape.SelectSingleNode('v:Cell[@N="Width"]',$ns).SetAttribute('V',[string]$width)
                    foreach ($old in @($shape.SelectNodes('v:Section[@N="Geometry"]',$ns))) { $shape.RemoveChild($old) | Out-Null }
                    $geom = $xml.CreateElement('Section',$uri); $geom.SetAttribute('N','Geometry'); $geom.SetAttribute('IX','0')
                    function Add-Cell($parent,[string]$n,[string]$v,[string]$f) {
                        $c = $xml.CreateElement('Cell',$uri); $c.SetAttribute('N',$n); $c.SetAttribute('V',$v)
                        if ($f) { $c.SetAttribute('F',$f) }; $parent.AppendChild($c) | Out-Null
                    }
                    Add-Cell $geom 'NoFill' '1' ''; Add-Cell $geom 'NoLine' '0' ''; Add-Cell $geom 'NoShow' '0' ''
                    $move = $xml.CreateElement('Row',$uri); $move.SetAttribute('IX','1'); $move.SetAttribute('T','MoveTo')
                    $arc = $xml.CreateElement('Row',$uri); $arc.SetAttribute('IX','2'); $arc.SetAttribute('T','EllipticalArcTo')
                    if ($name -eq 'RatioTop') {
                        Add-Cell $move 'X' '0' '0'; Add-Cell $move 'Y' '1' 'Height*0.5'
                        Add-Cell $arc 'X' ([string]$width) 'Width'; Add-Cell $arc 'Y' '1' 'Height*0.5'
                        Add-Cell $arc 'A' ([string]($width*0.5)) 'Width*0.5'; Add-Cell $arc 'B' '2' 'Height'
                    } else {
                        Add-Cell $move 'X' ([string]($width*0.5)) 'Width*0.5'; Add-Cell $move 'Y' '0' '0'
                        Add-Cell $arc 'X' ([string]($width*0.5)) 'Width*0.5'; Add-Cell $arc 'Y' '2' 'Height'
                        Add-Cell $arc 'A' ([string]$width) 'Width'; Add-Cell $arc 'B' '1' 'Height*0.5'
                    }
                    $theta = switch ($name) {
                        'TinyAngle' { '-6.921655089364709E-15' }
                        'ThetaNear' { '_ELLIPSE_THETA(-1.5707963267949,1.0000202209969,19,2,Width,Height)' }
                        'ThetaCircle' { '_ELLIPSE_THETA(-1.5707963267949,1,19,2,Width,Height)' }
                        default { '0' }
                    }
                    $thetaCache = if ($name -eq 'TinyAngle') { '-6.921655089364709E-15' } else { '0' }
                    Add-Cell $arc 'C' $thetaCache $theta
                    $ratio = switch ($name) {
                        'ThetaNear' { '_ELLIPSE_ECC(-1.5707963267949,1.0000202209969,19,2,Width,Height,Geometry1.C2)' }
                        'ThetaCircle' { '_ELLIPSE_ECC(-1.5707963267949,1,19,2,Width,Height,Geometry1.C2)' }
                        default { 'Width/Height' }
                    }
                    $ratioCache = if ($name -like 'Theta*') { '1' } else { '0' }
                    Add-Cell $arc 'D' $ratioCache $ratio
                    $geom.AppendChild($move) | Out-Null
                    if ($reference) {
                        $control = $xml.CreateElement('Row',$uri); $control.SetAttribute('IX','2'); $control.SetAttribute('T','LineTo')
                        Add-Cell $control 'X' '0' '0'
                        $cy = if ($name -eq 'RatioTop') { 'Height' } else { 'Height*0.5' }
                        $cyCache = if ($name -eq 'RatioTop') { '2' } else { '1' }
                        Add-Cell $control 'Y' $cyCache $cy
                        $geom.AppendChild($control) | Out-Null
                        $arc.SetAttribute('IX','3'); $arc.SetAttribute('T','LineTo')
                        foreach ($extra in @($arc.SelectNodes('v:Cell[@N="A" or @N="B" or @N="C" or @N="D"]',$ns))) { $arc.RemoveChild($extra) | Out-Null }
                    }
                    $geom.AppendChild($arc) | Out-Null; $shape.AppendChild($geom) | Out-Null
                    Log "INPUT $part $name Action=$visible Width=$width C=$theta"
                }
                $entry.Delete(); $replacement = $zip.CreateEntry($part)
                $w = [IO.StreamWriter]::new($replacement.Open(),[Text.UTF8Encoding]::new($false)); $w.Write($xml.OuterXml); $w.Dispose()
            }
            if ($patched -ne 10) { throw "Expected ten patched masters, saw $patched" }
        } finally { $zip.Dispose() }
    }
    function Snapshot($document,[string]$label) {
        if ($document.Masters.Count -ne 10) { throw 'Wrong master count' }

        foreach ($master in $document.Masters) {
            foreach ($shape in $master.Shapes) {
                if ($master.NameU.EndsWith('Reference')) { continue }
                foreach ($name in @('Width','Height','Actions.Visible.Action','Geometry1.X1','Geometry1.Y1','Geometry1.X2','Geometry1.Y2','Geometry1.A2','Geometry1.B2','Geometry1.C2','Geometry1.D2')) {
                    $c = $shape.CellsU($name)
                    try { $result = ([double]$c.ResultIU).ToString('R',$inv) }
                    catch { $result = "READ_ERROR:$($_.Exception.Message)" }
                    Log "CELL $label $($master.NameU) $name Formula=$($c.FormulaU) Result=$result"
                }
            }
        }
    }
    foreach ($visible in @(0)) {
        $label = if ($visible) { 'positive' } else { 'zero' }
        $input = Join-Path $out "$label-input.vssx"; Copy-Item $base $input; Patch-Input $input $visible
        $doc = $app.Documents.OpenEx($input,0)
        Snapshot $doc "$label-open"
        $doc.Clean(511,8,0,8,$true,'',0)
        Snapshot $doc "$label-recalc"
        foreach ($master in $doc.Masters) {
            foreach ($shape in $master.Shapes) {
                Cell $shape 'Height' '3 in'
                Cell $shape 'Height' '2 in'
            }
        }
        Snapshot $doc "$label-forced"

        $saved = Join-Path $out "$label-saved.vssx"; $doc.SaveAs($saved)
        Snapshot $doc "$label-save"
        Close-Document $doc; $doc = $app.Documents.OpenEx($saved,0)
        Snapshot $doc "$label-reopen"
        foreach ($master in $doc.Masters) {
            if ($master.Shapes.Count -ne 1) { throw 'Wrong master shape count' }
            $source = $master.Shapes.Item(1)
            if ($source.CellsU('Actions.Visible.Action').ResultIU -ne $visible -or $source.CellsU('Width').ResultIU -ne 2*$visible) { throw 'Wrong Action-scaled master width' }
            $picture = $app.Documents.Add(''); $page = $picture.Pages.Item(1)
            Cell $page.PageSheet 'PageWidth' '4 in'; Cell $page.PageSheet 'PageHeight' '4 in'
            $control = $page.DrawRectangle(0.25,0.25,0.75,0.75)
            Cell $control 'LineColor' 'RGB(0,180,0)'; Cell $control 'LineWeight' '0.05 in'; Cell $control 'FillPattern' '0'
            $instance = $page.Drop($master,2,2); $instance.NameU = $master.NameU
            Log "DROP $label $($master.NameU) Width=$($instance.CellsU('Width').ResultIU) Height=$($instance.CellsU('Height').ResultIU)"
            if ($instance.CellsU('Width').ResultIU -ne 2*$visible) { throw 'Dropped master width changed' }
            $imageLabel = "$label-$($master.NameU)"

            Export-Png $app $page (Join-Path $out "$imageLabel.png")
            $picture.SaveAs((Join-Path $out "$imageLabel.vsdx"))
            Close-Document $picture; $picture = $null
        }
        Close-Document $doc; $doc = $null
    }
    Log "SUMMARY cell_errors=$script:CellErrors"
    'OK' | Out-File (Join-Path $out 'SUCCESS')
} catch { "FAIL $($_.Exception.Message) $($_.ScriptStackTrace)" | Out-File $log -Append
} finally {
    if ($picture) { try { Close-Document $picture } catch { "CLOSE FAIL $_" | Out-File $log -Append } }
    if ($doc) { try { Close-Document $doc } catch { "CLOSE FAIL $_" | Out-File $log -Append } }
    if ($app) { try { $app.Quit() } catch { "QUIT FAIL $_" | Out-File $log -Append } }
    'DONE' | Out-File (Join-Path $out 'DONE')
}
