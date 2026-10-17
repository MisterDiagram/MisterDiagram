#!/usr/bin/env python3
"""Build the CLI two-boxes drawing through MCP. MIT OR Apache-2.0."""
import argparse
from pathlib import Path
from client import Client, default_binary


def build(binary, output):
    output = Path(output)
    output.mkdir(parents=True, exist_ok=True)
    with Client(binary) as client:
        client.command('file.new')
        for x in (1, 5):
            client.command('shape.draw.rect', x=x, y=6, width=1.5, height=1.5)
        connector = client.tool('drop_master', master=2, x=0, y=0)['id']
        for end, target in [('begin', 1), ('end', 2)]:
            client.tool('connect', id=connector, end=end, target=target)
        client.tool('save', path=str(output / 'two-boxes-glued.vsdx'))
    print('MCP: 2 boxes, 1 connector, 2 Connects')


if __name__ == '__main__':
    parser = argparse.ArgumentParser(description=__doc__)
    parser.add_argument('output', type=Path)
    parser.add_argument('--cli', type=Path, default=default_binary())
    args = parser.parse_args()
    build(args.cli, args.output)
