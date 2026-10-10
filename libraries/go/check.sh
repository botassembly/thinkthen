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
go_bin=${THINKTHEN_GO_BIN:-$(command -v go || true)}
python_bin=${THINKTHEN_PYTHON_BIN:-$(command -v python3 || true)}
for named in "$go_bin" "$python_bin"; do
    case "$named" in /*) ;; *) echo "go: not run: tool path is not absolute: $named" >&2; exit 77 ;; esac
    [ -x "$named" ] || { echo "go: not run: tool is unavailable: $named" >&2; exit 77; }
done
for tool in gofmt cargo cc nm node git flock; do
    command -v "$tool" >/dev/null 2>&1 || { echo "go: not run: no $tool" >&2; exit 77; }
done
# Refuse unsupported toolchains before any native build or installed check.
minimum=$(awk '$1 == "go" { print $2; exit }' go.mod)
installed=$("$go_bin" version | sed -n 's/^go version go1\.\([0-9][0-9]*\)\.[0-9][0-9]* [^ ]*$/\1/p')
[ -n "$installed" ] || { echo 'go: a stable Go 1.x release is required' >&2; exit 77; }
minimum_minor=${minimum#1.}
minimum_minor=${minimum_minor%%.*}
[ "$installed" -ge "$minimum_minor" ] || { echo "go: Go $minimum or newer is required" >&2; exit 77; }
if [ -n "${THINKTHEN_ARTIFACT:-}" ]; then
    [ -n "${THINKTHEN_C_ARTIFACT:-}" ] || { echo 'go: missing C archive' >&2; exit 1; }
fi
"$python_bin" -c 'import jsonschema' || { echo 'go: not run: Python jsonschema is unavailable' >&2; exit 77; }
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
"$python_bin" "$repo/sdlc/generators/results/generate.py" --target go --check
"$python_bin" fixtures/guard.py .
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
    "$python_bin" fixtures/package_owned.py "$PWD" "$out" "$module"
fi
# All compile checks target the installed module, whose static asset is bundled.
(cd "$module" && "$go_bin" vet .)
for example in decide smoke; do
    scratch_dir caller
    mkdir "$caller/$example"
    cp "examples/$example/main.go" "$caller/$example/main.go"
    printf 'module example.org/thinkthen-example\n\ngo 1.22\n\nrequire github.com/botassembly/thinkthen/libraries/go v0.0.0\nreplace github.com/botassembly/thinkthen/libraries/go => %s\n' "$module" >"$caller/$example/go.mod"
    (cd "$caller/$example" && "$go_bin" build -p 1 -buildvcs=false -o "$caller/example" .)
    if [ "${THINKTHEN_TEST_PROFILE:-}" = smoke ] && [ "$example" = smoke ]; then "$caller/example"; exit; fi
done
cargo build --locked --offline --manifest-path "$repo/Cargo.toml" --package conformance-backend -j2
"$python_bin" fixtures/owned_installed.py "$module" plan
"$python_bin" fixtures/owned_installed.py "$module"
"$python_bin" fixtures/owned_installed.py "$module" feed
"$python_bin" fixtures/owned_installed.py "$module" usage
"$python_bin" fixtures/shared_installed.py "$module"
echo 'go: pass'
