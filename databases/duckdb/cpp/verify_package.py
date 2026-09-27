"""Load the built C++ artifact and prove DuckDB refuses another version footer."""

from __future__ import annotations

import argparse
import subprocess
import sys
import tempfile
from pathlib import Path

sys.path.insert(0, str(Path(__file__).resolve().parents[1] / "tools"))
from harness import child_env  # noqa: E402  shared isolated host environment


def load(path: Path, folder: Path) -> subprocess.CompletedProcess[str]:
    code = (
        "import duckdb,sys; "
        "con=duckdb.connect(config={'allow_unsigned_extensions':'true'}); "
        "con.execute('LOAD ' + chr(39) + sys.argv[1] + chr(39))"
    )
    return subprocess.run(
        [sys.executable, "-c", code, str(path)],
        env=child_env("http://127.0.0.1:1/v1", folder),
        text=True,
        capture_output=True,
        timeout=20,
        check=False,
    )


def main() -> None:
    parser = argparse.ArgumentParser()
    parser.add_argument("--extension", required=True, type=Path)
    source = parser.parse_args().extension.resolve(strict=True)
    with tempfile.TemporaryDirectory(prefix="thinkthen-duckdb-package-") as scratch:
        root = Path(scratch)
        good = root / "matching" / "thinkthen.duckdb_extension"
        good.parent.mkdir()
        data = source.read_bytes()
        good.write_bytes(data)
        accepted = load(good, root / "matching-host")
        assert accepted.returncode == 0, accepted.stderr[:800]

        old, new = b"v1.5.5", b"v1.5.4"
        at = data.rfind(old)
        assert at >= len(data) - 512, "the pinned version is absent from the extension footer"
        wrong = root / "wrong" / "thinkthen.duckdb_extension"
        wrong.parent.mkdir()
        wrong.write_bytes(data[:at] + new + data[at + len(old) :])
        refused = load(wrong, root / "wrong-host")
        assert refused.returncode != 0, "the stock v1.5.5 host loaded a v1.5.4 footer"
        assert "can only be loaded with that version" in refused.stderr, refused.stderr[:800]
        assert "this version of DuckDB is 'v1.5.5'" in refused.stderr, refused.stderr[:800]
        print("C++ package loads in v1.5.5 and refuses a v1.5.4 footer")


if __name__ == "__main__":
    main()
