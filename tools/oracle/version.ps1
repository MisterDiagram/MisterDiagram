# Phase 0 runner smoke test: only observe the Visio version/build.
$out = '\\Mac\Home\Projects\diagramcraft\corpus\_vm\out\version'
$ErrorActionPreference = 'Stop'
New-Item -ItemType Directory -Force $out | Out-Null
$log = Join-Path $out 'probe-log.txt'
"START $(Get-Date -Format s) user=$env:USERNAME" | Out-File $log
$app = $null
try {
    $app = New-Object -ComObject Visio.Application
    $app.AlertResponse = 7
    $app.Visible = $false
    "Visio version=$($app.Version) build=$($app.Build) product=$($app.ProductName)" | Out-File $log -Append
} catch {
    "FAIL $($_.Exception.Message)" | Out-File $log -Append
} finally {
    if ($app) { try { $app.Quit() } catch { "QUIT FAIL $_" | Out-File $log -Append } }
    'DONE' | Out-File (Join-Path $out 'DONE')
}
