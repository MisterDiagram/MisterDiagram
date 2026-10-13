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


# Draft helpers to insert into accept_report.py in M3.18.
def read_serialized(path):
    """Local V/F/U/E text, never evaluated and never replaced by an Inh provider."""
    from diff_dump import unescape, MAX_LINE
    rows = []
    total_bytes = 0
    retained_bytes = 0
    with Path(path).open(encoding='utf-8-sig', newline='') as f:
        if f.readline(MAX_LINE+1).rstrip('\r\n').split('\t') != ['scope','owner','shape','name','V','F','U','E','present','active']:
            raise ValueError('serialized source header')
        while True:
            line = f.readline(MAX_LINE+1)
            if not line: break
            size = len(line.encode("utf-8"))
            total_bytes += size
            retained_bytes += size*4+512
            if retained_bytes > 256*1024*1024 or total_bytes > 96*1024*1024 or len(line) > MAX_LINE or len(rows) >= 2_000_000:
                raise ValueError('serialized source budget')
            fields = line.rstrip('\r\n').split('\t')
            if len(fields) != 10 or fields[0] not in {'document','style','page','master','page-shape','master-shape'}:
                raise ValueError('serialized source columns')
            for i in (1,2,8,9):
                if not fields[i].isascii() or not fields[i].isdecimal() or int(fields[i]) > 4294967295:
                    raise ValueError('serialized source index')
            bits = int(fields[8])
            if bits > 15 or fields[9] not in {'0','1'}: raise ValueError('serialized source flags')
            values = [unescape(fields[i]) if bits & (1 << (i-4)) else None for i in range(4,8)]
            rows.append((fields[0],int(fields[1]),int(fields[2]),unescape(fields[3]),*values,fields[9]=='1'))
    return rows


def com_identity(row):
    identity, shape = row[1], int(row[0])
    if identity.startswith('document/'): return ('document',0,0,row[5])
    if identity.startswith('style/'): return ('style',shape,shape,row[5])
    for prefix, kind in [('pagesheet:','page'),('page:','page-shape'),('master:','master-shape')]:
        if identity.startswith(prefix):
            owner = identity[len(prefix):].split(':',1)[0]
            if not owner.isdecimal(): raise ValueError('COM scope identity')
            return (kind,int(owner),0 if kind == 'page' else shape,row[5])
    raise ValueError('unknown COM identity')


def source_com_compare(raw, computed, observed, filters, font_names):
    """COM metadata supplies SRC/unit definitions only; values come from local V.

    No PageSheet XForm defaults are invented. RGB and font names have transient
    COM table IDs and remain literal XML comparisons, as in the earlier reporter.
    """
    metadata = {com_identity(v):v for v in computed.values()}
    actual = {com_identity(v):v for v in observed.values()}
    language_ids = project_language_ids()
    failures, counts = [], Counter()
    for kind, owner, shape, name, value, formula, unit, error, active in raw:
        key = (kind,owner,shape,name)
        if not active or kind == 'master' or value is None or value == 'Themed':
            counts['non_COM_local'] += 1
            continue
        want, got = metadata.get(key), actual.get(key)
        if excluded(formula or '',filters) or (got is not None and excluded(got[6],filters)):
            counts['excluded'] += 1
            continue
        if kind == 'style' and (name == 'LayerMember' or name.startswith(('FillGradientStops.','LineGradientStops.'))):
            counts['non_COM_local'] += 1
            continue
        if want is None and got is None and kind == 'style':
            counts['unobserved_style_local'] += 1
            continue
        if want is None:
            counts['unmapped_local'] += 1
            failures.append((key, 'local source cell has no COM metadata mapping'))
            continue
        # Style traversal deliberately omits XML gradient and LayerMember cells.
        if kind == 'style' and (want[2] in {'248','249'} or (want[2],want[3]) == ('1','6')):
            counts['non_COM_local'] += 1
            continue
        if (len(value) == 7 and value.startswith('#') and all(c in '0123456789abcdefABCDEF' for c in value[1:])) or (name.split('.')[-1].split('[')[0] == 'Font' and not value.isdecimal()):
            counts['table_identity_literal_only'] += 1
            continue
        counts['eligible'] += 1
        if got is None:
            failures.append((key,'missing'))
            continue
        units = want[8]
        explicit_units = {'IN':'65','MM':'70','CM':'69','M':'71','PT':'50','DL':'73','NUM':'32','BOOL':'32','STR':'231','COLOR':'251','DEG':'81','RAD':'83','PER':'33','PCT':'33','P':'51','FT':'66','DATE':'40','GUID':'95','NURBS':'138','POLYLINE':'139'}
        if unit in explicit_units and unit != "DL": units = explicit_units[unit]
        try:
            if name.split('.')[-1].split('[')[0] in {'LangID','DocLangID'} and value.lower() in language_ids:
                expected_value = float(language_ids[value.lower()])
            elif name.split('.')[-1].split('[')[0] in {'LangID','DocLangID'}:
                expected_value = float(value)
            elif units == '231': expected_value = 0.
            else:
                try: expected_value = float(value or '0')
                except ValueError: expected_value = 0.  # COM ResultIU for literal text.
            got_value = float(got[7])
            same = math.isfinite(expected_value) and math.isfinite(got_value) and abs(expected_value-got_value) <= 1e-6
        except ValueError: same = False
        if not same: failures.append((key,f'value local V={value!r} actual={got[7]!r}'))
        if got[8] != units: failures.append((key,f'units expected={units} actual={got[8]} raw U={unit!r}'))
    return counts, failures


