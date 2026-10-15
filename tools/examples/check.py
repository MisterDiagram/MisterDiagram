#!/usr/bin/env python3
"""P4a acceptance diffs against immutable private Visio observations."""
import argparse
import csv
import io
from pathlib import Path
import sys
import xml.etree.ElementTree as ET
import zipfile

ROOT = Path(__file__).resolve().parents[2]
sys.path.insert(0, str(ROOT / 'tools/oracle'))
import diff_dump


def connections(path):
    with zipfile.ZipFile(path) as package:
        if len(package.infolist()) > 10000:
            raise ValueError('package entry limit')
        info = package.getinfo('visio/pages/page1.xml')
        if info.file_size > 8 * 1024 * 1024:
            raise ValueError('page XML limit')
        with package.open(info) as stream:
            data = stream.read(8 * 1024 * 1024 + 1)
        if len(data) > 8 * 1024 * 1024 or b'<!DOCTYPE' in data or b'<!ENTITY' in data:
            raise ValueError('page XML size or declaration limit')
        page = ET.fromstring(data)
    namespace = '{http://schemas.microsoft.com/office/visio/2012/main}'
    if page.tag != namespace + 'PageContents':
        raise ValueError('page namespace or root mismatch')
    rows = []
    for node in page.findall(namespace + 'Connects/' + namespace + 'Connect'):
        if len(rows) >= 100000:
            raise ValueError('connection count limit')
        rows.append(tuple(node.get(key, '') for key in ('FromSheet', 'FromCell', 'ToSheet', 'ToCell', 'FromPart', 'ToPart')))
    return sorted(rows)


def check(mode, directory):
    stem = {'probe': 'probe-rebuild', 'static': 'static-glue', 'move': 'move-after'}[mode]
    oracle = ROOT / 'corpus/generated' / ('basic/0001-probe' if mode == 'probe' else 'connector/' + stem)
    actual = diff_dump.read_dump(directory / (stem + '.sheet.tsv'))
    expected = diff_dump.read_dump(oracle.with_suffix('.sheet.tsv'))
    filters = diff_dump.read_filters([ROOT / 'crates/sheet/tests/hooks.txt', ROOT / 'crates/sheet/tests/volatile.txt'])
    if mode == 'move':
        names = {'BeginX', 'BeginY', 'EndX', 'EndY', 'PinX', 'PinY', 'Width', 'Height', 'Controls.TextPosition', 'Controls.TextPosition.Y'}
        selected = lambda row: row[0] == '3' and row[1].startswith('page:') and row[5] in names
        actual = {key: row for key, row in actual.items() if selected(row)}
        expected = {key: row for key, row in expected.items() if selected(row)}
        if len(expected) != len(names):
            raise ValueError('move-after oracle does not contain all required cells')
        filters = set()
    total, skipped, failures = diff_dump.compare(actual, expected, filters)
    lines = [f'{key}: {message}\n' for key, message in failures]
    if total == 0:
        lines.append('no eligible cells\n')
    if mode == 'static':
        actual_rows = connections(directory / (stem + '.vsdx'))
        with oracle.with_suffix('.connects.tsv').open(newline='', encoding='utf-8-sig') as source:
            rows = csv.reader(source, delimiter='\t')
            if next(rows, None) != ['from_sheet', 'from_cell', 'to_sheet', 'to_cell', 'from_part', 'to_part']:
                raise ValueError('invalid Connects header')
            expected_rows = sorted(tuple(row) for row in rows)
        output = io.StringIO()
        writer = csv.writer(output, delimiter='\t', lineterminator='\n')
        writer.writerow(['from_sheet', 'from_cell', 'to_sheet', 'to_cell', 'from_part', 'to_part'])
        writer.writerows(actual_rows)
        (directory / (stem + '.connects.tsv')).write_text(output.getvalue())
        if actual_rows != expected_rows:
            lines.append(f'Connects expected={expected_rows!r} actual={actual_rows!r}\n')
    (directory / (stem + '.diff')).write_text(''.join(lines))
    print(f'P4a {stem}: eligible={total} excluded={skipped} differences={len(lines)} tolerance=1e-6')
    if lines:
        print(''.join(lines[:30]), end='')
    return int(bool(lines))


def main():
    parser = argparse.ArgumentParser(description=__doc__)
    parser.add_argument('mode', choices=['probe', 'static', 'move'])
    parser.add_argument('directory', type=Path)
    args = parser.parse_args()
    try:
        return check(args.mode, args.directory)
    except (OSError, ValueError, KeyError, zipfile.BadZipFile, ET.ParseError) as error:
        print(f'error: {error}', file=sys.stderr)
        return 1


if __name__ == '__main__':
    sys.exit(main())
