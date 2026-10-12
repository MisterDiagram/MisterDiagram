"""Exercise the real runner with a fake prlctl, never a VM or oracle output."""
from pathlib import Path
import json
import os
import shutil
import subprocess
import tempfile
import unittest

ROOT = Path(__file__).resolve().parents[2]
FAKE = r'''#!/usr/bin/env python3
from pathlib import Path
import json, os, re, sys
root = Path(os.environ['RUNNER_FIXTURE'])
args = ' '.join(sys.argv[1:])
with (root/'calls.jsonl').open('a') as f: f.write(json.dumps(args)+'\n')
if sys.argv[1:3] == ['list', '-a']:
    print('running Win11 Enterprise (ARM)')
elif 'Register-ScheduledTask ' in args:
    name = re.search(r'-File "[^\"]*\\([^\\]+)\.ps1"', args).group(1)
    stage=root/'corpus/_vm'
    if name == 'quit-visio':
        (stage/'QUIT-DONE').write_text('DONE')
    else:
        out=stage/'out'/name
        out.mkdir(parents=True,exist_ok=True)
        text='START\n'
        if name == 'visio-health':
            if os.environ['RUNNER_CASE'] != 'health-fails':
                text+='HEALTH OK seconds=5.000\n'
                (out/'HEALTHY').write_text('HEALTHY')
            else: text+='HEALTH FAIL seconds=5.000\n'
            (out/'DONE').write_text('DONE')
        elif os.environ['RUNNER_CASE'] == 'ok':
            (out/'DONE').write_text('DONE')
        (out/'probe-log.txt').write_text(text,encoding='utf-16')
'''


class RunnerTests(unittest.TestCase):
    def run_case(self, case, name='accept'):
        parent = ROOT / 'target/runner-tests'
        parent.mkdir(parents=True, exist_ok=True)
        with tempfile.TemporaryDirectory(dir=parent) as directory:
            root = Path(directory)
            scripts = root / 'tools/oracle'
            scripts.mkdir(parents=True)
            for source in (ROOT / 'tools/oracle').glob('*'):
                if source.suffix in {'.sh', '.ps1'}:
                    shutil.copyfile(source, scripts / source.name)
            binary = root / 'bin'
            binary.mkdir()
            mock = binary / 'prlctl'
            mock.write_text(FAKE)
            mock.chmod(0o755)
            probe = root / 'corpus/probe.vsdx'
            probe.parent.mkdir()
            probe.write_bytes(b'fixture, not an oracle')
            env = dict(os.environ, RUNNER_FIXTURE=str(root), RUNNER_CASE=case,
                       PATH=str(binary) + os.pathsep + os.environ['PATH'])
            command = ['bash', str(scripts / 'run-in-vm.sh'), name + '.ps1', '1']
            if name == 'accept':
                command.append(str(probe))
            result = subprocess.run(command, cwd=root, env=env, capture_output=True,
                                    text=True, timeout=40)
            calls = [json.loads(line) for line in (root / 'calls.jsonl').read_text().splitlines()]
            launches = [remainder.split('.ps1')[0].rsplit('\\', 1)[-1]
                        for call in calls if 'Register-ScheduledTask ' in call
                        for remainder in [call.split('-File "', 1)[1]]]
            return result, calls, launches

    def test_health_precedes_accept_and_shipped(self):
        for name in ['accept', 'dump-shipped']:
            result, _, launches = self.run_case('ok', name)
            self.assertEqual(result.returncode, 0, result.stderr)
            self.assertEqual(launches, ['visio-health', name])

    def test_failed_health_aborts_before_accept(self):
        result, _, launches = self.run_case('health-fails')
        self.assertNotEqual(result.returncode, 0)
        self.assertEqual(launches, ['visio-health'])
        self.assertIn('Visio health probe failed, see tools/oracle/README.md', result.stderr)

    def test_watchdog_quits_then_checks_liveness_then_probes(self):
        result, calls, launches = self.run_case('timeout')
        self.assertEqual(result.returncode, 124, result.stderr)
        self.assertEqual(launches, ['visio-health', 'accept', 'quit-visio', 'visio-health'])
        quit_call = next(i for i, c in enumerate(calls) if '-File ' in c and 'quit-visio.ps1' in c)
        self.assertIn('New-TimeSpan -Seconds 20', calls[quit_call])
        force_call = next(i for i, c in enumerate(calls) if 'Stop-Process -Name VISIO' in c)
        self.assertGreater(force_call, quit_call)
        self.assertIn('if (Get-Process -Name VISIO', calls[force_call])
        self.assertFalse(any('GetActiveObject' in c for c in calls))


if __name__ == '__main__':
    unittest.main()