def project_language_ids():
    """Use the project's normative vLanguageString table, including neutral tags.

    https://learn.microsoft.com/en-us/openspecs/sharepoint_protocols/ms-vsdx/f91b1356-cff1-4c71-8a04-247007cbe4c9
    """
    import re
    with (ROOT/'crates/sheet/src/language.rs').open(encoding='utf-8') as f:
        source = f.read(65537)
    if len(source) > 65536: raise ValueError('language table byte budget')
    _, marker, table = source.partition('const LANGUAGES:')
    if not marker or '];' not in table: raise ValueError('language table definition')
    table = table.split('];',1)[0]
    pairs = re.findall(r'\("([^"\n]{1,128})",\s*([0-9]{1,5})\)',table)
    if not pairs or len(pairs) > 4096: raise ValueError('language table entries')
    return {name.lower():int(value) for name,value in pairs}


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
                # Opaque payloads are byte contracts, including solution XML.
                known_xml = (info.filename == '[Content_Types].xml' or info.filename.endswith('.rels')
                    or info.filename == 'visio/windows.xml' or info.filename.startswith(('docProps/','visio/theme/')) and info.filename.endswith('.xml'))
                if not info.is_dir() and not known_xml:
                    h = hashlib.sha256()
                    with z.open(info) as payload:
                        for block in iter(lambda: payload.read(1024*1024), b''): h.update(block)
                    cells[(info.filename+'/Opaque',0)] = {'V':h.hexdigest(),'F':None,'U':None,'E':None,'kind':'opaque'}
                continue
            if info.file_size > MAX_XML:
                raise ValueError('XML budget exceeded')
            data = z.read(info)
            if b'\x00' in data or b'<!DOCTYPE' in data.upper() or b'<!ENTITY' in data.upper():
                raise ValueError('XML entities forbidden')
            root = ET.fromstring(data)
            pending = [(root, info.filename, 0, 0)]
            while pending:
                node, parent, depth, row_position = pending.pop()
                if depth > 128:
                    raise ValueError('XML depth exceeded')
                tag = node.tag.rsplit('}', 1)[-1]
                if len(parent) > 65536 or sum(len(v) for v in node.attrib.values()) > 65536:
                    raise ValueError('identity length budget')
                if tag in {'Shape', 'Master', 'Page', 'StyleSheet'}:
                    parent += '/' + tag + ':' + node.get('ID', '')
                elif tag in {'Section', 'Row'}:
                    index = node.get('IX', str(row_position) if tag == 'Row' and node.get('N') is None else '')
                    parent += '/' + tag + ':' + node.get('N', '') + ':' + index
                elif tag in {'PageSheet', 'DocumentSheet'}:
                    parent += '/' + tag
                if tag in {'Shape','Master','Page','Section','Row','PageSheet','DocumentSheet','StyleSheet'}:
                    key = parent+'/Present'
                    identity_bytes += len(key.encode('utf-8'))
                    if identity_bytes > 64*1024*1024 or len(cells) >= 2_000_000: raise ValueError('structure identity budget')
                    ordinal = ordinals[key]; ordinals[key] += 1
                    cells[key,ordinal] = {'V':tag,'F':None,'U':None,'E':None,'kind':'structure'}
                attrs = node.attrib if tag in {'Shape','Master','Page'} else ({k:v for k,v in node.attrib.items() if k in {'T','Del'}} if tag in {'Row','Section'} else {})
                for attr,value in sorted(attrs.items()):
                    key = parent + '/Attribute:' + attr
                    identity_bytes += len(key.encode('utf-8'))
                    if identity_bytes > 64*1024*1024 or len(cells) >= 2_000_000: raise ValueError('attribute identity budget')
                    ordinal = ordinals[key]; ordinals[key] += 1
                    kind = 'master_type' if tag == 'Master' and attr == 'MasterType' else 'master_uniqueid' if tag == 'Master' and attr == 'UniqueID' else 'attribute'
                    cells[key,ordinal] = {'V':value,'F':None,'U':None,'E':None,'kind':kind}
                if tag in {'Text','Icon'}:
                    key = parent+'/'+tag
                    identity_bytes += len(key.encode('utf-8'))
                    if identity_bytes > 64*1024*1024 or len(cells) >= 2_000_000: raise ValueError('text identity budget')
                    h = hashlib.sha256()
                    stack = [(node,0,False)]
                    while stack:
                        child,level,closing = stack.pop()
                        if level > 128: raise ValueError('text depth budget')
                        if closing:
                            h.update(b'</>')
                            if child is not node: h.update((child.tail or '').encode())
                        else:
                            h.update(json.dumps([child.tag,sorted(child.attrib.items()),child.text],ensure_ascii=False).encode())
                            stack.append((child,level,True))
                            stack.extend((c,level+1,False) for c in reversed(list(child)))
                    ordinal = ordinals[key]; ordinals[key] += 1
                    cells[key,ordinal] = {'V':h.hexdigest(),'F':None,'U':None,'E':None,'kind':tag.lower()}
                    continue
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
                children = []
                row_position = 0
                for child in node:
                    children.append((child,parent,depth+1,row_position))
                    if child.tag.rsplit('}',1)[-1] == 'Row': row_position += 1
                pending.extend(reversed(children))
        return sorted(names), cells


