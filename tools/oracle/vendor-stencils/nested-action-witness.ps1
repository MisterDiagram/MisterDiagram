# Contributor-original synthetic Action-cell observation, Brief 26.
# https://learn.microsoft.com/en-us/office/client-developer/visio/action-cell-actions-section
# https://learn.microsoft.com/en-us/office/vba/api/visio.document.clean
# https://learn.microsoft.com/en-us/office/vba/api/visio.master.drawrectangle
$out = '\\Mac\Home\Projects\diagramcraft\corpus\_vm\out\nested-action-witness'
New-Item -ItemType Directory -Force $out | Out-Null
$log = Join-Path $out 'probe-log.txt'
"START $(Get-Date -Format s)" | Out-File $log
$app = $null; $doc = $null
try {
    . (Join-Path $PSScriptRoot 'common.ps1')
    Add-Type -AssemblyName System.IO.Compression
    Add-Type -AssemblyName System.IO.Compression.FileSystem
    $app = New-Visio
    function Make-Cases($canvas) {
        foreach ($kind in @('GuardSetf','AddSetf','Setf')) {
            foreach ($stored in @(0,1)) {
                $carrier = $canvas.DrawRectangle(1,1,3,2)
                $carrier.NameU = "${kind}${stored}"
                $carrier.AddNamedRow(240,'Toggle',0) | Out-Null
                $carrier.CellsU('Actions.Toggle.Action').FormulaU = [string]$stored
                $carrier.AddNamedRow(242,'Flag',0) | Out-Null
                $carrier.CellsU('User.Flag').FormulaU = '0'
                $dep = $canvas.DrawRectangle(4,1,6,2)
                $dep.NameU = "${kind}${stored}Dependent"
                $id = $carrier.ID
                $dep.CellsU('Width').FormulaU = "GUARD(Sheet.$id!Width*Sheet.$id!Actions.Toggle.Action)"
            }
        }
    }
    function Patch-Input([string]$path) {
        $zip = [IO.Compression.ZipFile]::Open($path, [IO.Compression.ZipArchiveMode]::Update)
        try {
            foreach ($entry in @($zip.Entries | Where-Object { $_.FullName -match '^visio/(pages/page|masters/master)[0-9]+\.xml$' })) {
                $name = $entry.FullName
                $r = [IO.StreamReader]::new($entry.Open()); $xml = [xml]$r.ReadToEnd(); $r.Dispose()
                $ns = [Xml.XmlNamespaceManager]::new($xml.NameTable)
                $ns.AddNamespace('v','http://schemas.microsoft.com/office/visio/2012/main')
                foreach ($shape in $xml.SelectNodes('//v:Shape', $ns)) {
                    $shapeName = $shape.GetAttribute('NameU')
                    if ($shapeName -notmatch '^(GuardSetf|AddSetf|Setf)([01])$') { continue }
                    $kind = $Matches[1]; $stored = $Matches[2]
                    $cell = $shape.SelectSingleNode('v:Section[@N="Actions"]/v:Row[@N="Toggle"]/v:Cell[@N="Action"]', $ns)
                    if (-not $cell) { throw "Missing Action on $shapeName" }
                    $formula = switch ($kind) {
                        'GuardSetf' { 'GUARD(SETF(GETREF(User.Flag),NOT(User.Flag)))' }
                        'Setf' { 'SETF(GETREF(User.Flag),NOT(User.Flag))' }
                        'AddSetf' { '1+SETF(GETREF(User.Flag),NOT(User.Flag))' }
                    }
                    $cell.SetAttribute('F',$formula); $cell.SetAttribute('V',$stored); $cell.SetAttribute('U','BOOL')
                    Log "INPUT $name $shapeName Action F=$formula V=$stored U=BOOL"
                }
                $entry.Delete(); $new = $zip.CreateEntry($name)
                $w = [IO.StreamWriter]::new($new.Open(), [Text.UTF8Encoding]::new($false))
                $w.Write($xml.OuterXml); $w.Dispose()
            }
        } finally { $zip.Dispose() }
    }
    function Snapshot($document, [string]$label) {

        $canvases = @($document.Pages) + @($document.Masters)
        foreach ($canvas in $canvases) {
            foreach ($shape in $canvas.Shapes) {
                if ($shape.NameU -match '^(GuardSetf|AddSetf|Setf)[01]$') {
                    $a = $shape.CellsU('Actions.Toggle.Action')
                    Log "VALUE $label $($shape.NameU) Action=$($a.ResultIU) Formula=$($a.FormulaU) Flag=$($shape.CellsU('User.Flag').ResultIU) Width=$($shape.CellsU('Width').ResultIU)"
                } elseif ($shape.NameU -match 'Dependent$') {
                    Log "DEPENDENT $label $($shape.NameU) Width=$($shape.CellsU('Width').ResultIU) Formula=$($shape.CellsU('Width').FormulaU)"
                }
            }
        }
    }
    foreach ($kind in @('drawing','stencil')) {
        if ($kind -eq 'drawing') {
            $doc = $app.Documents.Add(''); Make-Cases $doc.Pages.Item(1); $extension = 'vsdx'
        } else {
            $doc = $app.Documents.AddEx('',0,0,2)
            $master = $doc.Masters.Add(); $master.NameU = 'ActionWitness'
            Make-Cases $master; $extension = 'vssx'
        }
        $base = Join-Path $out "$kind-base.$extension"
        $doc.SaveAs($base); Close-Document $doc; $doc = $null
        $input = Join-Path $out "$kind-input.$extension"
        Copy-Item $base $input
        Patch-Input $input
        $doc = $app.Documents.OpenEx($input,0)
        Snapshot $doc "$kind-open"
        # All document objects, only stale-result detection and repair, no alerts.
        # ActiveDocument is explicit so the observed recalculation target is logged.
        Log "RECALC ActiveDocument=$($app.ActiveDocument.Name) target=$($doc.Name)"
        if ($app.ActiveDocument.ID -ne $doc.ID) { throw 'Wrong active document' }
        $app.ActiveDocument.Clean(511,8,0,8,$true,'',0)
        Snapshot $doc "$kind-recalc"
        foreach ($canvas in (@($doc.Pages) + @($doc.Masters))) {
            foreach ($shape in $canvas.Shapes) {
                if ($shape.NameU -match '^(GuardSetf|AddSetf|Setf)[01]$') {
                    $shape.CellsU('Width').FormulaU = '3'
                }
            }
        }
        Snapshot $doc "$kind-edit"
        $saved = Join-Path $out "$kind-saved.$extension"
        $doc.SaveAs($saved)
        Snapshot $doc "$kind-save"
        Close-Document $doc; $doc = $app.Documents.OpenEx($saved,0)
        Snapshot $doc "$kind-reopen"
        Close-Document $doc; $doc = $null
        Log "WITNESS OK $kind"
    }
    Log "SUMMARY files=2 cell_errors=$script:CellErrors"
    'OK' | Out-File (Join-Path $out 'SUCCESS')
} catch { "FAIL $($_.Exception.Message) $($_.ScriptStackTrace)" | Out-File $log -Append
} finally {
    if ($doc) { try { Close-Document $doc } catch { "CLOSE FAIL $_" | Out-File $log -Append } }
    if ($app) { try { $app.Quit() } catch { "QUIT FAIL $_" | Out-File $log -Append } }
    'DONE' | Out-File (Join-Path $out 'DONE')
}
