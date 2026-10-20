#!/usr/bin/env python3
"""Contributor-original strict workloads for #39. MIT OR Apache-2.0.

All names, geometry and bitmap pixels are invented here. No external assets.
The minimal stencil uses the public Visio XML format:
https://learn.microsoft.com/en-us/office/client-developer/visio/foreigndata-element-shapesheet_type-complextypevisio-xml
https://learn.microsoft.com/en-us/office/client-developer/visio/master-element-masters_type-complextypevisio-xml
"""
import argparse
import json
from pathlib import Path
import struct
import zipfile
import zlib

VIS = "http://schemas.microsoft.com/office/visio/2012/main"
REL = "http://schemas.openxmlformats.org/officeDocument/2006/relationships"
VREL = "http://schemas.microsoft.com/visio/2010/relationships/"


def cell(name, value):
    return f'<Cell N="{name}" V="{value}"/>'


def transform(width=1, height=1):
    return ''.join(cell(n, v) for n, v in [
        ('PinX', width / 2), ('PinY', height / 2), ('Width', width),
        ('Height', height), ('LocPinX', width / 2), ('LocPinY', height / 2),
        ('Angle', 0), ('FlipX', 0), ('FlipY', 0)])


def rectangle(sid, width=1, height=1):
    geometry = '<Section N="Geometry" IX="0">'
    for ix, (x, y) in enumerate([(0, 0), (width, 0), (width, height), (0, height), (0, 0)], 1):
        geometry += f'<Row IX="{ix}" T="{"MoveTo" if ix == 1 else "LineTo"}">{cell("X", x)}{cell("Y", y)}</Row>'
    return f'<Shape ID="{sid}" Type="Shape">{transform(width, height)}{geometry}</Section></Shape>'


