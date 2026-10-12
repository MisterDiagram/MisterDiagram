# Health means all steps, including Quit, complete in less than 60 seconds.
$out = '\\Mac\Home\Projects\diagramcraft\corpus\_vm\out\visio-health'
New-Item -ItemType Directory -Force $out | Out-Null
$log = Join-Path $out 'probe-log.txt'
"START $(Get-Date -Format s)" | Out-File $log
$total = [Diagnostics.Stopwatch]::StartNew()
$app = $null; $template = $null; $blank = $null; $healthy = $false
try {
    . (Join-Path $PSScriptRoot 'common.ps1')
    $step = [Diagnostics.Stopwatch]::StartNew()
    Log 'BEGIN launch-and-attach'
    $app = New-Visio
    Log "OK launch-and-attach seconds=$($step.Elapsed.TotalSeconds.ToString('F3', $inv))"
    $path = Join-Path (Office-Folder) 'Visio Content\1033\ACTDIR_U.VSTX'
    $step.Restart()
    Log "BEGIN OpenEx read-only $path"
    $template = $app.Documents.OpenEx($path, 2)
    Log "OK OpenEx seconds=$($step.Elapsed.TotalSeconds.ToString('F3', $inv))"
    $step.Restart()
    Log 'BEGIN Documents.Add'
    $blank = $app.Documents.Add('')
    Log "OK Documents.Add seconds=$($step.Elapsed.TotalSeconds.ToString('F3', $inv))"
    $step.Restart()
    Log 'BEGIN close blank'
    Close-Document $blank; $blank = $null
    Log "OK close blank seconds=$($step.Elapsed.TotalSeconds.ToString('F3', $inv))"
    $step.Restart()
    Log 'BEGIN close template'
    Close-Document $template; $template = $null
    Log "OK close template seconds=$($step.Elapsed.TotalSeconds.ToString('F3', $inv))"
    $healthy = $true
} catch {
    "FAIL seconds=$($total.Elapsed.TotalSeconds) $($_.Exception.Message)" | Out-File $log -Append
} finally {
    if ($app) {
        foreach ($doc in @($blank, $template)) {
            if ($doc) {
                try { Close-Document $doc }
                catch { $healthy = $false; "CLOSE FAIL $_" | Out-File $log -Append }
            }
        }
        try {
            $quit = [Diagnostics.Stopwatch]::StartNew()
            Log 'BEGIN Quit'
            $app.Quit()
            Log "OK Quit seconds=$($quit.Elapsed.TotalSeconds.ToString('F3', $inv))"
        } catch { $healthy = $false; "QUIT FAIL $_" | Out-File $log -Append }
    }
    $total.Stop()
    if ($healthy -and $total.Elapsed.TotalSeconds -lt 60) {
        Log "HEALTH OK seconds=$($total.Elapsed.TotalSeconds.ToString('F3', $inv))"
        'HEALTHY' | Out-File (Join-Path $out 'HEALTHY')
    } else {
        "HEALTH FAIL seconds=$($total.Elapsed.TotalSeconds)" | Out-File $log -Append
    }
    'DONE' | Out-File (Join-Path $out 'DONE')
}
