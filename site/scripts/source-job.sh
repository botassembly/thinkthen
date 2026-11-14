#!/bin/sh
# Build source inputs and run docs consumers in one job, from the certified tree.
set -eu
cd "$(dirname "$0")/../.."
mode=${1:-routine}
shift || true
case "$mode" in routine|capture|replay|reference|stage|stage-check|stage-check-r|prepare-examples|bundle-runtimes|guide-check) ;; *) echo 'Use routine, capture, replay, reference, stage, stage-check, stage-check-r, prepare-examples or bundle-runtimes or guide-check, then arguments' >&2; exit 2 ;; esac
if [ -n "${SITE_RUNTIME_BUNDLE:-}" ]; then
  case "$mode" in capture|replay|routine)
    exec python3 -B site/scripts/runtime-examples.py "$mode" "$@" ;;
    *) echo 'A runtime bundle executes routine, capture or replay only' >&2; exit 2 ;;
  esac
fi
. scripts/native-tools-env
if [ "$mode" = prepare-examples ]; then
  prepared_out=$1; shift
  case "$prepared_out" in /*) ;; *) echo 'Use an absolute owned example bundle path' >&2; exit 2 ;; esac
  test ! -e "$prepared_out" && test "$#" -gt 0
  test "${THINKTHEN_BACKEND:-systemone}" != stub
  test -f "${SITE_PACKAGES:-$PWD/target/site-packages}/source-commit"
fi
if [ "$mode" = bundle-runtimes ]; then
  bundle_out=$1; shift
  test "$#" -eq 0
fi
packages_selected=${SITE_PACKAGES:+1}
SITE_PACKAGES=${SITE_PACKAGES:-$PWD/target/site-packages}
case "$SITE_PACKAGES" in /*) ;; *) SITE_PACKAGES=$PWD/$SITE_PACKAGES ;; esac
export SITE_PACKAGES
. site/scripts/node-env.sh
if [ "$mode" = stage-check ]; then exec sh site/scripts/prepare-packages.sh --check; fi
if [ "$mode" = stage-check-r ]; then exec sh site/scripts/prepare-packages.sh --check-r; fi
if [ "$mode" = stage ]; then
  if [ "$SITE_PACKAGES" = "$PWD/target/site-packages" ]; then
    rm -rf "$SITE_PACKAGES"
  else
    test ! -e "$SITE_PACKAGES" || { echo 'Use a fresh owned package prefix' >&2; exit 1; }
  fi
  exec sh site/scripts/prepare-packages.sh
fi
case "$mode" in routine|capture|replay|prepare-examples|bundle-runtimes)
  if [ ! -f "$SITE_PACKAGES/source-commit" ] && {
    [ -e "$SITE_PACKAGES" ] || [ -L "$SITE_PACKAGES" ] || {
      [ "$packages_selected" = 1 ] && [ "$SITE_PACKAGES" != "$PWD/target/site-packages" ];
    };
  }; then
    echo 'Selected site-packages is incomplete: source-commit is required' >&2
    exit 1
  fi
  ;;
esac
# The routine development profile shares normal Cargo compiler caches.
# release_install.py only writes distribution install notes/checksums; the
# site stage never imports it or calls release_workflow.collect(), its consumer.
# The C++ install checker is not read by sdk/cpp/CMakeLists.txt when staging.
# sqlite-package compiles named src/*.c files; this installed-host test is only
# invoked by SQLite test gates, never by the site stage or its package helpers.
if [ -f "$SITE_PACKAGES/source-commit" ]; then
  git diff --quiet "$(cat "$SITE_PACKAGES/source-commit")" HEAD -- \
    Cargo.toml Cargo.lock rust-toolchain.toml clippy.toml LICENSE README.md crates sdk schemas extensions databases licenses \
    scripts/jvm-generated scripts/typescript-prepare scripts/r-package scripts/swift-package scripts/zig-package \
    scripts/package_inventory.py scripts/native_platform.py scripts/native-frameworks scripts/runtime_floors.py \
    scripts/header-contract-check scripts/header_contract.py scripts/duckdb-build scripts/sqlite-package \
    scripts/native-build-env scripts/native-tools-env scripts/release-tools.json site/scripts/prepare-packages.sh \
    'scripts/release_*.py' ':(exclude)scripts/release_install.py' \
    ':(exclude)sdk/cpp/tests/check-installed' ':(exclude)extensions/sqlite/tests/run.py' || {
    echo 'Prepared packages differ from this source' >&2; exit 1;
  }
  local_packages=$PWD/target/site-packages
  if [ "$SITE_PACKAGES" != "$local_packages" ]; then
    if [ -e "$local_packages" ] || [ -L "$local_packages" ]; then
      test "$local_packages" -ef "$SITE_PACKAGES" || {
        echo 'Local site-packages conflicts with the validated package prefix' >&2; exit 1;
      }
    else
      mkdir -p target
      ln -s "$SITE_PACKAGES" "$local_packages"
    fi
  fi
  SITE_BIN_DIR=$SITE_PACKAGES/bin
  PATH="$SITE_PACKAGES/bin:$PATH"
  LD_LIBRARY_PATH="$SITE_PACKAGES/swift-libs${LD_LIBRARY_PATH:+:$LD_LIBRARY_PATH}"
  export PATH LD_LIBRARY_PATH
else
  cargo build --locked -p thinkthen --bin thinkthen
  build=${CARGO_TARGET_DIR:-$PWD/target}
  case "$build" in /*) ;; *) build=$PWD/$build ;; esac
  SITE_BIN_DIR=$build/debug
fi
export SITE_BIN_DIR
if [ "$mode" = guide-check ]; then
  cargo build --locked -p loopback --bin loopback
  status=0
  scripts/bash-guide || status=1
  scripts/service-guide-examples || status=1
  scripts/day-one-docs-test || status=1
  scripts/guide-examples --inventory --require-all || status=1
  exit "$status"
fi
selected_languages=
case "$mode" in routine|capture|replay|prepare-examples)
  if [ "$#" -gt 0 ]; then
    selected_languages=$(node --input-type=module -e 'process.chdir("site"); const { exampleSource } = await import("./site/src/lib/examples.mjs"); console.log([...new Set(process.argv.slice(1).map((id) => exampleSource(id).lang))].join("\n"));' "$@")
  fi
  prepare_go=0
  if [ "$#" -eq 0 ] || printf '%s\n' "$selected_languages" | grep -qx go; then prepare_go=1; fi
  if [ "$prepare_go" = 1 ]; then
    THINKTHEN_GO_HOME=${THINKTHEN_GO_HOME:-$(python3 -c 'import sys; sys.path.insert(0,"scripts"); from release_go import home; print(home())')}
    test -x "$THINKTHEN_GO_HOME/bin/go"
    PATH="$THINKTHEN_GO_HOME/bin:$PATH"
    export PATH THINKTHEN_GO_HOME GOTOOLCHAIN=local
    test "$(go env GOVERSION)" = "go$(python3 -c 'import sys; sys.path.insert(0,"scripts"); from release_go import version; print(version())')"
  fi
  ;;
esac
cd site
case "$mode" in routine|capture|replay|prepare-examples)
  prepare_typescript=0
  if [ "$#" -eq 0 ] || printf '%s\n' "$selected_languages" | grep -qx ts; then prepare_typescript=1; fi
  if [ "$prepare_typescript" = 1 ]; then npm ci --no-audit --no-fund; fi
  ;;
esac
case "$mode" in
 bundle-runtimes) exec node scripts/bundle-runtimes.mjs "$bundle_out" ;;
 prepare-examples)
   THINKTHEN_BUILD_TOOLS=$(node --input-type=module -e 'import product from "./product.config.mjs"; console.log(product.exampleSetup({ shell: process.env }).THINKTHEN_BUILD_TOOLS);')
   export THINKTHEN_BUILD_TOOLS
   node --input-type=module -e 'import fs from "node:fs"; import { needsPreparation } from "./src/lib/examples.mjs"; for (const id of process.argv.slice(1)) if (!needsPreparation(id) || !fs.readFileSync(`examples/${id}/run.sh`, "utf8").includes("--prepare")) throw new Error(`${id}: no prepare mode`);' "$@"
   mkdir -p "$prepared_out"
   for id do (cd "examples/$id" && bash run.sh --prepare "$prepared_out/$id"); done
   git rev-parse HEAD > "$prepared_out/source-commit"
   python3 - "$prepared_out" "$@" <<'PREPARED'
import hashlib, json, os, sys
from pathlib import Path
out = Path(sys.argv[1])
files, links = {}, {}
for p in out.rglob('*'):
    name = str(p.relative_to(out))
    if p.is_symlink(): links[name] = os.readlink(p)
    elif p.is_file(): files[name] = hashlib.sha256(p.read_bytes()).hexdigest()
(out / 'manifest.json').write_text(json.dumps({'source': (out / 'source-commit').read_text().strip(), 'ids': sys.argv[2:], 'files': files, 'links': links}, indent=2) + '\n')
print(f'Prepared {len(sys.argv[2:])} executable examples in {out}')
PREPARED
   ;;
 reference)
   npm ci --no-audit --no-fund
   node scripts/gen-reference-test.mjs
   node scripts/gen-reference.mjs
   node scripts/gen-reference.mjs --check
   node scripts/gen-answers.mjs
   mkdir -p ../target
   git add -N src/content/docs/reference src/content/docs/answers
   git diff --binary -- src/content/docs/reference src/content/docs/answers > "../target/site-reference-$(git rev-parse HEAD).patch"
   ;;
 routine) exec node scripts/run-examples.mjs --check "$@" ;;
 capture) exec node scripts/run-examples.mjs --capture "$@" ;;
 replay) exec node scripts/run-examples.mjs --replay "$@" ;;
esac
