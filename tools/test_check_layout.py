"""The wrapper must preserve a full-check refusal while exposing rule h counts."""
import contextlib
import importlib.util
import io
import json
from pathlib import Path
import unittest
from unittest.mock import patch

SPEC = importlib.util.spec_from_file_location('layout_wrapper', Path(__file__).with_name('check-layout.py'))
CHECKER = importlib.util.module_from_spec(SPEC)
SPEC.loader.exec_module(CHECKER)


class FakeClient:
    def __init__(self, binary):
        self.calls = 0

    def __enter__(self):
        return self

    def __exit__(self, *args):
        return False

    def command(self, name, **params):
        if name == 'file.open':
            return {}
        self.calls += 1
        if not params.get('shared_only'):
            raise RuntimeError('layout checker comparison limit')
        return {'shared_only': True, 'count': 0, 'violations': [],
                'shared_segments': {'raw': 5, 'style_exempt': 5, 'same_glue_exempt': 0, 'violating': 0}}


class CheckerContract(unittest.TestCase):
    def test_full_refusal_is_exit_two_with_overlap_statistics(self):
        output = io.StringIO()
        with patch.object(CHECKER, 'Client', FakeClient), contextlib.redirect_stdout(output):
            status = CHECKER.check('original.vsdx', 'unused-cli')
        result = json.loads(output.getvalue())
        self.assertEqual(status, 2)
        self.assertEqual(result['full_check_error'], 'layout checker comparison limit')
        self.assertEqual(result['shared_segments']['style_exempt'], 5)

    def test_explicit_shared_only_reports_its_own_success(self):
        output = io.StringIO()
        with patch.object(CHECKER, 'Client', FakeClient), contextlib.redirect_stdout(output):
            status = CHECKER.check('original.vsdx', 'unused-cli', shared_only=True)
        result = json.loads(output.getvalue())
        self.assertEqual(status, 0)
        self.assertNotIn('full_check_error', result)
        self.assertTrue(result['shared_only'])


if __name__ == '__main__':
    unittest.main()
