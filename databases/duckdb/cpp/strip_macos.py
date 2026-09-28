"""Strip the macOS DuckDB binary while retaining its appended host footer."""

from __future__ import annotations

import os
import subprocess
import sys
import tempfile
from pathlib import Path

sys.path.insert(0, str(Path(__file__).resolve().parents[3] / "conformance" / "children"))
from children import child_env  # noqa: E402
from verify_footer import FOOTER_BYTES, verify_common


def main(source: Path, destination: Path, platform: str) -> None:
    if platform not in ("osx_arm64", "osx_amd64"):
        raise SystemExit("duckdb: unsupported macOS C++ extension platform")
    # DuckDB appends this unsigned 534-byte metadata block after the Mach-O.
    # Apple's strip refuses a file whose bytes extend past __LINKEDIT.
    data = source.read_bytes()
    footer = data[-FOOTER_BYTES:]
    try:
        verify_common(footer, platform)
    except ValueError as error:
        raise SystemExit(f"duckdb: unexpected macOS C++ extension footer: {error}") from error
    if len(data) <= FOOTER_BYTES:
        raise SystemExit("duckdb: unexpected macOS C++ extension footer")

    destination.parent.mkdir(parents=True, exist_ok=True)
    with tempfile.TemporaryDirectory(dir=destination.parent) as folder:
        raw = Path(folder) / "thinkthen.dylib"
        raw.write_bytes(data[:-len(footer)])
        subprocess.run(["strip", "-S", "-x", str(raw)], check=True, env=child_env())
        stripped = raw.read_bytes()
        if str(Path.home()).encode() in stripped:
            raise SystemExit("duckdb: builder-home path remains in stripped Mach-O")
        raw.write_bytes(stripped + footer)
        os.chmod(raw, 0o644)
        os.replace(raw, destination)


if __name__ == "__main__":
    if len(sys.argv) != 4:
        raise SystemExit("usage: strip_macos.py SOURCE DESTINATION PLATFORM")
    main(Path(sys.argv[1]), Path(sys.argv[2]), sys.argv[3])
