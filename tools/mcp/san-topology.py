#!/usr/bin/env python3
"""Build the 24-switch, 40-ISL SAN through MCP stdio. MIT OR Apache-2.0."""
import argparse
import json
from pathlib import Path
import runpy
from client import Client, default_binary


def build(binary, output):
    output = Path(output).resolve()
    output.mkdir(parents=True, exist_ok=True)
    recipe = runpy.run_path(str(Path(__file__).resolve().parents[1] / 'examples/san-topology.py'))['recipe']
    with Client(binary) as client:
        for item in recipe():
            command, params = item.split('=', 1)
            result = client.command(command, **json.loads(params))
            if command == 'layout.fan' and result['crossings']:
                raise RuntimeError(f'route crosses a switch: {result["crossings"]}')
        document = client.tool('inspect_document')
        connects = sum(len(page['connects']) for page in document['pages'])
        if connects != 80:
            raise RuntimeError(f'expected 80 Connects, got {connects}')
        client.tool('save', path=str(output / 'p5a-san-topology.vsdx'))
        for extension in ('png', 'svg'):
            client.tool('export', path=str(output / f'p5a-san-topology.{extension}'))
    print(f'MCP SAN: 24 switches, 40 ISLs, {connects} Connects, no switch crossings')


if __name__ == '__main__':
    parser = argparse.ArgumentParser(description=__doc__)
    parser.add_argument('output', type=Path)
    parser.add_argument('--cli', type=Path, default=default_binary())
    args = parser.parse_args()
    build(args.cli, args.output)
