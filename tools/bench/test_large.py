#!/usr/bin/env python3
"""Validate generated workloads before using them as fixed equality inputs."""
import tempfile
import unittest
from pathlib import Path
import xml.etree.ElementTree as ET
import zipfile

import large


class Workloads(unittest.TestCase):
    def test_rectangle_population_and_chunk_contract(self):
        commands = large.shapes377()
        self.assertEqual(len(commands), 2500)
        self.assertEqual(sum(cid == 'shape.draw.rect' for cid, _ in commands), 377)
        self.assertEqual(sum(cid == 'shape.data.define' for cid, _ in commands), 754)
        self.assertEqual(sum(cid == 'shape.data.set' for cid, _ in commands), 754)
        self.assertFalse(any(cid.startswith('connect.') or cid == 'master.drop' for cid, _ in commands))
        self.assertTrue(all('fill' in p and 'line' in p for c, p in commands if c == 'shape.draw.rect'))

    def test_san_groups_media_ports_and_command_mix(self):
        commands = large.sanlarge(Path('invented.vssx'))
        self.assertEqual(len(commands), 2500)
        self.assertEqual(sum(c == 'connect.dynamic' for c, _ in commands), 98)
        self.assertEqual(sum(c == 'connect.static' for c, _ in commands), 294)
        self.assertEqual(sum(c == 'master.drop' and p.get('master') == 'Vela Host' for c, p in commands), 64)
        # Write test assets only inside the repository's allowed temporary tree.
        output = Path(__file__).resolve().parents[2] / 'plan/brief28'
        output.mkdir(parents=True, exist_ok=True)
        with tempfile.TemporaryDirectory(dir=output) as folder:
            path = Path(folder) / 'invented.vssx'
            large.stencil(path)
            with zipfile.ZipFile(path) as package:
                director = ET.fromstring(package.read('visio/masters/master1.xml'))
                host = ET.fromstring(package.read('visio/masters/master2.xml'))
                ns = {'v': large.VIS}
                self.assertEqual(len(director.findall('.//v:Section[@N="Connection"]/v:Row', ns)), 96)
                self.assertEqual(len(director.findall('.//v:Shape', ns)), 3)
                self.assertEqual(len(host.findall('.//v:Shape', ns)), 2)
                self.assertEqual(len(host.findall('.//v:ForeignData', ns)), 1)
                self.assertTrue(package.read('visio/media/grid.png').startswith(b'\x89PNG\r\n\x1a\n'))

    def test_edit_mix(self):
        commands = large.largeedit()
        self.assertEqual(len(commands), 20)
        for name in ('shape.move', 'format.fill', 'master.drop', 'shape.delete'):
            self.assertTrue(any(cid == name for cid, _ in commands))


if __name__ == '__main__':
    unittest.main()
