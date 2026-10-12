"""Hostile inputs and denominator integrity for the dump comparator."""
import importlib.util
from pathlib import Path
import tempfile
import unittest

ROOT = Path(__file__).resolve().parents[2]
spec = importlib.util.spec_from_file_location('diff_dump', ROOT / 'tools/oracle/diff_dump.py')
m = importlib.util.module_from_spec(spec)
spec.loader.exec_module(m)


class DumpTests(unittest.TestCase):
    def test_escapes_and_hostile_rows(self):
        self.assertEqual(m.unescape(r'a\tb\\c\n'), 'a\tb\\c\n')
        with self.assertRaises(ValueError):
            m.unescape('bad\\q')
        (ROOT / 'target').mkdir(exist_ok=True)
        with tempfile.TemporaryDirectory(dir=ROOT / 'target') as tmp:
            p = Path(tmp) / 'x.tsv'
            for body in ['bad\n', '\t'.join(m.HEADER) + '\nshort\n', '\t'.join(m.HEADER) + '\n' + 'x' * (m.MAX_LINE+1)]:
                p.write_text(body)
                with self.assertRaises(ValueError):
                    m.read_dump(p)

    def test_missing_and_errors_remain_failures(self):
        row = ['1', 'page:0/Sheet.1', '1', '0', '0', 'Width', '1+1', '2', '65']
        key = tuple(row[:6])
        self.assertEqual(m.compare({}, {key: row}, set())[:2], (1, 0))
        self.assertTrue(m.compare({}, {key: row}, set())[2])
        got = row.copy()
        got[7] = '#REF!'
        self.assertTrue(m.compare({key: got}, {key: row}, set())[2])
        got[7] = '2.0000001'
        self.assertFalse(m.compare({key: got}, {key: row}, set())[2])

    def test_filter_uses_calls_outside_strings(self):
        self.assertFalse(m.excluded('"NOW()"', {'NOW'}))
        self.assertTrue(m.excluded('GUARD(NOW())', {'NOW'}))
        self.assertTrue(m.excluded('_UCON_x(1)', {'_UCON_...'}))
        self.assertFalse(m.excluded('_ELLIPSE_ECC(1)', {'_UCON_...'}))
        self.assertFalse(m.excluded('Pages[NOW()]!PageWidth', {'NOW'}))
        self.assertFalse(m.excluded("'THEMEVAL()'!Width", {'THEMEVAL'}))
        self.assertTrue(m.excluded("5' + NOW()", {'NOW'}))
        self.assertTrue(m.excluded("5 ' + NOW()", {'NOW'}))


if __name__ == '__main__':
    unittest.main()
