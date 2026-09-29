#!/bin/sh
set -eu
here=$(CDPATH= cd -- "$(dirname -- "$0")" && pwd)
root=$(CDPATH= cd -- "$here/../.." && pwd)
lock=${THINKTHEN_HEAVY_LOCK:-${XDG_RUNTIME_DIR:-/tmp}/thinkthen-csharp.lock}
if [ "${THINKTHEN_HEAVY_LOCK_HELD:-}" != "$lock" ] && command -v flock >/dev/null 2>&1; then
    THINKTHEN_HEAVY_LOCK_HELD=$lock
    export THINKTHEN_HEAVY_LOCK_HELD
    exec flock -o "$lock" /bin/sh "$0" "$@"
fi
dotnet=${THINKTHEN_DOTNET:-$(command -v dotnet || true)}
[ -x "$dotnet" ] || exit 77
command -v python3 >/dev/null 2>&1 || exit 77
python3 "$here/tests/toolchains.py"
if [ -n "${THINKTHEN_ARTIFACT:-}" ]; then
    [ -f "${THINKTHEN_C_ARTIFACT:-}" ] || { echo 'C# installed: C archive missing' >&2; exit 1; }
    . "$root/sdlc/scripts/scratch.sh"
    . "$root/sdlc/scripts/installed.sh"
    installed_unpack
    managed=$scratch
    scratch_dir native
    tar -xzf "$THINKTHEN_C_ARTIFACT" -C "$native"
    python3 "$root/sdlc/scripts/check-c-exports.py" "$native/include/thinkthen.h" "$native/lib/libthinkthen.so"
    version=$(sed -n 's/^version = "\(.*\)"$/\1/p' "$root/crates/thinkthen/Cargo.toml" | head -n 1)
    THINKTHEN_RELEASE_NUPKG="$managed/Botassembly.ThinkThen.$version.nupkg" \
    THINKTHEN_RELEASE_C_DIR="$native" \
        python3 "$here/tests/isolated_consumer.py" release "$managed"
    echo 'C# installed release PASS: two exact calls from package files'
    exit 0
fi
mkdir -p "$here/target/scratch/lib" "$here/target/scratch/nuget" "$here/target/scratch/dotnet-home" "$here/target/scratch/managed" "$here/target/artifacts/native/lib" "$here/target/logs"
export DOTNET_CLI_HOME="$here/target/scratch/dotnet-home" NUGET_PACKAGES="$here/target/scratch/nuget"
export DOTNET_CLI_TELEMETRY_OPTOUT=1 DOTNET_SKIP_FIRST_TIME_EXPERIENCE=1 DOTNET_NOLOGO=1
RUSTC_WRAPPER= CARGO_NET_OFFLINE=true cargo build --manifest-path "$root/libraries/c/Cargo.toml" --locked --offline --lib -j2
native="$root/libraries/c/target/debug/libthinkthen_c.so"
python3 "$root/sdlc/scripts/check-c-exports.py" "$root/libraries/c/include/thinkthen.h" "$native"
cp "$native" "$here/target/scratch/lib/libthinkthen.so"
ln -sf libthinkthen.so "$here/target/scratch/lib/libthinkthen.so.0"
cp "$native" "$here/target/artifacts/native/lib/libthinkthen.so"
ln -sf libthinkthen.so "$here/target/artifacts/native/lib/libthinkthen.so.0"
tar -czf "$here/target/artifacts/thinkthen-c-0.0.1-x86_64-linux-gnu.tar.gz" -C "$here/target/artifacts/native" .
"$dotnet" pack "$here/ThinkThen.csproj" -c Release --source "$here/target/scratch/nuget" -o "$here/target/scratch/managed" -v quiet
test -f "$here/target/scratch/managed/Botassembly.ThinkThen.0.0.1.nupkg"
python3 "$here/tests/package_check.py"
python3 "$here/tests/run_matrix.py"
run_dir=$(mktemp -d "$here/target/logs/package-XXXXXX")
python3 "$here/tests/isolated_consumer.py" alpha "$run_dir"
python3 "$here/tests/isolated_consumer.py" beta "$run_dir"
"$dotnet" build "$here/tests/TypeCase.csproj" -c Release --source "$here/target/scratch/nuget" -v quiet
python3 "$here/tests/types.py"
echo 'C# package PASS: exact matrix, installed consumers, J1 corpus'
