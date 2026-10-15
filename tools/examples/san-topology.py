#!/usr/bin/env python3
"""Contributor-original SAN command recipe. MIT OR Apache-2.0.

https://learn.microsoft.com/en-us/office/vba/api/visio.masters.add
https://learn.microsoft.com/en-us/office/vba/api/visio.page.drop
https://learn.microsoft.com/en-us/office/vba/api/visio.cell.glueto
https://learn.microsoft.com/en-us/office/client-developer/visio/routestyle-cell-page-layout-section
https://learn.microsoft.com/en-us/office/client-developer/visio/linejumpstyle-cell-page-layout-section
https://learn.microsoft.com/en-us/office/client-developer/visio/linejumpcode-cell-page-layout-section
"""
import json
from pathlib import Path
import subprocess
import sys


def main():
    output = Path(sys.argv[1])
    cli = sys.argv[2:]
    drawing = output / 'san-topology.vsdx'
    commands = []

    def command(command_id, **params):
        commands.append(command_id + '=' + json.dumps(params, separators=(',', ':')))

    command('file.new')
    command('page.size', width=17, height=11)
    command('sheet.set', scope='page', name='RouteStyle', value=1)
    command('sheet.set', scope='page', name='LineJumpStyle', value=1)
    command('sheet.set', scope='page', name='LineJumpCode', value=1)
    for fabric in ('A', 'B'):
        command('layer.add', name='Fabric ' + fabric)
    command('shape.draw.rect', x=0, y=0, width=1.5, height=0.75)
    command('shape.text', id=1, text='Switch')
    command('format.text', id=1, font='Liberation Sans', size=10, align=1)
    command('master.new_from_shape', id=1, name='Switch')
    command('shape.delete', id=1)

    fabrics = []
    shape_id = 1
    for fabric_index, fabric in enumerate(('A', 'B')):
        left = fabric_index * 8.5
        positions = [(2.5, 8.75), (5.7, 8.75)]
        positions += [(0.9 + column * 1.6, y) for y in (5.75, 2.75) for column in range(5)]
        ids = []
        for index, (x, y) in enumerate(positions):
            name = f'{fabric}-Core-{index + 1}' if index < 2 else f'{fabric}-Edge-{index - 1:02}'
            command('master.drop', master='Switch', x=left + x, y=y)
            command('shape.text', id=shape_id, text=name)
            command('layer.assign', id=shape_id, layers=[fabric_index])
            for prop, value in [('Name', name), ('Fabric', fabric), ('DomainID', fabric_index * 100 + index + 1), ('Model', 'DC-SW48')]:
                command('shape.data.define', id=shape_id, name=prop, label=prop, type=2 if prop == 'DomainID' else 0, format='', invisible=False, sort_key=prop)
                command('shape.data.set', id=shape_id, name=prop, value=value)
            ids.append(shape_id)
            shape_id += 1
        fabrics.append(ids)
    for fabric_index, fabric in enumerate(('A', 'B')):
        command('shape.draw.rect', x=fabric_index * 8.5 + 2.5, y=9.75, width=3.2, height=0.6)
        command('shape.text', id=shape_id, text='Fabric ' + fabric)
        command('format.fill', id=shape_id, pattern=0)
        command('format.line', id=shape_id, pattern=0)
        command('format.text', id=shape_id, font='Liberation Sans', size=16, align=1, bold=True)
        command('layer.assign', id=shape_id, layers=[fabric_index])
        # Labels do not block routes.
        command('sheet.set', id=shape_id, name='ObjType', value=0)
        shape_id += 1
    for fabric_index, ids in enumerate(fabrics):
        for edge in ids[2:]:
            for core in ids[:2]:
                command('master.drop', master=2, x=0, y=0)
                for end, target in [('begin', edge), ('end', core)]:
                    command('connect.dynamic', id=shape_id, end=end, target=target)
                command('layer.assign', id=shape_id, layers=[fabric_index, 2])
                shape_id += 1
    command('route.reroute')

    # Keep each delivered command line below the Windows CreateProcess limit.
    # https://learn.microsoft.com/en-us/windows/win32/api/processthreadsapi/nf-processthreadsapi-createprocessw
    for offset in range(0, len(commands), 40):
        args = cli + ['run']
        if offset:
            args += ['--open', str(drawing)]
        for item in commands[offset:offset + 40]:
            args += ['--cmd', item]
        args += ['--save', str(drawing)]
        result = subprocess.run(args, capture_output=True, text=True)
        if result.returncode:
            print(f'SAN command batch {offset}: ' + '\n'.join(result.stdout.splitlines()[-2:]), file=sys.stderr)
            print(result.stderr, file=sys.stderr, end='')
            return result.returncode
    for extension in ('png', 'svg'):
        subprocess.run(cli + ['convert', str(drawing), str(drawing.with_suffix('.' + extension))], check=True)
    print(f'SAN: 24 Switch instances, 40 ISLs, saved {drawing}')
    return 0


if __name__ == '__main__':
    sys.exit(main())
