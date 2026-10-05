#!/usr/bin/env python3
"""Windows C DLL packaging and complete native ABI inspection (ticket 0381 A)."""
import argparse
import hashlib
import os
from pathlib import Path
import re
import runpy
import shutil
import struct
import subprocess
import sys
import tempfile
import zipfile

TARGET = 'x86_64-pc-windows-msvc'
MEMBERS = ('include/thinkthen.h', 'bin/thinkthen.dll', 'lib/thinkthen.dll.lib')
REPO = Path(__file__).resolve().parents[2]
HEADER = REPO / 'libraries/c/include/thinkthen.h'
CAPTURE = runpy.run_path(str(Path(__file__).with_name('release-bounded.py')))['capture']


def declarations(header):
    if re.search(r'/\*(?:(?!\*/).)*\Z', header, re.S):
        raise ValueError('unterminated header comment')
    code = re.sub(r'/\*.*?\*/|//[^\n]*', ' ', header, flags=re.S)
    code = re.sub(r'^\s*#.*$', '', code, flags=re.M)
    for statement in code.split(';'):
        statement = statement.strip()
        if ('thinkthen_' in statement and '(' not in statement and
                not statement.startswith(('typedef ', '}')) and '{' not in statement):
            raise ValueError('malformed header declaration')
    names = []
    for match in re.finditer(r'\b(thinkthen_\w*)\s*\(', code):
        prefix = re.split(r'[;{}]', code[:match.start()])[-1].strip()
        if not prefix or prefix.startswith('typedef ') or any(char in prefix for char in '()='):
            raise ValueError('malformed header declaration')
        tail = code[match.end():]
        if not re.match(r'[^();{}]+\)\s*;', tail):
            raise ValueError('malformed header declaration')
        names.append(match[1])
    if not names or len(set(names)) != len(names):
        raise ValueError('missing or duplicate header declaration')
    return sorted(names)


def definition(header):
    return 'LIBRARY thinkthen.dll\nEXPORTS\n    ' + '\n    '.join(declarations(header)) + '\n'


def dll(data):
    if len(data) < 64 or data[:2] != b'MZ':
        raise ValueError('C library is not a Windows DLL')
    at = struct.unpack_from('<I', data, 60)[0]
    if at + 26 > len(data) or data[at:at+4] != b'PE\0\0':
        raise ValueError('C DLL has no complete PE header')
    machine, flags, magic = (struct.unpack_from('<H', data, at+n)[0] for n in (4, 22, 24))
    if machine != 0x8664 or magic != 0x20b or flags & 0x2002 != 0x2002:
        raise ValueError('C library must be a PE32+ x86-64 DLL')


def imports(data):
    """Validate archive framing and x64 COFF members, including short imports."""
    if not data.startswith(b'!<arch>\n'):
        raise ValueError('C import library is not a COFF archive')
    at, imported, linkers, offsets = 8, [], [], set()
    while at < len(data):
        record = data[at:at+60]
        if len(record) != 60 or record[58:] != b'`\n':
            raise ValueError('C import library has malformed archive framing')
        try:
            size = int(record[48:58])
        except ValueError as error:
            raise ValueError('C import library has invalid member length') from error
        body = data[at+60:at+60+size]
        if size < 0 or len(body) != size:
            raise ValueError('C import library has truncated member')
        name = record[:16].strip()
        if name == b'/':
            linkers.append(body)
        elif name != b'//':
            offsets.add(at)
        if name not in (b'/', b'//'):
            if body[:4] == b'\0\0\xff\xff':
                if len(body) < 20 or struct.unpack_from('<H', body, 6)[0] != 0x8664:
                    raise ValueError('C import library must contain x64 imports')
                if struct.unpack_from('<H', body, 4)[0] != 0 or struct.unpack_from('<H', body, 18)[0] not in (4, 5, 6):
                    raise ValueError('C import library requires named short imports')
                length = struct.unpack_from('<I', body, 12)[0]
                strings = body[20:]
                if length != len(strings) or not strings.endswith(b'\0'):
                    raise ValueError('C import library has malformed short import')
                fields = strings.split(b'\0')
                if len(fields) != 3 or fields[1] != b'thinkthen.dll':
                    raise ValueError('C import library must target thinkthen.dll')
                imported.append(fields[0].decode('ascii'))
            elif len(body) < 20 or struct.unpack_from('<H', body)[0] != 0x8664:
                raise ValueError('C import library has non-x64 COFF member')
        at += 60 + size + size % 2
    if at != len(data) or not imported:
        raise ValueError('C import library has no complete imports')
    linker_tables(linkers, offsets)
    if len(imported) != len(set(imported)):
        raise ValueError('C import library has duplicate imports')
    return sorted(imported)


