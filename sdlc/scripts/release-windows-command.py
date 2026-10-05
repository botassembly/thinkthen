#!/usr/bin/env python3
"""Pack or check the Windows x86-64 command archive without extracting it."""

import argparse
import hashlib
from pathlib import Path
import re
import struct
import sys
import zipfile
import importlib.util

_spec = importlib.util.spec_from_file_location("windows_c", Path(__file__).with_name("release-windows-c.py"))
windows_c = importlib.util.module_from_spec(_spec)
_spec.loader.exec_module(windows_c)


TARGET = "x86_64-pc-windows-msvc"


def executable(data):
    """Require a PE32+ x86-64 executable, rather than a DLL or another target."""
    if len(data) < 64 or data[:2] != b"MZ":
        raise ValueError("command is not a Windows executable")
    offset = struct.unpack_from("<I", data, 60)[0]
    if offset + 26 > len(data) or data[offset:offset + 4] != b"PE\0\0":
        raise ValueError("command has no complete PE header")
    machine = struct.unpack_from("<H", data, offset + 4)[0]
    flags = struct.unpack_from("<H", data, offset + 22)[0]
    magic = struct.unpack_from("<H", data, offset + 24)[0]
    if machine != 0x8664 or magic != 0x20B or not flags & 2 or flags & 0x2000:
        raise ValueError("command must be a PE32+ x86-64 executable")


def archive_name(path):
    if not re.fullmatch(r"thinkthen-\d+\.\d+\.\d+-" + TARGET + r"\.zip", path.name):
        raise ValueError("command ZIP name must carry its version and Windows target")


def pack(binary, archive):
    archive_name(archive)
    if binary.is_symlink() or not binary.is_file():
        raise ValueError("command executable is missing or linked")
    data = binary.read_bytes()
    executable(data)
    with zipfile.ZipFile(archive, "w", compression=zipfile.ZIP_DEFLATED) as output:
        member = zipfile.ZipInfo("thinkthen.exe", (1980, 1, 1, 0, 0, 0))
        member.compress_type = zipfile.ZIP_DEFLATED
        member.external_attr = 0o100755 << 16
        output.writestr(member, data)


def check(archive):
    archive_name(archive)
    sidecar = archive.with_name(archive.name + ".sha256")
    if any(path.is_symlink() or not path.is_file() for path in (archive, sidecar)):
        raise ValueError("command ZIP or checksum is missing or linked")
    digest = hashlib.sha256(archive.read_bytes()).hexdigest()
    if sidecar.read_text().strip() != f"{digest}  {archive.name}":
        raise ValueError("command ZIP differs from its checksum")
    with zipfile.ZipFile(archive) as source:
        if source.namelist() != ["thinkthen.exe"]:
            raise ValueError("command ZIP must hold exactly thinkthen.exe")
        member = source.getinfo("thinkthen.exe")
        if member.external_attr >> 16 & 0o170000 == 0o120000:
            raise ValueError("command ZIP holds a linked executable")
        executable(source.read(member))


def platform(folder, version):
    """Reject stage 2 artifacts and incomplete Windows command bundles."""
    name = f"thinkthen-{version}-{TARGET}.zip"
    c_name = f"thinkthen-c-{version}-{TARGET}.zip"
    wheel = f"thinkthen-{version}-cp310-abi3-win_amd64.whl"
    wanted = {c_name, c_name + ".sha256", name, name + ".sha256", wheel, wheel + ".sha256",
              "thinkthen-first-run.tar.gz", "thinkthen-first-run.tar.gz.sha256"}
    if folder.is_symlink() or not folder.is_dir():
        raise ValueError("Windows platform folder is missing or linked")
    if {path.name for path in folder.iterdir()} != wanted:
        raise ValueError("Windows platform must hold exactly the command ZIP, C ZIP, Python wheel and first-run sample with checksums")
    check(folder / name)
    windows_c.check(folder / c_name)
    wheel_path, wheel_sum = folder / wheel, folder / (wheel + ".sha256")
    if any(path.is_symlink() or not path.is_file() for path in (wheel_path, wheel_sum)):
        raise ValueError("Python wheel or checksum is missing or linked")
    if wheel_sum.read_text().strip() != f"{hashlib.sha256(wheel_path.read_bytes()).hexdigest()}  {wheel}":
        raise ValueError("Python wheel differs from its checksum")
    sample = folder / "thinkthen-first-run.tar.gz"
    sidecar = folder / (sample.name + ".sha256")
    if any(path.is_symlink() or not path.is_file() for path in (sample, sidecar)):
        raise ValueError("first-run sample or checksum is missing or linked")
    if sidecar.read_text().strip() != f"{hashlib.sha256(sample.read_bytes()).hexdigest()}  {sample.name}":
        raise ValueError("first-run sample differs from its checksum")


def main():
    parser = argparse.ArgumentParser(description=__doc__)
    parser.add_argument("action", choices=("pack", "check", "platform"))
    parser.add_argument("paths", nargs="+")
    args = parser.parse_args()
    if len(args.paths) != (1 if args.action == "check" else 2):
        parser.error("pack needs BINARY ARCHIVE; check needs ARCHIVE; platform needs FOLDER VERSION")
    try:
        if args.action == "platform":
            platform(Path(args.paths[0]), args.paths[1])
        elif args.action == "pack":
            pack(*(Path(path) for path in args.paths))
        else:
            check(Path(args.paths[0]))
    except (ValueError, OSError, zipfile.BadZipFile, RuntimeError) as error:
        print(f"release-windows-command: {error}", file=sys.stderr)
        return 1
    return 0


if __name__ == "__main__":
    sys.exit(main())
