#!/usr/bin/env python3
"""Summarize JSONL command and stage clocks without hiding unmeasured work."""
import json
import pathlib
import sys
root = pathlib.Path(sys.argv[1])
for name in ('rectangles-25','rectangles-50','rectangles-100','data-40','ports-1900'):
    path = root / f'before-{name}.jsonl'
    if not path.exists():
        continue
    rows = [json.loads(line) for line in path.read_text().splitlines()]
    stages_path = root / f'before-{name}-stages.jsonl'
    stages = [json.loads(line) for line in stages_path.read_text().splitlines() if line.startswith('{')]
    totals = {key: sum(row.get(key, 0) for row in stages if row['kind']=='stages') for key in ('clone','prepare','reroute','write','import')}
    totals['cow_clone'] = sum(row['seconds'] for row in stages if row['kind']=='cow_clone')
    total = next((r for r in rows if r['kind']=='total'), None)
    print(json.dumps({'case':name, 'total':total, 'stages':totals, 'slices':[r for r in rows if r['kind']=='slice']}))
