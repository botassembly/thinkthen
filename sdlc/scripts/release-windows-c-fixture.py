#!/usr/bin/env python3
"""Synthetic PE/COFF inputs for portable release tests; no native ABI receipt."""
import importlib.util
from pathlib import Path
import struct

SPEC = importlib.util.spec_from_file_location('windows_c', Path(__file__).with_name('release-windows-c.py'))
C = importlib.util.module_from_spec(SPEC)
SPEC.loader.exec_module(C)


def pe():
    data = bytearray(128)
    data[:2] = b'MZ'
    struct.pack_into('<I', data, 60, 64)
    data[64:68] = b'PE\0\0'
    struct.pack_into('<H', data, 68, 0x8664)
    struct.pack_into('<H', data, 86, 0x2002)
    struct.pack_into('<H', data, 88, 0x20b)
    return bytes(data)


def library(header):
    def record(name, body):
        frame = f'{name:<16}{0:<12}{0:<6}{0:<6}{"100644":<8}{len(body):<10}`\n'.encode()
        return frame + body + (b'\n' if len(body) % 2 else b'')
    names = C.declarations(header.decode())
    symbols = b''.join(name.encode() + b'\0' for name in names)
    bodies = []
    for name in names:
        strings = name.encode() + b'\0thinkthen.dll\0'
        bodies.append(struct.pack('<HHHHIIHH', 0, 0xffff, 0, 0x8664, 0, len(strings), 0, 4) + strings)
    # Linker members precede the short import members and refer to their archive offsets.
    first_size = 4 + len(names)*4 + len(symbols)
    second_size = 8 + len(names)*6 + len(symbols)
    at = 8 + 60 + first_size + first_size%2 + 60 + second_size + second_size%2
    offsets = []
    for body in bodies:
        offsets.append(at)
        at += 60 + len(body) + len(body)%2
    first = struct.pack('>I', len(names)) + struct.pack(f'>{len(names)}I', *offsets) + symbols
    second = struct.pack('<I', len(names)) + struct.pack(f'<{len(names)}I', *offsets)
    second += struct.pack('<I', len(names)) + struct.pack(f'<{len(names)}H', *range(1, len(names)+1)) + symbols
    return b'!<arch>\n' + record('/', first) + record('/', second) + b''.join(record('fixture.obj/', body) for body in bodies)


def create(folder, version):
    import tempfile
    with tempfile.TemporaryDirectory(prefix='thinkthen-c-fixture-') as temporary:
        stage = Path(temporary)
        header = C.HEADER.read_bytes()
        for name, data in zip(C.MEMBERS, (header, pe(), library(header))):
            path = stage / name
            path.parent.mkdir(parents=True, exist_ok=True)
            path.write_bytes(data)
        archive = folder / f'thinkthen-c-{version}-{C.TARGET}.zip'
        C.pack(stage, archive)
    return archive
