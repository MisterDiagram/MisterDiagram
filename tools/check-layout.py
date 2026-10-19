#!/usr/bin/env python3
"""Check topology rules a-g using the engine's evaluated geometry (MIT OR Apache-2.0)."""
import argparse
import json
from pathlib import Path
import sys

sys.path.insert(0, str(Path(__file__).resolve().parent / 'mcp'))
from client import Client, default_binary


def check(path, binary, root=None, fit_page=False):
    with Client(binary) as client:
        client.command('file.open', path=str(Path(path).resolve()))
        params = {'fit_page': fit_page}
        if root is not None:
            params['root'] = root
        result = client.command('layout.check', **params)
    print(json.dumps(result, indent=2))
    return int(bool(result['count']))


def main():
    parser = argparse.ArgumentParser(description=__doc__)
    parser.add_argument('drawing', type=Path)
    parser.add_argument('--cli', type=Path, default=default_binary())
    parser.add_argument('--root', type=int, help='Check tree tiers and parent centres from this root ID')
    parser.add_argument('--fit-page', action='store_true', help='Require four equal page margins')
    args = parser.parse_args()
    try:
        return check(args.drawing, args.cli, args.root, args.fit_page)
    except (OSError, RuntimeError, ValueError, TimeoutError, KeyError) as error:
        print(f'layout checker: {error}', file=sys.stderr)
        return 2


if __name__ == '__main__':
    sys.exit(main())
