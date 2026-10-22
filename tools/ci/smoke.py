#!/usr/bin/env python3
"""Smoke-test one misterdiagram binary in an empty folder, with no repository files (MIT OR Apache-2.0).

Usage: smoke.py BINARY --version V --commit C --target T
C is a 12-hex commit or the words "unknown commit". Standard library only, so it runs on every
release runner and on a tester's machine. Exit 0 when every check passes, 1 otherwise.
"""
import argparse
import json
import os
from pathlib import Path
import queue
import shutil
import struct
import subprocess
import sys
import tempfile
import threading
import xml.etree.ElementTree as ET
import zlib

TIMEOUT = 300
REPLY_TIMEOUT = 120
BATCH = [
    {'command': 'stencil.open', 'params': {'builtin': 'network'}},
    {'command': 'master.drop', 'params': {'master': 'network.router', 'x': 2, 'y': 4}, 'as': 'a'},
    {'command': 'master.drop', 'params': {'master': 'compute.server', 'x': 5, 'y': 4}, 'as': 'b'},
    {'command': 'connector.add', 'params': {'from': '$a', 'to': '$b'}, 'as': 'c'},
]
failures = []


def check(label, ok, detail=''):
    print(('ok    ' if ok else 'FAIL  ') + label + (': ' + detail if detail and not ok else ''), flush=True)
    if not ok:
        failures.append(label)
    return ok


def run(binary, *args):
    r = subprocess.run([str(binary), *args], capture_output=True, text=True, encoding='utf-8', timeout=TIMEOUT)
    return r.returncode, r.stdout, r.stderr


