#!/bin/sh
# The TypeScript surface (ticket 0107). The surface rung passes a loopback
# port as $1; each test starts its own backend, so this check reads none.
# It runs only the pinned Node, fetches nothing, and holds no real key.
set -eu
profile=${THINKTHEN_TEST_PROFILE:-routine}
case $profile in routine|full|stress|smoke) ;; *) echo "typescript: unknown THINKTHEN_TEST_PROFILE: $profile" >&2; exit 2 ;; esac
tools_group=typescript
. "$(dirname -- "$0")/../../sdlc/scripts/resolved-tools"
cd -- "$(dirname -- "$0")"
unset THINKTHEN_API_KEY
if [ -n "${THINKTHEN_PARITY_PREPARE:-}${THINKTHEN_PARITY_PREPARED:-}" ]; then
    [ "$profile" = full ] || exit 2
fi
[ "$profile" = routine ] || unset THINKTHEN_CONFORMANCE_IDS
repo=$(cd ../.. && pwd)
if [ "$profile" = routine ]; then
    THINKTHEN_CONFORMANCE_IDS=${THINKTHEN_CONFORMANCE_IDS:-$repo/conformance/routine-ids.txt}
    export THINKTHEN_CONFORMANCE_IDS
fi
python3 "$repo/sdlc/generators/results/generate.py" --target typescript --check
. "$repo/sdlc/scripts/scratch.sh"
# ADR 0113: this run's engines write a scratch usage folder, never the real one.
usage_home
# macOS has no `timeout` (ticket 0128).
LIMIT=$repo/sdlc/scripts/time-limit
node_bin=$(command -v node || true)
step() { printf '== %s\n' "$*"; }
fail() { echo "typescript: $*" >&2; exit 1; }

public_types() {
    for extension in cts mts; do
        output=$project/target/public/$extension
        mkdir -p "$output"
        cp "$project/tests/owned_types.test.ts" "$output/owned_types.test.$extension"
        "$tsc" --strict --target ES2022 --module NodeNext --moduleResolution NodeNext --rootDir "$output" --outDir "$output" "$output/owned_types.test.$extension"
        node "$output/owned_types.test.$(printf '%s' "$extension" | sed 's/ts$/js/')"
    done
}

# Full qualification installs one real package, then runs isolated family members.
if [ "$profile" = full ] && [ -z "${THINKTHEN_ARTIFACT:-}" ] && [ -z "${THINKTHEN_PARITY_PREPARED:-}" ]; then
    [ -x "$node_bin" ] || exit 77
    sh build-addon.sh
    # Keep the exact installed input for release-pack --reuse after scratch cleanup.
    mkdir -p target/pack
    npm pack --offline --pack-destination target/pack >/dev/null
    version=$(node -p 'require("./package.json").version')
    THINKTHEN_ARTIFACT=$PWD/target/pack/thinkthen-$version.tgz
    [ -f "$THINKTHEN_ARTIFACT" ] || fail 'npm pack did not retain the release package'
