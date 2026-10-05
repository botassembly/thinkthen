#!/usr/bin/env python3
"""Download the pinned Rust toolchain and locked crates during configure_ci."""

from pathlib import Path
import platform
import subprocess
import sys
import tomllib

ROOT = Path(__file__).resolve().parents[3]
TOOLCHAIN = tomllib.loads((ROOT / "rust-toolchain.toml").read_text())["toolchain"]


def main():
    arch = sys.argv[1] if len(sys.argv) > 1 else ""
    if arch:
        targets = {"arm64": "aarch64-apple-darwin", "x86_64": "x86_64-apple-darwin"}
        if platform.system() != "Darwin" or arch not in targets:
            raise SystemExit("community: OSX_BUILD_ARCH requires a supported Apple target")
        target = targets[arch]
    else:
        target = None
    version = TOOLCHAIN["channel"]
    subprocess.run(["rustup", "toolchain", "install", version, "--profile", TOOLCHAIN["profile"],
                    "--component", ",".join(TOOLCHAIN["components"])], check=True)
    if target is None:
        host = subprocess.check_output(["rustc", f"+{version}", "-vV"], text=True)
        target = next(line.removeprefix("host: ") for line in host.splitlines() if line.startswith("host: "))
    subprocess.run(["rustup", "target", "add", "--toolchain", version, target], check=True)
    subprocess.run(["cargo", f"+{version}", "fetch", "--locked", "--target", target,
                    "--manifest-path", str(ROOT / "databases/duckdb/bridge/Cargo.toml")], check=True)


if __name__ == "__main__":
    main()
