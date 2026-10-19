#!/usr/bin/env python3
"""Rebuild the office network in one MCP batch. MIT OR Apache-2.0."""
import argparse
import json
from pathlib import Path
import zipfile
import xml.etree.ElementTree as ET
from client import Client, default_binary


def recipe():
    commands = [{'command': 'stencil.open', 'params': {'builtin': 'network'}}]
    names = ['Sky Internet', 'Alder Firewall', 'Birch Router', 'Cedar Core', 'Elm Access',
             'Fir Access', 'Grove Server', 'Hazel Wi-Fi', 'Iris PC', 'Juniper PC']
    masters = ['network.cloud', 'network.firewall', 'network.router', 'network.switch.core',
               'network.switch.access', 'network.switch.access', 'compute.server',
               'network.wireless_ap', 'compute.desktop', 'compute.desktop']
    aliases = ['sky', 'alder', 'birch', 'cedar', 'elm', 'fir', 'grove', 'hazel', 'iris', 'juniper']
    for alias, name, master in zip(aliases, names, masters):
        commands.append({'command': 'master.drop', 'as': alias, 'params': {'master': master, 'text': name}})
    edges = [('sky', 'alder'), ('alder', 'birch'), ('birch', 'cedar'), ('cedar', 'grove'),
             ('cedar', 'elm'), ('cedar', 'fir'), ('elm', 'iris'), ('fir', 'juniper'), ('fir', 'hazel')]
    commands.append({'command': 'layout.tree', 'params': {'root': '$sky',
                     'edges': [['$' + a, '$' + b] for a, b in edges],
                     'gutter_x': 2, 'gutter_y': 0.5, 'fit_page': True}})
    return commands


def build(binary, output):
    output = Path(output).resolve()
    output.mkdir(parents=True, exist_ok=True)
    commands = recipe()
    with Client(binary) as client:
        result = client.tool('batch', commands=commands)
        build_calls = client.tool_calls
        checked = client.command('layout.check', root=result['references']['sky'], fit_page=True)
        if checked['count']:
            raise RuntimeError(f'office layout violations: {checked}')
        drawing = output / 'office-network.vsdx'
        client.tool('save', path=str(drawing))
        client.tool('render', path=str(output / 'office-network.png'))
    with zipfile.ZipFile(drawing) as package:
        connects = len(ET.fromstring(package.read('visio/pages/page1.xml')).findall('.//{*}Connect'))
    if connects != 18:
        raise RuntimeError(f'expected 18 Connects, got {connects}')
    receipt = dict(build_commands=len(commands), build_tool_calls=build_calls, total_tool_calls=client.tool_calls,
                   devices=10, links=9, connects=connects, layout=checked)
    (output / 'receipt.json').write_text(json.dumps(receipt, indent=2) + '\n')
    print(json.dumps(receipt))


if __name__ == '__main__':
    parser = argparse.ArgumentParser(description=__doc__)
    parser.add_argument('output', type=Path)
    parser.add_argument('--cli', type=Path, default=default_binary())
    args = parser.parse_args()
    build(args.cli, args.output)
