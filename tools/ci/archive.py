#!/usr/bin/env python3
"""Package a CI build and the licences of its embedded fonts (five Latin families and the symbol face)."""
import os
from pathlib import Path
import zipfile

root = Path(__file__).resolve().parents[2]
name = 'misterdiagram-' + os.environ['RUNNER_OS'].lower() + '-' + os.environ['RUNNER_ARCH'].lower()
binary = root / 'target/release' / ('misterdiagram.exe' if os.name == 'nt' else 'misterdiagram')
files = [(binary, binary.name)]
files += [(root / name, name) for name in ('LICENSE-MIT', 'LICENSE-APACHE', 'ATTRIBUTION.md', 'NOTICE')]
files += [(p, 'fonts/' + p.parent.name + '/OFL.txt') for p in sorted((root / 'crates/fonts/assets').glob('*/OFL.txt'))]
if len(files) != 11:
    raise SystemExit('expected the binary, four project notices, five Latin font licences and the symbol face licence')
output = root / 'target/ci-artifacts'
output.mkdir(parents=True, exist_ok=True)
with zipfile.ZipFile(output / (name + '.zip'), 'w', compression=zipfile.ZIP_DEFLATED) as archive:
    for path, member in files:
        archive.write(path, member)
with open(os.environ['GITHUB_OUTPUT'], 'a', encoding='utf-8') as env:
    env.write('name=' + name + '\n')
print(name)
