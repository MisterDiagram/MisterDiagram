#!/usr/bin/env python3
"""P3: compare actual Visio saves, COM dumps and PNGs with the hashed writer outputs.

Inputs: corpus/_vm/out/accept/<basename>/{status.json,saved.*,observed.sheet.tsv,*.png}.
Expected manifest: corpus/_state/m3/accept-manifest.json. Generated expectations live
under corpus/_state/m3/accept-expected/. No input observation is modified.
"""
import argparse
import csv
from collections import Counter, defaultdict
import hashlib
import json
import math
from pathlib import Path
import sys
import xml.etree.ElementTree as ET
import zipfile
from diff_dump import compare, excluded, read_dump, read_filters

ROOT = Path(__file__).resolve().parents[2]
MAX_XML = 96 * 1024 * 1024
MAX_PACKAGE = 2 * 1024**3


def load_json(path):
    with Path(path).open('rb') as f:
        data = f.read(4*1024*1024+1)
    if len(data) > 4*1024*1024:
        raise ValueError('acceptance JSON byte budget')
    try:
        value = json.loads(data.decode('utf-8-sig'))
        pending = [(value,0)]
        nodes = 0
        while pending:
            item,depth = pending.pop()
            nodes += 1
            if depth > 128 or nodes > 100000:
                raise ValueError('acceptance JSON structure budget')
            if isinstance(item,dict): pending.extend((v,depth+1) for v in item.values())
            elif isinstance(item,list): pending.extend((v,depth+1) for v in item)
        return value
    except (RecursionError, UnicodeError) as error:
        raise ValueError('invalid or excessively nested acceptance JSON') from error


def read_connects(path):
    """Bounded COM connectivity table, independent of the post-save XML."""
    with Path(path).open('rb') as f:
        data = f.read(8*1024*1024+1)
    if len(data) > 8*1024*1024:
        raise ValueError('connects byte budget')
    try:
        lines = csv.reader(data.decode('utf-8-sig').splitlines(), delimiter='\t')
        if next(lines, None) != ['from_sheet','from_cell','to_sheet','to_cell','from_part','to_part']:
            raise ValueError('connects header')
        rows = []
        for row in lines:
            if len(rows) >= 100000 or len(row) != 6 or any(len(v) > 4096 for v in row):
                raise ValueError('connects row budget or columns')
            for i in (0,2,4,5):
                if not row[i].isascii() or not row[i].isdecimal() or int(row[i]) > 4294967295:
                    raise ValueError('connects integer')
            rows.append(tuple(row))
        return rows
    except (UnicodeError, csv.Error) as error:
        raise ValueError('invalid connects table') from error


def digest(path):
    h = hashlib.sha256()
    with Path(path).open('rb') as f:
        for block in iter(lambda: f.read(1024 * 1024), b''):
            h.update(block)
    return h.hexdigest()