def stencil(path):
    """Two group masters, with two geometric children or one bitmap child."""
    def rels(items):
        return ('<Relationships xmlns="http://schemas.openxmlformats.org/package/2006/relationships">'
                + ''.join(f'<Relationship Id="{i}" Type="{t}" Target="{p}"/>' for i, t, p in items)
                + '</Relationships>')

    def chunk(kind, payload):
        return struct.pack('>I', len(payload)) + kind + payload + struct.pack('>I', zlib.crc32(kind + payload))

    # An original two-tone grid, deliberately without an icon silhouette.
    pixels = b''.join(b'\0' + b''.join(bytes((32, 112, 144) if (x + y) % 2 else (208, 224, 232))
                                    for x in range(8)) for y in range(8))
    png = (b'\x89PNG\r\n\x1a\n' + chunk(b'IHDR', struct.pack('>IIBBBBB', 8, 8, 8, 2, 0, 0, 0))
           + chunk(b'IDAT', zlib.compress(pixels)) + chunk(b'IEND', b''))
    points = '<Section N="Connection">'
    for i in range(96):
        points += f'<Row N="Port{i + 1}" IX="{i}" T="Connection">'
        points += ''.join(cell(n, v) for n, v in [('X', (i % 16 + .5) / 8), ('Y', (i // 16 + .5) / 6),
                                                ('DirX', 0), ('DirY', 0), ('Type', 0)])
        points += '</Row>'
    points += '</Section>'
    director = (f'<Shape ID="5" Type="Group">{transform(2, 1)}{points}'
                f'<Shapes>{rectangle(6, 2, 1)}{rectangle(7, .25, .25)}</Shapes></Shape>')
    picture = (f'<Shape ID="6" Type="Foreign">{transform()}{cell("ImgWidth", 1)}{cell("ImgHeight", 1)}'
               '<ForeignData ForeignType="Bitmap"><Rel r:id="rImage"/></ForeignData></Shape>')
    host = f'<Shape ID="5" Type="Group">{transform()}<Shapes>{picture}</Shapes></Shape>'
    parts = {
        '_rels/.rels': rels([('rDoc', VREL + 'document', 'visio/document.xml')]),
        'visio/document.xml': f'<VisioDocument xmlns="{VIS}"><DocumentSheet/></VisioDocument>',
        'visio/_rels/document.xml.rels': rels([('rMasters', VREL + 'masters', 'masters/masters.xml')]),
        'visio/masters/masters.xml': (
            f'<Masters xmlns="{VIS}" xmlns:r="{REL}">'
            '<Master ID="1" Name="Aster Director" NameU="Aster Director"><Rel r:id="r1"/></Master>'
            '<Master ID="2" Name="Vela Host" NameU="Vela Host"><Rel r:id="r2"/></Master></Masters>'),
        'visio/masters/_rels/masters.xml.rels': rels([
            ('r1', VREL + 'master', 'master1.xml'), ('r2', VREL + 'master', 'master2.xml')]),
        'visio/masters/_rels/master2.xml.rels': rels([('rImage', REL + '/image', '../media/grid.png')]),
        'visio/media/grid.png': png,
    }
    for i, shapes in [(1, director), (2, host)]:
        parts[f'visio/masters/master{i}.xml'] = (
            f'<MasterContents xmlns="{VIS}" xmlns:r="{REL}"><Shapes>{shapes}</Shapes></MasterContents>')
    types = {'visio/document.xml': 'stencil.main', 'visio/masters/masters.xml': 'masters',
             'visio/masters/master1.xml': 'master', 'visio/masters/master2.xml': 'master'}
    parts['[Content_Types].xml'] = (
        '<Types xmlns="http://schemas.openxmlformats.org/package/2006/content-types">'
        '<Default Extension="rels" ContentType="application/vnd.openxmlformats-package.relationships+xml"/>'
        '<Default Extension="png" ContentType="image/png"/>'
        + ''.join(f'<Override PartName="/{name}" ContentType="application/vnd.ms-visio.{kind}+xml"/>'
                  for name, kind in types.items()) + '</Types>')
    with zipfile.ZipFile(path, 'w', zipfile.ZIP_DEFLATED) as package:
        for name, data in parts.items():
            package.writestr(name, data)


def shapes377():
    commands = [('file.new', {})]
    for i in range(377):
        sid = i + 1
        commands += [
            ('shape.draw.rect', dict(x=1 + i % 20, y=1 + i // 20, width=.75, height=.5,
                                     fill={'color': '#d0e0e8'}, line={'color': '#207090'})),
            ('shape.text', dict(id=sid, text=f'Orbit_{sid:03}')),
            ('shape.data.define', dict(id=sid, name='Role')),
            ('shape.data.set', dict(id=sid, name='Role', value='Invented endpoint')),
            ('shape.data.define', dict(id=sid, name='Zone')),
            ('shape.data.set', dict(id=sid, name='Zone', value=f'Vela_{i % 7}'))]
        if i < 237:
            commands.append(('format.line', dict(id=sid, weight=.75, units='PT')))
    assert len(commands) == 2500
    return commands


def sanlarge(path):
    commands = [('file.new', {}), ('page.size', dict(width=48, height=32)),
                ('stencil.open', dict(path=str(path)))]
    sid = 1
    devices = []
    directors = []
    switches = [[], []]
    hosts = [[], []]
    # 312 top-level shapes plus 68 group children: 380 shapes in total.
    for fabric in range(2):
        director = sid
        directors.append(sid)
        commands += [('master.drop', dict(stencil=1, master='Aster Director', x=6 + fabric * 24, y=28)),
                     ('shape.text', dict(id=sid, text=f'Aster_Fabric_{fabric + 1}'))]
        devices.append(sid)
        sid += 3
        for i in range(12):
            commands += [('shape.draw.rect', dict(x=2 + fabric * 24 + i % 6 * 3, y=22 - i // 6 * 3, width=2, height=1)),
                         ('shape.text', dict(id=sid, text=f'Lyra_{fabric + 1}_{i + 1}'))]
            devices.append(sid)
            switches[fabric].append(sid)
            sid += 1
        for i in range(32):
            commands += [('master.drop', dict(stencil=1, master='Vela Host', x=2 + fabric * 24 + i % 8 * 2.5, y=3 + i // 8 * 3)),
                         ('shape.text', dict(id=sid, text=f'Vela_{fabric + 1}_{i + 1}'))]
            devices.append(sid)
            hosts[fabric].append(sid)
            sid += 2
        # Plain rectangles, no container semantics.
        commands += [('shape.draw.rect', dict(x=1 + fabric * 24, y=1, width=22, height=29)),
                     ('format.fill', dict(id=sid, pattern=0)),
                     ('shape.text', dict(id=sid, text=f'Fabric_{fabric + 1}'))]
        sid += 1
        for i in range(12):
            commands += [('shape.draw.rect', dict(x=2 + fabric * 24 + i % 6 * 3, y=17 - i // 6, width=2, height=.5)),
                         ('format.fill', dict(id=sid, pattern=0)),
                         ('format.line', dict(id=sid, pattern=0)),
                         ('shape.text', dict(id=sid, text=f'Lane_{fabric + 1}_{i + 1}'))]
            sid += 1
    for index, device in enumerate(devices):
        commands += [('format.fill', dict(id=device, color='#d0e0e8')),
                     ('format.line', dict(id=device, color='#207090', weight=1, units='PT')),
                     ('shape.data.define', dict(id=device, name='Zone')),
                     ('shape.data.set', dict(id=device, name='Zone', value=f'Orbit_{index % 2 + 1}'))]
        if device not in hosts[0] + hosts[1]:
            commands.append(('shape.connection.add', dict(id=device, name='Link', x='Width*0.5', y='Height')))
    for i in range(196):
        fabric = (i // 2) % 2
        targets = hosts[fabric] if i % 2 else switches[fabric]
        target = targets[(i // 4) % len(targets)]
        source = directors[fabric]
        commands += [('master.drop', dict(master=2, x=0, y=0)),
                     ('sheet.set', dict(id=sid, name='ShapeRouteStyle', value=2)),
                     ('connect.static', dict(id=sid, end='begin', target=source, point=f'Connections.Port{i % 96 + 1}.X')),
                     (('connect.dynamic', dict(id=sid, end='end', target=target)) if i % 2
                      else ('connect.static', dict(id=sid, end='end', target=target, point='Connections.Link.X'))),
                     ('shape.text', dict(id=sid, text=f'Circuit_{i + 1:03}')),
                     ('format.line', dict(id=sid, color='#207090', weight=.5, units='PT'))]
        sid += 1
    for i in range(2500 - len(commands)):
        commands.append(('shape.data.set', dict(id=devices[i % len(devices)], name='Zone', value=f'Orbit_lane_{i + 1}')))
    assert len(commands) == 2500
    assert sid == 381
    return commands


def largeedit():
    commands = []
    for i in range(4):
        commands += [('shape.move', dict(id=i + 1, x=2 + i, y=3)),
                     ('format.fill', dict(id=i + 1, color='#90b0c0'))]
    # Each addition is a real glued connector; each deletion removes it again.
    # The engine allocates max(existing ID)+1, so deleting the last link allows
    # the second link to reuse 378.
    for i in range(2):
        commands += [('master.drop', dict(master=2, x=0, y=0)),
                     ('sheet.set', dict(id=378, name='ShapeRouteStyle', value=2)),
                     ('connect.dynamic', dict(id=378, end='begin', target=1)),
                     ('connect.dynamic', dict(id=378, end='end', target=2)),
                     ('format.line', dict(id=378, color='#207090')),
                     ('shape.delete', dict(id=378))]
    assert len(commands) == 20
    return commands


def main():
    parser = argparse.ArgumentParser(description=__doc__)
    parser.add_argument('output', type=Path)
    args = parser.parse_args()
    out = args.output.resolve()
    out.mkdir(parents=True, exist_ok=True)
    stencil(out / 'invented.vssx')
    for name, commands in [('shapes-377', shapes377()), ('san-large', sanlarge(out / 'invented.vssx')),
                           ('large-edit', largeedit())]:
        (out / f'{name}.json').write_text(json.dumps(commands, indent=2) + '\n')
        print(name, len(commands))


if __name__ == '__main__':
    main()