def cell_diffs(before, after, filters):
    failures = []
    for key in sorted(before.keys() | after.keys()):
        a, b = before.get(key), after.get(key)
        if (a is not None and excluded(a.get('F') or '', filters)) or (b is not None and excluded(b.get('F') or '', filters)):
            continue
        if a is None or b is None:
            failures.append((str(key), 'added' if a is None else 'removed', a, b))
            continue
        for field in ('V', 'F', 'U', 'E'):
            x, y = a[field], b[field]
            same = x == y
            if not same:
                failures.append((str(key), field, x, y))
    return failures


def pixels(a, b):
    from PIL import Image, ImageChops
    with Image.open(a) as ia, Image.open(b) as ib:
        sizes = ia.size, ib.size
        if any(im.width <= 0 or im.height <= 0 or im.width > 16384 or im.height > 16384 or im.width*im.height > 32*1024*1024 for im in (ia,ib)):
            raise ValueError('image header pixel budget')
        if sizes[0] != sizes[1]:
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


def package_empty_views(path):
    """Resolve actual saved page/master content, rather than trusting an expectation."""
    import posixpath
    empty = set()
    with zipfile.ZipFile(path) as z:
        def xml(name):
            info = z.getinfo(name)
            if info.file_size > MAX_XML: raise ValueError('view XML budget')
            data = z.read(info)
            if b'\x00' in data or b'<!DOCTYPE' in data.upper() or b'<!ENTITY' in data.upper(): raise ValueError('view XML entities')
            return ET.fromstring(data)
        for plural, singular in [('pages','Page'),('masters','Master')]:
            listing = f'visio/{plural}/{plural}.xml'
            if listing not in z.namelist(): continue
            rels_path = f'visio/{plural}/_rels/{plural}.xml.rels'
            rels = {} if rels_path not in z.namelist() else {r.get('Id'):r.get('Target') for r in xml(rels_path) if r.get('TargetMode') != 'External'}
            for node in xml(listing):
                if node.tag.rsplit('}',1)[-1] != singular: continue
                rel = node.find('{*}Rel')
                rid = next((v for k,v in rel.attrib.items() if k.rsplit('}',1)[-1] == 'id'),None) if rel is not None else None
                target = rels.get(rid)
                if not target: continue
                part = posixpath.normpath(posixpath.join(posixpath.dirname(listing),target))
                if part.startswith('../') or part.startswith('/'): raise ValueError('view relationship traversal')
                content = xml(part)
                if node.get('BackPage') is None and content.find('.//{*}Shape') is None:
                    empty.add(('page-' if singular == 'Page' else 'master-')+node.get('ID',''))
    return empty