def package_cells(path):
    """Literal serialized sheet dump, keyed by part, sheet IDs and section/row identities.

    This is separate from the COM-compatible evaluated dump. Neither replaces the
    other: a cache change must not disappear through re-evaluation by our own engine.
    """
    cells = {}
    ordinals = Counter()
    identity_bytes = 0
    with zipfile.ZipFile(path) as z:
        infos = z.infolist()
        if len(infos) > 100000 or sum(i.file_size for i in infos) > MAX_PACKAGE:
            raise ValueError('package budget exceeded')
        names = [i.filename for i in infos if not i.is_dir()]
        if len(set(names)) != len(names):
            raise ValueError('duplicate package part')
        for info in infos:
            # Solution/theme/media payloads are not ShapeSheet parts. Some shipped
            # solution XML is opaque to the reader and not standalone namespace-valid.
            if not (info.filename == 'visio/document.xml' or info.filename.startswith(('visio/pages/','visio/masters/'))) or not info.filename.endswith('.xml'):
                continue
            if info.file_size > MAX_XML:
                raise ValueError('XML budget exceeded')
            data = z.read(info)
            if b'\x00' in data or b'<!DOCTYPE' in data.upper() or b'<!ENTITY' in data.upper():
                raise ValueError('XML entities forbidden')
            root = ET.fromstring(data)
            pending = [(root, info.filename, 0)]
            while pending:
                node, parent, depth = pending.pop()
                if depth > 128:
                    raise ValueError('XML depth exceeded')
                tag = node.tag.rsplit('}', 1)[-1]
                if len(parent) > 65536 or sum(len(v) for v in node.attrib.values()) > 65536:
                    raise ValueError('identity length budget')
                if tag in {'Shape', 'Master', 'Page', 'StyleSheet'}:
                    parent += '/' + tag + ':' + node.get('ID', '')
                elif tag in {'Section', 'Row'}:
                    parent += '/' + tag + ':' + node.get('N', '') + ':' + node.get('IX', '')
                elif tag in {'PageSheet', 'DocumentSheet'}:
                    parent += '/' + tag
                if tag == 'Cell':
                    key = parent + '/Cell:' + node.get('N', '')
                    # Duplicate source entries cannot mask one another.
                    identity_bytes += len(key.encode('utf-8'))
                    if identity_bytes > 64*1024*1024:
                        raise ValueError('cell identity budget')
                    ordinal = ordinals[key]
                    ordinals[key] += 1
                    cells[key, ordinal] = {k: node.get(k) for k in ('V', 'F', 'U', 'E')}
                    if len(cells) > 2_000_000:
                        raise ValueError('cell budget exceeded')
                elif tag == 'Connect':
                    key = parent + '/Connect:' + json.dumps(dict(sorted(node.attrib.items())))
                    ordinal = ordinals[key]
                    ordinals[key] += 1
                    identity_bytes += len(key.encode('utf-8'))
                    if identity_bytes > 64*1024*1024 or len(cells) >= 2_000_000:
                        raise ValueError('connect identity budget')
                    cells[key, ordinal] = {'V': json.dumps(dict(sorted(node.attrib.items()))), 'F': None, 'U': None, 'E': None}
                pending.extend((c, parent, depth + 1) for c in reversed(list(node)))
        return sorted(names), cells


def cell_diffs(before, after, filters):
    failures = []
    for key in sorted(before.keys() | after.keys()):
        a, b = before.get(key), after.get(key)
        if a is not None and excluded(a.get('F') or '', filters):
            continue
        if a is None or b is None:
            failures.append((str(key), 'added' if a is None else 'removed', a, b))
            continue
        for field in ('V', 'F', 'U', 'E'):
            x, y = a[field], b[field]
            same = x == y
            if not same and field == 'V' and x is not None and y is not None:
                try:
                    xv, yv = float(x), float(y)
                    same = math.isfinite(xv) and math.isfinite(yv) and abs(xv-yv) <= 1e-6
                except ValueError:
                    pass
            if not same:
                failures.append((str(key), field, x, y))
    return failures


def pixels(a, b):
    from PIL import Image, ImageChops
    with Image.open(a) as ia, Image.open(b) as ib:
        sizes = ia.size, ib.size
        if any(im.width <= 0 or im.height <= 0 or im.width > 16384 or im.height > 16384 or im.width*im.height > 32*1024*1024 for im in (ia,ib)):
            raise ValueError('image header pixel budget')
        if any(abs(x-y) > 1 for x, y in zip(*sizes)):
            return {'sizes': sizes, 'error': 'hard size mismatch'}
        w, h = min(ia.width, ib.width), min(ia.height, ib.height)
        if w*h > 32*1024*1024 or w*h == 0:
            raise ValueError('image pixel budget')
        # Both renderers export against white. Explicit alpha compositing avoids hidden RGB.
        def rgb(im):
            im = im.crop((0, 0, w, h)).convert('RGBA')
            white = Image.new('RGBA', im.size, 'white')
            return Image.alpha_composite(white, im).convert('RGB')
        d = ImageChops.difference(rgb(ia), rgb(ib))
        mad = sum((i % 256)*n for i,n in enumerate(d.histogram()))/(w*h*3)
        r,g,b = d.split()
        over = sum(ImageChops.lighter(ImageChops.lighter(r,g),b).histogram()[17:])
        return {'sizes': sizes, 'mad': mad, 'over': over, 'pixels': w*h}


