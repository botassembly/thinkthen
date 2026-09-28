"""Copy legal notices from the pinned DuckDB source and resolved Rust crates."""

from __future__ import annotations

import json
import shutil
import subprocess
import sys
from pathlib import Path

HERE = Path(__file__).resolve().parent
BRIDGE = HERE.parent / "bridge" / "Cargo.toml"
LEGAL = ("license", "notice", "copying", "authors", "copyright", "patents")


def legal_files(folder: Path) -> list[Path]:
    return sorted(p for p in folder.iterdir() if p.is_file() and p.name.lower().startswith(LEGAL))


def main() -> None:
    if len(sys.argv) != 3:
        raise SystemExit("usage: package_notices.py DUCKDB_SOURCE PACKAGE_FOLDER")
    source, destination = (Path(value).resolve(strict=True) for value in sys.argv[1:])
    licenses = destination / "LICENSES"
    duckdb = sorted(p for p in source.rglob("*") if p.is_file() and p.name.lower().startswith(LEGAL) and p != source / "LICENSE")
    if not duckdb:
        raise SystemExit("the pinned DuckDB source contains no third-party notices")
    for path in duckdb:
        target = licenses / "duckdb" / path.relative_to(source)
        target.parent.mkdir(parents=True, exist_ok=True)
        shutil.copyfile(path, target)

    metadata = json.loads(subprocess.check_output([
        "cargo", "metadata", "--format-version", "1", "--locked", "--offline",
        "--manifest-path", str(BRIDGE),
    ]))
    archives = [line.split(maxsplit=1)[1] for line in (HERE / "archive-sha256.txt").read_text().splitlines()]
    inventory = ["Pinned DuckDB static archives:", *archives, "", "Bundled Rust crates (name version | declared license):"]
    for package in sorted(metadata["packages"], key=lambda value: (value["name"], value["version"])):
        folder = Path(package["manifest_path"]).parent
        if "/registry/" not in str(folder):
            continue
        files = legal_files(folder)
        if not files:
            raise SystemExit(f"missing legal file for {package['name']} {package['version']}")
        name = f"{package['name']}-{package['version']}"
        inventory.append(f"{name} | {package['license'] or 'see copied legal files'}")
        target = licenses / "rust" / name
        target.mkdir(parents=True, exist_ok=True)
        for path in files:
            shutil.copyfile(path, target / path.name)
    inventory.extend(("", "Pinned DuckDB source license and notice files:"))
    inventory.extend(str(path.relative_to(source)) for path in duckdb)
    (destination / "DEPENDENCIES.txt").write_text("\n".join(inventory) + "\n")


if __name__ == "__main__":
    main()
