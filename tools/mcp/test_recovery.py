#!/usr/bin/env python3
"""Real stdio recovery. Panic cases require the non-default test-panic-injection build."""
import json
import os
from pathlib import Path
import queue
import subprocess
import sys
import threading
import unittest

ROOT = Path(__file__).resolve().parents[2]
CLI = Path(os.environ.get('MISTERDIAGRAM_CLI', ROOT / 'target/debug/misterdiagram')).resolve()


class Wire:
    def __init__(self, point=''):
        self.p = subprocess.Popen([str(CLI), 'mcp'], stdin=subprocess.PIPE, stdout=subprocess.PIPE,
                                  stderr=subprocess.PIPE, env=dict(os.environ, MISTERDIAGRAM_TEST_PANIC=point))
        self.closed = False
        self.q = queue.Queue()
        self.errors = []
        def read():
            for line in self.p.stdout:
                self.q.put(json.loads(line))
            self.q.put(None)
        def stderr():
            self.errors.append(self.p.stderr.read())
        threading.Thread(target=read, daemon=True).start()
        self.stderr_thread = threading.Thread(target=stderr, daemon=True)
        self.stderr_thread.start()
        self.id = 0

    def send(self, message):
        self.p.stdin.write(message)
        self.p.stdin.flush()
        reply = self.q.get(timeout=20)
        assert reply is not None, 'server exited instead of replying'
        return reply

    def request(self, method, params=None):
        self.id += 1
        reply = self.send(json.dumps(dict(jsonrpc='2.0', id=self.id, method=method, params=params or {})).encode() + b'\n')
        assert reply['id'] == self.id, reply
        return reply

    def tool(self, name, /, **arguments):
        reply = self.request('tools/call', dict(name=name, arguments=arguments))
        assert 'error' not in reply, reply
        result = reply['result']
        assert not result.get('isError'), result
        return json.loads(result['content'][0]['text'])

    def command(self, command, **params):
        return self.tool('run_command', command=command, params=params)

    def close(self):
        if self.closed:
            return
        self.closed = True
        self.p.stdin.close()
        try:
            code = self.p.wait(timeout=20)
        except subprocess.TimeoutExpired:
            self.p.kill()
            self.p.wait()
            raise
        self.stderr_thread.join(timeout=5)
        self.p.stdout.close()
        self.p.stderr.close()
        assert code == 0, (code, self.errors)


class Recovery(unittest.TestCase):
    def prepared(self, point=''):
        wire = Wire(point)
        self.addCleanup(wire.close)
        wire.command('stencil.open', builtin='network')
        wire.command('shape.draw.rect', x=1, y=1, width=2, height=1)
        wire.command('shape.move', id=1, x=3, y=3)
        wire.command('edit.undo')
        out = ROOT / 'target/mcp-recovery' / str(os.getpid()) / self._testMethodName
        out.mkdir(parents=True, exist_ok=True)
        wire.tool('save', path=str(out / 'before.vsdx'))
        return wire, out, wire.tool('inspect_document'), wire.tool('stencil_list')

    def preserved(self, wire, out, before, stencils):
        self.assertEqual(before, wire.tool('inspect_document'))
        self.assertEqual(stencils, wire.tool('stencil_list'))
        wire.tool('save', path=str(out / 'after.vsdx'))
        subprocess.run([sys.executable, str(ROOT / 'tools/bench/compare.py'),
                        str(out / 'before.vsdx'), str(out / 'after.vsdx')], check=True)
        wire.command('edit.redo')
        self.assertEqual(wire.tool('sheet_get', id=1, name='PinX')['value'], '3')
        wire.command('shape.text', id=1, text='Still serving')

    def test_oversized_then_tools_list(self):
        wire, *state = self.prepared()
        reply = wire.send(b'x' * (5 * 1024 * 1024) + b'\n')
        self.assertEqual(reply['id'], None)
        self.assertEqual(reply['error']['message'], 'MCP message exceeds cap')
        self.assertEqual(reply['error']['code'], -32700)
        self.assertEqual(len(wire.request('tools/list')['result']['tools']), 17)
        self.preserved(wire, *state)

    def test_invalid_utf8_then_tools_list(self):
        wire, *state = self.prepared()
        reply = wire.send(b'\xff\xfe\n')
        self.assertEqual(reply['id'], None)
        self.assertEqual(reply['error']['code'], -32700)
        self.assertEqual(len(wire.request('tools/list')['result']['tools']), 17)
        self.preserved(wire, *state)

    @unittest.skipUnless(os.environ.get('MISTERDIAGRAM_TEST_INJECTIONS') == '1', 'requires test-panic-injection binary')
    def test_composition_orchestration_panic(self):
        wire, *state = self.prepared('composition-orchestration')
        reply = wire.request('tools/call', dict(name='run_command', arguments=dict(command='shape.draw.rect', params=dict(x=2, y=2, width=1, height=1, text='Oak'))))
        self.assertTrue(reply['result']['isError'])
        self.assertIn('injected composition-orchestration panic', reply['result']['content'][0]['text'])
        self.preserved(wire, *state)

    @unittest.skipUnless(os.environ.get('MISTERDIAGRAM_TEST_INJECTIONS') == '1', 'requires test-panic-injection binary')
    def test_batch_orchestration_panic(self):
        wire, *state = self.prepared('batch-orchestration')
        reply = wire.request('tools/call', dict(name='batch', arguments=dict(commands=[
            dict(command='stencil.close', params=dict(stencil=1)),
            dict(command='page.add', params=dict(name='Transient')),
            dict(command='shape.delete', params=dict(id=1))])))
        self.assertTrue(reply['result']['isError'])
        self.assertIn('injected batch-orchestration panic', reply['result']['content'][0]['text'])
        self.preserved(wire, *state)

    @unittest.skipUnless(os.environ.get('MISTERDIAGRAM_TEST_INJECTIONS') == '1', 'requires test-panic-injection binary')
    def test_server_dispatch_panic(self):
        wire, *state = self.prepared()
        reply = wire.request('tools/call', dict(name='__test_panic_dispatch'))
        self.assertEqual(reply['error']['code'], -32603)
        self.assertIn('injected server-dispatch panic', reply['error']['message'])
        self.preserved(wire, *state)
        wire.close()
        self.assertIn(b'MCP request panic: injected server-dispatch panic', b''.join(wire.errors))


if __name__ == '__main__':
    if '--verify-production' in sys.argv:
        binary = CLI.read_bytes()
        for marker in (b'__test_panic_dispatch', b'MISTERDIAGRAM_TEST_PANIC',
                       b'composition-orchestration', b'batch-orchestration'):
            assert marker not in binary, f'test injection found in production binary: {marker!r}'
        print('Production binary contains no panic-injection markers')
    else:
        unittest.main()
