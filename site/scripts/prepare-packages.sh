#!/bin/sh
# Stage the language packages and database extensions the site's examples load.
# Run through pm job from the source tree. Everything lands in target/site-packages.
# Shared native inputs use the ordinary development profile from this same tree.
# Prepare the pinned DuckDB prerequisites with scripts/duckdb-prepare first.
#
#   cargo fetch --locked               # every crate of the workspace lock, for the R package
#
# Host tools: python3, ruby with the ffi gem, go, cc, c++, cmake, patchelf, pkg-config,
# node, sqlite3 with libsqlite3 and libjansson headers, Maven, the .NET 10 SDK, and R 4.2.0 or
# later with jsonlite, vctrs, rlang, cli and tibble, and PostgreSQL 18 with its server headers
# (on Ubuntu, postgresql-18 and postgresql-server-dev-18 from apt.postgresql.org; set PG_CONFIG
# when the PostgreSQL 18 pg_config is not /usr/lib/postgresql/18/bin/pg_config). In $THINKTHEN_BUILD_TOOLS (default
# ~/.local/share/thinkthen-build-tools): jvm/jdk-22.0.2+9, swift/swift-6.3-RELEASE-ubuntu24.04
# and zig/zig-x86_64-linux-0.17.0. Maven and NuGet download dependencies, so this needs network.
set -eu
export PYTHONDONTWRITEBYTECODE=1
cd "$(dirname "$0")/../.."
root=$PWD
scratch=$(mktemp -d)
trap 'status=$?; trap - EXIT; set +e; rm -rf "$scratch"; exit "$status"' EXIT
trap 'exit 1' HUP INT TERM
build=${CARGO_TARGET_DIR:-$root/target}
case "$build" in /*) ;; *) build=$root/$build ;; esac
native=$build/debug
stage_sqlite() {
    mkdir -p "$1"
    THINKTHEN_SQL_TEST_ARCHIVE= THINKTHEN_SQL_ARCHIVE="$native/libthinkthen_ffi.a" \
        scripts/sqlite-package "$scratch/sqlite-built"
    cp "$scratch/sqlite-built/package/thinkthen_sqlite.so" "$1/"
}
stage_r() {
    mkdir -p "$1" "$scratch/r-installed"
    python3 -B scripts/r-package --source "$scratch/r"
    TMPDIR="$scratch" scripts/runtime-floor-build scripts/native-tools-env env \
        R_LIBS_USER="$THINKTHEN_RUNTIME_PREFIX/r-frames/library" THINKTHEN_R_SOURCE_NATIVE=1 \
        R CMD INSTALL --no-test-load -l "$scratch/r-installed" "$scratch/r"
    cp -R "$scratch/r-installed/." "$1/"
}
case "${1:-}" in ''|--check|--check-r) ;; *) echo 'Use prepare-packages.sh [--check|--check-r]' >&2; exit 2 ;; esac
if [ "${1:-}" = --check-r ]; then
    out=${SITE_PACKAGES:-$root/target/site-packages}
    test -x "$out/bin/Rscript" || { echo 'Provide the preserved copied R runtime for this check' >&2; exit 2; }
    stage_r "$scratch/r-library"
    cat > "$scratch/installed.R" <<'RCHECK'
library(thinkthen)
is_refund <- tt_decide("Please refund my order.",
                       question = "Does the customer request a refund?",
                       client = tt_client(backend = "stub"))
stopifnot(is.logical(is_refund), length(is_refund) == 1L, !is.na(is_refund))
print(is_refund)
RCHECK
    R_LIBS="$scratch/r-library" R_LIBS_USER="$THINKTHEN_RUNTIME_PREFIX/r-frames/library" \
        "$out/bin/Rscript" "$scratch/installed.R"
    exit
fi
scripts/source-linux-tools
PATH="$root/target/source-tools/bin:$PATH"
export PATH
if [ "${1:-}" = --check ]; then
    out=${SITE_PACKAGES:-$root/target/site-packages}
    tools=${THINKTHEN_BUILD_TOOLS:-$HOME/.local/share/thinkthen-build-tools}
    PATH="$out/bin:$PATH"
    LD_LIBRARY_PATH="$out/swift-libs${LD_LIBRARY_PATH:+:$LD_LIBRARY_PATH}"
    export PATH LD_LIBRARY_PATH
    status=0
    for tool in cargo rustc python3 uv ruby gem go cc c++ cmake patchelf pkg-config node mvn dotnet make docker git tar R Rscript; do
        command -v "$tool" || status=1
    done
    for executable in "$tools/jvm/jdk-22.0.2+9/bin/java" \
        "$tools/swift/6.3.2-ubuntu22.04/usr/bin/swift" \
        "$tools/zig/0.17.0/zig" \
        "$out/postgres/runtime/bin/pg_config" "$THINKTHEN_RUNTIME_PREFIX/bin/sqlite3"; do
        test -x "$executable" || { echo "Missing prepared executable: $executable" >&2; status=1; }
    done
    patchelf --version || status=1
    "$tools/swift/6.3.2-ubuntu22.04/usr/bin/swift" --version || status=1
    "$out/postgres/runtime/bin/pg_config" --version || status=1
    R --version || status=1
    pkg-config --exists jansson || status=1
    python3 scripts/release_images.py floor || status=1
    python3 scripts/release_images.py swift || status=1
    uv python find --offline 3.10 || status=1
    test -f "$tools/python-frames/cp310/requirements.txt" || status=1
    test -d "$THINKTHEN_RUNTIME_PREFIX/r-frames/library" || status=1
    for gem in "$tools/ruby/packages/"ffi-*.gem; do test -f "$gem" || status=1; done
    stage_sqlite "$scratch/sqlite" || status=1
    "$THINKTHEN_RUNTIME_PREFIX/bin/sqlite3" ':memory:' ".load $scratch/sqlite/thinkthen_sqlite.so" 'select 1;' || status=1
    exit "$status"
fi
cargo build --locked -p thinkthen --bin thinkthen --lib --features stream
cargo build --locked -p thinkthen-ffi -p thinkthen-python -p thinkthen-node
out=${SITE_PACKAGES:-$root/target/site-packages}
case "$out" in /*) ;; *) out=$root/$out ;; esac
mkdir -p "$out/bin"
cp "$native/thinkthen" "$out/bin/"
tools=${THINKTHEN_BUILD_TOOLS:-$HOME/.local/share/thinkthen-build-tools}
version=$(sed -n 's/^version = "\(.*\)"$/\1/p' Cargo.toml | head -n 1)

# Retain the already prepared native runtime closure, as preview preparation does.
python3 - "$out" <<'RUNTIMES'
from pathlib import Path
import subprocess
import sys
sys.path.insert(0, 'scripts')
from build_targets import container_labels
from release_images import image
out = Path(sys.argv[1])
for name, source, destination in [('floor', '/opt/R', 'R-runtime'),
                                  ('floor', '/opt/thinkthen-pg18', 'postgres/runtime')]:
    container = subprocess.check_output(['docker', 'create', *container_labels(Path.cwd()), image(name)], text=True).strip()
    try:
        (out / destination).parent.mkdir(parents=True, exist_ok=True)
        subprocess.run(['docker', 'cp', container + ':' + source, str(out / destination)], check=True)
        if destination == 'R-runtime':
            libraries = out / destination / 'lib64/R/lib'
            for library in ('libreadline.so.7', 'libicuuc.so.60', 'libicui18n.so.60', 'libicudata.so.60'):
                subprocess.run(['docker', 'cp', '-L', container + ':/usr/lib64/' + library, str(libraries / library)], check=True)
    finally:
        primary_failed = sys.exc_info()[0] is not None
        try:
            subprocess.run(['docker', 'rm', container], check=True, stdout=subprocess.DEVNULL)
        except subprocess.CalledProcessError as cleanup_error:
            if not primary_failed:
                raise
            print(f'Container cleanup also failed: {cleanup_error}', file=sys.stderr)
r_home = out / 'R-runtime/lib64/R'
setup = '#!/bin/sh\nexport R_HOME="' + str(r_home) + '"\nexport LD_LIBRARY_PATH="' + str(r_home / 'lib') + '${LD_LIBRARY_PATH:+:$LD_LIBRARY_PATH}"\n'
(out / 'bin/R').write_text(setup + 'if [ "${1:-}" = CMD ]; then shift; exec "$R_HOME/bin/Rcmd" "$@"; fi\nexec "$R_HOME/bin/exec/R" "$@"\n')
(out / 'bin/Rscript').write_text(setup + 'script=$1\nshift\nexec "$R_HOME/bin/exec/R" --no-echo --no-restore --file="$script" --args "$@"\n')
for binary in ('R', 'Rscript'):
    (out / 'bin' / binary).chmod(0o755)
libraries = out / 'swift-libs'
libraries.mkdir()
container = subprocess.check_output(['docker', 'create', *container_labels(Path.cwd()), image('swift')], text=True).strip()
try:
    for library in ('libxml2.so.2', 'libicuuc.so.70', 'libicui18n.so.70', 'libicudata.so.70'):
        subprocess.run(['docker', 'cp', '-L', container + ':/lib/x86_64-linux-gnu/' + library, str(libraries / library)], check=True)
finally:
    primary_failed = sys.exc_info()[0] is not None
    try:
        subprocess.run(['docker', 'rm', container], check=True, stdout=subprocess.DEVNULL)
    except subprocess.CalledProcessError as cleanup_error:
        if not primary_failed:
            raise
        print(f'Container cleanup also failed: {cleanup_error}', file=sys.stderr)
RUNTIMES
PATH="$out/bin:$PATH"
LD_LIBRARY_PATH="$out/swift-libs${LD_LIBRARY_PATH:+:$LD_LIBRARY_PATH}"
export PATH LD_LIBRARY_PATH
pg_config=$out/postgres/runtime/bin/pg_config

# Python: the package source with the native module beside it.
rm -rf "$out/python"
mkdir -p "$out/python"
cp -R sdk/python/thinkthen "$out/python/"
cp "$native/lib_native.so" "$out/python/thinkthen/_native.abi3.so"
python3 -B -I -c 'import sys; from pathlib import Path; sys.path.insert(0, "sdk/python"); from relation_type import relation_type; relation_type(Path(sys.argv[1]))' "$out/python/thinkthen"
python=$(uv python find --offline 3.10)
uv pip install --python "$python" --target "$out/python" --no-index \
    --find-links "$tools/python-frames/cp310/wheels" -r "$tools/python-frames/cp310/requirements.txt"

# C and C++: one prefix with the engine, headers and pkg-config file.
rm -rf "$out/c"
mkdir -p "$scratch/engine"
cp "$native/libthinkthen_ffi.so" "$scratch/engine/"
cmake -S sdk/c -B "$scratch/c" -DCMAKE_BUILD_TYPE=Release -DCMAKE_INSTALL_LIBDIR=lib \
    -DCMAKE_INSTALL_PREFIX="$out/c" -DTHINKTHEN_FFI_LIBRARY="$scratch/engine/libthinkthen_ffi.so" \
    -DTHINKTHEN_FFI_INCLUDE_DIR="$root/crates/thinkthen-ffi/include" >/dev/null
cmake --build "$scratch/c" --parallel 2 >/dev/null
cmake --install "$scratch/c" >/dev/null
cmake -S sdk/cpp -B "$scratch/cpp" -DCMAKE_BUILD_TYPE=Release -DCMAKE_INSTALL_LIBDIR=lib \
    -DCMAKE_INSTALL_PREFIX="$out/c" -DCMAKE_PREFIX_PATH="$out/c" >/dev/null
cmake --build "$scratch/cpp" --parallel 2 >/dev/null
cmake --install "$scratch/cpp" >/dev/null

# Ruby: the gem's lib folder with the engine beside native.rb.
rm -rf "$out/ruby"
mkdir -p "$out/ruby"
cp -R sdk/ruby/lib "$out/ruby/"
cp "$native/libthinkthen_ffi.so" "$out/ruby/lib/thinkthen/"
for gem in "$tools/ruby/packages/"ffi-*.gem; do
    GEM_HOME="$out/ruby-gems" GEM_PATH="$out/ruby-gems" gem install "$gem" --local --no-document --install-dir "$out/ruby-gems"
done

# TypeScript: the npm package folder with its generated error list and native module.
rm -rf "$out/node"
mkdir -p "$out/node/thinkthen"
python3 -B scripts/typescript-prepare
python3 - "$out/node/thinkthen" <<'NODEFILES'
import json
from pathlib import Path
import shutil
import sys
source = Path('sdk/typescript')
for name in ['package.json', *json.loads((source / 'package.json').read_text())['files']]:
    if name.endswith('.node'):
        continue
    origin = Path('LICENSE') if name == 'LICENSE' else source / name
    shutil.copy2(origin, Path(sys.argv[1]) / name)
NODEFILES
cp "$native/libthinkthen_node.so" "$out/node/thinkthen/thinkthen.node"
# The release's own step writes errors.js and errors.d.ts, which git ignores, beside the package source.
python3 -B scripts/typescript-prepare
cp sdk/typescript/errors.js sdk/typescript/errors.d.ts "$out/node/thinkthen/"

# DuckDB: use the version and prerequisites owned by its source preparation.
deps=$HOME/.local/share/thinkthen-build-tools/duckdb/$(python3 scripts/duckdb_release.py --version)
rm -rf "$out/duckdb"
THINKTHEN_DUCKDB_DEPS=$deps THINKTHEN_SQL_ARCHIVE=$native/libthinkthen_ffi.a \
    THINKTHEN_DUCKDB_OUTPUT=$out/duckdb scripts/duckdb-build >/dev/null
cp "$deps/cli/duckdb" "$out/bin/duckdb"
cp "$THINKTHEN_RUNTIME_PREFIX/bin/sqlite3" "$out/bin/"

# SQLite: the extension compiled against the host's SQLite headers and jansson.
rm -rf "$out/sqlite"
mkdir -p "$out/sqlite"
stage_sqlite "$out/sqlite"

# PostgreSQL 18: the extension built with PGXS, in a lib and share/extension prefix that a
# server reads through dynamic_library_path and extension_control_path, and the server's bindir.
pg_config=${PG_CONFIG:-$pg_config}
mkdir -p "$out/postgres/lib" "$out/postgres/share/extension" "$scratch/postgres"
cp extensions/postgres/*.c extensions/postgres/*.h extensions/postgres/Makefile \
    extensions/postgres/*.control extensions/postgres/*.sql "$scratch/postgres/"
python3 -B -I extensions/postgres/generate.py schemas/contract.json "$scratch/postgres/thinkthen--$version.sql"
make -s -C "$scratch/postgres" PG_CONFIG="$pg_config" PG_CPPFLAGS="-I$root/crates/thinkthen-ffi/include" \
    THINKTHEN_ARCHIVE="$native/libthinkthen_ffi.a" >/dev/null
cp "$scratch/postgres/thinkthen_postgres.so" "$out/postgres/lib/"
cp "$scratch/postgres/thinkthen.control" "$scratch/postgres/thinkthen--$version.sql" "$out/postgres/share/extension/"
"$pg_config" --bindir > "$out/postgres/bindir"

# Java and Kotlin: both jars with the engine inside, their runtime jars, and a Kotlin compiler.
jdk=$tools/jvm/jdk-22.0.2+9
test -x "$jdk/bin/java" || { echo "prepare-packages: no JDK 22 at $jdk" >&2; exit 1; }
rm -rf "$out/jvm"
mkdir -p "$out/jvm/lib" "$scratch/kotlinc"
cp -R sdk/java "$scratch/java"
cp -R sdk/kotlin "$scratch/kotlin"
rm -rf "$scratch/java/library/target" "$scratch/kotlin/target"
python3 -B scripts/jvm-generated "$scratch/java"
mkdir -p "$scratch/java/library/target/classes/io/thinkthen/native/linux-amd64"
cp "$native/libthinkthen_ffi.so" "$scratch/java/library/target/classes/io/thinkthen/native/linux-amd64/"
m2=$root/target/site-m2
JAVA_HOME=$jdk mvn -B -q -f "$scratch/java/pom.xml" -Dmaven.repo.local="$m2" install -DskipTests
JAVA_HOME=$jdk mvn -B -q -f "$scratch/kotlin/pom.xml" -Dmaven.repo.local="$m2" \
    dependency:copy-dependencies -DincludeScope=runtime -DoutputDirectory="$out/jvm/lib"
cp "$scratch/kotlin/target/thinkthen-kotlin-$version.jar" "$out/jvm/lib/"
kotlin=$(sed -n 's:.*<kotlin.version>\([^<]*\)</kotlin.version>.*:\1:p' sdk/java/pom.xml)
printf '%s\n' '<project xmlns="http://maven.apache.org/POM/4.0.0"><modelVersion>4.0.0</modelVersion>' \
    '<groupId>site</groupId><artifactId>kotlinc</artifactId><version>1</version><dependencies><dependency>' \
    "<groupId>org.jetbrains.kotlin</groupId><artifactId>kotlin-compiler-embeddable</artifactId><version>$kotlin</version>" \
    '</dependency></dependencies></project>' > "$scratch/kotlinc/pom.xml"
JAVA_HOME=$jdk mvn -B -q -f "$scratch/kotlinc/pom.xml" -Dmaven.repo.local="$m2" \
    dependency:copy-dependencies -DoutputDirectory="$out/jvm/kotlinc"

# C#: a local NuGet feed with the package, and every package the example restores.
export DOTNET_CLI_TELEMETRY_OPTOUT=1 DOTNET_NOLOGO=1 DOTNET_SKIP_FIRST_TIME_EXPERIENCE=1
rm -rf "$out/csharp"
cp -R sdk/csharp "$scratch/csharp"
rm -rf "$scratch/csharp/bin" "$scratch/csharp/obj"
NUGET_PACKAGES="$out/csharp/packages" dotnet pack "$scratch/csharp/ThinkThen.csproj" -c Release \
    -o "$out/csharp/feed" -p:LinuxNative="$native/libthinkthen_ffi.so" >/dev/null
mkdir -p "$scratch/csharp-example"
cp site/examples/lang-csharp/example.csproj site/examples/lang-csharp/show.cs "$scratch/csharp-example/"
printf '%s\n' "<configuration><packageSources><clear /><add key=\"site\" value=\"$out/csharp/feed\" />" \
    '<add key="nuget" value="https://api.nuget.org/v3/index.json" /></packageSources></configuration>' \
    > "$scratch/csharp-example/nuget.config"
NUGET_PACKAGES="$out/csharp/packages" dotnet restore "$scratch/csharp-example/example.csproj" >/dev/null

# R: the source package, built and installed into a library folder.
rm -rf "$out/r"
mkdir -p "$out/r"
cp -R "$THINKTHEN_RUNTIME_PREFIX/r-frames/library" "$out/r-deps"
stage_r "$out/r"

# Swift: the source package with the engine in its resources, in a folder named for its package identity.
rm -rf "$out/swift"
mkdir -p "$scratch/swift-native/linux-x64"
cp "$native/libthinkthen_ffi.so" "$scratch/swift-native/linux-x64/"
THINKTHEN_SWIFT_NATIVE_ASSETS="$scratch/swift-native" sh scripts/swift-package "$out/swift/thinkthen" >/dev/null
swift=$tools/swift/6.3.2-ubuntu22.04/usr/bin/swift
"$swift" build --package-path "$out/swift/thinkthen" -c debug --jobs "${CARGO_BUILD_JOBS:-2}"
"$swift" build --package-path "$out/swift/thinkthen" -c debug --show-bin-path > "$out/swift/module-path"
printf '%s\n' "${swift%/*}/swiftc" > "$out/swift/compiler-path"

# Rust consumers link this one built library and its matching dependency set.
mkdir -p "$out/rust"
cp "$native/libthinkthen.rlib" "$out/rust/"
cp -R "$native/deps" "$out/rust/deps"

# Zig: the package archive that `zig fetch` reads.
rm -rf "$out/zig"
THINKTHEN_ZIG_NATIVE_FILE="$native/libthinkthen_ffi.so" python3 -B scripts/zig-package "$scratch/zig" >/dev/null
mkdir -p "$out/zig"
cp "$scratch/zig/thinkthen-zig-linux-x64.tar.gz" "$out/zig/"

ln -s "$python" "$out/bin/python3"
# Record the runtimes actually selected by this staging run. The execution
# bundle consumes these paths instead of maintaining another version map.
python3 - "$out" "$python" "$jdk" "$swift" <<'RUNTIME_ROOTS'
import json
from pathlib import Path
import shutil
import sys
out, python, jdk, swift = map(Path, sys.argv[1:])
roots = {
    'python': {'executable': str(python.resolve()), 'root': str(python.resolve().parent.parent)},
    'java': {'executable': str(jdk / 'bin/java'), 'root': str(jdk)},
    'swift': {'root': str(swift.parent.parent / 'lib/swift/linux')},
}
for name in ('ruby', 'dotnet', 'node'):
    selected = shutil.which(name)
    if selected is None:
        raise SystemExit(f'Missing staged runtime command: {name}')
    executable = Path(selected).resolve()
    root = executable.parent if name == 'dotnet' else executable.parent.parent
    roots[name] = {'executable': str(executable), 'root': str(root)}
for name, value in roots.items():
    if not Path(value['root']).is_dir():
        raise SystemExit(f'Missing selected {name} runtime root: {value["root"]}')
(out / 'runtime-roots.json').write_text(json.dumps(roots, indent=2, sort_keys=True) + '\n')
RUNTIME_ROOTS
git rev-parse HEAD > "$out/source-commit"

echo "staged $out"