def png_pixels(path):
    """Decode an 8-bit RGB or RGBA non-interlaced PNG; return (width, height, rows of bytes)."""
    data = path.read_bytes()
    if data[:8] != b'\x89PNG\r\n\x1a\n':
        raise ValueError('not a PNG')
    pos, idat, header = 8, b'', None
    while pos < len(data):
        length, kind = struct.unpack('>I4s', data[pos:pos + 8])
        body = data[pos + 8:pos + 8 + length]
        if kind == b'IHDR':
            header = struct.unpack('>IIBBBBB', body)
        elif kind == b'IDAT':
            idat += body
        pos += 12 + length
    width, height, depth, colour, _, _, interlace = header
    if depth != 8 or colour not in (2, 6) or interlace:
        raise ValueError(f'unsupported PNG form depth={depth} colour={colour} interlace={interlace}')
    bpp = 3 if colour == 2 else 4
    raw, stride, rows, prev = zlib.decompress(idat), width * bpp, [], bytearray(width * bpp)
    for y in range(height):
        kind, line = raw[y * (stride + 1)], bytearray(raw[y * (stride + 1) + 1:(y + 1) * (stride + 1)])
        for i in range(stride):
            a = line[i - bpp] if i >= bpp else 0
            b, c = prev[i], prev[i - bpp] if i >= bpp else 0
            if kind == 1:
                line[i] = (line[i] + a) & 255
            elif kind == 2:
                line[i] = (line[i] + b) & 255
            elif kind == 3:
                line[i] = (line[i] + (a + b) // 2) & 255
            elif kind == 4:
                p = a + b - c
                pa, pb, pc = abs(p - a), abs(p - b), abs(p - c)
                line[i] = (line[i] + (a if pa <= pb and pa <= pc else b if pb <= pc else c)) & 255
        rows.append(bytes(line))
        prev = line
    return width, height, rows


def mcp_session(binary, work, version):
    stderr = open(work / 'mcp-stderr.txt', 'w+', encoding='utf-8')
    proc = subprocess.Popen([str(binary), 'mcp'], cwd=work, stdin=subprocess.PIPE, stdout=subprocess.PIPE,
                            stderr=stderr, text=True, encoding='utf-8')
    ids, lines = iter(range(1, 100)), queue.Queue()
    threading.Thread(target=lambda: [lines.put(line) for line in iter(proc.stdout.readline, '')] and None, daemon=True).start()

    def request(method, params):
        n = next(ids)
        proc.stdin.write(json.dumps({'jsonrpc': '2.0', 'id': n, 'method': method, 'params': params}) + '\n')
        proc.stdin.flush()
        try:
            line = lines.get(timeout=REPLY_TIMEOUT)
        except queue.Empty:
            raise RuntimeError(f'{method}: no reply within {REPLY_TIMEOUT} s') from None
        reply = json.loads(line) if line else {}
        if reply.get('id') != n or 'error' in reply:
            raise RuntimeError(f'{method}: {line.strip() or "no reply"}')
        return reply['result']

    def tool(name, **arguments):
        result = request('tools/call', {'name': name, 'arguments': arguments})
        if result.get('isError'):
            raise RuntimeError(f'{name}: {result["content"][0]["text"]}')
        return json.loads(result['content'][0]['text'])

    try:
        init = request('initialize', {'protocolVersion': '2025-06-18', 'capabilities': {},
                                      'clientInfo': {'name': 'misterdiagram-smoke', 'version': '1'}})
        check('mcp initialize', init.get('serverInfo') == {'name': 'misterdiagram', 'version': version}, json.dumps(init.get('serverInfo')))
        proc.stdin.write(json.dumps({'jsonrpc': '2.0', 'method': 'notifications/initialized'}) + '\n')
        proc.stdin.flush()
        names = {t['name'] for t in request('tools/list', {})['tools']}
        check('mcp tools/list', {'batch', 'save', 'render', 'run_command', 'inspect_document'} <= names and len(names) >= 16, str(sorted(names)))
        result = tool('batch', commands=[{'command': 'file.new', 'params': {}}] + BATCH)
        check('mcp batch', result.get('references', {}).keys() >= {'a', 'b', 'c'}, json.dumps(result)[:300])
        saved = tool('save', path='mcp.vsdx')
        check('mcp save', (work / 'mcp.vsdx').is_file() and saved.get('warning_count') == 0, json.dumps(saved)[:300])
    except Exception as error:  # report, then still close the session
        check('mcp session', False, repr(error))
    try:
        proc.stdin.close()
    except OSError:
        pass
    try:
        code = proc.wait(timeout=60)
        stderr.seek(0)
        check('mcp exits cleanly on EOF', code == 0, f'exit {code}: {stderr.read()[-300:]}')
    except subprocess.TimeoutExpired:
        proc.kill()
        proc.wait()
        check('mcp exits cleanly on EOF', False, 'still running 60 s after EOF')
    stderr.close()


def main():
    parser = argparse.ArgumentParser(description=__doc__.splitlines()[0])
    parser.add_argument('binary')
    parser.add_argument('--version', required=True)
    parser.add_argument('--commit', required=True)
    parser.add_argument('--target', required=True)
    args = parser.parse_args()
    binary = Path(args.binary).resolve()
    work = Path(tempfile.mkdtemp(prefix='misterdiagram-smoke-'))
    os.chdir(work)
    print(f'smoke: {binary} in {work}', flush=True)
    try:
        code, out, err = run(binary, '--version')
        want = f'misterdiagram {args.version} ({args.commit}, {args.target})'
        check('--version', code == 0 and out.strip() == want, f'got {out.strip()!r}, want {want!r}')
        print('      ' + out.strip())

        code, out, err = run(binary, 'commands')
        try:
            ids = {c['id'] for c in json.loads(out) if c.get('params', {}).get('type') == 'object'}
        except (ValueError, TypeError, KeyError):
            ids = set()
        check('commands lists the command schemas', code == 0 and {'master.drop', 'connector.add', 'file.save'} <= ids, err[-300:])

        (work / 'batch.json').write_text(json.dumps({'commands': BATCH}), encoding='utf-8')
        code, out, err = run(binary, 'run', '--batch', 'batch.json', '--save', 'two-boxes.vsdx')
        check('run --batch builds and saves a glued two-box drawing', code == 0 and (work / 'two-boxes.vsdx').is_file(), (out + err)[-300:])

        code, out, err = run(binary, 'info', 'two-boxes.vsdx')
        check('info reads it back', code == 0 and 'visio/pages/page1.xml' in out, err[-300:])
        code, out, err = run(binary, 'sheet', 'two-boxes.vsdx', '--shape', '3')
        cells = {f[5]: f[6] for f in (line.split('\t') for line in out.splitlines()) if len(f) > 6}
        check('the connector is glued at both ends', code == 0 and 'Sheet.1!' in cells.get('BeginX', '') and 'Sheet.2!' in cells.get('EndX', ''),
              f'BeginX={cells.get("BeginX")!r} EndX={cells.get("EndX")!r}')

        code, out, err = run(binary, 'render', 'two-boxes.vsdx', '--page', '1', 'two-boxes.png')
        try:
            width, height, rows = png_pixels(work / 'two-boxes.png')
            distinct = len({row[i:i + 3] for row in rows for i in range(0, len(row), 3 if len(row) == width * 3 else 4)})
        except (OSError, ValueError, zlib.error, TypeError) as error:
            width = height = distinct = 0
            err += repr(error)
        check('render PNG is a nonempty image', code == 0 and width > 0 and height > 0 and distinct > 2, f'{width}x{height}, {distinct} colours; {err[-200:]}')

        code, out, err = run(binary, 'render', 'two-boxes.vsdx', '--page', '1', 'two-boxes.svg')
        try:
            root = ET.parse(work / 'two-boxes.svg').getroot()
            marks = sum(1 for e in root.iter() if e.tag.rsplit('}', 1)[-1] in ('path', 'rect', 'polyline', 'polygon', 'line', 'ellipse', 'circle', 'image'))
        except (OSError, ET.ParseError) as error:
            marks = 0
            err += repr(error)
        check('render SVG is a nonempty image', code == 0 and marks >= 3, f'{marks} drawing elements; {err[-200:]}')

        mcp_session(binary, work, args.version)
        if (work / 'mcp.vsdx').is_file():
            code, out, err = run(binary, 'info', 'mcp.vsdx')
            check('info reads the MCP drawing back', code == 0 and 'visio/pages/page1.xml' in out, err[-300:])
    finally:
        os.chdir(Path(tempfile.gettempdir()))
        shutil.rmtree(work, ignore_errors=True)
    print('SMOKE ' + ('PASS' if not failures else 'FAIL: ' + ', '.join(failures)), flush=True)
    return 1 if failures else 0


if __name__ == '__main__':
    sys.exit(main())