def linker_tables(tables, offsets):
    """Both MSVC linker tables must reference real COFF members and complete names."""
    if len(tables) != 2:
        raise ValueError('C import library requires both MSVC linker tables')
    try:
        first = tables[0]
        count = struct.unpack_from('>I', first)[0]
        boundary = 4 + count * 4
        entries = struct.unpack_from(f'>{count}I', first, 4)
        names = first[boundary:].split(b'\0')
        if boundary > len(first) or len(names) != count + 1 or names[-1] or any(not name for name in names[:-1]):
            raise ValueError('C import library has malformed first linker table')
        if not set(entries) <= offsets:
            raise ValueError('C import library linker table points outside COFF members')
        second = tables[1]
        members = struct.unpack_from('<I', second)[0]
        member_offsets = struct.unpack_from(f'<{members}I', second, 4)
        place = 4 + members * 4
        count = struct.unpack_from('<I', second, place)[0]
        indices = struct.unpack_from(f'<{count}H', second, place + 4)
        names = second[place + 4 + count * 2:].split(b'\0')
        if len(names) != count + 1 or names[-1] or any(not name for name in names[:-1]):
            raise ValueError('C import library has malformed second linker table')
        if not set(member_offsets) <= offsets or any(index < 1 or index > members for index in indices):
            raise ValueError('C import library second linker table has invalid member index')
    except struct.error as error:
        raise ValueError('C import library has truncated linker table') from error


def payload(header, binary, library):
    dll(binary)
    if imports(library) != declarations(header.decode('utf-8')):
        raise ValueError('C import library differs from header declarations')


def name_check(archive):
    if not re.fullmatch(r'thinkthen-c-\d+\.\d+\.\d+-' + TARGET + r'\.zip', archive.name):
        raise ValueError('C ZIP name must carry its version and Windows target')


def checksum(archive):
    archive.with_name(archive.name + '.sha256').write_text(
        f'{hashlib.sha256(archive.read_bytes()).hexdigest()}  {archive.name}\n', encoding='ascii')


def pack(stage, archive):
    name_check(archive)
    if archive.is_symlink() or archive.with_name(archive.name + '.sha256').is_symlink():
        raise ValueError('C ZIP output is linked')
    files = [stage / name for name in MEMBERS]
    if any(path.is_symlink() or not path.is_file() for path in files):
        raise ValueError('C package input is missing or linked')
    contents = [path.read_bytes() for path in files]
    if contents[0] != HEADER.read_bytes():
        raise ValueError('C ZIP header differs from committed header')
    payload(*contents)
    with zipfile.ZipFile(archive, 'w', compression=zipfile.ZIP_DEFLATED) as out:
        for name, data in zip(MEMBERS, contents):
            member = zipfile.ZipInfo(name, (1980, 1, 1, 0, 0, 0))
            member.create_system = 3
            member.compress_type = zipfile.ZIP_DEFLATED
            member.external_attr = 0o100644 << 16
            out.writestr(member, data)
    checksum(archive)


def check(archive):
    name_check(archive)
    sidecar = archive.with_name(archive.name + '.sha256')
    if any(path.is_symlink() or not path.is_file() for path in (archive, sidecar)):
        raise ValueError('C ZIP or checksum is missing or linked')
    if sidecar.read_text().strip() != f'{hashlib.sha256(archive.read_bytes()).hexdigest()}  {archive.name}':
        raise ValueError('C ZIP differs from its checksum')
    with zipfile.ZipFile(archive) as source:
        if source.namelist() != list(MEMBERS):
            raise ValueError('C ZIP must hold exactly header, DLL and import library')
        if any(info.external_attr >> 16 & 0o170000 != 0o100000 for info in source.infolist()):
            raise ValueError('C ZIP members must be regular files')
        contents = [source.read(name) for name in MEMBERS]
    if contents[0] != HEADER.read_bytes():
        raise ValueError('C ZIP header differs from committed header')
    payload(*contents)
    return contents


def environment():
    """No ambient compiler switches, credentials or runtime settings."""
    names = ('PATH', 'SystemRoot', 'SystemDrive', 'TEMP', 'TMP', 'INCLUDE', 'LIB', 'LIBPATH')
    return {name: os.environ[name] for name in names if name in os.environ}


def run(args):
    result = CAPTURE(args, env=environment(), text=True, timeout=120)
    if result.returncode:
        raise ValueError(f'{Path(args[0]).name} failed (exit {result.returncode}): {result.stdout}{result.stderr}')
    return result.stdout


