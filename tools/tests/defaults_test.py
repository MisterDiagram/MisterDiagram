"""Hostile input coverage for the bounded baseline-default generator."""
import importlib.util
import tempfile
import unittest
from pathlib import Path
ROOT = Path(__file__).resolve().parents[2]
spec = importlib.util.spec_from_file_location('defaults', ROOT/'tools/generate-defaults.py')
defaults = importlib.util.module_from_spec(spec)
spec.loader.exec_module(defaults)

class DefaultsTest(unittest.TestCase):
    def test_hostile_default_records(self):
        header = '\t'.join(defaults.HEADER)+'\n'
        row = '0\tdocument/TheDoc\t3\t0\t0\tChar.Font\t4\t4\t32\n'
        with tempfile.TemporaryDirectory(dir=ROOT/'target') as directory:
            p = Path(directory)/'defaults.tsv'
            for bad in ['wrong\n', header+row+row, header+row.replace('\t3\t','\t-1\t'), header+row+'short\n', 'x'*(defaults.LIMIT+1)]:
                p.write_text(bad)
                with self.assertRaises(ValueError): defaults.rows(p)
            p.write_text(header+row)
            records, digest = defaults.rows(p)
            self.assertEqual(len(records), 1)
            self.assertEqual(len(digest), 64)

if __name__ == '__main__': unittest.main()