def empty_pixels(a,b):
    from PIL import Image,ImageChops
    with Image.open(a) as ia, Image.open(b) as ib:
        if any(im.width <= 0 or im.height <= 0 or im.width*im.height > 32*1024*1024 for im in (ia,ib)):
            raise ValueError('empty image header budget')
        if ia.size != (480,288): raise ValueError('empty acceptance canvas must be 480x288')
        rgba = ia.convert('RGBA')
        white = Image.new('RGBA',rgba.size,'white')
        if ImageChops.difference(Image.alpha_composite(white,rgba).convert('RGB'),white.convert('RGB')).getbbox():
            raise ValueError('empty source rendered ink')
        return {'sizes':(ia.size,ib.size),'empty':True}


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


def computed_diagnostic(observed, computed, old_cells, new_cells, filters):
    """Disclose invented PageSheet XForm unit slots separately from actual cells.

    Probe pages.xml has no XForm row while the harness changes its virtual unit
    from32 to73. This adjustment never applies to a serialized Cell or its value.
    """
    phantom = set()
    for key,want in computed.items():
        got = observed.get(key)
        if not got or not want[1].startswith('pagesheet:') or want[2:4] != ['1','1'] or {want[8],got[8]} != {'32','73'}:
            continue
        page = want[1].split(':',2)[1]
        suffix = '/Page:'+page+'/PageSheet/Cell:'+want[5]
        if any(k[0].endswith(suffix) for cells in (old_cells,new_cells) for k in cells): continue
        phantom.add(key)
    actual = {k:v for k,v in observed.items() if not excluded(v[6],filters) and not excluded(computed.get(k,v)[6],filters)}
    expected = {k:v for k,v in computed.items() if not excluded(v[6],filters) and not excluded(observed.get(k,v)[6],filters)}
    _,_,failures = compare(actual,expected,filters)
    return [f for f in failures if not(f[0] in phantom and f[1].startswith('units '))],len(phantom)


def scratch_render_assertion(name, stem, result, image):
    """Concrete from-scratch witnesses required by M3.13 to M3.16."""
    if 'error' in result: return
    expected = None
    if name == 'scratch__dropped-master' and stem == 'page-0': expected = (193,97)
    if name == 'scratch__three-masters':
        expected = {'master-1':(97,97),'master-2':(145,97),'master-3':(193,97)}.get(stem)
    if expected and any(tuple(size) != expected for size in result['sizes']):
        raise ValueError(f'from-scratch required size {expected}, measured {result["sizes"]}')
    if name == 'scratch__dynamic-glue' and stem == 'page-0':
        from PIL import Image
        with Image.open(image) as source:
            rgb=source.convert('RGB')
            # The two authored rectangles span x1..3 and x4..6. Their empty gap
            # occupies the middle fifth of the export; the route lies at mid-height.
            left,right=rgb.width*9//20,rgb.width*11//20
            mid=rgb.height//2
            if not any(max(rgb.getpixel((x,y))) < 240 for y in range(max(0,mid-1),min(rgb.height,mid+2)) for x in range(left,right)):
                raise ValueError('dynamic connector has no dark pixels across its gap')