def prepare_manifest():
    plan = load_json(ROOT/'corpus/_state/m3/accept-plan.json')
    if len(plan['files']) != 230:
        raise ValueError('acceptance set must contain 230 files')
    expected = ROOT/'corpus/_state/m3/accept-expected'
    for entry in plan['files']:
        path = ROOT/entry['output']
        entry['sha256'] = digest(path)
        if entry['source']:
            entry['source_sha256'] = digest(ROOT/entry['source'])
        directory = expected/path.stem
        parts, cells = package_cells(path)
        (directory/'parts.json').write_text(json.dumps(parts, indent=2)+'\n')
        (directory/'serialized.cells.json').write_text(json.dumps([[k, v] for k, v in sorted(cells.items())], indent=2)+'\n')
        entry['expected'] = {f.name: digest(f) for f in directory.iterdir() if f.is_file()}
    plan['script_sha256'] = digest(ROOT/'tools/oracle/accept.ps1')
    plan['common_sha256'] = digest(ROOT/'tools/oracle/common.ps1')
    plan['status'] = 'materialized writer outputs, not Visio acceptance'
    target = ROOT/'corpus/_state/m3/accept-manifest.json'
    target.write_text(json.dumps(plan, indent=2)+'\n')
    print(f'{target}: 230 hashed inputs')


