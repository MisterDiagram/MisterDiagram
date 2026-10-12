#!/usr/bin/env python3
"""Preserve completed, unchanged Visio exports after a runner timeout."""
import argparse
import hashlib
import json
from pathlib import Path
import re
import shutil

ROOT = Path(__file__).resolve().parents[2]
EXTENSIONS = ('vsdx', 'png', 'svg', 'pdf', 'sheet.tsv', 'connects.tsv')

def digest(path):
    with path.open('rb') as stream:
        return hashlib.file_digest(stream, 'sha256').hexdigest()

def completed_names(log):
    if len(log) > 4 * 1024 * 1024:
        raise ValueError('checkpoint log too large')
    names = re.findall(r' DRAWING (?:OK|RESUMED) ([a-z0-9/-]+(?:\.[a-z0-9]+)?)\s*$', log, re.MULTILINE)
    if any('..' in name or name.startswith('/') for name in names):
        raise ValueError('unsafe drawing name')
    return list(dict.fromkeys(names))

def main():
    parser = argparse.ArgumentParser(description=__doc__)
    parser.add_argument('source', type=Path, help='preserved make-corpus output folder under corpus/')
    args = parser.parse_args()
    source = args.source.resolve()
    if not source.is_relative_to(ROOT / 'corpus'):
        parser.error('source must be under this repository corpus/')
    cache = ROOT / 'corpus/_vm/resume-make-corpus'
    if cache.exists():
        parser.error('checkpoint already exists; preserve or remove it explicitly first')
    if (source / 'probe-log.txt').stat().st_size > 8 * 1024 * 1024:
        parser.error('checkpoint log too large')
    raw = (source / 'probe-log.txt').read_bytes()
    log = raw.decode('utf-16' if raw.startswith(b'\xff\xfe') else 'utf-8')
    names = completed_names(log)
    data = {'source': str(source), 'common_sha256': digest(ROOT / 'corpus/_vm/common.ps1'),
            'source_recipe_sha256': digest(ROOT / 'corpus/_vm/make-corpus.ps1'),
            'resume_recipe_sha256': digest(ROOT / 'tools/oracle/make-corpus.ps1'), 'drawings': {}}
    for name in names:
        # Names come from the recipe log, never from an arbitrary manifest path.
        if '..' in name or name.startswith('/'):
            parser.error('unsafe drawing name')
        files = {ext: Path(str(source / name) + '.' + ext) for ext in EXTENSIONS}
        if any(not p.is_file() or not p.stat().st_size for p in files.values()):
            parser.error(f'incomplete drawing logged as complete: {name}')
        hashes = {ext: digest(path) for ext, path in files.items()}
        for ext, path in files.items():
            target = Path(str(cache / 'data' / name) + '.' + ext)
            target.parent.mkdir(parents=True, exist_ok=True)
            shutil.copy2(path, target)
            if digest(target) != hashes[ext]:
                raise RuntimeError(f'copy checksum mismatch: {target}')
        data['drawings'][name] = hashes
    if not data['drawings']:
        parser.error('no completed drawings found')
    shutil.copy2(ROOT / 'corpus/_vm/make-corpus.ps1', cache / 'source-recipe.ps1')
    shutil.copy2(ROOT / 'corpus/_vm/common.ps1', cache / 'source-common.ps1')
    (cache / 'manifest.json').write_text(json.dumps(data, indent=2) + '\n')
    print(f'Checkpoint: {len(data["drawings"])} complete drawings, six unchanged files each, {cache}')

if __name__ == '__main__':
    main()
