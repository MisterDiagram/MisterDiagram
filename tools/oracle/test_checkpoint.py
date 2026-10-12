"""Hostile checkpoint-log parsing cases. No VM calls or oracle fixture mutations."""
import unittest
from checkpoint_generated import completed_names

class CheckpointLogTests(unittest.TestCase):
    def test_only_complete_safe_names_are_selected(self):
        log = ('t DRAWING OK basic/0001-probe\n'
               't DRAWING FAIL fill/pattern-01 failed\n'
               't DRAWING RESUMED line/weight-0.25\n'
               't DRAWING OK basic/0001-probe\n'
               't DRAWING OK ../../outside\n'
               't DRAWING OK name with spaces\n')
        self.assertEqual(completed_names(log), ['basic/0001-probe', 'line/weight-0.25'])

    def test_absolute_path_and_oversized_log_fail(self):
        with self.assertRaises(ValueError):
            completed_names('t DRAWING OK /outside\n')
        with self.assertRaises(ValueError):
            completed_names('x' * (4 * 1024 * 1024 + 1))

if __name__ == '__main__':
    unittest.main()
