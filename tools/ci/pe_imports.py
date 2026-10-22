#!/usr/bin/env python3
"""List the DLLs a Windows executable imports; fail on the Visual C++ runtime (MIT OR Apache-2.0).

Usage: pe_imports.py misterdiagram.exe
Release binaries link the C runtime statically (+crt-static), so they run on a Windows machine
without the Visual C++ Redistributable. The runners and most developer machines have it, so
running the binary there cannot show the dependency; reading the import table does.
Standard library only.
"""
from pathlib import Path
import struct
import sys

FORBIDDEN = ('vcruntime', 'msvcp', 'ucrtbase', 'api-ms-win-crt-')


def imports(data):
    pe = struct.unpack_from('<I', data, 0x3C)[0]
    if data[:2] != b'MZ' or data[pe:pe + 4] != b'PE\0\0':
        raise ValueError('not a PE file')
    sections, optional_size = struct.unpack_from('<H', data, pe + 6)[0], struct.unpack_from('<H', data, pe + 20)[0]
    optional = pe + 24
    magic = struct.unpack_from('<H', data, optional)[0]
    directories = optional + (112 if magic == 0x20B else 96)
    import_rva = struct.unpack_from('<I', data, directories + 8)[0]
    table = [struct.unpack_from('<IIII', data, optional + optional_size + 40 * i + 8) for i in range(sections)]

    def offset(rva):
        for virtual_size, address, raw_size, raw in table:
            if address <= rva < address + max(virtual_size, raw_size):
                return rva - address + raw
        raise ValueError(f'RVA {rva:#x} outside every section')

    names, at = [], offset(import_rva)
    while True:
        name_rva = struct.unpack_from('<I', data, at + 12)[0]
        if name_rva == 0:
            return names
        start = offset(name_rva)
        names.append(data[start:data.index(b'\0', start)].decode('ascii'))
        at += 20


def main():
    if len(sys.argv) != 2:
        raise SystemExit(__doc__)
    names = imports(Path(sys.argv[1]).read_bytes())
    print('imports: ' + ', '.join(names))
    bad = [n for n in names if n.lower().startswith(FORBIDDEN)]
    if bad:
        raise SystemExit(f'the binary needs the Visual C++ runtime ({", ".join(bad)}); build with -C target-feature=+crt-static')
    print('no Visual C++ runtime dependency')


if __name__ == '__main__':
    main()
