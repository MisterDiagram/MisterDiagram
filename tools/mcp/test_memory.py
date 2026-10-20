#!/usr/bin/env python3
"""Session retention through real stdio, using runtime original stencils only."""
import os
from pathlib import Path
import unittest
import zipfile
from client import Client, default_binary
import test_stencils

CLI = Path(os.environ.get('MISTERDIAGRAM_CLI', default_binary())).resolve()


class Memory(unittest.TestCase):
    @classmethod
    def setUpClass(cls):
        Stencils = test_stencils.Stencils
        Stencils.setUpClass()
        cls.stencil = Stencils.stencils[0]
        cls.media = Stencils.out / 'pictures.vssx'
        with zipfile.ZipFile(cls.stencil) as z:
            parts = {n: z.read(n) for n in z.namelist() if n != 'original-padding.bin'}
        parts['[Content_Types].xml'] = parts['[Content_Types].xml'].replace(
            b'</Types>', b'<Default Extension="bmp" ContentType="image/bmp"/></Types>')
        with zipfile.ZipFile(cls.media, 'w', compression=zipfile.ZIP_DEFLATED) as z:
            for n, data in parts.items():
                z.writestr(n, data)
            # Original opaque picture padding, never rendered. No vendor input.
            z.writestr('visio/media/original.bmp', bytes(280 * 1024 * 1024))

    def test_compound_preserves_outer_rollback_media(self):
        paths = []
        for source, mib, suffix in [(test_stencils.Stencils.small, 60, 'vsdx'), (self.stencil, 70, 'vssx')]:
            path = test_stencils.Stencils.out / ('rollback.' + suffix)
            with zipfile.ZipFile(source) as z:
                parts = {n: z.read(n) for n in z.namelist() if n != 'original-padding.bin'}
            parts['[Content_Types].xml'] = parts['[Content_Types].xml'].replace(
                b'</Types>', b'<Default Extension="bmp" ContentType="image/bmp"/></Types>')
            with zipfile.ZipFile(path, 'w', compression=zipfile.ZIP_DEFLATED) as z:
                for n, data in parts.items():
                    z.writestr(n, data)
                z.writestr('visio/media/original.bmp', bytes(mib * 1024 * 1024))
            paths.append(path)
        for compound, wrapped, changed in [(False, False, True), (True, False, True), (True, True, True), (True, True, False)]:
            with Client(CLI, args=("--memory-limit", "1024")) as c:
                c.tool('open', path=str(paths[0]))
                c.command('stencil.open', path=str(paths[1]))
                before = c.tool('inspect_document'), c.tool('stencil_list')
                params = {'x': 1, 'y': 1, 'width': 2, 'height': 1}
                if compound:
                    params['text'] = 'hello'
                inventory = {'command': 'stencil.close', 'params': {'stencil': 1}} if changed else {'command': 'stencil.open', 'params': {'path': str(paths[1])}}
                if wrapped:
                    inventory = {'command': 'engine.execute', 'params': inventory}
                commands = [inventory, {'command': 'shape.draw.rect', 'params': params}]
                if not compound:
                    commands.append({'command': 'shape.text', 'params': {'id': 2, 'text': 'hello'}})
                # The original 60/70 MiB inputs fit when unchanged pictures share
                # allocations across publication and rollback owners. Keep the
                # same 1 GiB setting and exercise the original compound variants.
                c.tool('batch', commands=commands)
                after = c.tool('inspect_document'), c.tool('stencil_list')
                self.assertTrue(c.command('edit.undo')['changed'])
                self.assertEqual((c.tool('inspect_document'), c.tool('stencil_list')), before)
                self.assertTrue(c.command('edit.redo')['changed'])
                self.assertEqual((c.tool('inspect_document'), c.tool('stencil_list')), after)
                self.assertTrue(c.command('edit.undo')['changed'])
                failing = commands + [{'command': 'shape.move', 'params': {'id': 999999, 'x': 0, 'y': 0}}]
                with self.assertRaisesRegex(RuntimeError, 'shape'):
                    c.tool('batch', commands=failing)
                self.assertEqual((c.tool('inspect_document'), c.tool('stencil_list')), before)

    def test_memory_setting_never_raises_file_node_admission(self):
        path = test_stencils.Stencils.out / 'hostile-nodes.vssx'
        with zipfile.ZipFile(self.stencil) as z:
            parts = {n: z.read(n) for n in z.namelist() if n != 'original-padding.bin'}
        key = 'visio/document.xml'
        content = parts[key]
        closing = content.rfind(b'</')
        self.assertGreater(closing, 0)
        parts[key] = content[:closing] + b'<x/>' * 4_000_001 + content[closing:]
        with zipfile.ZipFile(path, 'w', compression=zipfile.ZIP_DEFLATED) as z:
            for name, data in parts.items():
                z.writestr(name, data)
        with Client(CLI, args=('--memory-limit', '8192')) as c:
            before = c.tool('stencil_list')
            with self.assertRaisesRegex(RuntimeError, 'retained XML node limit.*limit 4000000') as refused:
                c.command('stencil.open', path=str(path))
            self.assertIn('--memory-limit and MISTERDIAGRAM_MEMORY_LIMIT do not change this fixed admission bound', str(refused.exception))
            self.assertRegex(str(refused.exception), r'needs at least [0-9]+ new nodes, [0-9]+ already retained')
            self.assertEqual(c.tool('stencil_list'), before)

    def test_failure1_inventory_batch_shares_unchanged_stencil(self):
        with Client(CLI) as c:
            c.command('stencil.open', path=str(self.stencil))
            before = c.tool('inspect_document'), c.tool('stencil_list')
            commands = [{'command': 'stencil.open', 'params': {'builtin': 'network'}},
                        {'command': 'master.drop', 'params': {'master': 'compute.server', 'x': 2, 'y': 2}}]
            c.tool('batch', commands=commands)
            after = c.tool('inspect_document'), c.tool('stencil_list')
            self.assertEqual([r['stencil'] for r in after[1]['stencils']], [0, 1, 2])
            self.assertTrue(c.command('edit.undo')['changed'])
            self.assertEqual((c.tool('inspect_document'), c.tool('stencil_list')), before)
            self.assertTrue(c.command('edit.redo')['changed'])
            self.assertEqual((c.tool('inspect_document'), c.tool('stencil_list')), after)
            c.tool('batch', commands=[{'command': 'stencil.close', 'params': {'stencil': 1}}])
            self.assertEqual(len(c.tool('stencil_list')['stencils']), 2)
            c.command('edit.undo')
            self.assertEqual((c.tool('inspect_document'), c.tool('stencil_list')), after)
            with self.assertRaises(RuntimeError):
                c.tool('batch', commands=[{'command': 'stencil.close', 'params': {'stencil': 1}},
                                         {'command': 'shape.move', 'params': {'id': 999999, 'x': 0, 'y': 0}}])
            self.assertEqual((c.tool('inspect_document'), c.tool('stencil_list')), after)

    def test_failure2_open_admission_and_drop_agree(self):
        with Client(CLI) as c:
            before = c.tool('inspect_document'), c.tool('stencil_list')
            try:
                opened = c.command('stencil.open', path=str(self.media))
            except RuntimeError as e:
                self.assertRegex(str(e), r'needs [0-9.]+ MiB, [0-9.]+ MiB already (?:retained|held).*limit [0-9]+ MiB')
                self.assertIn('MISTERDIAGRAM_MEMORY_LIMIT', str(e))
                self.assertEqual((c.tool('inspect_document'), c.tool('stencil_list')), before)
                return
            rows = c.command('stencil.masters', stencil=opened['stencil'])['masters']
            for row in rows:
                # These masters use no pictures: admission must guarantee drops
                # do not fail because unrelated stencil pictures are recounted.
                c.tool('drop_master', stencil=opened['stencil'], master=row['id'], x=2, y=2)
            self.assertGreater(len(c.tool('inspect_document')['pages'][0]['shapes']), 0)


if __name__ == '__main__':
    unittest.main()
