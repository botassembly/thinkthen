#!/bin/sh
# Build the release wheel with every path under $HOME remapped, then check
# what it holds: the stub, the py.typed marker, the MIT license, no builder
# home path, and no test-only hook. Nothing is installed or published.
unset THINKTHEN_API_KEY
set -eu
cd -- "$(dirname -- "$0")"
out=$(mktemp -d)
trap 'rm -rf -- "$out"' EXIT

RUSTFLAGS="${RUSTFLAGS:+$RUSTFLAGS }--remap-path-prefix=$HOME=/build"
export RUSTFLAGS
maturin build --quiet --locked --offline --release -o "$out"

python3 - "$out"/thinkthen-*.whl "$HOME" <<'PY'
import sys
import zipfile

wheel, home = sys.argv[1], sys.argv[2].encode()
with zipfile.ZipFile(wheel) as held:
    names = held.namelist()
    metadata = next(held.read(n) for n in names if n.endswith(".dist-info/METADATA"))
    extensions = [held.read(n) for n in names if n.endswith(".so")]
wrong = [f"it lacks {need}" for need in ("thinkthen/__init__.pyi", "thinkthen/py.typed")
         if need not in names]
if b"License: MIT" not in metadata:
    wrong.append("its METADATA lacks License: MIT")
if len(extensions) != 1:
    wrong.append(f"it holds {len(extensions)} extensions, not 1")
for extension in extensions:
    if home in extension:
        wrong.append("its extension names the builder's home")
    for hook in (b"_live_workers", b"_arrow_probe"):
        if hook in extension:
            wrong.append(f"its extension carries the test hook {hook.decode()}")
if wrong:
    sys.exit(f"{wheel}: " + "; ".join(wrong))
print(f"{wheel.rsplit('/', 1)[-1]}: stub, marker, license, no home path, no test hook")
PY
