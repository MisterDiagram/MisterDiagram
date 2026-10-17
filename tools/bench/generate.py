#!/usr/bin/env python3
"""Generate contributor-original authoring workloads, without vendor assets."""
import json
import pathlib
import sys

out = pathlib.Path(sys.argv[1])
out.mkdir(parents=True, exist_ok=True)

def save(name, commands):
    (out / (name + '.json')).write_text(json.dumps(commands, indent=2) + '\n')
    print(name, len(commands))

for count in (25, 50, 100):
    commands = [('file.new', {})]
    for i in range(count):
        commands.append(('shape.draw.rect', dict(x=1+i%10, y=1+i//10, width=0.8, height=0.6)))
    save(f'rectangles-{count}', commands)
commands = [('file.new', {})]
for i in range(40):
    commands += [('shape.draw.rect', dict(x=1+i%10, y=1+i//10, width=0.8, height=0.6)),
                 ('shape.data.define', dict(id=i+1, name='Role', label='Invented role')),
                 ('shape.data.set', dict(id=i+1, name='Role', value=f'Orbit_{i+1}'))]
save('data-40', commands)
commands = [('file.new', {})]
# One invented director, 58 port symbols and 46 endpoint symbols. The 58
# connector instances below bring the page to 163 shapes. Permeability is the
# documented baseline workaround for #39's existing 60-obstacle refusal.
for i in range(105):
    name = 'Aster_192' if i == 0 else f'Vela_{i:03}'
    commands += [('shape.draw.rect', dict(x=1+(i%13)*1.5, y=1+(i//13)*1.1, width=1, height=0.7)),
                 ('shape.text', dict(id=i+1, text=name))]
    for prop, value in [('Role', 'Director' if i == 0 else 'Port' if i <= 58 else 'Endpoint'),
                        ('Capacity', '192' if i == 0 else '1'), ('Label', name)]:
        commands += [('shape.data.define', dict(id=i+1, name=prop)),
                     ('shape.data.set', dict(id=i+1, name=prop, value=value))]
    for cell in ('ShapePermeableX', 'ShapePermeableY'):
        commands.append(('sheet.set', dict(id=i+1, name=cell, formula='1')))
    commands.append(('shape.connection.add', dict(id=i+1, name='Link', x=0.5, y=0.35)))
for port in range(192):
    commands.append(('shape.connection.add', dict(id=1, name=f'Port_{port+1:03}', x=(port%16+0.5)/16, y=(port//16+0.5)/12*0.7)))
for i in range(58):
    shape = 106+i
    commands += [('master.drop', dict(master=2, x=0, y=0)),
                 ('sheet.set', dict(id=shape, name='ShapeRouteStyle', formula='2')),
                 ('connect.static', dict(id=shape, end='begin', target=2+i, point='Connections.Link.X')),
                 ('connect.static', dict(id=shape, end='end', target=60+i%46, point='Connections.Link.X')),
                 ('shape.data.define', dict(id=shape, name='Circuit')),
                 ('shape.data.set', dict(id=shape, name='Circuit', value=f'Lyra_{i+1:03}')),
                 ('shape.text', dict(id=shape, text=f'Lyra_{i+1:03}'))]
# Fill each remaining command with a meaningful final port label, cycling through
# the 58 connected port symbols. All phases and final command count are stable.
for i in range(1900-len(commands)):
    commands.append(('shape.data.set', dict(id=2+i%58, name='Label', value=f'Aster_port_{i%58+1:03}_lane_{i//58+1}')))
save('ports-1900', commands)
