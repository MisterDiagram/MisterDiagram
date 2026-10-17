"""Charles's 2026-10-17 router-owned Geometry acceptance ruling.

Only finite numeric V fields use the existing absolute 1e-6 route bound.
Ownership is conservative: unresolved or ambiguous metadata stays byte-exact.
MIT OR Apache-2.0.
"""
import math
import re
import xml.etree.ElementTree as ET
import zipfile

NS = 'http://schemas.microsoft.com/office/visio/2012/main'
REL = 'http://schemas.openxmlformats.org/officeDocument/2006/relationships'
PKG = 'http://schemas.openxmlformats.org/package/2006/relationships'
BOUND = 1e-6


def number(value):
    try:
        result = float(value)
        return result if math.isfinite(result) else None
    except (ValueError, TypeError, OverflowError):
        return None


def owned_shapes(path, include_rows=False):
    """Resolve literal ownership flags through unique masters and document/default sheets.

    Mirrors the router's ObjType=2, BeginX present, ConFixedCode!=2 contract.
    Arbitrary flag formulas, groups, external relationships and ambiguous IDs
    are deliberately not eligible. No inferred ownership from names or glue.
    """
    with zipfile.ZipFile(path) as z:
        infos = z.infolist()
        if len(infos) > 100000 or sum(i.file_size for i in infos) > 2*1024*1024*1024:
            raise ValueError('route ownership package budget')
        if len({i.filename for i in infos}) != len(infos):
            raise ValueError('duplicate route ownership part')
        cache = {}

        def xml(name):
            if name in cache:
                return cache[name]
            if name not in z.namelist():
                return None
            if z.getinfo(name).file_size > 96*1024*1024:
                raise ValueError('route ownership XML budget')
            data = z.read(name)
            if b'\x00' in data or b'<!DOCTYPE' in data.upper() or b'<!ENTITY' in data.upper():
                raise ValueError('route ownership unsafe XML')
            root = ET.fromstring(data)
            pending = [(root, 0)]
            count = 0
            while pending:
                node, depth = pending.pop()
                count += 1
                if depth > 128 or count > 2_000_000:
                    raise ValueError('route ownership tree budget')
                pending.extend((child, depth+1) for child in node)
            cache[name] = root
            return root

        def children(node, tag):
            return [] if node is None else node.findall('{'+NS+'}'+tag)

        def unique(nodes, attr):
            result = {}
            for node in nodes:
                key = node.get(attr)
                if attr in ('ID', 'IX'):
                    if key is None or not re.fullmatch(r'[0-9]{1,10}', key) or int(key) > 4294967295:
                        continue
                    key = str(int(key))
                result[key] = node if key not in result else None
            return result

        document = xml('visio/document.xml')
        styles = unique([] if document is None else document.findall('.//{'+NS+'}StyleSheet'), 'ID')
        document_sheets = children(document, 'DocumentSheet')
        document_sheet = document_sheets[0] if len(document_sheets) == 1 else None
        masters = unique(children(xml('visio/masters/masters.xml'), 'Master'), 'ID')
        relroot = xml('visio/masters/_rels/masters.xml.rels')
        rels = unique([] if relroot is None else relroot.findall('{'+PKG+'}Relationship'), 'Id')

        def master(shape):
            declaration = masters.get(shape.get('Master'))
            links = children(declaration, 'Rel')
            if len(links) != 1:
                return None
            relationship = rels.get(links[0].get('{'+REL+'}id'))
            if relationship is None or relationship.get('TargetMode') is not None:
                return None
            target = relationship.get('Target', '')
            if (relationship.get('Type') != 'http://schemas.microsoft.com/visio/2010/relationships/master'
                or not re.fullmatch(r'[^/\\:.]+\.xml', target)):
                return None
            contents = xml('visio/masters/'+target)
            roots = [s for group in children(contents, 'Shapes') for s in children(group, 'Shape')]
            members = unique([] if contents is None else contents.findall('.//{'+NS+'}Shape'), 'ID')
            provider = members.get(shape.get('MasterShape')) if shape.get('MasterShape') is not None else (roots[0] if len(roots) == 1 else None)
            if (provider is None or members.get(provider.get('ID')) is not provider
                or provider.get('Type') != 'Shape' or provider.get('Master') is not None or provider.get('MasterShape') is not None):
                return None
            return provider

        def local(node, name):
            matches = [] if node is None else [c for c in node if c.tag.rsplit('}',1)[-1] == 'Cell' and c.get('N','').casefold() == name.casefold()]
            if any(c.tag != '{'+NS+'}Cell' for c in matches):
                return False, None
            if len(matches) > 1:
                return False, None
            if not matches or matches[0].get('F') == 'Inh':
                return True, None
            return True, matches[0]

        def resolve(shape, provider, name):
            for node in (shape, provider):
                valid, cell = local(node, name)
                if not valid:
                    return None
                if cell is not None:
                    return cell
            # Ownership cells are non-format cells: selected style families do
            # not apply. Match Evaluator::baseline_provider, document before0.
            if len(document_sheets) > 1:
                return None
            for node in (document_sheet, styles.get('0')):
                valid, cell = local(node, name)
                if not valid:
                    return None
                if cell is not None:
                    return cell
            return None

        result = set()
        route_cells = set()
        for name in z.namelist():
            if not re.fullmatch(r'visio/pages/[^/]+\.xml', name) or name.endswith('/pages.xml'):
                continue
            root = xml(name)
            if root is None or root.tag != '{'+NS+'}PageContents':
                continue
            shapes = [s for group in children(root, 'Shapes') for s in children(group, 'Shape')]
            ids = unique(root.findall('.//{'+NS+'}Shape'), 'ID')
            for shape in shapes:
                ident = shape.get('ID')
                if ids.get(ident) is not shape:
                    continue
                if shape is None or not ident or not ident.isdecimal() or shape.get('Type') != 'Shape':
                    continue
                provider = master(shape) if shape.get('Master') else None
                if shape.get('Master') and provider is None:
                    continue
                flags = [resolve(shape, provider, n) for n in ('ObjType', 'ConFixedCode')]
                if any(c is None or c.get('F') not in (None, '') or c.get('E') is not None for c in flags):
                    continue
                if number(flags[0].get('V')) != 2 or number(flags[1].get('V')) in (None, 2):
                    continue
                if any(resolve(shape, provider, n) is None for n in ('BeginX', 'BeginY', 'EndX', 'EndY')):
                    continue
                prefix = name+'/Shape:'+ident
                result.add(prefix)
                sections = [n for n in shape if n.tag.rsplit('}',1)[-1] == 'Section' and n.get('N','').casefold() == 'geometry' and re.fullmatch(r'0+', n.get('IX','0'))]
                if len(sections) != 1 or sections[0].tag != '{'+NS+'}Section':
                    continue
                section = sections[0]
                rows = unique([n for n in section if n.tag.rsplit('}',1)[-1] == 'Row'], 'IX')
                for ix, row in rows.items():
                    if row is None or row.tag != '{'+NS+'}Row' or not ix or not ix.isdecimal() or row.get('N') is not None:
                        continue
                    columns = unique([n for n in row if n.tag.rsplit('}',1)[-1] == 'Cell'], 'N')
                    names = [n.get('N','').casefold() for n in row if n.tag.rsplit('}',1)[-1] == 'Cell']
                    if len(names) != len(set(names)):
                        continue
                    for column, cell in columns.items():
                        if cell is not None and cell.tag == '{'+NS+'}Cell' and column:
                            route_cells.add(prefix+'/Section:Geometry:0/Row::'+ix+'/Cell:'+column)
        return (result, route_cells) if include_rows else result


