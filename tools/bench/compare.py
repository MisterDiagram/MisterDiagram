#!/usr/bin/env python3
"""Compare saved parts byte-for-byte, masking only fresh master IDs/save times."""
import re
import sys
import zipfile
import xml.etree.ElementTree as ET


def parts(path):
    with zipfile.ZipFile(path) as package:
        entries = package.infolist()
        if len(entries) > 4096 or sum(e.file_size for e in entries) > 128 * 1024 * 1024:
            raise ValueError('comparison package limit')
        result = {}
        for entry in entries:
            if entry.file_size > 32 * 1024 * 1024 or entry.filename in result:
                raise ValueError('comparison part size or duplicate name limit')
            raw = package.read(entry)
            if entry.filename == 'visio/masters/masters.xml':
                raw = re.sub(rb'((?:UniqueID|BaseID)=")\{[0-9A-Fa-f-]+\}(")',
                             rb'\1{FRESH-MASTER-GUID}\2', raw)
            if entry.filename == 'docProps/core.xml':
                # Match only the emitted creation/modification metadata elements.
                # Do not mask timestamp-shaped text in custom properties or cells.
                raw = re.sub(rb'(<dcterms:(created|modified)\b[^>]*>)'
                             rb'\d{4}-\d{2}-\d{2}T\d{2}:\d{2}:\d{2}(?:\.\d+)?Z'
                             rb'(</dcterms:\2>)', rb'\1FRESH-SAVE-TIME\3', raw)
            if entry.filename == 'docProps/custom.xml':
                # skeleton_properties.rs emits this document save-time property.
                # Other custom properties, including other filetime values, stay exact.
                raw = re.sub(rb'(<property\b[^>]*\bname="TimeEdited"[^>]*><vt:filetime>)'
                             rb'\d{4}-\d{2}-\d{2}T\d{2}:\d{2}:\d{2}(?:\.\d+)?Z'
                             rb'(</vt:filetime></property>)', rb'\1FRESH-SAVE-TIME\2', raw)
            result[entry.filename] = raw
        return result


def detail(a, b):
    try:
        pending = [(ET.fromstring(a), ET.fromstring(b), '', 0)]
        visited = 0
        while pending:
            left, right, parent, depth = pending.pop()
            visited += 1
            if depth > 128 or visited > 1_000_000:
                return 'XML diagnostic limit; byte inequality remains a failure'
            tag = left.tag.rsplit('}', 1)[-1]
            keys = ','.join(f'{k}={left.get(k)}' for k in ('ID','N','IX') if left.get(k) is not None)
            path = f'{parent}/{tag}' + (f'[{keys}]' if keys else '')
            if (left.tag, left.attrib, left.text, left.tail) != (right.tag, right.attrib, right.text, right.tail):
                return f'{path}: strict={left.attrib!r}/{left.text!r}, batch={right.attrib!r}/{right.text!r}'
            if len(left) != len(right):
                return f'{path}: strict child count={len(left)}, batch child count={len(right)}'
            pending.extend((x,y,path,depth+1) for x,y in reversed(list(zip(left,right))))
        return 'XML values match but serialized bytes differ'
    except ET.ParseError:
        return 'non-XML or invalid XML; serialized bytes differ'


def compare(left, right):
    a, b = parts(left), parts(right)
    if a.keys() != b.keys():
        print('FAIL part lists:', sorted(a.keys()-b.keys()), sorted(b.keys()-a.keys()))
        return 1
    for name in sorted(a):
        if a[name] != b[name]:
            print('FAIL first differing part:', name)
            print(detail(a[name], b[name]))
            offset = next((i for i,(x,y) in enumerate(zip(a[name],b[name])) if x != y), min(len(a[name]),len(b[name])))
            print('First differing byte:', offset)
            print('strict:', repr(a[name][max(0,offset-60):offset+160]))
            print('batch: ', repr(b[name][max(0,offset-60):offset+160]))
            return 1
    print(f'PASS {len(a)} parts byte-equal after master GUID and docProps save-time masks')
    return 0


if __name__ == '__main__':
    if len(sys.argv) != 3:
        raise SystemExit('usage: compare.py FIRST.vsdx SECOND.vsdx')
    try:
        raise SystemExit(compare(sys.argv[1],sys.argv[2]))
    except (OSError, ValueError, zipfile.BadZipFile) as error:
        print(f'ERROR: {error}', file=sys.stderr)
        raise SystemExit(2)
