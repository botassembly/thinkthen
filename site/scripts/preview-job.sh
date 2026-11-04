#!/bin/sh
# Bounded docs consumer job; engines come from the completed qualification artifact.
set -eu
cd "$(dirname "$0")/../.."
candidate=$1
mode=${2:-build}
if [ -n "${3:-}" ]; then export NODE_PATH=$3; fi
tools=$HOME/.local/share/thinkthen-build-tools
export PATH="$PWD/target/site-packages/bin:$tools/dotnet10:$tools/jvm/apache-maven-3.9.11/bin:$tools/jvm/jdk-22.0.2+9/bin:$tools/ruby/runtime/bin:$tools/go/bin:$PWD/target/site-packages/bin:$PATH"
export NODE_OPTIONS=--max-old-space-size=2048
export CARGO_BUILD_JOBS=2
export LD_LIBRARY_PATH="$PWD/target/site-packages/swift-libs${LD_LIBRARY_PATH:+:$LD_LIBRARY_PATH}"
if [ "$mode" = grade-red ]; then
  red=$(mktemp -d)
  trap 'status=$?; trap - EXIT; set +e; rm -rf "$red"; exit "$status"' EXIT
  trap 'exit 1' HUP INT TERM
  tar -xzf "$candidate" -C "$red"
  cd site
  node scripts/check-grade.mjs --dist "$red" --binary "$3"
elif [ "$mode" = prepare ]; then
  ruff check --select F401,F841,ERA,BLE,E722,S110 site/scripts/prepare-preview.py site/scripts/check-install-checksum.py
  bash -n site/examples/lang-zig/run.sh
  python3 site/scripts/prepare-preview.py "$candidate"
  cd site
  npm ci --no-audit --no-fund
elif [ "$mode" = examples ]; then
  shift 3
  cd site
  npm run examples -- "$@"
elif [ "$mode" = prepare-pandas ]; then
  python3 site/scripts/prepare-preview.py "$candidate" --pandas-only
  cd site
  npm run examples -- use-annotate-pandas use-filter-pandas use-rank-pandas use-score-pandas
elif [ "$mode" = install-checksum ]; then
  output=$(dirname "$candidate")/docs-preview-$(git rev-parse HEAD)
  ruff check --select F401,F841,ERA,BLE,E722,S110 site/scripts/check-install-checksum.py
  python3 site/scripts/check-install-checksum.py --output "$output/install-checksum.json"
else
  output=$(dirname "$candidate")/docs-preview-$(git rev-parse HEAD)
  mkdir -p "$output"
  if [ "$mode" = look ] || [ "$mode" = look-check ] || [ "$mode" = browser ]; then
    export SITE_LOOK_SHOTS="$output/screenshots/after"
    export SITE_REVIEW_ROUTES=${4:-}
  fi
  cd site
  if [ "$mode" = browser ]; then
    exec node scripts/browser-box-job.mjs scripts/check-landing.mjs --strict --no-shots
  fi
  if [ "$mode" = refresh ]; then
    npm run examples
    npm run reference
    npm run answers
    git add -N src/content/docs/reference src/content/docs/answers
    git diff --binary -- . > "$output/site-refresh.patch"
  fi
  if [ "$mode" = generate ] || [ "$mode" = look-check ]; then
    if [ "$mode" = generate ]; then
    npm run reference
    npm run answers
    git add -N src/content/docs/reference src/content/docs/answers
    git diff --binary -- . > "$output/site-refresh.patch"
    fi
    node scripts/astro-build.mjs
    node scripts/agent-files.mjs
    node scripts/host-files.mjs
    node scripts/check.mjs
    node scripts/browser-box-job.mjs scripts/check-landing.mjs --strict --no-shots
  else
    npm run build
  fi
  tar -czf "$output/site-dist.tar.gz" -C dist .
  printf '%s\n' "Preview artifact: $output/site-dist.tar.gz"
fi