def judge(before_path, after_path, before, after, differences):
    """Apply named router_owned_geometry_v rule; all other comparisons stay exact."""
    old_owners, old_rows = owned_shapes(before_path, True)
    new_owners, new_rows = owned_shapes(after_path, True)
    owners = old_owners & new_owners
    owned_cells = old_rows & new_rows
    accepted = set()
    judged = []
    pattern = re.compile(r'(visio/pages/[^/]+\.xml/Shape:[0-9]+)/Section:Geometry:0/Row::[0-9]+/Cell:(X|Y|A|B|C|D)')
    for key in sorted(before.keys() & after.keys()):
        match = pattern.fullmatch(key[0])
        if key[1] != 0 or match is None or match[1] not in owners or key[0] not in owned_cells:
            continue
        row = key[0].rsplit('/Cell:', 1)[0]
        section = row.rsplit('/Row:', 1)[0]
        a, b = before[key], after[key]
        # Structural duplicates or changes never establish an eligible route row.
        valid = True
        for parent in (section, row):
            for field in ('Present', 'Attribute:T', 'Attribute:Del'):
                k = (parent+'/'+field, 0)
                if before.get(k) != after.get(k) or (parent+'/'+field, 1) in before or (parent+'/'+field, 1) in after:
                    valid = False
            if before.get((parent+'/Attribute:Del', 0), {}).get('V') not in (None, '0'):
                valid = False
        kind = before.get((row+'/Attribute:T', 0), {}).get('V')
        columns = {'MoveTo': {'X','Y'}, 'LineTo': {'X','Y'}, 'ArcTo': {'X','Y','A'},
                   'PolylineTo': {'X','Y'}}
        if not valid or match[2] not in columns.get(kind, set()):
            continue
        if (key[0], 1) in before or (key[0], 1) in after or any(a.get(f) != b.get(f) for f in ('F','U','E')):
            continue
        x, y = number(a.get('V')), number(b.get('V'))
        if x is None or y is None or a.get('E') is not None:
            continue
        delta = abs(x-y)
        judged.append({'cell': str(key), 'before': a['V'], 'after': b['V'], 'delta': delta, 'passed': delta <= BOUND})
        if delta <= BOUND:
            accepted.add((str(key), 'V'))
    retained = [d for d in differences if (d[0], d[1]) not in accepted]
    return retained, {'rule': 'router_owned_geometry_v', 'bound': BOUND, 'cells_judged': len(judged),
                      'changed_within_bound': sum(d['before'] != d['after'] and d['passed'] for d in judged),
                      'outside_bound': sum(not d['passed'] for d in judged), 'cells': judged}
