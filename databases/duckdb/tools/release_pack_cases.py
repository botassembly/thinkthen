"""Exercise the real reuse packer against altered copies of one built artifact."""

from __future__ import annotations

import hashlib
import os
import shutil
import subprocess
import sys
import tempfile
from pathlib import Path


REPO = Path(__file__).resolve().parents[3]
sys.path.insert(0, str(REPO / "conformance" / "children"))
from children import CARGO, child_env  # noqa: E402

WRONG_PLATFORM = {
    "x86_64-unknown-linux-gnu": "osx_amd64",
    "aarch64-unknown-linux-gnu": "osx_arm64",
    "aarch64-apple-darwin": "linux_arm64",
    "x86_64-apple-darwin": "linux_amd64",
}


def digest(path: Path) -> str:
    with path.open("rb") as source:
        return hashlib.file_digest(source, "sha256").hexdigest()


def main(target: str) -> None:
    wrong_platform = WRONG_PLATFORM[target]
    artifact = REPO / "databases/duckdb/build/artifacts/cpp" / target / "thinkthen.duckdb_extension"
    if not artifact.is_file():
        raise SystemExit(f"the built {target} artifact is missing")
    original = digest(artifact)
    with tempfile.TemporaryDirectory(prefix="thinkthen-reuse-") as folder:
        backup = Path(folder) / "original.duckdb_extension"
        shutil.copy2(artifact, backup)
        env = child_env(CARGO, RUSTC_WRAPPER="", CARGO_NET_OFFLINE="true",
                        THINKTHEN_TOOLCHAINS=os.environ.get(
                            "THINKTHEN_TOOLCHAINS", str(Path.home() / ".cache/thinkthen-toolchains")))
        try:
            for name, index, value in (("platform", 6, wrong_platform),
                                       ("DuckDB version", 5, "v1.5.4"),
                                       ("extension version", 4, "9.9.9"),
                                       ("ABI", 3, "C")):
                shutil.copy2(backup, artifact)
                with artifact.open("r+b") as output:
                    output.seek(-534 + 22 + 32 * index, 2)
                    output.write(value.encode().ljust(32, b"\0"))
                destination = Path(folder) / name.replace(" ", "-")
                result = subprocess.run([str(REPO / "sdlc/scripts/release-pack"), "--reuse", target,
                                         str(destination), "duckdb"], cwd=REPO, capture_output=True, text=True,
                                        check=False, env=env)
                if result.returncode == 0 or f"DuckDB footer {name} is" not in result.stderr:
                    raise AssertionError(f"reuse did not refuse the wrong {name}: exit {result.returncode}")
                if list(destination.glob("*.tar.gz")):
                    raise AssertionError(f"reuse archived the wrong {name}")
                print(f"ok   reuse refuses wrong {name}")
        finally:
            shutil.copy2(backup, artifact)
    if digest(artifact) != original:
        raise AssertionError("the built artifact changed after the reuse probes")


if __name__ == "__main__":
    if len(sys.argv) != 2 or sys.argv[1] not in WRONG_PLATFORM:
        raise SystemExit("usage: release_pack_cases.py RUST_TARGET")
    main(sys.argv[1])
