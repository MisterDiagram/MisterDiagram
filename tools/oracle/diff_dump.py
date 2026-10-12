#!/usr/bin/env python3
"""Compare computed nine-column dumps with immutable COM observations."""
import argparse
import math
from pathlib import Path
import re
import sys

HEADER = 'shape_id shape_name section row cell name formula result_iu units'.split()
MAX_LINE = 1024 * 1024
MAX_ROWS = 10_000_000


def unescape(text):
    out = []
    i = 0
    while i < len(text):
        ch = text[i]
        i += 1
        if ch == '\\':
            if i == len(text) or text[i] not in '\\trn':
                raise ValueError('invalid TSV escape')
            ch = {'\\': '\\', 't': '\t', 'r': '\r', 'n': '\n'}[text[i]]
            i += 1
        out.append(ch)
    return ''.join(out)


def read_dump(path):
    rows = {}
    with Path(path).open(encoding='utf-8-sig', newline='') as f:
        if f.readline(MAX_LINE + 1).rstrip('\r\n').split('\t') != HEADER:
            raise ValueError(f'{path}: invalid header')
        while True:
            line = f.readline(MAX_LINE + 1)
            if not line:
                break
            if len(line) > MAX_LINE or len(rows) >= MAX_ROWS:
                raise ValueError(f'{path}: dump limit exceeded')
            fields = line.rstrip('\r\n').split('\t')
            if len(fields) != 9:
                raise ValueError(f'{path}: expected nine columns')
            for i in (1, 5, 6):
                fields[i] = unescape(fields[i])
            for i in (0, 2, 3, 4, 8):
                if not fields[i].isdigit() or int(fields[i]) > 4294967295:
                    raise ValueError(f'{path}: invalid index or units')
            key = tuple(fields[:6])
            if key in rows:
                raise ValueError(f'{path}: duplicate cell {key}')
            rows[key] = fields
    return rows


def function_calls(formula):
    # Quoted text can contain function-looking words; it never excludes a cell.
    if len(formula) > MAX_LINE:
        raise ValueError('formula limit exceeded')
    out = []
    i = 0
    while i < len(formula):
        ch = formula[i]
        if ch == '[':
            depth = 1
            i += 1
            while i < len(formula) and depth:
                depth += (formula[i] == '[') - (formula[i] == ']')
                i += 1
                if depth > 128:
                    raise ValueError('reference nesting limit')
            out.append(' ')
        elif ch in ('"', "'"):
            # A prime immediately following a number can be an angular unit.
            prior = formula[:i].rstrip()
            if prior and prior[-1].isdigit():
                i += 1
                out.append(' ')
                continue
            quote = ch
            i += 1
            while i < len(formula):
                if formula[i] == quote:
                    i += 1
                    if i < len(formula) and formula[i] == quote:
                        i += 1
                        continue
                    break
                i += 1
            out.append(' ')
        else:
            out.append(ch)
            i += 1
    return set(re.findall(r'(?i)([_A-Z][_A-Z0-9.]*)\s*\(', ''.join(out).upper()))


def read_filters(paths):
    names = set()
    for path in paths:
        if path:
            for line in Path(path).read_text(encoding='utf-8').splitlines():
                name = line.split('#', 1)[0].strip().upper()
                if name:
                    names.add(name)
    return names


def excluded(formula, names):
    for function in function_calls(formula):
        if function in names or any(n.endswith('...') and function.startswith(n[:-3]) for n in names):
            return True
    return False


def compare(actual, expected, names):
    failures = []
    total = skipped = 0
    for key, want in expected.items():
        if excluded(want[6], names):
            skipped += 1
            continue
        total += 1
        got = actual.get(key)
        if got is None:
            failures.append((key, 'missing'))
            continue
        try:
            x, y = float(got[7]), float(want[7])
            equal = math.isfinite(x) and math.isfinite(y) and abs(x-y) <= 1e-6
        except ValueError:
            equal = False  # COM ResultIU is numeric; an error string cannot pass.
        if not equal:
            failures.append((key, f'value expected={want[7]!r} actual={got[7]!r}'))
        if got[8] != want[8]:
            failures.append((key, f'units expected={want[8]} actual={got[8]}'))
    for key in actual.keys() - expected.keys():
        if not excluded(actual[key][6], names):
            failures.append((key, 'unexpected'))
    return total, skipped, failures


def main():
    parser = argparse.ArgumentParser(description=__doc__)
    parser.add_argument('actual')
    parser.add_argument('expected')
    parser.add_argument('--ignore-hooks')
    parser.add_argument('--ignore-volatile')
    args = parser.parse_args()
    try:
        total, skipped, failures = compare(read_dump(args.actual), read_dump(args.expected),
                                           read_filters([args.ignore_hooks, args.ignore_volatile]))
        print(f'eligible={total} excluded={skipped} differences={len(failures)} tolerance=1e-6')
        for key, message in failures[:100]:
            print(f'{key}: {message}')
        return int(bool(failures) or total == 0)
    except (OSError, ValueError) as error:
        print(f'error: {error}', file=sys.stderr)
        return 1


if __name__ == '__main__':
    sys.exit(main())
