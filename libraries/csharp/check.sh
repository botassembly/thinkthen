#!/bin/sh
set -eu
python3 "$(dirname -- "$0")/../../sdlc/scripts/check_verdict.py" platform libraries/csharp
tools_group=csharp
. "$(dirname -- "$0")/../../sdlc/scripts/resolved-tools"
here=$(CDPATH= cd -- "$(dirname -- "$0")" && pwd)
root=$(CDPATH= cd -- "$here/../.." && pwd)
version=$(sed -n 's:.*<Version>\(.*\)</Version>.*:\1:p' "$here/ThinkThen.csproj")
# ADR 0113: this run's engines write a scratch usage folder, never the real one.
. "$root/sdlc/scripts/scratch.sh"
usage_home
dotnet=$(command -v dotnet || true)
[ -x "$dotnet" ] || exit 1
command -v python3 >/dev/null 2>&1 || exit 1
case "${THINKTHEN_TEST_PROFILE:-routine}" in
    routine|full|smoke) ;;
    *) echo 'C#: unknown test profile' >&2; exit 2 ;;
esac
python3 "$here/tests/toolchains.py"
python3 "$root/conformance/children/privacy.py" --sources "$here" --public --artifact-env
python3 "$here/tests/generate_fixture_questions.py" --check
python3 "$root/sdlc/generators/results/generate.py" --inputs --check
python3 "$root/sdlc/generators/results/generate.py" --bridge --check
python3 "$root/sdlc/generators/results/generate.py" --check
if [ "${THINKTHEN_TEST_PROFILE:-}" = smoke ]; then
    smoke_guard
    # The replay smoke (ticket 0335): the packed package, restored into a fresh app, over the installed C interface.
    . "$root/sdlc/scripts/installed.sh"
    scratch_dir smoke
    native_install "$root" "$smoke/native"
    mkdir "$smoke/feed" "$smoke/app" "$smoke/home"
    export DOTNET_CLI_HOME="$smoke/home" NUGET_PACKAGES="$smoke/home/nuget" DOTNET_CLI_TELEMETRY_OPTOUT=1 \
        DOTNET_SKIP_FIRST_TIME_EXPERIENCE=1 DOTNET_NOLOGO=1
    "$dotnet" pack "$here/ThinkThen.csproj" -p:ThinkThenNativeAsset="$smoke/native/lib/libthinkthen.$native_dl" -p:ThinkThenNativeRid=linux-x64 -p:ThinkThenNativeName=libthinkthen.so -c Release -o "$smoke/feed" -v quiet >&2
    cp "$here/tests/Smoke.csproj" "$here/tests/source/Smoke.cs" "$smoke/app/"
    python3 "$here/tests/package_check.py" "$smoke/native/include/thinkthen.h" "$smoke/feed/Botassembly.ThinkThen.$version.nupkg"
    "$dotnet" build "$smoke/app/Smoke.csproj" -c Release -o "$smoke/app/out" -p:RestoreSources="$smoke/feed" -v quiet >&2
    "$dotnet" "$smoke/app/out/Smoke.dll"
    exit
fi
[ "${THINKTHEN_PORTABLE_BATCH:-}" != 1 ] || [ -n "${THINKTHEN_ARTIFACT:-}" ] || {
    echo 'C# portable batch needs an installed artifact' >&2; exit 2;
}
if [ -n "${THINKTHEN_ARTIFACT:-}" ]; then
    . "$root/sdlc/scripts/scratch.sh"
    . "$root/sdlc/scripts/installed.sh"
    installed_unpack
    managed=$scratch
    version=$(sed -n 's/^version = "\(.*\)"$/\1/p' "$root/crates/thinkthen/Cargo.toml" | head -n 1)
    python3 "$here/tests/package_check.py" "$root/libraries/c/include/thinkthen.h" "$managed/Botassembly.ThinkThen.$version.nupkg"
    python3 - "$managed/Botassembly.ThinkThen.$version.nupkg" "$managed" <<'ASSEMBLY'
import sys, zipfile
with zipfile.ZipFile(sys.argv[1]) as package:
    package.extract('lib/net8.0/ThinkThen.dll', sys.argv[2])
    from pathlib import Path
    (Path(sys.argv[2]) / 'libthinkthen.so').write_bytes(package.read('runtimes/linux-x64/native/libthinkthen.so'))
ASSEMBLY
    [ -f "$managed/lib/net8.0/ThinkThen.dll" ] || exit 1
    cp -R "$here/tests/source" "$managed/source"
    cp "$here/tests/TypeCase.csproj" "$managed/TypeCase.csproj"
    "$dotnet" build "$managed/TypeCase.csproj" -c Release -o "$managed/app" \
        -p:ThinkThenAssembly="$managed/lib/net8.0/ThinkThen.dll" --source "$managed" -v quiet
    cp "$managed/libthinkthen.so" "$managed/app/libthinkthen.so"
    THINKTHEN_TYPECASE_DLL="$managed/app/TypeCase.dll" \
        python3 "$here/tests/public_types.py"
    exit 0
