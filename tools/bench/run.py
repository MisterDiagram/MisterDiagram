#!/usr/bin/env python3
"""Compare strict authoring on two clean revisions. MIT OR Apache-2.0."""
import argparse
import json
import os
from pathlib import Path
import subprocess
import sys

ROOT = Path(__file__).resolve().parents[2]


def run(*args, **kwargs):
    return subprocess.run(args, check=True, **kwargs)


def main():
    parser = argparse.ArgumentParser(description=__doc__)
    parser.add_argument('--baseline', default='origin/master')
    parser.add_argument('--candidate', default='HEAD')
    parser.add_argument('--output', type=Path, default=ROOT / 'target/performance-pr')
    args = parser.parse_args()
    output = args.output.resolve()
    if ROOT / 'target' not in output.parents:
        parser.error('output must be a new directory below this repository/target')
    output.mkdir(parents=True, exist_ok=False)
    run(sys.executable, str(ROOT / 'tools/bench/generate.py'), str(output / 'workloads'))
    binaries = {}
    for label, revision in [('before', args.baseline), ('after', args.candidate)]:
        checkout = output / label
        run('git', '-C', str(ROOT), 'worktree', 'add', '--detach', str(checkout), revision)
        harness = output / f'{label}-harness'
        harness.mkdir()
        # An external harness leaves both source checkouts untouched. No timing probes.
        (harness / 'main.rs').write_bytes((ROOT / 'tools/bench/authoring.rs').read_bytes())
        (harness / 'Cargo.toml').write_text('[package]\nname="authoring-bench"\nversion="0.0.0"\nedition="2024"\n[workspace]\n[[bin]]\nname="authoring-bench"\npath="main.rs"\n[dependencies]\nserde_json="1"\nmisterdiagram-engine={path=' + json.dumps(str(checkout / 'crates/engine')) + '}\n')
        env = dict(os.environ, CARGO_TARGET_DIR=str(output / f'{label}-build'))
        run('cargo', 'build', '--release', '--manifest-path', str(harness / 'Cargo.toml'), env=env)
        binaries[label] = output / f'{label}-build/release' / ('authoring-bench.exe' if os.name == 'nt' else 'authoring-bench')
    for case in ('rectangles-25', 'rectangles-50', 'rectangles-100', 'data-40', 'ports-1900'):
        for label in ('before', 'after'):
            package = output / f'{label}-{case}.vsdx'
            with (output / f'{label}-{case}.jsonl').open('w') as log:
                run(str(binaries[label]), str(output / f'workloads/{case}.json'), str(package), stdout=log)
            total = json.loads((output / f'{label}-{case}.jsonl').read_text().splitlines()[-1])
            print(json.dumps(dict(revision=label, case=case, **total)), flush=True)
        run(sys.executable, str(ROOT / 'tools/bench/compare.py'), str(output / f'before-{case}.vsdx'), str(output / f'after-{case}.vsdx'))


if __name__ == '__main__':
    main()
