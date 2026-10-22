#!/usr/bin/env python3
"""Write THIRD-PARTY-LICENSES.txt for the misterdiagram binary (MIT OR Apache-2.0).

Usage: third_party.py OUTPUT TARGET [TARGET...]
Lists every crate linked into misterdiagram-cli for the given targets (normal dependencies only:
build scripts and proc macros run at build time and are not shipped), with its licence
expression and the licence and notice files from its published package. Standard library only;
needs cargo and the crate sources a build has already fetched.
"""
import json
from pathlib import Path
import re
import subprocess
import sys

ROOT = Path(__file__).resolve().parents[2]
LICENCE_FILE = re.compile(r'^(LICEN[CS]E|COPYING|NOTICE|UNLICENSE|COPYRIGHT)([-_.].*)?$', re.IGNORECASE)


def linked(target):
    meta = json.loads(subprocess.run(
        ['cargo', 'metadata', '--format-version', '1', '--locked', '--filter-platform', target],
        cwd=ROOT, check=True, capture_output=True, text=True).stdout)
    packages = {p['id']: p for p in meta['packages']}
    nodes = {n['id']: n for n in meta['resolve']['nodes']}
    root = next(p['id'] for p in meta['packages'] if p['name'] == 'misterdiagram-cli')
    seen, stack = set(), [root]
    while stack:
        node = nodes[stack.pop()]
        for dep in node['deps']:
            if any(k['kind'] is None for k in dep['dep_kinds']) and dep['pkg'] not in seen:
                if 'proc-macro' in {k for t in packages[dep['pkg']]['targets'] for k in t['kind']}:
                    continue
                seen.add(dep['pkg'])
                stack.append(dep['pkg'])
    members = set(meta['workspace_members'])
    return {packages[i]['name'] + ' ' + packages[i]['version']: packages[i] for i in seen if i not in members}


def main():
    if len(sys.argv) < 3:
        raise SystemExit(__doc__)
    output, targets = Path(sys.argv[1]), sys.argv[2:]
    crates = {}
    for target in targets:
        crates.update(linked(target))
    if not crates:
        raise SystemExit('no third-party crates found; is cargo metadata working?')
    parts = [
        'Third-party software in the misterdiagram binary\n',
        '=' * 48 + '\n\n',
        f'Targets: {", ".join(targets)}. {len(crates)} Rust crates are linked into this binary.\n',
        'Each is listed with its licence expression and the licence and notice files from its\n',
        'published package. The embedded fonts are listed in ATTRIBUTION.md, with their licences\n',
        'under fonts/.\n\n',
    ]
    missing = []
    for key in sorted(crates, key=str.lower):
        package = crates[key]
        folder = Path(package['manifest_path']).parent
        files = sorted(p for p in folder.iterdir() if p.is_file() and LICENCE_FILE.match(p.name))
        parts.append('-' * 78 + '\n')
        parts.append(f'{key}\nLicence: {package.get("license") or package.get("license_file") or "not stated"}\n')
        if package.get('repository'):
            parts.append(f'Source: {package["repository"]}\n')
        if not files:
            missing.append(key)
            parts.append('(The published package carries no licence file; the standard text of the licence\n'
                         'named above applies. LICENSE-MIT and LICENSE-APACHE in this archive hold the\n'
                         'MIT and Apache-2.0 texts.)\n')
        for path in files:
            text = path.read_text(encoding='utf-8', errors='replace').rstrip()
            parts.append(f'\n--- {path.name} ---\n{text}\n')
        parts.append('\n')
    output.parent.mkdir(parents=True, exist_ok=True)
    output.write_text(''.join(parts), encoding='utf-8', newline='\n')
    print(f'{output}: {len(crates)} crates, {len(missing)} without a licence file in the package')
    for key in missing:
        print(f'  no licence file: {key}')


if __name__ == '__main__':
    main()
