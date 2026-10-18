# Read-only observation of three masters selected by the pre-observation classifier.
# https://learn.microsoft.com/en-us/office/vba/api/visio.documents.openex
# https://learn.microsoft.com/en-us/office/vba/api/visio.master.export
$out = '\\Mac\Home\Projects\diagramcraft\corpus\_vm\out\vendor-three-witness'
New-Item -ItemType Directory -Force $out | Out-Null
$log = Join-Path $out 'probe-log.txt'; "START $(Get-Date -Format s)" | Out-File $log
$app=$null; $doc=$null
try {
    . (Join-Path $PSScriptRoot 'common.ps1')
    $app=New-Visio
    $root='\\Mac\Home\Projects\craft-apps\scope\visio\stencil-survey\extracted\dell\Dell-Networking\'
    $cases=@(
        @('zero_angle','Dell-Networking-FixedPort.vssx',20,6,'Geometry1.C3','Geometry1.D3'),
        @('nonzero_angle','Dell-Networking-AccessPlatforms.vssx',1,6,'Geometry3.C3','Geometry3.D3'),
        @('theta','Dell-Networking-FixedPort.vssx',150,6,'Geometry1.C3','Geometry1.D3')
    )
    foreach ($case in $cases) {
        $label=$case[0]; $path=Join-Path $root $case[1]
        $before=(Get-FileHash -Algorithm SHA256 $path).Hash
        $doc=$app.Documents.OpenEx($path,2+8+128+256)
        $master=$doc.Masters.ItemFromID([int]$case[2])
        function Find-Shape($shapes,[int]$id) {
            foreach ($s in $shapes) {
                if ($s.ID -eq $id) { return $s }
                if ($s.Type -eq 2) { $found=Find-Shape $s.Shapes $id; if ($found) { return $found } }
            }
            return $null
        }
        $shape=Find-Shape $master.Shapes ([int]$case[3]); if (-not $shape) { throw 'Representative shape missing' }
        foreach ($cell in @('Width','Height',$case[4],$case[5])) {
            $c=$shape.CellsU($cell)
            Log "CELL $label master=$($master.ID) shape=$($shape.ID) $cell F=$($c.FormulaU) V=$(([double]$c.ResultIU).ToString('R',$inv))"
        }
        Export-Png $app $master (Join-Path $out "$label.png")
        Close-Document $doc; $doc=$null
        $after=(Get-FileHash -Algorithm SHA256 $path).Hash
        if ($before -ne $after) { throw "Vendor input changed: $path" }
        Log "UNCHANGED $label $before"
    }
    'OK' | Out-File (Join-Path $out 'SUCCESS')
} catch { "FAIL $($_.Exception.Message) $($_.ScriptStackTrace)" | Out-File $log -Append
} finally {
    if ($doc) { try { Close-Document $doc } catch { "CLOSE FAIL $_" | Out-File $log -Append } }
    if ($app) { try { $app.Quit() } catch { "QUIT FAIL $_" | Out-File $log -Append } }
    'DONE' | Out-File (Join-Path $out 'DONE')
}
