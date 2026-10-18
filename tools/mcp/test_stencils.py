#!/usr/bin/env python3
"""Brief 26 issues #58, #59 and #63 through the real stdio server.

Only contributor-original packages, generated at runtime, with 38 MiB of opaque
synthetic padding. No vendor data or large fixture is committed.
"""
import os
from pathlib import Path
import unittest
import zipfile
from client import Client, default_binary

ROOT = Path(__file__).resolve().parents[2]
CLI = Path(os.environ.get('MISTERDIAGRAM_CLI', default_binary())).resolve()


class Stencils(unittest.TestCase):
    @classmethod
    def setUpClass(cls):
        cls.out = ROOT / 'target' / 'mcp-stencils' / str(os.getpid())
        cls.out.mkdir(parents=True, exist_ok=True)
        cls.small = cls.out / 'small.vsdx'
        cls.large = cls.out / 'large.vsdx'
        cls.stencils = [cls.out / 'first.vssx', cls.out / 'second.vssx']
        with Client(CLI) as client:
            client.command('shape.draw.rect', x=1, y=1, width=2, height=1)
            client.command('master.new_from_shape', id=1, name='Brief26 Original')
            client.tool('save', path=str(cls.small))
            for path in cls.stencils:
                client.tool('save', path=str(path))
        cls.pad(cls.small, cls.large)
        for path in cls.stencils:
            cls.pad(path, path)
            with zipfile.ZipFile(path) as package:
                assert sum(item.file_size for item in package.infolist()) > 38 * 1024 * 1024

    @staticmethod
    def pad(source, target):
        with zipfile.ZipFile(source) as package:
            parts = {item.filename: package.read(item) for item in package.infolist()}
        parts['[Content_Types].xml'] = parts['[Content_Types].xml'].replace(
            b'</Types>', b'<Default Extension="bin" ContentType="application/octet-stream"/></Types>')
        with zipfile.ZipFile(target, 'w', compression=zipfile.ZIP_DEFLATED) as package:
            for name, content in parts.items():
                package.writestr(name, content)
            package.writestr('original-padding.bin', bytes(38 * 1024 * 1024))

    def test_58_document_batch_with_large_open_stencil_and_real_over_budget_rollback(self):
        commands = [{'command': 'shape.draw.rect', 'params': {'x': 1, 'y': 1, 'width': 2, 'height': 1}}]
        with Client(CLI) as client:
            client.command('stencil.open', path=str(self.stencils[0]))
            client.command('file.new')
            before = client.tool('inspect_document')
            client.tool('batch', commands=commands)
            after = client.tool('inspect_document')
            self.assertNotEqual(before, after)
            self.assertTrue(client.command('edit.undo')['changed'])
            self.assertEqual(before, client.tool('inspect_document'))
            self.assertTrue(client.command('edit.redo')['changed'])
            self.assertEqual(after, client.tool('inspect_document'))
            client.tool('open', path=str(self.large))
            before = client.tool('inspect_document')
            with self.assertRaisesRegex(RuntimeError, 'batch undo snapshot exceeds retained budget'):
                client.tool('batch', commands=commands)
            self.assertEqual(before, client.tool('inspect_document'))
            self.assertEqual(client.command('stencil.open', path=str(self.stencils[0]))['stencil'], 1)

    def test_63_list_close_and_document_lifetime_with_large_stencils(self):
        with Client(CLI) as client:
            tools = {tool['name']: tool for tool in client.request('tools/list', {})['tools']}
            self.assertIn('stencil_list', tools)
            self.assertIn('stencil_close', tools)
            self.assertEqual(tools['stencil_close']['inputSchema']['required'], ['stencil'])
            for index, path in enumerate(self.stencils, 1):
                self.assertEqual(client.command('stencil.open', path=str(path))['stencil'], index)
            rows = client.tool('stencil_list')['stencils']
            self.assertEqual([row['stencil'] for row in rows], [0, 1, 2])
            for row, path in zip(rows[1:], self.stencils):
                self.assertEqual(row['name'], path.name)
                self.assertTrue(os.path.samefile(row['path'], path))
                self.assertGreater(row['masters'], 0)
            client.command('file.new')
            self.assertEqual(client.tool('stencil_list')['stencils'][1:], rows[1:])
            client.tool('open', path=str(self.small))
            self.assertEqual(client.tool('stencil_list')['stencils'][1:], rows[1:])
            client.tool('stencil_close', stencil=1)
            current = client.tool('stencil_list')['stencils']
            self.assertEqual([row['stencil'] for row in current], [0, 1])
            self.assertTrue(os.path.samefile(current[1]['path'], self.stencils[1]))
            for index in [0, 2, -1, 65]:
                with self.assertRaises(RuntimeError):
                    client.tool('stencil_close', stencil=index)
                self.assertEqual(current, client.tool('stencil_list')['stencils'])
            client.tool('stencil_close', stencil=1)
            self.assertEqual(len(client.tool('stencil_list')['stencils']), 1)

    def test_59_searches_document_and_every_open_large_stencil(self):
        with Client(CLI) as client:
            client.tool('open', path=str(self.small))
            for path in self.stencils:
                client.command('stencil.open', path=str(path))
            result = client.tool('stencil_search', query='Brief26')
            self.assertEqual(result['matched'], 3)
            self.assertEqual([row['stencil'] for row in result['masters']], [0, 1, 2])
            for index in [0, 1, 2]:
                one = client.tool('stencil_search', stencil=index, query='Brief26')
                self.assertEqual(one['matched'], 1)
                self.assertEqual(one['masters'][0]['stencil'], index)
            limited = client.tool('stencil_search', query='Brief26', limit=1)
            self.assertEqual(limited['matched'], 3)
            self.assertEqual(len(limited['masters']), 1)
            empty = client.tool('stencil_search', query='Brief26', limit=0)
            self.assertEqual(empty, {'masters': [], 'matched': 3})
            client.tool('drop_master', stencil=0, master='Brief26 Original', x=4, y=4)


if __name__ == '__main__':
    unittest.main()
