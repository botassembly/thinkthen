#!/usr/bin/env bash
# The Python surface's wheel rehearsal: build the release wheel with every
# path under $HOME remapped to a neutral prefix, then prove the extension
# inside the wheel carries none of the builder's home directory. Cargo
# embeds absolute source paths (the workspace, the registry) in panic
# locations; an artifact that names /home/<builder> leaks who built it and
# where. Nothing is installed and nothing is published.
#
# The same remap rides in check.sh so the extension the tests import and
# the wheel a host installs carry the same clean paths.
set -euo pipefail
cd "$(dirname "$0")"

export RUSTFLAGS="${RUSTFLAGS:+$RUSTFLAGS }--remap-path-prefix=$HOME=/build"
maturin build --release -o dist

wheel=$(ls -t dist/thinkthen-*.whl | head -1)
python3 - "$wheel" "$HOME" << 'PY'
import sys, tempfile, zipfile, subprocess, os

wheel, home = sys.argv[1], sys.argv[2]
with zipfile.ZipFile(wheel) as z:
    names = [n for n in z.namelist() if n.endswith(".so")]
    if not names:
        sys.exit(f"{wheel}: no extension inside")
    for name in names:
        with tempfile.NamedTemporaryFile(suffix=".so", delete=False) as tmp:
            tmp.write(z.read(name))
            path = tmp.name
        try:
            out = subprocess.run(["strings", path], capture_output=True, text=True, check=True).stdout
        finally:
            os.unlink(path)
        hits = [line for line in out.splitlines() if home in line]
        if hits:
            sys.exit(f"{wheel}: {name} still carries {home} ({len(hits)} lines)")
        print(f"{wheel}: {name} carries no {home}")
PY
