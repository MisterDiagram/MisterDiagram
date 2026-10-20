#!/usr/bin/env python3
"""Hostile workload files are refused before spawning a benchmark."""
import io
import json
import unittest

from measure import read_commands


class Input:
    def __init__(self, data):
        self.data = data

    def open(self, mode):
        assert mode == 'rb'
        return io.BytesIO(self.data)


class WorkloadInput(unittest.TestCase):
    def test_invalid_files(self):
        for data in [b'not json', b'null', b'{}', b'[]', b'[[1,{}]]',
                     b'[["file.new",[]]]', b'[["file.new",{},1]]',
                     b'\xff', b'[' * 2000 + b'0' + b']' * 2000]:
            with self.subTest(data=data[:40]), self.assertRaises((ValueError, RecursionError)):
                read_commands(Input(data))

    def test_byte_and_count_limits(self):
        with self.assertRaisesRegex(ValueError, 'byte limit'):
            read_commands(Input(b' ' * (16 * 1024 * 1024 + 1)))
        with self.assertRaisesRegex(ValueError, 'count limit'):
            read_commands(Input(json.dumps([['file.new', {}]] * 8193).encode()))
        self.assertEqual(read_commands(Input(b'[["file.new",{}]]')), [['file.new', {}]])


if __name__ == '__main__':
    unittest.main()
