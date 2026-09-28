"""Strip the macOS DuckDB binary while retaining its appended host footer."""

from __future__ import annotations

import os
import subprocess
import sys
import tempfile
from pathlib import Path


def main(source: Path, destination: Path) -> None:
    # DuckDB appends this unsigned 534-byte metadata block after the Mach-O.
    # Apple's strip refuses a file whose bytes extend past __LINKEDIT.
    data = source.read_bytes()
    footer = data[-534:]
    if (len(data) <= len(footer) or not footer.startswith(b"\x00\x93\x04\x10duckdb_signature\x80\x04")
            or footer[22 + 3 * 32:22 + 4 * 32].rstrip(b"\x00") != b"CPP"
            or footer[22 + 6 * 32:22 + 7 * 32].rstrip(b"\x00") != b"osx_arm64"
            or footer[22 + 7 * 32:22 + 8 * 32].rstrip(b"\x00") != b"4"):
        raise SystemExit("duckdb: unexpected macOS C++ extension footer")

    destination.parent.mkdir(parents=True, exist_ok=True)
    with tempfile.TemporaryDirectory(dir=destination.parent) as folder:
        raw = Path(folder) / "thinkthen.dylib"
        raw.write_bytes(data[:-len(footer)])
        subprocess.run(["strip", "-S", "-x", str(raw)], check=True)
        stripped = raw.read_bytes()
        if str(Path.home()).encode() in stripped:
            raise SystemExit("duckdb: builder-home path remains in stripped Mach-O")
        raw.write_bytes(stripped + footer)
        os.chmod(raw, 0o644)
        os.replace(raw, destination)


if __name__ == "__main__":
    if len(sys.argv) != 3:
        raise SystemExit("usage: strip_macos.py SOURCE DESTINATION")
    main(Path(sys.argv[1]), Path(sys.argv[2]))
