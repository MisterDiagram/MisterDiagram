"""Hostile-input and exact-comparison checks for the benchmark helper."""
import contextlib
import importlib.util
import io
from pathlib import Path
import tempfile
import unittest
import warnings
import zipfile

spec = importlib.util.spec_from_file_location('compare', Path(__file__).with_name('compare.py'))
compare = importlib.util.module_from_spec(spec)
spec.loader.exec_module(compare)

class ComparisonTests(unittest.TestCase):
    def setUp(self):
        root = Path(__file__).resolve().parents[2] / 'target/bench-helper-tests'
        root.mkdir(parents=True, exist_ok=True)
        self.folder = tempfile.TemporaryDirectory(dir=root)
        self.addCleanup(self.folder.cleanup)
        self.root = Path(self.folder.name)

    def package(self, name, entries):
        path = self.root / name
        with zipfile.ZipFile(path, 'w', compression=zipfile.ZIP_DEFLATED) as z:
            for part, data in entries:
                z.writestr(part, data)
        return path

    def test_hostile_zip_rejected_before_unbounded_decompression(self):
        path = self.root/'bad.vsdx'
        path.write_bytes(b'not a zip')
        with self.assertRaises(zipfile.BadZipFile):
            compare.parts(path)
        with warnings.catch_warnings():
            warnings.simplefilter('ignore', UserWarning)
            duplicate = self.package('duplicate.vsdx', [('a', b'1'), ('a', b'2')])
        with self.assertRaises(ValueError):
            compare.parts(duplicate)
        oversized = self.package('oversized.vsdx', [('a', b'0'*(32*1024*1024+1))])
        with self.assertRaises(ValueError):
            compare.parts(oversized)
        self.assertIn('invalid XML', compare.detail(b'<bad>', b'<also-bad>'))

    def test_masks_do_not_hide_a_cell_or_serialization_change(self):
        a = self.package('a.vsdx', [('visio/pages/page1.xml', b'<Cell N="PinX" V="1"/>')])
        b = self.package('b.vsdx', [('visio/pages/page1.xml', b'<Cell N="PinX" V="2"/>')])
        with contextlib.redirect_stdout(io.StringIO()) as output:
            self.assertEqual(compare.compare(a,b),1)
        self.assertIn('PinX',output.getvalue())
        c = self.package('c.vsdx', [('visio/pages/page1.xml', b'<Cell V="1" N="PinX"/>')])
        with contextlib.redirect_stdout(io.StringIO()):
            self.assertEqual(compare.compare(a,c),1)
            self.assertEqual(compare.compare(a,a),0)

    def test_only_designated_save_time_elements_are_masked(self):
        def core(year):
            return (f'<cp:coreProperties xmlns:cp="http://schemas.openxmlformats.org/package/2006/metadata/core-properties" xmlns:dcterms="http://purl.org/dc/terms/">'
                    f'<dcterms:modified>{year}-01-01T00:00:00Z</dcterms:modified></cp:coreProperties>').encode()
        a = self.package('time-a.vsdx', [('docProps/core.xml', core(2020))])
        b = self.package('time-b.vsdx', [('docProps/core.xml', core(2021))])
        with contextlib.redirect_stdout(io.StringIO()):
            self.assertEqual(compare.compare(a, b), 0)
        for part in ['docProps/custom.xml', 'docProps/core.xml']:
            a = self.package('custom-a.vsdx', [(part, b'<Milestone>2020-01-01T00:00:00Z</Milestone>')])
            b = self.package('custom-b.vsdx', [(part, b'<Milestone>2021-01-01T00:00:00Z</Milestone>')])
            with contextlib.redirect_stdout(io.StringIO()):
                self.assertEqual(compare.compare(a, b), 1)
        for name, expected in [('TimeEdited', 0), ('Milestone', 1)]:
            def custom(year):
                return (f'<Properties xmlns:vt="http://schemas.openxmlformats.org/officeDocument/2006/docPropsVTypes">'
                        f'<property name="{name}" pid="6"><vt:filetime>{year}-01-01T00:00:00Z</vt:filetime></property></Properties>').encode()
            a = self.package('property-a.vsdx', [('docProps/custom.xml', custom(2020))])
            b = self.package('property-b.vsdx', [('docProps/custom.xml', custom(2021))])
            with contextlib.redirect_stdout(io.StringIO()):
                self.assertEqual(compare.compare(a, b), expected)

if __name__ == '__main__':
    unittest.main()
