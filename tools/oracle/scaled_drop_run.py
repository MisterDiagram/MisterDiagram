"""Verify Brief31c native receipts and its authorized same-run blank window control.

This module replaces no comparison rules. The historical non-window control,
strict cell comparator, allowed rewrites and route judge stay in accept_report.
"""
import ast
from pathlib import Path
import re
import accept_report as a

CASES = (
    'brief31-noscale-control-1-0', 'brief31-noscale-control-1-3',
    'brief31-noscale-control-10-3', 'brief31-noscale-control-100-3',
    'brief31-noscale-control-4-3', 'brief31-scale-boundary-1-0',
    'brief31-scale-boundary-1-3', 'brief31-scale-matrix-1-0',
    'brief31-scale-matrix-1-3', 'brief31-scale-matrix-10-3',
    'brief31-scale-matrix-100-3', 'brief31-scale-matrix-4-3', 'issue70',
)
BLANK = 'blank-window-control'


def window_difference(difference):
    key = difference[0]
    path = key[5:] if key.startswith('part:') else ast.literal_eval(key)[0]
    return path == 'visio/windows.xml' or path.startswith('visio/windows.xml/')


def verify_run(inputs, output, script, common):
    """Reject missing, duplicate, stale or substituted native evidence."""
    inputs, output = Path(inputs), Path(output)
    run = a.load_json(output/'run.json')
    if (not isinstance(run, dict)
            or not re.fullmatch(r'[0-9a-f]{32}', str(run.get('run_id', '')))
            or run.get('names') != [BLANK, *CASES]
            or run.get('completed') is not True
            or not all(isinstance(run.get(k), str) and 0 < len(run[k]) <= 256
                       for k in ('started_utc', 'finished_utc', 'visio_version', 'visio_build', 'user', 'machine'))):
        raise ValueError('incomplete native run identity')
    for name, path, retained in [('script_sha256', script, 'capture.ps1'),
                                 ('common_sha256', common, 'common.ps1')]:
        if run.get(name) != a.digest(path) or run[name] != a.digest(output/retained):
            raise ValueError('native script/helper hash mismatch')
    rows = a.load_json(output/'status.json')
    if not isinstance(rows, list) or len(rows) != len(CASES)+1:
        raise ValueError('native cohort count')
    if [r.get('name') for r in rows if isinstance(r, dict)] != [BLANK, *CASES]:
        raise ValueError('native cohort names or duplicates')
    for row in rows:
        name = row['name']
        directory = output/name
        if (row.get('run_id') != run['run_id']
                or row.get('opened') is not True or row.get('saved') is not True or row.get('error')
                or any(row.get(k) != run[k] for k in ('script_sha256', 'common_sha256'))
                or row.get('input_sha256') != a.digest(inputs/(name+'.vsdx'))
                or row['input_sha256'] != a.digest(directory/'input.vsdx')):
            raise ValueError('invalid or stale native observation: '+name)
        artifacts = row.get('artifacts')
        if not isinstance(artifacts, list) or len(artifacts) != 3:
            raise ValueError('native artifact inventory')
        if {r.get('file') for r in artifacts if isinstance(r, dict)} != {'saved.vsdx', 'observed.sheet.tsv', 'connects.tsv'}:
            raise ValueError('native artifact names')
        for artifact in artifacts:
            if artifact.get('sha256') != a.digest(directory/artifact['file']):
                raise ValueError('native artifact hash mismatch: '+name)
    for file in ('input.vsdx', 'saved.vsdx'):
        _, cells = a.package_cells(output/BLANK/file)
        if any(key[0].startswith('visio/pages/page') and '/Shape:' in key[0] for key in cells):
            raise ValueError('window control contains page shapes')
    return run, rows


def window_control(historical, output, filters):
    """Replace only the authorized window observation, retaining exact witnesses."""
    output = Path(output)
    before_parts, before = a.package_cells(output/BLANK/'input.vsdx')
    after_parts, after = a.package_cells(output/BLANK/'saved.vsdx')
    fresh = [d for d in a.package_differences(before_parts, before, after_parts, after, filters)
             if window_difference(d)]
    retained = [d for d in historical if not window_difference(d)]
    return retained + fresh, fresh
