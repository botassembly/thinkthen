#!/bin/sh
# Stage the Go module and run its installed routine consumers offline.
set -eu
cd -- "$(dirname -- "$0")"
repo=$(cd ../.. && pwd)
case "${THINKTHEN_TEST_PROFILE:-routine}" in
    routine|full|smoke) ;;
    stress) echo 'go: not run: no stress gate'; exit 77 ;;
    *) echo 'go: unknown profile' >&2; exit 2 ;;
esac
if [ "$(uname -s)" != Linux ] || [ "$(uname -m)" != x86_64 ]; then
    echo 'go: not run: this package gate is proven on Linux x86_64'; exit 77
fi
for tool in go gofmt python3 cargo cc nm node git flock; do
    command -v "$tool" >/dev/null 2>&1 || { echo "go: not run: no $tool" >&2; exit 77; }
done
lock=${THINKTHEN_HEAVY_LOCK:-/run/user/1000/thinkthen-codex-6.lock}
if [ "${THINKTHEN_HEAVY_LOCK_HELD:-}" != "$lock" ]; then
    export THINKTHEN_HEAVY_LOCK_HELD="$lock"
    exec flock -w 180 -o "$lock" /bin/sh "$repo/libraries/go/check.sh" "$@"
fi
. "$repo/sdlc/scripts/scratch.sh"
usage_home
export CARGO_NET_OFFLINE=true CARGO_BUILD_JOBS=2
export CARGO_PROFILE_DEV_DEBUG=${CARGO_PROFILE_DEV_DEBUG:-0} CARGO_INCREMENTAL=${CARGO_INCREMENTAL:-0}
export CARGO_BUILD_RUSTC_WRAPPER= RUSTC_WRAPPER=
export GOPROXY=off GOSUMDB=off GOTOOLCHAIN=local CGO_ENABLED=1
export GOCACHE="$repo/target/go/cache" GOMODCACHE="$repo/target/go/modcache"
node "$repo/sdlc/scripts/ratchet.mjs" ratchet.go.json
node "$repo/sdlc/scripts/ratchet.mjs" ratchet.py.json
python3 "$repo/sdlc/generators/results/generate.py" --target go --check
python3 fixtures/guard.py .
test -z "$(gofmt -l .)"
. "$repo/sdlc/scripts/installed.sh"
if [ -n "${THINKTHEN_ARTIFACT:-}" ]; then
    installed_unpack
    module=$scratch
else
    cargo build --locked --offline --manifest-path "$repo/libraries/c/Cargo.toml" --lib -j2
    out="$repo/target/go/native"
    mkdir -p "$out/include" "$out/lib"
    cp "$repo/libraries/c/include/thinkthen.h" "$out/include/thinkthen.h"
    sh "$repo/libraries/c/localize.sh" "$repo/libraries/c/target/debug/libthinkthen_c.a" "$out/lib/libthinkthen.a"
    scratch_dir installed
    module="$installed/module"
    python3 fixtures/package_owned.py "$PWD" "$out" "$module"
fi
# All compile checks target the installed module, whose static asset is bundled.
(cd "$module" && go vet .)
for example in decide smoke; do
    scratch_dir caller
    mkdir "$caller/$example"
    cp "examples/$example/main.go" "$caller/$example/main.go"
    printf 'module example.org/thinkthen-example\n\ngo 1.22\n\nrequire github.com/botassembly/thinkthen/libraries/go v0.0.0\nreplace github.com/botassembly/thinkthen/libraries/go => %s\n' "$module" >"$caller/$example/go.mod"
    (cd "$caller/$example" && go build -p 1 -buildvcs=false -o "$caller/example" .)
    if [ "${THINKTHEN_TEST_PROFILE:-}" = smoke ] && [ "$example" = smoke ]; then "$caller/example"; exit; fi
done
cargo build --locked --offline --manifest-path "$repo/Cargo.toml" --package conformance-backend -j2
python3 fixtures/owned_installed.py "$module"
python3 fixtures/owned_installed.py "$module" feed
python3 fixtures/owned_installed.py "$module" usage
python3 fixtures/shared_installed.py "$module"
echo 'go: pass'