def allowed_rewrites(old_cells, new_cells, differences, part_diff):
    """Only Charles's M3.20 classes may differ without being a repair.

    Witness: scratch rectangle-text original/saved master UniqueID and MasterType;
    missing thumbnail accepted, 2x2 PNG rejected in diag-variants/v1-nothumb.vsdx.
    """
    import re
    classes = {'thumbnail_added': [], 'master_type': [], 'master_uniqueid': []}
    if 'docProps/thumbnail.emf' in part_diff['added']:
        classes['thumbnail_added'].append('docProps/thumbnail.emf')
        part_diff = dict(part_diff, added=[n for n in part_diff['added'] if n != 'docProps/thumbnail.emf'])
    kinds = {str(k):v.get('kind') for k,v in old_cells.items()}
    kinds.update({str(k):v.get('kind') for k,v in new_cells.items()})
    remaining = []
    guid = re.compile(r'^\{[0-9A-F]{8}-[0-9A-F]{4}-[0-9A-F]{4}-8E40-00608CF305B2\}$', re.I)
    for diff in differences:
        key, field, before, after = diff
        kind = kinds.get(key)
        if kind == 'opaque' and key == str(('docProps/thumbnail.emf/Opaque',0)) and field == 'added' and classes['thumbnail_added']:
            continue
        if kind == 'master_type' and field == 'V':
            classes['master_type'].append(diff)
        elif kind == 'master_uniqueid' and field == 'V' and isinstance(before,str) and isinstance(after,str) and guid.fullmatch(after) and before[:19].lower() == after[:19].lower():
            classes['master_uniqueid'].append(diff)
        else: remaining.append(diff)
    return remaining, part_diff, classes


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
            cells, part_diff, allowed = allowed_rewrites(old_cells,new_cells,cells,part_diff)
            row['allowed'] = allowed
            for kind, witnesses in allowed.items(): counter[kind] += len(witnesses)
            row['part_diff'] = part_diff
            row['cell_diff'] = cells[:200]
            row['repair_count'] = len(cells)+len(part_diff['added'])+len(part_diff['removed'])
            if row['repair_count']:
                counter['repaired'] += 1
            failures, phantom = computed_diagnostic(read_dump(actual/'observed.sheet.tsv'), read_dump(expected/'computed.sheet.tsv'), old_cells, new_cells, filters)
            row['harness_phantom_unit_slots'] = phantom
            row['computed_diagnostic_failures'] = len(failures)
            row['computed_diagnostic_diff'] = failures[:100]
            source = read_serialized(expected/'serialized.sheet.tsv')
            observed = read_dump(actual/'observed.sheet.tsv')
            with zipfile.ZipFile(path) as package:
                doc_info = package.getinfo('visio/document.xml')
                if doc_info.file_size > MAX_XML: raise ValueError('font metadata budget')
                doc_root = ET.fromstring(package.read(doc_info))
                font_names = [n.get('NameU','') for n in doc_root.findall('./{*}FaceNames/{*}FaceName')]
            counts, failures = source_com_compare(source,read_dump(expected/'computed.sheet.tsv'),observed,filters,font_names)
            row['source_com_rows'] = counts['eligible']
            row['source_com_counts'] = counts
            metadata_keys = {com_identity(v) for v in read_dump(expected/'computed.sheet.tsv').values()}
            observed_keys = {com_identity(v) for v in observed.values()}
            row['unobserved_style_local'] = [list(r[:4]) for r in source if r[0] == 'style' and tuple(r[:4]) not in metadata_keys and tuple(r[:4]) not in observed_keys and r[8] and r[4] is not None and r[4] != 'Themed' and not excluded(r[5] or '', filters) and r[3] != 'LayerMember' and not r[3].startswith(('FillGradientStops.','LineGradientStops.'))]
            row['serialized_records'] = len(old_cells)
            row['com_failures'] = len(failures)
            row['com_diff'] = failures[:100]
            empty_before = package_empty_views(path)
            empty_after = package_empty_views(actual/saved)
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
                    if item.get('empty') and stem in empty_before and stem in empty_after:
                        result = empty_pixels(expected/(stem+'.png'), actual/(stem+'.png'))
                    else:
                        result = pixels(expected/(stem+'.png'), actual/(stem+'.png'))
                    scratch_render_assertion(name,stem,result,actual/(stem+'.png'))
                    scratch_render_assertion(name,stem,result,expected/(stem+'.png'))
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
        print(f"accept file {name}: repair_diffs={row['repair_count']} COM_failures={row['com_failures']} unobserved_style_local={len(row.get('unobserved_style_local',[]))} errors={row['errors']}")
    success = (out/'DONE').exists()
    summaries = {}
    for group, items in sorted(groups.items()):
        valid = [i for i in items if 'error' not in i]
        scored = [i for i in valid if not i.get('empty')]
        empty_count = len(valid)-len(scored)
        mad = sum(i['mad'] for i in scored)/len(scored) if scored else math.inf
        count = sum(i['pixels'] for i in scored)
        frac = sum(i['over'] for i in scored)/count if count else math.inf
        ok = len(valid) == len(items) and (not scored or (mad <= (8 if group == 'text' else 4) and frac <= (.05 if group == 'text' else .02)))
        success &= ok
        worst = sorted((i for i in items if not i.get('empty')), key=lambda i:i.get('mad',math.inf),reverse=True)[:3]
        summaries[group] = {'images':len(items),'empty':empty_count,'errors':len(items)-len(valid),'mad':mad if math.isfinite(mad) else None,'fraction':frac if math.isfinite(frac) else None,'passed':ok,'worst':worst}
        print(f'accept render {group}: images={len(items)} empty={empty_count} errors={len(items)-len(valid)} MAD={mad:.6f} over16={frac:.6%} pass={ok}')
        for i in worst:
            print('  worst',i)
    for name,c in sorted(sets.items()):
        print(f"accept set {name}: files={c['files']} opened={c['opened']} repaired={c['repaired']} failed={c['failed']} thumbnail_added={c['thumbnail_added']} master_type={c['master_type']} master_uniqueid={c['master_uniqueid']}")
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
