#!/usr/bin/env python3
"""Real stdio process, strict batch and CLI saved-package equivalence."""
import os
from pathlib import Path
import subprocess
import sys
import unittest
from client import Client, default_binary

ROOT = Path(__file__).resolve().parents[2]
CLI = Path(os.environ.get('MISTERDIAGRAM_CLI', default_binary())).resolve()


def bash():
    """Use Git for Windows Bash, not the Windows WSL launcher."""
    if os.name != 'nt':
        return 'bash'
    directory = Path(subprocess.check_output(['git', '--exec-path'], text=True).strip())
    for parent in (directory, *directory.parents):
        candidate = parent / 'bin/bash.exe'
        if candidate.is_file():
            return str(candidate)
    raise RuntimeError('Git for Windows Bash is required')


class Stdio(unittest.TestCase):
    def test_two_boxes_equals_cli_script(self):
        out = ROOT / 'target' / 'mcp-stdio' / str(os.getpid())
        env = dict(os.environ, MISTERDIAGRAM_CLI=CLI.as_posix())
        subprocess.run([bash(), (ROOT / 'tools/examples/two-boxes-glued.sh').as_posix(), (out / 'cli').as_posix()],
                       cwd=ROOT, env=env, check=True)
        subprocess.run([sys.executable, str(ROOT / 'tools/mcp/two-boxes.py'), str(out / 'mcp'), '--cli', str(CLI)], check=True)
        subprocess.run([sys.executable, str(ROOT / 'tools/bench/compare.py'),
                        str(out / 'cli/two-boxes-glued.vsdx'), str(out / 'mcp/two-boxes-glued.vsdx')], check=True)

    def test_batch_equals_strict_commands_and_rolls_back(self):
        out = ROOT / 'target' / 'mcp-stdio' / str(os.getpid())
        out.mkdir(parents=True, exist_ok=True)
        commands = [
            {'command': 'shape.draw.rect', 'params': {'x': 1, 'y': 1, 'width': 2, 'height': 1}},
            {'command': 'shape.move', 'params': {'id': 1, 'x': 4, 'y': 6}},
            {'command': 'master.new_from_shape', 'params': {'id': 1, 'name': 'MCP Original'}},
            {'command': 'master.drop', 'params': {'master': 'MCP Original', 'x': 7, 'y': 6}},
            {'command': 'shape.text', 'params': {'id': 2, 'text': 'Cedar'}}]
        for mode in ('batch', 'strict'):
            with Client(CLI) as client:
                if mode == 'batch':
                    client.tool('batch', commands=commands)
                else:
                    for item in commands:
                        client.tool('run_command', **item)
                client.tool('save', path=str(out / f'{mode}.vsdx'))
                before = client.tool('inspect_document')
                bad = commands[:2] + [{'command': 'shape.move', 'params': {'id': 999, 'x': 0, 'y': 0}}] + commands[3:]
                with self.assertRaisesRegex(RuntimeError, r'index 2 \(command 3\)'):
                    client.tool('batch', commands=bad)
                self.assertEqual(before, client.tool('inspect_document'))
        subprocess.run([sys.executable, str(ROOT / 'tools/bench/compare.py'), str(out / 'batch.vsdx'), str(out / 'strict.vsdx')], check=True)


if __name__ == '__main__':
    unittest.main()
