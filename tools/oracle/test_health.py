"""Run the health script against fake COM objects, including close and Quit failures."""
from pathlib import Path
import os
import subprocess
import tempfile
import unittest

ROOT = Path(__file__).resolve().parents[2]
COMMON = r'''
$ErrorActionPreference = 'Stop'
$inv = [Globalization.CultureInfo]::InvariantCulture
function Log([string]$message) { $message | Out-File $log -Append }
function Office-Folder { 'fixture' }
function Close-Document($doc) {
    if ($env:HEALTH_CASE -eq 'close-fails') { throw 'fixture close failure' }
}
function New-Visio {
    $docs = [pscustomobject]@{}
    $docs | Add-Member ScriptMethod OpenEx {
        param($path, $flags)
        if ($flags -ne 2) { throw 'must open read-only' }
        if ($env:HEALTH_CASE -eq 'open-fails') { throw 'fixture open failure' }
        'template'
    }
    $docs | Add-Member ScriptMethod Add {
        param($name)
        if ($env:HEALTH_CASE -eq 'add-fails') { throw 'fixture add failure' }
        'blank'
    }
    $app = [pscustomobject]@{ Documents = $docs }
    $app | Add-Member ScriptMethod Quit {
        'called' | Out-File (Join-Path $out 'QUIT-CALLED')
        if ($env:HEALTH_CASE -eq 'quit-fails') { throw 'fixture quit failure' }
    }
    $app
}
'''


class HealthTests(unittest.TestCase):
    def test_failures_never_publish_healthy_and_still_attempt_quit(self):
        parent = ROOT / 'target/health-tests'
        parent.mkdir(parents=True, exist_ok=True)
        for case in ['healthy', 'open-fails', 'add-fails', 'close-fails', 'quit-fails']:
            with self.subTest(case=case), tempfile.TemporaryDirectory(dir=parent) as directory:
                root = Path(directory)
                source = (ROOT / 'tools/oracle/visio-health.ps1').read_text()
                # Redirect only the fixture output. Execute all health logic unchanged.
                lines = source.splitlines()
                index = next(i for i, line in enumerate(lines) if line.startswith('$out = '))
                lines[index] = "$out = Join-Path $PSScriptRoot 'out'"
                script = root / 'visio-health.ps1'
                script.write_text('\n'.join(lines) + '\n')
                (root / 'common.ps1').write_text(COMMON)
                result = subprocess.run(['pwsh', '-NoProfile', '-File', str(script)],
                                        env=dict(os.environ, HEALTH_CASE=case), capture_output=True,
                                        text=True, timeout=15)
                self.assertEqual(result.returncode, 0, result.stderr)
                self.assertTrue((root / 'out/DONE').exists())
                self.assertTrue((root / 'out/QUIT-CALLED').exists(), case)
                self.assertEqual((root / 'out/HEALTHY').exists(), case == 'healthy')


if __name__ == '__main__':
    unittest.main()
