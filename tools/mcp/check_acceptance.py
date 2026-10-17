#!/usr/bin/env python3
"""P5a fresh VM acceptance receipt. MIT OR Apache-2.0."""
from pathlib import Path
import json,sys,subprocess,math
root=Path(__file__).resolve().parents[2]
sys.path.insert(0,str(root/'tools/oracle'))
import accept_report as a
from diff_dump import read_dump,read_filters
import argparse
parser=argparse.ArgumentParser(description='Fresh P5a SAN/office acceptance, including Charles route ownership ruling')
parser.add_argument('--state',type=Path,default=root/'corpus/_state/p5a-final')
parser.add_argument('--cli',type=Path,default=root/'target/debug/misterdiagram')
args=parser.parse_args()
state=args.state.resolve();cli=args.cli.resolve()
filters=read_filters([root/'crates/sheet/tests/hooks.txt',root/'crates/sheet/tests/volatile.txt'])
bm=a.load_json(root/'corpus/_state/m3/baseline-manifest.json')
base=next(e for e in bm['files'] if Path(e['output']).stem=='blank__blank')
bp=root/base['output'];ba=root/'corpus/_vm/out/baseline/blank__blank'
a.verified_observation(bp,ba,base,bm)
bparts,bcells=a.package_cells(bp);sparts,scells=a.package_cells(ba/'saved.vsdx')
basediff=[d for d in a.package_differences(bparts,bcells,sparts,scells,filters) if a.document_level_difference(d)]
results=[]
for name,count in [('p5a-final-san',80),('p5a-final-office',18)]:
 src=state/'accept-inputs'/f'{name}.vsdx';actual=root/'corpus/_vm/out/accept'/name
 status=a.load_json(actual/'status.json')
 assert status['input_sha256']==a.digest(src)
 assert status['script_sha256']==a.digest(root/'tools/oracle/accept.ps1')
 assert status['common_sha256']==a.digest(root/'tools/oracle/common.ps1')
 assert status['opened'] and status['saved'] and not status['error'],status
 for item in status['artifacts']:assert a.digest(actual/item['file'])==item['sha256']
 connects=a.read_connects(actual/'connects.tsv');assert len(connects)==count
 oldparts,old=a.package_cells(src);newparts,new=a.package_cells(actual/'saved.vsdx')
 diffs=a.package_differences(oldparts,old,newparts,new,filters);remaining,subtracted=a.subtract_baseline(diffs,basediff)
 retained,parts,allowed=a.allowed_rewrites(old,new,a.cell_diffs(old,new,filters),{'added':sorted(set(newparts)-set(oldparts)),'removed':sorted(set(oldparts)-set(newparts))})
 keys={(d[0],d[1]) for d in retained}|{('part:'+p,k) for k in ('added','removed') for p in parts[k]}
 repairs=[d for d in remaining if (d[0],d[1]) in keys]
 repairs,route_rule=a.judge_router_geometry(src,actual/'saved.vsdx',old,new,repairs)
 output=state/name;output.mkdir(exist_ok=True)
 for label,path in [('before',src),('after',actual/'saved.vsdx')]:
  with (output/(label+'.tsv')).open('w') as f:subprocess.run([str(cli),'sheet',str(path)],stdout=f,check=True)
 rows=[read_dump(output/(label+'.tsv')) for label in ('before','after')]
 ids={c[0] for c in connects};geometry=[];cells=0
 names={'BeginX','BeginY','EndX','EndY','PinX','PinY','Width','Height','LocPinX','LocPinY','Angle','FlipX','FlipY'}
 for key in rows[0].keys()|rows[1].keys():
  if key[0] not in ids or not key[1].startswith('page:'):continue
  if not (key[5].startswith('Geometry') or key[5] in names):continue
  cells+=1;x=rows[0].get(key);y=rows[1].get(key)
  try:equal=x is not None and y is not None and math.isfinite(float(x[7])) and math.isfinite(float(y[7])) and abs(float(x[7])-float(y[7]))<=1e-6
  except ValueError:equal=x==y
  if not equal:geometry.append((key,x,y))
 image=output/'ours.png';subprocess.run([str(cli),'convert',str(src),str(image)],check=True,capture_output=True)
 render=a.pixels(image,actual/'page-0.png')
 render['passed']=render['sizes'][0]==render['sizes'][1] and render.get('mad',999)<=4 and render.get('over',math.inf)/render.get('pixels',1)<=.02
 import ast
 details=[]
 for d in repairs:
  try:key=ast.literal_eval(d[0])
  except (ValueError,SyntaxError):key=None
  details.append(dict(path=d[0],field=d[1],before=d[2],after=d[3],before_cell=old.get(key),after_cell=new.get(key)))
 result=dict(router_geometry=route_rule,repair_details=details,name=name,opened=True,saved=True,connects=len(connects),connectors=len(ids),route_cells=cells,route_differences=geometry,repair_count=len(repairs),repair_differences=repairs,allowed={k:len(v) for k,v in allowed.items()},baseline_subtracted=len(subtracted),render=render)
 results.append(result)
 print(json.dumps(result,default=str),flush=True)
(state/'accept-report.json').write_text(json.dumps(results,indent=2,default=str)+'\n')
raise SystemExit(0 if all(r['repair_count']==0 and r['route_cells']>0 and not r['route_differences'] and r['render']['passed'] for r in results) else 1)
