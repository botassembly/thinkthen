#!/usr/bin/env python3
"""Build the native Rust bridge without downloading any toolchain or crate."""

import argparse
import os
from pathlib import Path
import re
import shlex
import subprocess
import sys
import tomllib

ROOT = Path(__file__).resolve().parents[3]
VERSION = tomllib.loads((ROOT / "rust-toolchain.toml").read_text())["toolchain"]["channel"]


def environment(args):
    env = os.environ.copy()
    env["CARGO_NET_OFFLINE"] = "true"
    env["RUSTUP_TOOLCHAIN"] = VERSION
    target_key = args.target.replace("-", "_")
    rustflags = shlex.split(env.get("RUSTFLAGS", ""))
    if env.get("CARGO_ENCODED_RUSTFLAGS"):
        rustflags = env["CARGO_ENCODED_RUSTFLAGS"].split("\x1f")
    rustflags += ["-C", "relocation-model=pic", f"--remap-path-prefix={Path.home()}=/build"]
    if args.target.endswith("-musl"):
        rustflags += ["-C", "target-feature=-crt-static"]
    cflags = shlex.split(env.get(f"CFLAGS_{target_key}", env.get("CFLAGS", ""))) + shlex.split(args.cflags)
    cflags += ["-fPIC", f"-ffile-prefix-map={Path.home()}=/build"]
    if args.cc:
        env[f"CC_{target_key}"] = args.cc
    if args.ar:
        env[f"AR_{target_key}"] = args.ar
    if args.target.endswith("-apple-darwin"):
        if not args.deployment:
            raise SystemExit("community: the Apple deployment target must come from DuckDB CMake")
        arch = "arm64" if args.target.startswith("aarch64-") else "x86_64"
        cflags += ["-arch", arch, f"-mmacosx-version-min={args.deployment}"]
        env["MACOSX_DEPLOYMENT_TARGET"] = args.deployment
        sysroot = args.sysroot
        if sysroot and not Path(sysroot).is_dir():
            sysroot = subprocess.check_output(["xcrun", "--sdk", sysroot, "--show-sdk-path"], text=True).strip()
        if sysroot:
            cflags += ["-isysroot", sysroot]
            env["SDKROOT"] = sysroot
        check_apple_std(args, env)
    env[f"CFLAGS_{target_key}"] = shlex.join(cflags)
    env["CARGO_ENCODED_RUSTFLAGS"] = "\x1f".join(rustflags)
    env.pop("RUSTFLAGS", None)
    return env


def check_apple_std(args, env):
    sysroot = subprocess.check_output(["rustc", f"+{VERSION}", "--print", "sysroot"], env=env, text=True).strip()
    archives = list((Path(sysroot) / "lib/rustlib" / args.target / "lib").glob("libstd-*.rlib"))
    if len(archives) != 1:
        raise SystemExit("community: prepare the selected Apple Rust standard library")
    loads = subprocess.check_output(["otool", "-l", str(archives[0])], text=True)
    minimums = re.findall(r"^\s+(?:minos|version)\s+(\d+(?:\.\d+)+)\s*$", loads, re.MULTILINE)
    def version(value):
        parts = tuple(int(part) for part in value.split("."))
        return parts + (0,) * (3 - len(parts))
    if not minimums or any(version(value) > version(args.deployment) for value in minimums):
        raise SystemExit(f"community: Rust's Apple standard library exceeds macOS {args.deployment}")


def main():
    parser = argparse.ArgumentParser(description=__doc__)
    parser.add_argument("--target", required=True)
    parser.add_argument("--output", type=Path, required=True)
    parser.add_argument("--cc", default="")
    parser.add_argument("--ar", default="")
    parser.add_argument("--cflags", default="")
    parser.add_argument("--sysroot", default="")
    parser.add_argument("--deployment", default="")
    args = parser.parse_args()
    if not re.fullmatch(r"(?:x86_64|aarch64)-(?:unknown-linux-(?:gnu|musl)|apple-darwin)", args.target):
        raise SystemExit("community: the bridge supports only native Linux and Apple targets")
    env = environment(args)
    result = subprocess.run(["cargo", f"+{VERSION}", "rustc", "--locked", "--offline", "--release",
                             "--target", args.target, "--manifest-path",
                             str(ROOT / "databases/duckdb/bridge/Cargo.toml"),
                             "--target-dir", str(args.output), "--", "--print", "native-static-libs"],
                            env=env, text=True, stdout=sys.stdout, stderr=subprocess.PIPE)
    sys.stderr.write(result.stderr)
    if result.returncode:
        raise SystemExit(result.returncode)
    match = re.search(r"native-static-libs: ([^\n]+)", result.stderr)
    if match is None:
        raise SystemExit("community: Rust did not report the bridge's native static libraries")
    # GNU and Apple linkers both accept their native library flags in a response file.
    (args.output / "native-libs.rsp").write_text(match.group(1) + "\n")


if __name__ == "__main__":
    main()
