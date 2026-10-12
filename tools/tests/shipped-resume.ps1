# Host-only fixture: never starts Visio or calls the VM.
$ErrorActionPreference = 'Stop'
$root = Split-Path (Split-Path $PSScriptRoot)
$fixture = Join-Path $root "target/shipped-resume-$PID"
try {
    $out = Join-Path $fixture 'out/dump-shipped'
    $office = Join-Path $fixture 'office'
    $content = Join-Path $office 'Visio Content/1033'
    New-Item -ItemType Directory -Force "$out/1033", $content | Out-Null
    'old log' | Set-Content "$out/probe-log.txt" -Encoding utf8
    'keep exact bytes' | Set-Content "$out/1033/ALREADY.VSSX.tsv" -Encoding utf8
    $before = (Get-FileHash "$out/1033/ALREADY.VSSX.tsv").Hash
    'one' | Set-Content "$content/ALREADY.VSSX"
    'two' | Set-Content "$content/NEW.VSTX"
    $script = Get-Content (Join-Path $root 'tools/oracle/dump-shipped.ps1') -Raw
    $script = $script.Replace("'\\Mac\Home\Projects\diagramcraft\corpus\_vm\out\dump-shipped'", "'$out'")
    $script | Set-Content (Join-Path $fixture 'dump-shipped.ps1')
    @'
$script:CellErrors=0; $script:CellCount=0; $script:SupplementCells=0
function Office-Folder { Join-Path $PSScriptRoot 'office' }
function New-Visio {
    $docs=[pscustomobject]@{}
    $docs | Add-Member ScriptMethod OpenEx {
        param($path,$flags)
        if ($path -like '*ALREADY*') { throw 'existing TSV should have been skipped' }
        [pscustomobject]@{ Masters=@([pscustomobject]@{ID=1;NameU='fixture';Shapes=@()}) }
    }
    $app=[pscustomobject]@{Documents=$docs}
    $app | Add-Member ScriptMethod Quit {}
    $app
}
function Writer($path) { [IO.StreamWriter]::new($path,$false,[Text.Encoding]::Unicode) }
function Sheet-Header($w) { $w.WriteLine('fixture header') }
function Dump-Shapes($shapes,$w,$name) { $w.WriteLine('fixture cell'); $script:CellCount++ }
function Close-Document($doc) {}
function Begin-File($name) {}
function End-File {}
'@ | Set-Content (Join-Path $fixture 'common.ps1')
    & (Join-Path $fixture 'dump-shipped.ps1')
    if ((Get-FileHash "$out/1033/ALREADY.VSSX.tsv").Hash -ne $before) { throw 'existing TSV changed' }
    if (-not (Test-Path "$out/1033/NEW.VSTX.tsv")) { throw 'new TSV absent' }
    if (Test-Path "$out/1033/NEW.VSTX.tsv.partial") { throw 'completed partial not renamed' }
    $log = Get-Content "$out/probe-log.txt" -Raw
    if ($log -notmatch 'old log' -or $log -notmatch 'dumped=1 skipped=1 coverage=2 failed=0') { throw "incorrect appended log: $log" }
    if (@(Get-ChildItem (Join-Path $fixture 'out/dump-shipped-files/1033') -File).Count -ne 2) { throw 'not all files copied' }
    'Shipped resume fixture passed: existing bytes preserved, log appended, all files copied, only new file opened.'
} finally {
    if (Test-Path $fixture) { Remove-Item $fixture -Recurse -Force }
}
