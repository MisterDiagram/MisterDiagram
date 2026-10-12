# Host-only tests. Never invokes Visio or prlctl, and writes only below repo target/.
$ErrorActionPreference = 'Stop'
$root = Split-Path (Split-Path $PSScriptRoot)
$tokens = $null; $errors = $null
$ast = [Management.Automation.Language.Parser]::ParseFile((Join-Path $root 'tools/oracle/make-corpus.ps1'), [ref]$tokens, [ref]$errors)
if ($errors) { throw $errors[0] }
$fn = $ast.Find({ param($node) $node -is [Management.Automation.Language.FunctionDefinitionAst] -and $node.Name -eq 'Resume-Drawing' }, $true)
if (-not $fn) { throw 'Resume-Drawing missing' }
Invoke-Expression $fn.Extent.Text
function Log([string]$message) { Write-Output $message | Out-Null }
$work = Join-Path $root "target/checkpoint-test-$PID"
if (Test-Path $work) { throw 'test folder already exists' }
try {
    $resumeRoot = Join-Path $work 'cache'; $out = Join-Path $work 'out'
    New-Item -ItemType Directory -Force (Join-Path $resumeRoot 'data/basic') | Out-Null
    $entry = @{}
    foreach ($ext in @('vsdx','png','svg','pdf','sheet.tsv','connects.tsv')) {
        $path = Join-Path $resumeRoot "data/basic/test.$ext"
        [IO.File]::WriteAllText($path, "disposable fixture $ext")
        $entry[$ext] = (Get-FileHash $path -Algorithm SHA256).Hash
    }
    $resumeByName = @{ 'basic/test' = [pscustomobject]$entry }
    $script:Made = 0; $script:Resumed = 0
    if (-not (Resume-Drawing 'basic/test')) { throw 'resume failed' }
    if ($script:Made -ne 1 -or $script:Resumed -ne 1) { throw 'incorrect counts' }
    if (@(Get-ChildItem (Join-Path $out 'basic')).Count -ne 6) { throw 'missing exports' }
    $out = Join-Path $work 'rejected'
    [IO.File]::WriteAllText((Join-Path $resumeRoot 'data/basic/test.sheet.tsv'), 'tampered fixture')
    $rejected = $false
    try { Resume-Drawing 'basic/test' | Out-Null } catch {
        if ($_.Exception.Message -notlike '*checksum mismatch*') { throw }
        $rejected = $true
    }
    if (-not $rejected -or (Test-Path $out)) { throw 'tampered checkpoint was copied' }
    if (Resume-Drawing 'missing/test') { throw 'unlisted drawing was reused' }
    Write-Output 'Checkpoint tests passed: six exact files, counts, checksum rejection before copy, missing entry.'
} finally { if (Test-Path $work) { Remove-Item $work -Recurse -Force } }
