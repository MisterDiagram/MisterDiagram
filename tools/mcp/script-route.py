#!/usr/bin/env python3
"""Rebuild an invented data-driven drawing using the CLI. MIT OR Apache-2.0."""
import argparse
import json
from pathlib import Path
import subprocess


def main():
    parser = argparse.ArgumentParser(description=__doc__)
    parser.add_argument('output', type=Path)
    parser.add_argument('--cli', type=Path, default=Path('target/debug/misterdiagram'))
    args = parser.parse_args()
    args.output.mkdir(parents=True, exist_ok=True)
    devices = [('cedar', 'Cedar Host'), ('willow', 'Willow Array')]
    commands = [{'command': 'shape.draw.rect', 'as': key,
                 'params': {'x': 1 + 4 * index, 'y': 3, 'width': 2, 'height': 1, 'text': label}}
                for index, (key, label) in enumerate(devices)]
    commands.append({'command': 'connector.add', 'params': {'from': '$cedar', 'to': '$willow'}})
    recipe = args.output / 'commands.json'
    recipe.write_text(json.dumps({'commands': commands}, indent=2) + '\n')
    subprocess.run([str(args.cli), 'run', '--batch', str(recipe), '--save', str(args.output / 'drawing.vsdx')], check=True)


if __name__ == '__main__':
    main()
