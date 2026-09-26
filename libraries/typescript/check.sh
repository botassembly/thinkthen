#!/bin/sh
# The TypeScript surface (ticket 0107). The surface rung passes a loopback
# port as $1; each test starts its own backend, so this check reads none.
# It runs only the pinned Node, fetches nothing, and holds no real key.
set -eu
cd -- "$(dirname -- "$0")"
unset THINKTHEN_API_KEY
repo=$(cd ../.. && pwd)
# macOS has no `timeout` (ticket 0128).
LIMIT=$repo/sdlc/scripts/time-limit
node_home="$HOME/.cache/thinkthen-toolchains/node-v22.22.3-linux-x64"
step() { printf '== %s\n' "$*"; }
fail() { echo "typescript: $*" >&2; exit 1; }

if [ -n "${THINKTHEN_ARTIFACT:-}" ]; then
    # The installed-file mode (ticket 0128): the tarball in a fresh project, and the shared
    # cases and examples from a copy of tests/, which import the package by its name.
    [ -x "$node_home/bin/node" ] || { echo 'typescript: not run; place Node with libraries/typescript/setup-toolchain.sh'; exit 77; }
    PATH="$node_home/bin:$PATH" scratch=$(cd "$(mktemp -d)" && pwd -P)
    trap 'rm -rf -- "$scratch"' EXIT
    project=$scratch/libraries/typescript
    mkdir -p "$project" "$scratch/conformance" && cp -R tests examples.json "$project/"
    cp -R "$repo/conformance/cases.json" "$repo/conformance/children" "$scratch/conformance/"
    echo '{"private": true}' >"$project/package.json"
    (cd "$project" && npm install --offline --no-audit --no-fund --silent "$THINKTHEN_ARTIFACT")
    resolved=$(cd "$project/tests" && node --input-type=module -e 'console.log(import.meta.resolve("thinkthen"))')
    case $resolved in "file://$project/node_modules/thinkthen/"*) ;; *) fail "thinkthen resolved to $resolved, outside the fresh project" ;; esac
    export THINKTHEN_TEST_BACKEND="${CARGO_TARGET_DIR:-$repo/target}/debug/conformance-backend"
    (cd "$project" && sh "$LIMIT" 300 node --test --test-timeout=30000 tests/conformance.test.mjs tests/examples.test.mjs)
    echo 'typescript: pass, installed'
    exit 0
fi

step 'rust: format, lint, and unit tests with no Node'
cargo fmt --check
cargo clippy --quiet --locked --offline --all-targets -- -D warnings
cargo test --quiet --lib --locked --offline

step 'deny: the lock, then a git dependency meets the sources rule'
plant=$(mktemp -d)
trap 'rm -rf -- "$plant"' EXIT
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
    CARGO_HOME="$plant/home" cargo fetch --quiet --manifest-path "$plant/copy/Cargo.toml"
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
if [ ! -x "$node_home/bin/node" ]; then
    echo "typescript: not run; place Node with libraries/typescript/setup-toolchain.sh${missing:+ and install $missing}"
    exit 77
fi
PATH="$node_home/bin:$PATH"
[ "$(node --version)" = v22.22.3 ] || fail "node is $(node --version), not v22.22.3"

step 'the backend and the command'
(cd "$repo" && cargo build --quiet --locked --offline -p conformance-backend -p thinkthen)
built="${CARGO_TARGET_DIR:-$repo/target}/debug"
export THINKTHEN_TEST_BACKEND="$built/conformance-backend" THINKTHEN_TEST_COMMAND="$built/thinkthen"

step 'package: npm ci and the addon'
mkdir -p target/npm
cp package.json package-lock.json target/npm/
npm ci --offline --no-audit --no-fund --silent --prefix target/npm
sh build-addon.sh

step 'node tests'
sh "$LIMIT" 300 node --test --test-timeout=30000 tests/*.test.mjs

step 'the conformance runner fails a corrupted case and names it'
for id in 12-score-upper 17-annotate-mixed 27-decide-many; do
    node -e '
        const fs = require("node:fs");
        const file = JSON.parse(fs.readFileSync(process.argv[1], "utf8"));
        file.cases.find((one) => one.id === process.argv[2]).expect.success.answers[0].bare = "corrupted";
        fs.writeFileSync(process.argv[3], JSON.stringify(file));
    ' "$repo/conformance/cases.json" "$id" "$plant/cases.json"
    set +e
    THINKTHEN_TEST_CASES="$plant/cases.json" sh "$LIMIT" 300 node --test tests/conformance.test.mjs >"$plant/run" 2>&1
    code=$?
    set -e
    [ "$code" -ne 0 ] && grep -q "fail $id" "$plant/run" || fail "a corrupted $id passed (exit $code)"
done

step 'types'
target/npm/node_modules/.bin/tsc --noEmit --strict --module node16 --moduleResolution node16 --target es2022 tests/types.test.ts

step 'the loader refuses a platform it does not ship, with the pinned sentence'
refused=$(node -e 'Object.defineProperty(process, "platform", { value: "win32" }); try { require("./loader.js") } catch (e) { console.log(e.message) }')
[ "$refused" = "thinkthen: no native addon for win32-$(node -p process.arch); this package ships linux-x64, linux-arm64, darwin-x64, and darwin-arm64" ] ||
    fail "the loader said: $refused"

step 'the pack list, the license, and no home path'
addon="thinkthen-$(node -p 'process.platform + "-" + process.arch').node"
# release-pack takes the tarball from target/pack (ticket 0128).
mkdir -p target/pack
npm pack --json --offline --pack-destination target/pack 2>/dev/null >"$plant/pack"
packed=$(node -e 'console.log(JSON.parse(require("node:fs").readFileSync(process.argv[1], "utf8"))[0].files.map((f) => f.path).sort().join(" "))' "$plant/pack")
[ "$packed" = "LICENSE README.md index.d.ts index.js index.mjs loader.js package.json $addon" ] || fail "npm pack lists $packed"
[ "$(node -p 'require("./package.json").license')" = MIT ] || fail 'package.json names no MIT license'
[ "$(grep -c -- "$HOME" "$addon" || true)" = 0 ] || fail "$addon names $HOME"

step 'flags, pins, one guard, no unsafe, and the pinned sentence'
if grep -hE '^[^#]*cargo (build|test|clippy)' check.sh build-addon.sh | grep -vE -- '--locked.*--offline|--offline.*--locked'; then
    fail 'a cargo call above lacks --locked or --offline'
fi
if grep -hE '^[^#]*cargo (deny|fetch)' check.sh | grep -v -- '--offline' | grep -vE -- '--version|cargo fetch --quiet --manifest-path "\$plant/copy'; then
    fail 'a cargo call above lacks --offline'
fi
grep -q 'npm ci --offline' check.sh || fail 'npm ci runs without --offline'
node -e 'const d = require("./package.json").devDependencies; for (const [n, v] of Object.entries(d)) if (!/^\d+\.\d+\.\d+$/.test(v)) process.exit(1)' ||
    fail 'a dev dependency is not an exact version'
[ "$(grep -rc 'catch_unwind(' src | awk -F: '{ n += $2 } END { print n }')" = 1 ] || fail 'src holds other than one catch_unwind'
! grep -rnP '\bunsafe\b' src || fail 'src holds unsafe'
for page in README.md index.d.ts; do
    grep -q 'No deadline is spelled null, left out, or -1.' "$page" || fail "$page lacks the deadline sentence"
done
if [ -n "$missing" ]; then
    echo "typescript: not run; the steps above passed, and $missing is missing"
    exit 77
fi
echo 'typescript: pass'
