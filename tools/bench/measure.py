#!/usr/bin/env python3
"""Run strict workloads sequentially, preserving every CLI chunk. MIT OR Apache-2.0."""
import argparse
import json
from pathlib import Path
import shutil
import subprocess
import time

CASES = ('rectangles-25', 'rectangles-50', 'rectangles-100', 'data-40',
         'ports-1900', 'san-large', 'shapes-377', 'large-edit')


def read_commands(path):
    """Match the authoring harness's file and command admission bounds."""
    with path.open('rb') as source:
        data = source.read(16 * 1024 * 1024 + 1)
    if len(data) > 16 * 1024 * 1024:
        raise ValueError('benchmark input byte limit')
    commands = json.loads(data)
    if not isinstance(commands, list) or not commands or len(commands) > 8192:
        raise ValueError('benchmark command count limit')
    for command in commands:
        if (not isinstance(command, list) or len(command) != 2
                or not isinstance(command[0], str) or not isinstance(command[1], dict)):
            raise ValueError('benchmark command must be an ID and parameter object')
    return commands


def main():
    parser = argparse.ArgumentParser(description=__doc__)
    parser.add_argument('--cli', type=Path, required=True)
    parser.add_argument('--harness', type=Path, required=True)
    parser.add_argument('--workloads', type=Path, required=True)
    parser.add_argument('--output', type=Path, required=True)
    parser.add_argument('--cases', nargs='+', choices=CASES, default=CASES)
    parser.add_argument('--profile', type=Path)
    parser.add_argument('--guard', type=Path, help='Optional existing guard.sh; 7200 seconds, 2048 MiB')
    args = parser.parse_args()
    args.output.mkdir(parents=True, exist_ok=True)
    for name in args.cases:
        commands = read_commands(args.workloads / f'{name}.json')
        package = args.output / f'{name}.vsdx'
        if name in ('shapes-377', 'large-edit'):
            chunks = [commands[i:i + 250] for i in range(0, len(commands), 250)]
        else:
            chunks = [commands]
        rows = []
        for index, chunk in enumerate(chunks):
            if name in ('shapes-377', 'large-edit'):
                command = [str(args.cli.resolve()), 'run']
                if index or name == 'large-edit':
                    source = package if index else args.output / 'shapes-377.vsdx'
                    command += ['--open', str(source.resolve())]
                for cid, params in chunk:
                    command += ['--cmd', cid + '=' + json.dumps(params, separators=(',', ':'))]
                command += ['--save', str(package.resolve())]
            else:
                command = [str(args.harness.resolve()), str((args.workloads / f'{name}.json').resolve()), str(package.resolve())]
            if args.profile:
                command = [str(args.profile), 'record', '--save-only', '--no-open', '--unstable-presymbolicate',
                           '-o', str((args.output / f'{name}-{index}.json.gz').resolve()), '--'] + command
            # /usr/bin/time measures just the workload and leaves the raw RSS receipt.
            command = ['/usr/bin/time', '-l'] + command
            if args.guard:
                command = [str(args.guard), '7200', '2048'] + command
            start = time.monotonic()
            with (args.output / f'{name}-{index}.stdout').open('w') as out, (args.output / f'{name}-{index}.stderr').open('w') as err:
                result = subprocess.run(command, stdout=out, stderr=err)
            elapsed = time.monotonic() - start
            row = {'case': name, 'slice': index, 'commands': len(chunk), 'wall_seconds': elapsed, 'exit': result.returncode,
                   'command': command}
            rows.append(row)
            (args.output / f'{name}-receipts.json').write_text(json.dumps(rows, indent=2) + '\n')
            print(json.dumps({k: v for k, v in row.items() if k != 'command'}), flush=True)
            if result.returncode:
                raise SystemExit(result.returncode)
            emitted = []
            for line in (args.output / f'{name}-{index}.stdout').read_text().splitlines():
                if line.startswith('{'):
                    emitted.append(json.loads(line))
            if name in ('shapes-377', 'large-edit'):
                if len([r for r in emitted if 'command' in r]) != len(chunk) + 1 or emitted[-1].get('command') != 'file.save':
                    raise RuntimeError(f'{name} chunk {index}: missing command/save output')
            elif not any(r.get('kind') == 'total' and r.get('commands') == len(commands) for r in emitted):
                raise RuntimeError(f'{name}: missing complete harness total')
            if not package.exists():
                raise RuntimeError(f'{name}: saved package missing')
            if name == 'shapes-377':
                shutil.copyfile(package, args.output / f'{name}-chunk-{index + 1}.vsdx')


if __name__ == '__main__':
    main()
