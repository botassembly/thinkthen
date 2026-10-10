#!/bin/sh
# The Rust examples surface. The surface rung passes its loopback port as $1.
set -eu
cd -- "$(dirname -- "$0")"
profile=${THINKTHEN_TEST_PROFILE:-routine}
case $profile in routine|full|stress|smoke) ;; *) echo "rust: unknown THINKTHEN_TEST_PROFILE: $profile" >&2; exit 2 ;; esac
[ "$profile" = routine ] || unset THINKTHEN_CONFORMANCE_IDS
if [ "$profile" = stress ]; then
    echo 'rust: not run: no port load campaign'
    exit 77
fi
# ADR 0113: this run's engines write a scratch usage folder, never the real one.
. ../../sdlc/scripts/scratch.sh
usage_home
if [ -n "${THINKTHEN_ARTIFACT:-}" ]; then
    [ -f "$THINKTHEN_ARTIFACT" ] && [ ! -L "$THINKTHEN_ARTIFACT" ] || { echo 'rust: installed crate is missing or linked' >&2; exit 1; }
    root=$(CDPATH= cd ../.. && pwd)
    scratch_dir installed
    mkdir "$installed/crate" "$installed/consumer"
    tar -xzf "$THINKTHEN_ARTIFACT" -C "$installed/crate" --strip-components=1
    python3 - "$root" "$installed" <<'RUSTCONSUMER'
import shutil,sys
from pathlib import Path
root,installed=map(Path,sys.argv[1:])
project=installed/'consumer'
manifest=(root/'libraries/python/Cargo.toml').read_text()
start=manifest.index('[lib]\n');end=manifest.index('[dependencies]\n',start)
manifest=manifest[:start]+manifest[end:]
shutil.copytree(root/'crates/thinkthen-host',installed/'thinkthen-host',ignore=shutil.ignore_patterns('target'))
host=installed/'thinkthen-host/Cargo.toml'
host.write_text(host.read_text().replace('../thinkthen\"',str(installed/'crate')+'\"'))
manifest=manifest.replace('../../crates/thinkthen-host',str(installed/'thinkthen-host')).replace('../../crates/thinkthen',str(installed/'crate'))
(project/'Cargo.toml').write_text(manifest)
shutil.copyfile(root/'libraries/python/Cargo.lock',project/'Cargo.lock')
shutil.copytree(root/'libraries/python/examples/native_case',project/'src')
RUSTCONSUMER
    python3 - "$root" "$installed/consumer/Cargo.toml" <<'RUSTNATIVE'
import sys
from pathlib import Path
root=Path(sys.argv[1]);sys.path.insert(0,str(root/'libraries/python/tests'))
from native_fixture import run
binary=root/'libraries/python/target/debug/thinkthen-python'
sys.exit(bool(run('rust',[str(binary)],root,rust_manifest=Path(sys.argv[2]))))
RUSTNATIVE
    echo 'rust: pass, installed'
    exit 0
fi
if [ "$profile" = smoke ]; then
    smoke_guard
    cargo build --quiet --locked --offline --bin smoke
    "${CARGO_TARGET_DIR:-target}/debug/smoke"
    exit
fi

cargo fmt --check
cargo clippy --locked --offline --all-targets -- -D warnings
cargo test --locked --offline
# The rung's own backend answers the slide as the test's backends do.
cargo build --quiet --locked --offline --example slide
. ../../sdlc/scripts/scratch.sh
scratch_dir cache
env -i THINKTHEN_CACHE="$cache" THINKTHEN_API_KEY=sk-examples-loopback \
    THINKTHEN_BASE_URL="http://127.0.0.1:$1/generic/v1" \
    "${CARGO_TARGET_DIR:-target}/debug/examples/slide" >"$cache/out" 2>"$cache/err"
[ ! -s "$cache/err" ] || { cat -- "$cache/err" >&2; exit 1; }
diff -u examples/slide.txt "$cache/out"