def import_library(header, stage):
    """Used by both source door setup and final archive production."""
    definition_path = stage / 'thinkthen.def'
    definition_path.write_text(definition(header.read_text()), encoding='ascii')
    destination = stage / 'lib/thinkthen.dll.lib'
    destination.parent.mkdir(parents=True, exist_ok=True)
    run(['lib.exe', '/nologo', '/DEF:' + str(definition_path), '/MACHINE:X64', '/OUT:' + str(destination)])
    if imports(destination.read_bytes()) != declarations(header.read_text()):
        raise ValueError('generated import library differs from header declarations')


def exports(output):
    """Read every dumpbin export row; reject forwarded and ordinal-only exports."""
    names = []
    in_table = False
    for line in output.splitlines():
        if re.search(r'ordinal\s+hint\s+RVA\s+name', line):
            in_table = True
            continue
        if not in_table:
            continue
        if line.strip() == 'Summary':
            break
        if re.match(r'\s*\d+\s', line):
            row = re.fullmatch(r'\s*\d+\s+[0-9A-Fa-f]+\s+[0-9A-Fa-f]{8}\s+(\w+)\s*', line)
            if not row:
                raise ValueError('DLL has decorated, forwarded or ordinal-only export')
            names.append(row[1])
    if not in_table or not names or len(set(names)) != len(names):
        raise ValueError('DLL has missing or duplicate export inventory')
    return sorted(names)


def inspect(stage, consumer=None):
    expected = declarations((stage / MEMBERS[0]).read_text())
    report = run(['dumpbin.exe', '/nologo', '/exports', str(stage / MEMBERS[1])])
    if exports(report) != expected:
        raise ValueError('DLL exports differ from header declarations')
    print(report)
    print(run(['dumpbin.exe', '/nologo', '/dependents', str(stage / MEMBERS[1])]))
    for name in MEMBERS[1:]:
        report = run(['dumpbin.exe', '/nologo', '/headers', str(stage / name)])
        if '8664 machine (x64)' not in report or '14C machine' in report:
            raise ValueError('native C binary is not x64')
    report = run(['dumpbin.exe', '/nologo', '/all', str(stage / MEMBERS[2])])
    if 'thinkthen.dll' not in report or 'thinkthen_c.dll' in report:
        raise ValueError('native import descriptor must name thinkthen.dll')
    if consumer:
        report = run(['dumpbin.exe', '/nologo', '/imports', str(consumer)])
        if 'thinkthen.dll' not in report or 'thinkthen_c.dll' in report:
            raise ValueError('consumer must import only the public DLL name')


def stage(binary, header, folder):
    if any(path.is_symlink() or not path.is_file() for path in (binary, header)):
        raise ValueError('C stage input is missing or linked')
    if header.read_bytes() != HEADER.read_bytes():
        raise ValueError('C stage header differs from committed header')
    dll(binary.read_bytes())
    if folder.is_symlink() or not folder.is_dir() or any(folder.iterdir()):
        raise ValueError('C stage requires a new empty regular scratch directory')
    for name in MEMBERS[:2]:
        (folder / name).parent.mkdir(parents=True, exist_ok=True)
    shutil.copyfile(header, folder / MEMBERS[0])
    shutil.copyfile(binary, folder / MEMBERS[1])
    import_library(header, folder)


def build_pack(binary, header, archive):
    # TemporaryDirectory removes only its own newly created scratch, never caller input.
    with tempfile.TemporaryDirectory(prefix='thinkthen-windows-c-') as temporary:
        folder = Path(temporary)
        stage(binary, header, folder)
        inspect(folder)
        pack(folder, archive)
        check(archive)


def main():
    parser = argparse.ArgumentParser(description=__doc__)
    parser.add_argument('action', choices=('pack', 'check', 'stage', 'inspect'))
    parser.add_argument('paths', nargs='+', type=Path)
    args = parser.parse_args()
    try:
        if args.action == 'check' and len(args.paths) == 1:
            check(*args.paths)
        elif args.action == 'pack' and len(args.paths) == 3:
            build_pack(*args.paths)
        elif args.action == 'stage' and len(args.paths) == 3:
            stage(*args.paths)
        elif args.action == 'inspect' and len(args.paths) in (1, 2):
            inspect(*args.paths)
        else:
            parser.error('wrong argument count')
    except (ValueError, OSError, UnicodeError, struct.error, zipfile.BadZipFile, subprocess.TimeoutExpired) as error:
        print(f'release-windows-c: {error}', file=sys.stderr)
        return 1
    return 0

if __name__ == '__main__':
    sys.exit(main())
