# Shared helpers for the named P0 oracle scripts. No standalone guest entry point.
# COM traversal follows scope/visio/tools/probe.ps1.
$ErrorActionPreference = 'Stop'
$script:CellErrors = 0
$script:CellCount = 0
$script:SupplementCells = 0
$inv = [Globalization.CultureInfo]::InvariantCulture
function Log([string]$message) { "$(Get-Date -Format s) $message" | Out-File $log -Append }
function Tsv($value) {
    ([string]$value).Replace('\', '\\').Replace("`t", '\t').Replace("`r", '\r').Replace("`n", '\n')
}
function Writer([string]$path) {
    New-Item -ItemType Directory -Force (Split-Path $path) | Out-Null
    [IO.StreamWriter]::new($path, $false, [Text.Encoding]::Unicode, 65536)
}
function Sheet-Header($w) { $w.WriteLine("shape_id`tshape_name`tsection`trow`tcell`tname`tformula`tresult_iu`tunits") }
# https://learn.microsoft.com/en-us/office/vba/api/visio.shape.sectionexists
# https://learn.microsoft.com/en-us/office/vba/api/visio.shape.rowscellcount
# https://learn.microsoft.com/en-us/office/vba/api/visio.cell.resultiu
function Dump-Cell($sheet, $w, $id, [string]$name, [int]$section, [int]$row, [int]$column) {
$cellName = ''; $formula = ''; $result = ''; $units = ''
try {
    $cell = $sheet.CellsSRC($section, $row, $column)
    $cellName = $cell.Name
    $formula = $cell.FormulaU
    $units = $cell.Units
    $result = ([double]$cell.ResultIU).ToString('R', $inv)
} catch {
    $script:CellErrors++
    if ($script:CellErrors -le 25) { Log "CELL FAIL $name SRC=$section,$row,$column $($_.Exception.Message)" }
}
$w.WriteLine((@($id, (Tsv $name), $section, $row, $column, (Tsv $cellName), (Tsv $formula), $result, $units) -join "`t"))
$script:CellCount++
}
function Dump-Sheet($sheet, $w, [string]$scope, [switch]$Style) {
    $seen = @{}
    $id = $sheet.ID
    $name = "$scope/$($sheet.NameU)"
    for ($section = 1; $section -le 250; $section++) {
        if ($Style) {
            # Style lacks Shape.SectionExists/RowCount/RowsCellCount. Use its documented Section/Row objects.
            # https://learn.microsoft.com/en-us/office/vba/api/visio.style.section
            try { $styleSection = $sheet.Section($section); $rows = $styleSection.Count } catch { continue }
        } else {
            if (-not $sheet.SectionExists($section, 0)) { continue }
            $rows = $sheet.RowCount($section)
        }
        for ($row = 0; $row -lt $rows; $row++) {
            if ($Style) { try { $cells = $styleSection.Row($row).Count } catch { continue } }
            else { $cells = $sheet.RowsCellCount($section, $row) }
            for ($column = 0; $column -lt $cells; $column++) {
                Dump-Cell $sheet $w $id $name $section $row $column
                $seen["$section,$row,$column"] = $true
            }
        }
    }
    # RowCount is a local-row count for fixed sections, not a logical index bound.
    # Preserve the probe loop above; add missing documented Object rows without duplicates.
    # https://learn.microsoft.com/en-us/office/vba/api/visio.shape.rowcount
    # https://learn.microsoft.com/en-us/office/vba/api/visio.visrowindices
    # https://learn.microsoft.com/en-us/office/vba/api/visio.style.cellssrcexists
    foreach ($row in @(1,2,3,4,5,6,8,9,10,11,12,14,15,16,17,18,20,21,22,23,24,25,26,27,28,29,30,31,32)) {
        if (-not $sheet.CellsSRCExists(1,$row,0,0)) { continue }
        if ($Style) {
            # An inherited style row may exist by SRC without a local Row object.
            $cells = 0
            while ($cells -lt 32767 -and $sheet.CellsSRCExists(1,$row,$cells,0)) { $cells++ }
        } else { $cells = $sheet.RowsCellCount(1,$row) }
        for ($column = 0; $column -lt $cells; $column++) {
            if (-not $seen.ContainsKey("1,$row,$column") -and $sheet.CellsSRCExists(1,$row,$column,0)) {
                Dump-Cell $sheet $w $id $name 1 $row $column
                $seen["1,$row,$column"] = $true
                $script:SupplementCells++
            }
        }
    }

}
function Dump-Shapes($shapes, $w, [string]$scope) {
    foreach ($shape in $shapes) {
        Dump-Sheet $shape $w $scope
        if ($shape.Type -eq 2) { Dump-Shapes $shape.Shapes $w "$scope/$($shape.NameU)" }
    }
}
function Dump-Document($doc, [string]$path) {
    $w = Writer $path
    try {
        Sheet-Header $w
        Dump-Sheet $doc.DocumentSheet $w 'document'
        foreach ($style in $doc.Styles) { Dump-Sheet $style $w 'style' -Style }
        foreach ($master in $doc.Masters) { Dump-Shapes $master.Shapes $w "master:$($master.ID):$($master.NameU)" }
        foreach ($page in $doc.Pages) {
            Dump-Sheet $page.PageSheet $w "pagesheet:$($page.ID):$($page.NameU)"
            Dump-Shapes $page.Shapes $w "page:$($page.ID):$($page.NameU)"
        }
    } finally { $w.Dispose() }
}
function Office-Folder {
    $exe = Get-ChildItem "$env:ProgramFiles\Microsoft Office\root\Office16\VISIO.EXE","${env:ProgramFiles(x86)}\Microsoft Office\root\Office16\VISIO.EXE" -ErrorAction SilentlyContinue | Select-Object -First 1
    if (-not $exe) { throw 'VISIO.EXE not found in Office16' }
    Log "VISIO.EXE fileversion=$($exe.VersionInfo.FileVersion)"
    $exe.DirectoryName
}
function New-Visio {
    $v = New-Object -ComObject Visio.Application
    $v.AlertResponse = 7
    $v.Visible = $false
    Log "Visio version=$($v.Version) build=$($v.Build) product=$($v.ProductName) user=$env:USERNAME"
    $v
}
# https://learn.microsoft.com/en-us/office/vba/api/visio.applicationsettings.setrasterexportresolution
# https://learn.microsoft.com/en-us/office/vba/api/visio.applicationsettings.setrasterexportsize
function Raster-96($app) {
    $app.Settings.SetRasterExportResolution(3, 96, 96, 0)
    $app.Settings.SetRasterExportSize(2, 0, 0, 0)
}
function Export-Png($app, $page, [string]$path) {
    Raster-96 $app
    $page.Export($path)
    # PNG IHDR is big-endian. Inspect bytes without loading the entire image.
    $f = [IO.File]::OpenRead($path)
    try {
        $b = New-Object byte[] 24
        if ($f.Read($b, 0, 24) -ne 24) { throw 'truncated PNG' }
        $width = [uint32]$b[16]*16777216 + [uint32]$b[17]*65536 + [uint32]$b[18]*256 + [uint32]$b[19]
        $height = [uint32]$b[20]*16777216 + [uint32]$b[21]*65536 + [uint32]$b[22]*256 + [uint32]$b[23]
        Log "PNG $path dpi=96 pixels=${width}x${height}"
    } finally { $f.Dispose() }
}
function Dump-Connects($doc, [string]$path) {
    $w = Writer $path
    try {
        $w.WriteLine("from_sheet`tfrom_cell`tto_sheet`tto_cell`tfrom_part`tto_part")
        foreach ($page in $doc.Pages) {
            foreach ($c in $page.Connects) {
                $w.WriteLine((@($c.FromSheet.ID, (Tsv $c.FromCell.Name), $c.ToSheet.ID, (Tsv $c.ToCell.Name), $c.FromPart, $c.ToPart) -join "`t"))
            }
        }
    } finally { $w.Dispose() }
}
function Close-Document($doc) { if ($doc) { $doc.Saved = $true; $doc.Close() } }
function Cell($shape, [string]$name, [string]$formula) { $shape.CellsU($name).FormulaU = $formula }
function Begin-File([string]$name) { $name | Out-File (Join-Path $out 'CURRENT'); Log "BEGIN $name" }
function End-File { Remove-Item (Join-Path $out 'CURRENT') -ErrorAction SilentlyContinue }

# https://learn.microsoft.com/en-us/office/vba/api/visio.selection.layout
# Restrict layout to connectors so the authored obstacle and endpoint positions stay fixed.
function Route-Connectors($page) {
    $selection = $page.CreateSelection(0,0,0)
    foreach ($shape in $page.Shapes) { if ($shape.OneD) { $selection.Select($shape,2) } }
    if ($selection.Count -gt 0) { $selection.Layout() }
}
