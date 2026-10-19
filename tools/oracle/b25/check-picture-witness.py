#!/usr/bin/env python3
"""Pre-observation picture mapping/clip assertions, MIT OR Apache-2.0."""
import json
from pathlib import Path
from PIL import Image
root=Path('corpus/_vm/out/picture-witness')
cases={'zero':(0,2,0,0),'zero-height':(3,0,0,0),'normal':(3,2,0,0),'small':(1.5,1,0,0),'large':(6,4,0,0),'negative-x':(3,2,-.5,0),'positive-x':(3,2,.5,0),'negative-y':(3,2,0,-.5),'positive-y':(3,2,0,.5),'offset-small':(1.5,1,.5,.5),'offset-large':(6,4,-1,-1)}
results=[]
def color(p):
 r,g,b=p[:3]
 if max(r,g,b)-min(r,g,b)<70:return -1
 return max(range(3),key=lambda i:p[i])
for kind in ['png','emf']:
 for case,(w,h,ox,oy)in cases.items():
  im=Image.open(root/f'{kind}-{case}.png').convert('RGB')
  assert im.width>=600,(kind,case,im.size)
  colored=[(x,y,color(im.getpixel((x,y))))for y in range(im.height)for x in range(im.width)if color(im.getpixel((x,y)))>=0]
  errors=[]
  if w==0 or h==0:
   assert not colored,(kind,case,'zero picture size must draw no colored pixels',len(colored))
  else:
   # Known black frame is 3.5 by 2.5 inches, centered on the 3 by 2 picture.
   for iy in range(1,20):
    y=iy/10
    for ix in range(1,30):
     x=ix/10
     u=(x-ox)/w;v=(y-oy)/h
     # Exclude source-color and picture boundaries; antialiasing is not this test.
     if min(abs(u-i/3)for i in range(4))<.025 or min(abs(v),abs(v-1))<.025:continue
     expected=min(2,int(u*3))if 0<u<1 and 0<v<1 else -1
     px=round((x+.25)/3.5*(im.width-2)+1);py=round((2.25-y)/2.5*(im.height-2)+1)
     actual=color(im.getpixel((px,py)))
     if actual!=expected:errors.append((x,y,expected,actual))
   for x,y,c in colored:
    sx=(x-1)/(im.width-2)*3.5-.25;sy=2.25-(y-1)/(im.height-2)*2.5
    if sx<-.02 or sx>3.02 or sy<-.02 or sy>2.02:errors.append(('outside-shape',x,y,c));break
   assert not errors,(kind,case,errors[:8])
  results.append(dict(format=kind,case=case,pixels=im.size,colored=len(colored),errors=errors))
for name in ['Z9264F-ON','VEP4600','Z9100-ON']:
 with Image.open(root/f'{name}.png')as im:assert im.width>=600,(name,im.size)
print(json.dumps(results,indent=2))
