#!/bin/sh
# Source-only docs structure checks; no package builds or host browser installs.
set -eu
cd "$(dirname "$0")/../.."
. scripts/native-tools-env
case ${1:-check} in
  pages-installers)
    python3 site/scripts/pages-installers-test.py
    actionlint .github/workflows/pages.yml
    git diff --check
    exit
    ;;
  node-version-refusal)
    scratch=$(mktemp -d)
    trap 'status=$?; trap - EXIT; set +e; rm -rf "$scratch"; exit "$status"' EXIT
    trap 'exit 1' HUP INT TERM
    mkdir "$scratch/bin"
    printf '#!/bin/sh\nprintf "v18.0.0\\n"\n' > "$scratch/bin/node"
    chmod +x "$scratch/bin/node"
    if (set -e; THINKTHEN_NODE_HOME=$scratch; . site/scripts/node-env.sh) > "$scratch/output" 2>&1; then
      echo 'Unsupported Node was accepted' >&2
      exit 1
    fi
    cat "$scratch/output"
    grep -q 'Node v18.0.0 does not match site/package.json' "$scratch/output"
    echo 'Unsupported Node refused'
    exit
    ;;
  prepare|render|build|browser-smoke|browser-review|syntax|red|capture-controls|browser-output-controls|help-tables)
    . site/scripts/node-env.sh
    ;;
esac
cd site
case ${1:-check} in
  red) exec node scripts/function-cells-test.mjs ;;
  help-tables) exec node scripts/help-tables.mjs ;;
  browser-output-controls) exec node scripts/browser-output-test.mjs ;;
  capture-controls)
    python3 scripts/windows-example-test.py
    python3 ../scripts/release-download-cache-test DownloadCache.test_windows_site_replay_uses_the_validated_final_installer_binary
    node scripts/capture-backend-test.mjs
    exec node scripts/capture-runner-test.mjs
    ;;
  syntax)
    python3 scripts/python-examples-test.py
    python3 scripts/check-python-examples.py
    node scripts/function-cells-test.mjs
    node scripts/check-function-cells.mjs
    node scripts/capture-backend-test.mjs
    node scripts/capture-runner-test.mjs
    for file in scripts/*.mjs src/lib/*.mjs; do node --check "$file"; done
    ;;
  prepare) exec npm ci --no-audit --no-fund ;;
  render)
    npm ci --no-audit --no-fund
    node scripts/astro-build.mjs
    ;;
  browser-smoke)
    npm ci --no-audit --no-fund
    node scripts/browser-box-job.mjs scripts/social-card.mjs
    ;;
  browser-review)
    python3 ../scripts/day-one-docs-test
    python3 ../scripts/guide-examples --inventory --require-all
    ../scripts/words
    npm ci --no-audit --no-fund
    node scripts/gen-reference-test.mjs
    node scripts/gen-reference.mjs --check
    node scripts/check-function-cells.mjs
    node scripts/astro-build.mjs
    node scripts/agent-files.mjs
    node scripts/host-files.mjs
    node scripts/check.mjs
    shift
    SITE_LOOK_SHOTS=${SITE_LOOK_SHOTS:-landing-shots/after}
    export SITE_LOOK_SHOTS
    browser_status=0
    node scripts/browser-box-job.mjs scripts/check-landing.mjs --strict --no-shots "$@" || browser_status=$?
    if [ -f landing-report.json ]; then
      cp landing-report.json "$(dirname "$SITE_LOOK_SHOTS")/landing-report.json"
    fi
    exit "$browser_status"
    ;;
  build) exec npm run build ;;
  *) echo 'Use red, syntax, prepare, render, browser-smoke or build' >&2; exit 2 ;;
esac
