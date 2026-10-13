"""Host-only acceptance parser and comparison regressions. Never edits oracle evidence."""
import sys
from pathlib import Path
import tempfile
import unittest
import zipfile
sys.path.insert(0, str(Path(__file__).resolve().parent))
import accept_report as report

class Acceptance(unittest.TestCase):
    def setUp(self):
        root = report.ROOT/'target/oracle-tests'
        root.mkdir(parents=True, exist_ok=True)
        self.tmp = tempfile.TemporaryDirectory(dir=root)
        self.addCleanup(self.tmp.cleanup)
        self.root = Path(self.tmp.name)

    def package(self, xml):
        path = self.root/'test.vsdx'
        with zipfile.ZipFile(path, 'w') as z:
            z.writestr('visio/document.xml', xml)
        return path

    def test_hostile_json_is_bounded(self):
        path=self.root/'bad.json'
        for payload in [b'x'*(4*1024*1024+1), b'['*10000+b']'*10000, b'\xff']:
            path.write_bytes(payload)
            with self.assertRaises(ValueError): report.load_json(path)

    def test_serialized_cache_change_survives_formula_equality(self):
        _, a = report.package_cells(self.package('<DocumentSheet><Cell N="X" V="1" F="2"/></DocumentSheet>'))
        _, b = report.package_cells(self.package('<DocumentSheet><Cell N="X" V="2" F="2"/></DocumentSheet>'))
        self.assertEqual(len(report.cell_diffs(a,b,set())),1)

    def test_duplicates_missing_and_volatile_are_explicit(self):
        _, a = report.package_cells(self.package('<DocumentSheet><Cell N="X" V="1"/><Cell N="X" V="2"/><Cell N="Y" V="3" F="NOW()"/></DocumentSheet>'))
        self.assertEqual(len(a),3)
        self.assertEqual(len(report.cell_diffs(a,{}, {'NOW'})),2)
        self.assertEqual(len(report.cell_diffs({},a, {'NOW'})),3)

    def test_opaque_solution_payload_is_in_part_list_not_sheet_dump(self):
        path = self.package('<DocumentSheet><Cell N="X" V="1"/></DocumentSheet>')
        with zipfile.ZipFile(path,'a') as z:
            z.writestr('visio/solutions/solution1.xml','<vendor:opaque/>')
        parts,cells = report.package_cells(path)
        self.assertIn('visio/solutions/solution1.xml',parts)
        self.assertEqual(len(cells),1)

    def test_lost_connectivity_is_visible(self):
        _, a = report.package_cells(self.package('<PageContents><Connects><Connect FromSheet="3" FromCell="BeginX" ToSheet="1" ToCell="PinX"/></Connects></PageContents>'))
        _, b = report.package_cells(self.package('<PageContents/>'))
        self.assertEqual(len(report.cell_diffs(a,b,set())),1)

    def test_hostile_xml_and_duplicate_parts_rejected(self):
        for xml in ['<!DOCTYPE x [<!ENTITY x "boom">]><x>&x;</x>', '<x>'*130+'</x>'*130]:
            with self.assertRaises(ValueError):
                report.package_cells(self.package(xml))
        path=self.package('<x/>')
        import warnings
        with warnings.catch_warnings():
            warnings.simplefilter('ignore')
            with zipfile.ZipFile(path,'a') as z:
                z.writestr('visio/document.xml','<x/>')
        with self.assertRaises(ValueError):
            report.package_cells(path)

    def test_common_area_one_pixel_and_hard_two_pixel(self):
        from PIL import Image
        a,b=self.root/'a.png',self.root/'b.png'
        Image.new('RGB',(4,4),'white').save(a)
        Image.new('RGB',(5,4),'white').save(b)
        result=report.pixels(a,b)
        self.assertEqual((result['mad'],result['pixels']),(0,16))
        Image.new('RGB',(6,4),'white').save(b)
        self.assertEqual(report.pixels(a,b)['error'],'hard size mismatch')

if __name__ == '__main__':
    unittest.main()