fi
if [ -n "${THINKTHEN_ARTIFACT:-}" ] || [ -n "${THINKTHEN_PARITY_PREPARED:-}" ]; then
    # The installed-file mode (ticket 0128): the tarball in a fresh project, and the shared
    # cases and examples from a copy of tests/, which import the package by its name.
    case $(uname -s):$(uname -m) in
        Linux:x86_64) expected=linux-x64 ;;
        Linux:aarch64) expected=linux-arm64 ;;
        Darwin:x86_64) expected=darwin-x64 ;;
        Darwin:arm64) expected=darwin-arm64 ;;
        *) echo 'typescript: not run; no native Node target for this host'; exit 77 ;;
    esac
    command -v node >/dev/null 2>&1 || { echo 'typescript: not run; no native Node 22.22.3'; exit 77; }
    [ "$(node --version)" = v22.22.3 ] || fail "node is $(node --version), not v22.22.3"
    [ "$(node -p 'process.platform + "-" + process.arch')" = "$expected" ] || fail "Node is not native $expected"
    if [ -n "${THINKTHEN_PARITY_PREPARED:-}" ]; then
        scratch_dir project
        cp -R "$THINKTHEN_PARITY_PREPARED/tests" "$project/tests"
        ln -s "$THINKTHEN_PARITY_PREPARED/node_modules" "$project/node_modules"
        echo '{"private":true}' >"$project/package.json"
        tsc=$THINKTHEN_PARITY_PREPARED/target/compiler/node_modules/.bin/tsc
    else
        . "$repo/sdlc/scripts/installed.sh"
        installed_tests "$repo" libraries/typescript
        project=$scratch/libraries/typescript
        echo '{"private": true}' >"$project/package.json"
        (cd "$project" && npm install --offline --no-audit --no-fund --silent "$THINKTHEN_ARTIFACT")
        python3 "$repo/sdlc/scripts/package-inventory.py" npm --platform "$(node -p 'process.platform + "-" + process.arch')" --check "$project/node_modules/thinkthen" >/dev/null
        mkdir -p "$project/target/compiler"
        cp "$repo/libraries/typescript/package.json" "$repo/libraries/typescript/package-lock.json" "$project/target/compiler/"
        npm ci --offline --no-audit --no-fund --silent --prefix "$project/target/compiler"
        tsc=$project/target/compiler/node_modules/.bin/tsc
    fi
    resolved=$(cd "$project/tests" && node --input-type=module -e 'console.log(import.meta.resolve("thinkthen"))')
    package_node=${THINKTHEN_PARITY_PREPARED:-$project}/node_modules/thinkthen
    case $resolved in "file://$package_node/"*) ;; *) fail "thinkthen resolved to $resolved, outside the fresh project" ;; esac
    export THINKTHEN_TEST_BACKEND="${CARGO_TARGET_DIR:-$repo/target}/debug/conformance-backend"
    if [ -n "${THINKTHEN_COVERAGE_OBJECT:-}" ]; then
        cp "$package_node"/*.node "$THINKTHEN_COVERAGE_OBJECT"
    fi
    if [ -z "${THINKTHEN_PARITY_PREPARED:-}" ]; then
    public_types
    if [ "${THINKTHEN_PARITY_CONSUMER:-javascript}" = javascript ]; then
    (cd "$project" && sh "$LIMIT" 300 node --test --test-timeout=30000 tests/owned_session.test.mjs tests/examples.test.mjs)
    fi
    if [ "${THINKTHEN_PARITY_CONSUMER:-typescript}" = typescript ]; then
    (cd "$project" && THINKTHEN_OWNED_MODULE=cjs node --test --test-timeout=30000 tests/owned_session.test.mjs tests/examples.test.mjs)
    fi
    fi
    if [ -n "${THINKTHEN_PARITY_PREPARE:-}" ]; then
        cp -a "$project/." "$THINKTHEN_PARITY_PREPARE/"
        exit 0
    fi
    python3 - "$repo" "$project" "$tsc" <<'NODENATIVE'
import os, sys
from pathlib import Path
root=Path(sys.argv[1]);project=Path(sys.argv[2]);tsc=Path(sys.argv[3]);sys.path.insert(0,str(root/'conformance/children'))
from native_fixture import run
for language,suffix in [('javascript','mjs'),('typescript','js')]:
    if os.environ.get('THINKTHEN_PARITY_CONSUMER',language)!=language:continue
    if run(language,['node',str(project/'tests'/('native_case.'+suffix))],root,extra_env={'THINKTHEN_OWNED_MODULE':'cjs' if language=='typescript' else 'esm'},typescript_compiler=tsc):sys.exit(1)
NODENATIVE
    echo 'typescript: pass, installed'
    exit 0
fi

if [ "$profile" = smoke ]; then
    smoke_guard
    # The replay smoke (ticket 0335): the package laid out in a fresh project's node_modules,
    # required by its name from that project.
    [ -x "$node_bin" ] || { echo 'typescript: not run; place Node with libraries/typescript/setup-toolchain.sh'; exit 77; }
    cargo build --quiet --locked --offline
    case $(uname -s) in Darwin) library=libthinkthen_typescript.dylib ;; *) library=libthinkthen_typescript.so ;; esac
    scratch_dir project
    mkdir -p "$project/node_modules/thinkthen"
    for file in $(python3 "$repo/sdlc/scripts/package-inventory.py" npm --field files); do
        case $file in *.node) continue ;; esac
        cp "$file" "$project/node_modules/thinkthen/"
    done
    cp -- "${CARGO_TARGET_DIR:-target}/debug/$library" \
        "$project/node_modules/thinkthen/thinkthen-$(node -p 'process.platform + "-" + process.arch').node"
    cd "$project"
    THINKTHEN_API_KEY=sk-smoke-loopback node -e '
        const tt = require("thinkthen");
        if (!require.resolve("thinkthen").startsWith(process.argv[1])) throw new Error("thinkthen loaded from outside the project");
        new tt.Client().decide(process.env.THINKTHEN_TEST_SMOKE_QUESTION, process.env.THINKTHEN_TEST_SMOKE_TEXT)
            .then((call) => {
                const value = call.results[0].value;
                if (typeof value !== "boolean") throw new Error("expected a boolean");
                console.log(`smoke: ${value}`);
            });
    ' "$project/node_modules/"
    exit
fi

step 'rust: format, lint, and unit tests with no Node'
cargo fmt --check
cargo clippy --quiet --locked --offline --all-targets -- -D warnings
cargo test --quiet --lib --locked --offline

step 'deny: the lock, then a git dependency meets the sources rule'
scratch_dir plant
missing=
if cargo deny --version >/dev/null 2>&1; then
    cargo deny --offline --manifest-path Cargo.toml check --config "$repo/deny.toml" advisories bans licenses sources
    # A file:// git source under a scratch CARGO_HOME: no network and no residue.
    mkdir -p "$plant/dep/src" "$plant/copy/src"
    printf '[package]\nname = "planted"\nversion = "0.0.1"\nedition = "2024"\nlicense = "MIT"\n' >"$plant/dep/Cargo.toml"
    : >"$plant/dep/src/lib.rs"
    : >"$plant/copy/src/lib.rs"
    git -C "$plant/dep" init -q && git -C "$plant/dep" add -A
    git -C "$plant/dep" -c user.name=plant -c user.email=plant@example.invalid commit -qm plant
    printf '[package]\nname = "binding"\nversion = "0.0.1"\nedition = "2024"\nlicense = "MIT"\n[dependencies]\nplanted = { git = "file://%s/dep" }\n' "$plant" >"$plant/copy/Cargo.toml"
    CARGO_NET_OFFLINE=false CARGO_HOME="$plant/home" cargo fetch --quiet --manifest-path "$plant/copy/Cargo.toml"
    set +e
    # cargo-deny exits with bit 8 set when the sources check fails.
    CARGO_HOME="$plant/home" cargo deny --offline --manifest-path "$plant/copy/Cargo.toml" check --config "$repo/deny.toml" sources >"$plant/deny" 2>&1
    code=$?
    set -e
    [ "$code" -eq 8 ] && grep -q source-not-allowed "$plant/deny" || fail "deny passed a git source (exit $code)"
else
    missing='cargo-deny'
fi

step 'toolchain'
if [ ! -x "$node_bin" ]; then
    echo "typescript: not run; place Node with libraries/typescript/setup-toolchain.sh${missing:+ and install $missing}"
    exit 77
fi
[ "$(node --version)" = v22.22.3 ] || fail "node is $(node --version), not v22.22.3"

step 'the backend and the command'
(cd "$repo" && cargo build --quiet --locked --offline -p conformance-backend -p thinkthen)
built="${CARGO_TARGET_DIR:-$repo/target}/debug"
export THINKTHEN_TEST_BACKEND="$built/conformance-backend" THINKTHEN_TEST_COMMAND="$built/thinkthen"

step 'package: npm ci and the addon'
mkdir -p target/npm
cp package.json package-lock.json target/npm/
npm ci --offline --no-audit --no-fund --silent --prefix target/npm
tsc=$PWD/target/npm/node_modules/.bin/tsc
sh build-addon.sh

step 'node tests'
if [ "$profile" = stress ]; then
    sh "$LIMIT" 300 node --test --test-timeout=60000 --test-name-pattern='^stress:' tests/*.test.mjs
    echo 'typescript: pass, stress'
    exit 0
fi
sh "$LIMIT" 300 node --test --test-timeout=60000 --test-skip-pattern='^stress:' tests/*.test.mjs

step 'native JavaScript and compiled TypeScript consumers'
project=$PWD
public_types

    python3 - "$repo" "$project" "$tsc" <<'NODENATIVE'
import sys
from pathlib import Path
root=Path(sys.argv[1]);project=Path(sys.argv[2]);tsc=Path(sys.argv[3]);sys.path.insert(0,str(root/'conformance/children'))
from native_fixture import run
for language,suffix in [('javascript','mjs'),('typescript','js')]:
    if run(language,['node',str(project/'tests'/('native_case.'+suffix))],root,extra_env={'THINKTHEN_OWNED_MODULE':'cjs' if language=='typescript' else 'esm'},typescript_compiler=tsc):sys.exit(1)
NODENATIVE
step 'the loader refuses a platform it does not ship, with the pinned sentence'
refused=$(node -e 'Object.defineProperty(process, "platform", { value: "freebsd" }); try { require("./loader.js") } catch (e) { console.log(e.message) }')
[ "$refused" = "thinkthen: no native addon for freebsd-$(node -p process.arch); this package ships $(node -p 'Object.keys(require("./native-platforms.json")).join(", ")')" ] ||
    fail "the loader said: $refused"

step 'the pack list, the license, and no home path'
addon="thinkthen-$(node -p 'process.platform + "-" + process.arch').node"
# release-pack takes the tarball from target/pack (ticket 0128).
mkdir -p target/pack
npm pack --json --offline --pack-destination target/pack 2>/dev/null >"$plant/pack"
packed=$(node -e 'console.log(JSON.parse(require("node:fs").readFileSync(process.argv[1], "utf8"))[0].files.map((f) => f.path).sort().join(" "))' "$plant/pack")
expected_pack=$(python3 "$repo/sdlc/scripts/package-inventory.py" npm --platform "$(node -p 'process.platform + "-" + process.arch')" --field files | tr '\n' ' ' | sed 's/ $//')
[ "$packed" = "$expected_pack" ] || fail "npm pack lists $packed"
[ "$(node -p 'require("./package.json").license')" = MIT ] || fail 'package.json names no MIT license'
[ "$(grep -c -- "$HOME" "$addon" || true)" = 0 ] || fail "$addon names $HOME"

step 'flags, pins, one guard, and no unsafe'
node -e 'const d = require("./package.json").devDependencies; for (const [n, v] of Object.entries(d)) if (!/^\d+\.\d+\.\d+$/.test(v)) process.exit(1)' ||
    fail 'a dev dependency is not an exact version'
[ "$(grep -rc 'catch_unwind(' src | awk -F: '{ n += $2 } END { print n }')" = 0 ] || fail 'src catches panics outside thinkthen::contained'
! grep -rnP '\bunsafe\b' src || fail 'src holds unsafe'
if [ -n "$missing" ]; then
    echo "typescript: not run; the steps above passed, and $missing is missing"
    exit 77
fi
echo 'typescript: pass'
