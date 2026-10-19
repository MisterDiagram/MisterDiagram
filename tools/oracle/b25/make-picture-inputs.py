#!/usr/bin/env python3
"""Contributor-original PNG and EMF color panels for the picture-cell witness.
MIT OR Apache-2.0. No vendor media or fonts.
"""
import struct
from pathlib import Path
from PIL import Image
out=Path('corpus/_state/brief25/picture-inputs');out.mkdir(parents=True,exist_ok=True)
im=Image.new('RGB',(300,200))
colors=[(220,40,40),(30,180,70),(30,70,220)]
for y in range(200):
 for x in range(300): im.putpixel((x,y),colors[x//100])
im.save(out/'panels.png')
def rec(kind,*values):return struct.pack('<II'+'I'*len(values),kind,8+4*len(values),*values)
records=[rec(37,0x80000008)] # NULL_PEN
for i,(r,g,b) in enumerate(colors):
 records += [rec(39,i+1,0,r|(g<<8)|(b<<16),0),rec(37,i+1),rec(43,i*100,0,(i+1)*100,200)]
records.append(rec(14,0,0,20))
size=88+sum(map(len,records))
header=struct.pack('<II4i4iIIIIHHIII4i',1,88,0,0,300,200,0,0,7620,5080,0x464d4520,0x10000,size,len(records)+1,4,0,0,0,0,300,200,76,51)
assert len(header)==88
(out/'panels.emf').write_bytes(header+b''.join(records))
