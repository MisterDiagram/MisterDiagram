#!/usr/bin/env python3
"""Create original 96x64 PNG/JPEG inputs locally; stage the read-only probe EMF."""
from pathlib import Path
import shutil
import struct
import subprocess
import zlib

root = Path(__file__).resolve().parents[2]
assets = root / 'corpus/_vm/assets'
assets.mkdir(parents=True, exist_ok=True)
width, height = 96, 64
pixels = b''.join(b'\0' + b''.join(bytes((20 + 2*x, 30 + 3*y, 140))
                                for x in range(width)) for y in range(height))

def chunk(kind, data):
    return (struct.pack('>I', len(data)) + kind + data
            + struct.pack('>I', zlib.crc32(kind + data)))

png = assets / 'sample.png'
png.write_bytes(b'\x89PNG\r\n\x1a\n'
                + chunk(b'IHDR', struct.pack('>2I5B', width, height, 8, 2, 0, 0, 0))
                + chunk(b'IDAT', zlib.compress(pixels)) + chunk(b'IEND', b''))
subprocess.run(['sips', '-s', 'format', 'jpeg', str(png), '--out',
                str(assets / 'sample.jpg')], check=True)
shutil.copy2('/Users/charles/Projects/craft-apps/exam-out/visio/oracle-basic.emf',
             assets / 'sample.emf')
