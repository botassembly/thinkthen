#!/bin/sh
# Build the release wheel with every path under $HOME remapped, then check
# what it holds: the stub, the py.typed marker, the MIT license, no builder
# home path in any file, and no test-only hook. Nothing is installed or published.
# pyproject.toml turns off the Rust SBOM, which names each path crate by its
# absolute folder (ticket 0128).
unset THINKTHEN_API_KEY
set -eu
cd -- "$(dirname -- "$0")"
. ../../sdlc/scripts/scratch.sh
scratch_dir out

builder_home=$HOME
case $(uname -s) in MINGW*|MSYS*) builder_home=$(cygpath -m "$HOME") ;; esac
RUSTFLAGS="${RUSTFLAGS:+$RUSTFLAGS }--remap-path-prefix=$builder_home=/build"
export RUSTFLAGS
maturin build --quiet --locked --offline --release -o "$out"

python3 - "$out"/thinkthen-*.whl "$builder_home" <<'PY'
import sys
import zipfile

wheel, home = sys.argv[1], sys.argv[2].encode()
with zipfile.ZipFile(wheel) as held:
    names = held.namelist()
    metadata = next(held.read(n) for n in names if n.endswith(".dist-info/METADATA"))
    extensions = [held.read(n) for n in names if n.endswith((".so", ".pyd"))]
    homed = [n for n in names if home in held.read(n)]
wrong = [f"it lacks {need}" for need in ("thinkthen/__init__.pyi", "thinkthen/py.typed",
                                      "thinkthen/judge.py", "thinkthen/stream.py", "thinkthen/complete.py", "thinkthen/complete.pyi", "thinkthen/_complete.py")
         if need not in names]
if b"License: MIT" not in metadata:
    wrong.append("its METADATA lacks License: MIT")
if len(extensions) != 1:
    wrong.append(f"it holds {len(extensions)} extensions, not 1")
if homed:
    wrong.append(f"{', '.join(homed)} name the builder's home")
for extension in extensions:
    for hook in (b"_live_workers", b"_arrow_probe", b"_raw_producer", b"_probe_trace"):
        if hook in extension:
            wrong.append(f"its extension carries the test hook {hook.decode()}")
if wrong:
    sys.exit(f"{wheel}: " + "; ".join(wrong))
print(f"{wheel.rsplit('/', 1)[-1]}: stub, marker, license, no home path, no test hook")
PY
# release-pack takes the wheel from here (ticket 0128). maturin writes its own builds and editable
# wheels to target/wheels, so this checked wheel gets a folder nothing else writes.
mkdir -p target/release-wheel && cp -- "$out"/thinkthen-*.whl target/release-wheel/
