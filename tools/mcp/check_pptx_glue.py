#!/usr/bin/env python3
"""Independent python-pptx readback of attachment indexes and embedded pictures."""
import argparse
import json
import math
import subprocess
from pathlib import Path
from pptx import Presentation
from pptx.oxml.ns import qn


def nodes(shapes):
    for s in shapes:
        yield s
        if hasattr(s, 'shapes'):
            yield from nodes(s.shapes)


def box(element):
    xf=element.xpath('./p:spPr/a:xfrm')[0]
    off=xf.find(qn('a:off'));ext=xf.find(qn('a:ext'))
    return xf,tuple(int(off.get(n)) for n in ['x','y'])+tuple(int(ext.get(n)) for n in ['cx','cy'])


def endpoint(element,end):
    xf,(x,y,w,h)=box(element)
    px,py=(w,h) if end else (0,0)
    if xf.get('flipH') in ['1','true']:px=w-px
    if xf.get('flipV') in ['1','true']:py=h-py
    angle=int(xf.get('rot','0'))/60000*math.pi/180
    dx,dy=px-w/2,py-h/2
    return x+w/2+dx*math.cos(angle)-dy*math.sin(angle),y+h/2+dx*math.sin(angle)+dy*math.cos(angle)


def site_point(target,site,guides):
    _,(x,y,w,h)=box(target)
    pos=site.find(qn('a:pos'));values=[]
    for axis in ['x','y']:
        op,value,dimension,denominator=guides[pos.get(axis)].split()
        assert op=='*/' and dimension in ['w','h']
        values.append(int(value)*({'w':w,'h':h}[dimension])/int(denominator))
    return x+values[0],y+values[1]


def inspect(path, manifest):
    expected=[]; pages={}; slides=None
    for line in manifest.read_text().splitlines():
        f=line.split('\t')
        if f[0]=='S': slides=int(f[1])
        elif f[0]=='P':pages[int(f[1])]=(int(f[2]),int(f[3]))
        elif f[0]=='C':expected.append(tuple(map(int,f[1:])))
        else: raise ValueError('unknown intent row')
    prs=Presentation(path)
    assert len(prs.slides)==slides
    actual=[]; total_pictures=0
    for pi,slide in enumerate(prs.slides,1):
        shapes=list(nodes(slide.shapes)); byid={s.shape_id:s for s in shapes}
        assert len(byid)==len(shapes),'duplicate shape ids'
        pictures=0; connections=0
        for s in shapes:
            el=s._element
            assert not el.xpath('./p:style'),'style reference'
            if el.tag==qn('p:pic'):
                assert s.image.blob.startswith(b'\x89PNG\r\n\x1a\n'),'picture must embed PNG'
                pictures+=1
            if el.tag!=qn('p:cxnSp'): continue
            assert not el.xpath('./p:spPr/a:custGeom'),'custom geometry connector'
            geom=el.xpath('./p:spPr/a:prstGeom')
            assert len(geom)==1
            assert geom[0].get('prst') in {'straightConnector1',*(f'{t}Connector{n}' for t in ['bent','curved'] for n in range(2,6))}
            for tag,end in [('stCxn',0),('endCxn',1)]:
                attached=el.xpath(f'./p:nvCxnSpPr/p:cNvCxnSpPr/a:{tag}')
                assert len(attached)<=1
                for c in attached:
                    target=int(c.get('id'));idx=int(c.get('idx'))
                    assert target in byid,'missing attachment target'
                    sites=byid[target]._element.xpath('./p:spPr/a:custGeom/a:cxnLst/a:cxn')
                    assert 0<=idx<len(sites),'site index out of range'
                    guides={g.get('name'):g.get('fmla') for g in byid[target]._element.xpath('./p:spPr/a:custGeom/a:gdLst/a:gd')}
                    for site in sites:
                        pos=site.find(qn('a:pos'));assert pos is not None
                        for axis in ['x','y']:
                            formula=guides[pos.get(axis)].split()
                            assert len(formula)==4 and formula[0]=='*/' and formula[2] in ['w','h']
                            int(formula[1]);assert int(formula[3])>0
                    target_el=byid[target]._element
                    a=endpoint(el,end);b=site_point(target_el,sites[idx],guides)
                    assert max(abs(v-u) for v,u in zip(a,b))<=1.01,f'endpoint/site mismatch {s.shape_id} {end}: {a} {b}'
                    _,(x,y,w,h)=box(target_el)
                    for site,expected_point in zip(sites[:4],[(x+w/2,y),(x,y+h/2),(x+w/2,y+h),(x+w,y+h/2)]):
                        point=site_point(target_el,site,guides)
                        assert max(abs(a-b) for a,b in zip(point,expected_point))<=1.01,'rectangle site order/position'
                    actual.append((pi,s.shape_id,end,target,idx));connections+=1
        assert (pictures,connections)==pages[pi],f'page {pi}: pictures/glue {(pictures,connections)} != {pages[pi]}'
        total_pictures+=pictures
    assert sorted(actual)==sorted(expected),'attachment target/index differs from intent TSV'
    return dict(slides=slides,glued_ends=len(actual),pictures=total_pictures)


def main():
    p=argparse.ArgumentParser(description=__doc__)
    p.add_argument('inputs',nargs='*',type=Path)
    p.add_argument('--cli',type=Path,default=Path('target/release/misterdiagram'))
    p.add_argument('--manifest-binary',type=Path,default=Path('target/debug/examples/glue_manifest'))
    p.add_argument('--out',type=Path)
    p.add_argument('--inspect',type=Path);p.add_argument('--expected',type=Path)
    a=p.parse_args()
    if a.inspect:
        if not a.expected:p.error('--inspect requires --expected')
        print(json.dumps(inspect(a.inspect,a.expected)));return
    if not a.out or not a.inputs:p.error('inputs and --out required')
    a.out.mkdir(parents=True,exist_ok=False)
    files=[]
    for path in a.inputs:
        files.extend(sorted(path.rglob('*.vsdx')) if path.is_dir() else [path])
    rows=[]
    for i,path in enumerate(files):
        pptx=a.out/f'{i:04d}.pptx';tsv=a.out/f'{i:04d}.tsv'
        for cmd in [[str(a.manifest_binary),str(path),str(tsv)],[str(a.cli),'render',str(path),str(pptx)]]:
            subprocess.run(cmd,check=True,capture_output=True,timeout=120)
        rows.append(dict(input=str(path),**inspect(pptx,tsv)))
    result=dict(files=len(rows),glued_ends=sum(x['glued_ends'] for x in rows),pictures=sum(x['pictures'] for x in rows),results=rows)
    (a.out/'report.json').write_text(json.dumps(result,indent=2)+'\n')
    print(json.dumps({k:v for k,v in result.items() if k!='results'}))

if __name__=='__main__':main()