def report(out, manifest):
    m = load_json(manifest)
    if len(m['files']) != 230 or len({e['output'] for e in m['files']}) != 230:
        raise ValueError('expected exactly 230 unique acceptance files')
    filters = read_filters([ROOT/'crates/sheet/tests/hooks.txt', ROOT/'crates/sheet/tests/volatile.txt'])
    sets, groups = defaultdict(Counter), defaultdict(list)
    all_rows = []
    for entry in m['files']:
        path = ROOT/entry['output']
        name = path.stem
        counter = sets[entry['set']]
        counter['files'] += 1
        row = {'file': name, 'errors': [], 'repair_count': 0, 'com_failures': 0, 'renders': []}
        expected = ROOT/'corpus/_state/m3/accept-expected'/name
        actual = out/name
        try:
            if digest(path) != entry['sha256']:
                raise ValueError('writer input changed since manifest')
            for file, want in entry['expected'].items():
                if Path(file).name != file or digest(expected/file) != want:
                    raise ValueError('expected artifact hash mismatch')
            status = load_json(actual/'status.json')
            if status['input_sha256'] != entry['sha256'] or status['script_sha256'] != m['script_sha256'] or status.get('common_sha256') != m['common_sha256']:
                raise ValueError('stale VM observation')
            if type(status.get('opened')) is not bool or type(status.get('saved')) is not bool:
                raise ValueError('invalid VM status flags')
            if status['opened']:
                counter['opened'] += 1
            if status['error'] or not status['saved']:
                raise ValueError(status['error'] or 'no post-open save')
            hashes = {a['file']: a['sha256'] for a in status['artifacts']}
            for file, want in hashes.items():
                if Path(file).name != file or digest(actual/file) != want:
                    raise ValueError('VM artifact hash mismatch')
            saved = 'saved'+path.suffix
            if not {'observed.sheet.tsv', 'connects.tsv', 'parts.txt', saved} <= hashes.keys():
                raise ValueError('incomplete VM artifact inventory')
            connects = read_connects(actual/'connects.tsv')
            row['connects'] = len(connects)
            if name == 'scratch__dynamic-glue' and Counter(connects) != Counter([
                ('3','BeginX','1','PinX','9','3'), ('3','EndX','2','PinX','12','3')]):
                raise ValueError('scratch dynamic glue must have its two observed Connects rows')
            old_parts, old_cells = package_cells(path)
            new_parts, new_cells = package_cells(actual/saved)
            if (actual/'parts.txt').read_text(encoding='utf-8-sig').splitlines() != new_parts:
                raise ValueError('post-save part list disagrees with saved package')
            part_diff = {'added': sorted(set(new_parts)-set(old_parts)), 'removed': sorted(set(old_parts)-set(new_parts))}
            cells = cell_diffs(old_cells, new_cells, filters)
            row['part_diff'] = part_diff
            row['cell_diff'] = cells[:200]
            row['repair_count'] = len(cells)+len(part_diff['added'])+len(part_diff['removed'])
            if row['repair_count']:
                counter['repaired'] += 1
            _, _, failures = compare(read_dump(actual/'observed.sheet.tsv'), read_dump(expected/'computed.sheet.tsv'), filters)
            row['computed_diagnostic_failures'] = len(failures)
            row['computed_diagnostic_diff'] = failures[:100]
            source = read_dump(expected/'serialized.sheet.tsv')
            observed = read_dump(actual/'observed.sheet.tsv')
            # COM has inherited/default rows which were not serialized locally. Those
            # are diagnostics above, not invented source caches. XML catches new cells.
            _, _, failures = compare({k:v for k,v in observed.items() if k in source}, source, filters)
            row['source_com_rows'] = len(source)
            row['serialized_cells_and_connects'] = len(old_cells)
            row['com_failures'] = len(failures)
            row['com_diff'] = failures[:100]
            if failures and not row['repair_count']:
                counter['repaired'] += 1
            images = load_json(expected/'images.json')
            for item in images:
                stem = item['stem']
                if Path(stem).name != stem:
                    raise ValueError('invalid image name')
                try:
                    if item['error']:
                        raise ValueError('our renderer: '+item['error'])
                    if stem+'.png' not in hashes:
                        raise ValueError('missing Visio image')
                    result = pixels(expected/(stem+'.png'), actual/(stem+'.png'))
                except (OSError, ValueError) as error:
                    result = {'error': str(error)}
                result['file'] = name+'#'+stem
                row['renders'].append(result)
                group = name.split('__')[1] if name.startswith('generated__') and stem.startswith('page-') else entry['set']+('-masters' if stem.startswith('master-') else '-pages')
                groups[group].append(result)
            if row['repair_count'] or failures or any('error' in r for r in row['renders']):
                counter['failed'] += 1
        except (OSError, ValueError, KeyError, TypeError, AttributeError, zipfile.BadZipFile, ET.ParseError) as error:
            row['errors'].append(str(error))
            counter['failed'] += 1
        all_rows.append(row)
        print(f"accept file {name}: repair_diffs={row['repair_count']} COM_failures={row['com_failures']} errors={row['errors']}")
    success = (out/'DONE').exists()
    summaries = {}
    for group, items in sorted(groups.items()):
        valid = [i for i in items if 'error' not in i]
        mad = sum(i['mad'] for i in valid)/len(valid) if valid else math.inf
        count = sum(i['pixels'] for i in valid)
        frac = sum(i['over'] for i in valid)/count if count else math.inf
        ok = len(valid) == len(items) and mad <= (8 if group == 'text' else 4) and frac <= (.05 if group == 'text' else .02)
        success &= ok
        worst = sorted(items, key=lambda i:i.get('mad',math.inf),reverse=True)[:3]
        summaries[group] = {'images':len(items),'errors':len(items)-len(valid),'mad':mad if math.isfinite(mad) else None,'fraction':frac if math.isfinite(frac) else None,'passed':ok,'worst':worst}
        print(f'accept render {group}: images={len(items)} errors={len(items)-len(valid)} MAD={mad:.6f} over16={frac:.6%} pass={ok}')
        for i in worst:
            print('  worst',i)
    for name,c in sorted(sets.items()):
        print(f"accept set {name}: files={c['files']} opened={c['opened']} repaired={c['repaired']} failed={c['failed']}")
        success &= c['opened'] == c['files'] and c['repaired'] == c['failed'] == 0
    success &= bool(groups)
    target = ROOT/'corpus/_state/m3/accept-report.json'
    target.write_text(json.dumps({'sets':sets,'groups':summaries,'files':all_rows,'passed':success},indent=2)+'\n')
    return 0 if success else 1


def main():
    parser = argparse.ArgumentParser(description=__doc__)
    parser.add_argument('out',type=Path,nargs='?',default=ROOT/'corpus/_vm/out/accept')
    parser.add_argument('--prepare',action='store_true')
    parser.add_argument('--manifest',type=Path,default=ROOT/'corpus/_state/m3/accept-manifest.json')
    args = parser.parse_args()
    try:
        if args.prepare:
            prepare_manifest()
            return 0
        return report(args.out,args.manifest)
    except (OSError,ValueError,KeyError,TypeError,AttributeError,zipfile.BadZipFile,ET.ParseError) as error:
        print(f'accept report: {error}',file=sys.stderr)
        return 1

if __name__ == '__main__':
    sys.exit(main())
