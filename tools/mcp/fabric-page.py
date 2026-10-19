#!/usr/bin/env python3
"""Rebuild a five-device dual-fabric page with parallel links. MIT OR Apache-2.0."""
import argparse
import json
from pathlib import Path
import zipfile
import xml.etree.ElementTree as ET
from client import Client, default_binary


def recipe():
    commands = []

    def command(command_id, alias=None, **params):
        item = {'command': command_id, 'params': params}
        if alias is not None:
            item['as'] = alias
        commands.append(item)

    command('file.new')
    command('page.size', width=11, height=8.5)
    command('stencil.open', builtin='network')
    for alias, text, master in [('host1', 'Alder Host', 'compute.server'), ('host2', 'Birch Host', 'compute.server'),
                                ('switchA', 'Cedar Switch', 'storage.switch'), ('switchB', 'Maple Switch', 'storage.switch'),
                                ('array', 'Willow Array', 'storage.array')]:
        command('master.drop', alias, master=master, text=text)
    command('layout.rows', ids=['$host1', '$host2', '$switchA', '$switchB', '$array'], counts=[2, 2, 1],
            region={'x': 1, 'y': 0.75, 'width': 9, 'height': 6.25}, gutter_x=3, gutter_y=1.25)
    links = [('host1', 'switchA'), ('host1', 'switchB'), ('host2', 'switchA'), ('host2', 'switchB')]
    links += [('array', 'switchA')] * 3 + [('array', 'switchB')] * 3
    for index, (source, target) in enumerate(links):
        command('connector.add', f'link{index}', **{'from': '$' + source, 'to': '$' + target},
                line={'color': '#B23442' if target == 'switchA' else '#286FA3', 'weight': 1.5, 'units': 'PT'})
    for hub in ('switchA', 'switchB'):
        command('layout.fan', hub='$' + hub, spokes=['$host1', '$host2', '$array'])
    return commands


def build(binary, output):
    output = Path(output).resolve()
    output.mkdir(parents=True, exist_ok=True)
    commands = recipe()
    with Client(binary) as client:
        result = client.tool('batch', commands=commands)
        placed = next(value['placed'] for item, value in zip(commands, result['results']) if item['command'] == 'layout.rows')
        bounds = {item['id']: item['bounds'] for item in placed}
        decorations = []
        for fabric, label, color in [('A', 'Cedar Fabric', '#B23442'), ('B', 'Maple Fabric', '#286FA3')]:
            box = bounds[result['references']['switch' + fabric]]
            alias = 'box' + fabric
            decorations.extend([
                {'command': 'shape.draw.rect', 'as': alias, 'params': {
                    'x': box['x'] - 0.375, 'y': box['y'] - 0.25,
                    'width': box['width'] + 0.75, 'height': box['height'] + 0.625,
                    'text': label, 'fill': {'pattern': 0},
                    'line': {'color': color, 'pattern': 2, 'weight': 1, 'units': 'PT'},
                    'font': {'color': color, 'bold': True, 'size': 10}}},
                {'command': 'sheet.set', 'params': {'id': '$' + alias, 'name': 'VerticalAlign', 'value': 0}},
                {'command': 'shape.order', 'params': {'id': '$' + alias, 'order': 'back'}},
            ])
        decorations.append({'command': 'shape.draw.rect', 'as': 'title', 'params': {
            'x': 3, 'y': 7.6, 'width': 5, 'height': 0.5, 'text': 'Willow dual-fabric SAN',
            'fill': {'pattern': 0}, 'line': {'pattern': 0}, 'font': {'size': 18, 'bold': True}}})
        client.tool('batch', commands=decorations)
        build_calls = client.tool_calls
        checked = client.command('layout.check')
        disallowed = [v for v in checked['violations'] if v['rule'] not in ('b', 'g')]
        if disallowed:
            raise RuntimeError(f'fabric layout violations outside the container exceptions: {disallowed}')
        drawing = output / 'fabric-page.vsdx'
        client.tool('save', path=str(drawing))
        client.tool('render', path=str(output / 'fabric-page.png'))
    with zipfile.ZipFile(drawing) as package:
        connects = len(ET.fromstring(package.read('visio/pages/page1.xml')).findall('.//{*}Connect'))
    if connects != 20:
        raise RuntimeError(f'expected 20 Connects, got {connects}')
    receipt = dict(build_commands=len(commands) + len(decorations), build_tool_calls=build_calls, total_tool_calls=client.tool_calls,
                   recount_comparable_commands=len(commands) + len(decorations) + 1,
                   devices=5, fabric_boxes=2, titles=1, links=10, connects=connects, layout=checked)
    (output / 'receipt.json').write_text(json.dumps(receipt, indent=2) + '\n')
    print(json.dumps(receipt))


if __name__ == '__main__':
    parser = argparse.ArgumentParser(description=__doc__)
    parser.add_argument('output', type=Path)
    parser.add_argument('--cli', type=Path, default=default_binary())
    args = parser.parse_args()
    build(args.cli, args.output)
