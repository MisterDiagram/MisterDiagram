#!/usr/bin/env python3
"""Build the 24-switch, 40-ISL SAN through one MCP batch. MIT OR Apache-2.0."""
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
    command('page.size', width=17, height=11)
    command('sheet.set', scope='page', name='RouteStyle', value=1)
    command('sheet.set', scope='page', name='LineJumpStyle', value=1)
    for fabric in ('A', 'B'):
        command('layer.add', name='Fabric ' + fabric)
    command('stencil.open', builtin='network')
    fabrics = []
    for fabric_index, fabric in enumerate(('A', 'B')):
        ids = []
        for index in range(12):
            alias = f'{fabric}-device-{index}'
            ref = '$' + alias
            name = f'{fabric}-Core-{index + 1}' if index < 2 else f'{fabric}-Edge-{index - 1:02}'
            command('master.drop', alias, master='storage.switch', text=name)
            command('layer.assign', id=ref, layers=[fabric_index])
            for prop, value in [('Name', name), ('Fabric', fabric), ('DomainID', fabric_index * 100 + index + 1), ('Model', 'DC-SW48')]:
                command('shape.data.define', id=ref, name=prop, label=prop, type=2 if prop == 'DomainID' else 0,
                        format='', invisible=False, sort_key=prop)
                command('shape.data.set', id=ref, name=prop, value=value)
            ids.append(ref)
        fabrics.append(ids)
        command('layout.rows', ids=ids[2:7] + ids[:2] + ids[7:], counts=[5, 2, 5],
                region={'x': fabric_index * 8.5 + 0.3125, 'y': 1.5, 'width': 7.875, 'height': 8},
                gutter_x=0.09375, gutter_y=2)
    for fabric_index, fabric in enumerate(('A', 'B')):
        alias = fabric + '-label'
        command('shape.draw.rect', alias, x=fabric_index * 8.5 + 2.5, y=9.75, width=3.1875, height=0.625,
                text='Fabric ' + fabric, fill={'pattern': 0}, line={'pattern': 0}, font={'size': 16, 'align': 1, 'bold': True})
        command('layer.assign', id='$' + alias, layers=[fabric_index])
    for fabric_index, ids in enumerate(fabrics):
        for edge_index, edge in enumerate(ids[2:]):
            for core_index, core in enumerate(ids[:2]):
                alias = f'link-{fabric_index}-{edge_index}-{core_index}'
                command('connector.add', alias, **{'from': edge, 'to': core})
                command('layer.assign', id='$' + alias, layers=[fabric_index, 2])
    for ids in fabrics:
        for hub in ids[:2]:
            command('layout.fan', hub=hub, spokes=ids[2:])
    command('route.reroute')
    return commands


def build(binary, output):
    output = Path(output).resolve()
    output.mkdir(parents=True, exist_ok=True)
    commands = recipe()
    with Client(binary) as client:
        result = client.tool('batch', commands=commands)
        for item, value in zip(commands, result['results']):
            if item['command'] == 'layout.fan' and value['crossings']:
                raise RuntimeError(f'route crosses a switch: {value["crossings"]}')
        build_calls = client.tool_calls
        checked = client.command('layout.check')
        if checked['count']:
            raise RuntimeError(f'layout violations: {checked}')
        drawing = output / 'p5a-san-topology.vsdx'
        client.tool('save', path=str(drawing))
        for extension in ('png', 'svg'):
            client.tool('export', path=str(output / f'p5a-san-topology.{extension}'))
    with zipfile.ZipFile(drawing) as package:
        connects = len(ET.fromstring(package.read('visio/pages/page1.xml')).findall('.//{*}Connect'))
    if connects != 80:
        raise RuntimeError(f'expected 80 Connects, got {connects}')
    receipt = dict(build_commands=len(commands), build_tool_calls=build_calls, total_tool_calls=client.tool_calls,
                   devices=24, links=40, connects=connects, layout=checked)
    (output / 'receipt.json').write_text(json.dumps(receipt, indent=2) + '\n')
    print(json.dumps(receipt))


if __name__ == '__main__':
    parser = argparse.ArgumentParser(description=__doc__)
    parser.add_argument('output', type=Path)
    parser.add_argument('--cli', type=Path, default=default_binary())
    args = parser.parse_args()
    build(args.cli, args.output)
