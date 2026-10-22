#!/usr/bin/env python3
"""Check topology rules a-h using evaluated geometry (MIT OR Apache-2.0).

Rule h: no collinear straight-segment overlap greater than 1e-6 inch.
Exempt pairs: both effective styles are Flowchart/Tree (5,6,7,8,12,13,14,15)
or witnessed office style 17; or both segments end at the same actual glue
target and point (the named fan-out gap). Rules a-g keep their thresholds."""
import argparse
import json
from pathlib import Path
import sys

sys.path.insert(0, str(Path(__file__).resolve().parent / 'mcp'))
from client import Client, default_binary


def check(path, binary, root=None, fit_page=False, shared_only=False):
    full_error = None
    with Client(binary) as client:
        client.command('file.open', path=str(Path(path).resolve()))
        params = {'fit_page': fit_page, 'shared_only': shared_only}
        if root is not None:
            params['root'] = root
        try:
            result = client.command('layout.check', **params)
        except RuntimeError as error:
            if shared_only or 'layout checker comparison limit' not in str(error):
                raise
            full_error = str(error)
            result = client.command('layout.check', shared_only=True)
            result['full_check_error'] = full_error
    print(json.dumps(result, indent=2))
    return 2 if full_error else int(bool(result['count']))


def main():
    parser = argparse.ArgumentParser(description=__doc__)
    parser.add_argument('drawing', type=Path)
    parser.add_argument('--cli', type=Path, default=default_binary())
    parser.add_argument('--root', type=int, help='Check tree tiers and parent centres from this root ID')
    parser.add_argument('--fit-page', action='store_true', help='Require four equal page margins')
    parser.add_argument('--shared-only', action='store_true', help='Run only rule h; full-check comparison refusals otherwise retain exit 2 and include rule-h counts')
    args = parser.parse_args()
    try:
        return check(args.drawing, args.cli, args.root, args.fit_page, args.shared_only)
    except (OSError, RuntimeError, ValueError, TimeoutError, KeyError) as error:
        print(f'layout checker: {error}', file=sys.stderr)
        return 2


if __name__ == '__main__':
    sys.exit(main())