fi
mkdir -p "$here/target/scratch/lib" "$here/target/scratch/nuget" "$here/target/scratch/dotnet-home" "$here/target/scratch/managed" "$here/target/artifacts/native/lib" "$here/target/logs"
scratch_dir package_cache
export DOTNET_CLI_HOME="$here/target/scratch/dotnet-home" NUGET_PACKAGES="$package_cache"
export DOTNET_CLI_TELEMETRY_OPTOUT=1 DOTNET_SKIP_FIRST_TIME_EXPERIENCE=1 DOTNET_NOLOGO=1
if [ "${THINKTHEN_FULL_COMPILER:-}" != 1 ]; then
export RUSTFLAGS="${RUSTFLAGS:+$RUSTFLAGS }--remap-path-prefix=$HOME=/build" CFLAGS="${CFLAGS:+$CFLAGS }-ffile-prefix-map=$HOME=/build"
fi
native=${THINKTHEN_NATIVE_ASSET:-$root/libraries/c/target/debug/libthinkthen_c.so}
if [ -z "${THINKTHEN_NATIVE_ASSET:-}" ]; then
    RUSTC_WRAPPER= CARGO_NET_OFFLINE=true cargo build --manifest-path "$root/libraries/c/Cargo.toml" --locked --offline --lib -j2
    RUSTC_WRAPPER= CARGO_NET_OFFLINE=true cargo build --manifest-path "$root/Cargo.toml" --locked --offline --package conformance-backend -j2
fi
test -f "$native"
test -x "${THINKTHEN_BACKEND_BIN:-${CARGO_TARGET_DIR:-$root/target}/debug/conformance-backend}"
python3 "$root/sdlc/scripts/check-c-exports.py" "$root/libraries/c/include/thinkthen.h" "$native"
cp "$native" "$here/target/scratch/lib/libthinkthen.so"
ln -sf libthinkthen.so "$here/target/scratch/lib/libthinkthen.so.0"
cp "$native" "$here/target/artifacts/native/lib/libthinkthen.so"
ln -sf libthinkthen.so "$here/target/artifacts/native/lib/libthinkthen.so.0"
tar -czf "$here/target/artifacts/thinkthen-c-$version-x86_64-linux-gnu.tar.gz" -C "$here/target/artifacts/native" .
"$dotnet" pack "$here/ThinkThen.csproj" -p:ThinkThenNativeAsset="$native" -p:ThinkThenNativeRid=linux-x64 -p:ThinkThenNativeName=libthinkthen.so -c Release --source "$here/target/scratch/nuget" -o "$here/target/scratch/managed" -v quiet
test -f "$here/target/scratch/managed/Botassembly.ThinkThen.$version.nupkg"
python3 "$root/sdlc/scripts/package-inventory.py" csharp --out "$here/obj/Release" --check "$here/target/scratch/managed/Botassembly.ThinkThen.$version.nupkg" > /dev/null
# Decoder-only refusal checks stay in the generated assembly; installed callers receive typed values.
"$dotnet" build "$root/sdlc/generators/results/csharp/Conformance.csproj" -c Release \
    -p:BaseIntermediateOutputPath="$here/target/scratch/decoder-obj/" \
    --source "$here/target/scratch/nuget" -o "$here/target/scratch/decoder" -v quiet
"$dotnet" "$here/target/scratch/decoder/Conformance.dll" "$root/conformance/fixtures/complete.json"
python3 "$here/tests/package_check.py"
python3 "$here/tests/named_backends.py"
python3 "$here/tests/run_matrix.py"
run_dir=$(mktemp -d "$here/target/logs/package-XXXXXX")
python3 "$here/tests/isolated_consumer.py" alpha "$run_dir"
python3 "$here/tests/isolated_consumer.py" example "$run_dir"
python3 "$here/tests/isolated_consumer.py" example-public "$run_dir"
for function in choose score tag filter rank find annotate recognize relate; do
    python3 "$here/tests/isolated_consumer.py" "example-$function" "$run_dir"
done
python3 "$here/tests/isolated_consumer.py" sessions "$run_dir"
for state in disabled written failed; do
    python3 "$here/tests/isolated_consumer.py" "usage-$state" "$run_dir"
done
"$dotnet" build "$here/tests/TypeCase.csproj" -c Release --source "$here/target/scratch/managed" -v quiet
python3 "$here/tests/public_types.py"
echo 'C# package PASS: installed matrix, sessions, usage, J1 corpus and shared fixtures'
