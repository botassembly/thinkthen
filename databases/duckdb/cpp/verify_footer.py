"""Refuse a staged DuckDB extension whose host metadata names another target."""

from __future__ import annotations

import sys
from pathlib import Path


PLATFORMS = {
    "x86_64-unknown-linux-gnu": "linux_amd64",
    "aarch64-unknown-linux-gnu": "linux_arm64",
    "aarch64-apple-darwin": "osx_arm64",
    "x86_64-apple-darwin": "osx_amd64",
}
HEADER = b"\x00\x93\x04\x10duckdb_signature\x80\x04"
FOOTER_BYTES = len(HEADER) + 16 * 32


def verify_common(footer: bytes, platform: str) -> list[bytes]:
    if len(footer) != FOOTER_BYTES or footer[: len(HEADER)] != HEADER:
        raise ValueError("DuckDB extension has no signed-format footer")
    fields = [footer[len(HEADER) + index * 32:len(HEADER) + (index + 1) * 32].rstrip(b"\0")
              for index in range(16)]
    for index, wanted, name in ((3, b"CPP", "ABI"), (6, platform.encode(), "platform"),
                                (7, b"4", "ABI version")):
        if fields[index] != wanted:
            raise ValueError(f"DuckDB footer {name} is {fields[index]!r}, expected {wanted!r}")
    return fields


def verify(path: Path, target: str, duckdb_version: str, extension_version: str) -> None:
    platform = PLATFORMS.get(target)
    if platform is None:
        raise ValueError(f"no pinned DuckDB footer platform for {target}")
    with path.open("rb") as source:
        source.seek(0, 2)
        if source.tell() <= FOOTER_BYTES:
            raise ValueError("DuckDB extension has no complete footer")
        source.seek(-FOOTER_BYTES, 2)
        footer = source.read(FOOTER_BYTES)
    fields = verify_common(footer, platform)
    for index, wanted in ((4, extension_version.encode()), (5, duckdb_version.encode())):
        if fields[index] != wanted:
            name = "extension version" if index == 4 else "DuckDB version"
            raise ValueError(f"DuckDB footer {name} is {fields[index]!r}, expected {wanted!r}")


if __name__ == "__main__":
    if len(sys.argv) != 5:
        raise SystemExit("usage: verify_footer.py EXTENSION TARGET DUCKDB_VERSION EXTENSION_VERSION")
    try:
        verify(Path(sys.argv[1]), sys.argv[2], sys.argv[3], sys.argv[4])
    except (OSError, ValueError) as error:
        raise SystemExit(f"release-pack: {error}") from error
