#!/bin/sh
# Source-only docs structure checks; no package builds or host browser installs.
set -eu
cd "$(dirname "$0")/../.."
. scripts/native-tools-env
case ${1:-check} in
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
    node scripts/capture-backend-test.mjs
    exec node scripts/capture-runner-test.mjs
    ;;
  syntax)
    python3 scripts/python-examples-test.py
    python3 scripts/check-python-examples.py
    node scripts/function-cells-test.mjs
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
    npm ci --no-audit --no-fund
    node scripts/astro-build.mjs
    node scripts/agent-files.mjs
    node scripts/host-files.mjs
    node scripts/check.mjs
    shift
    SITE_LOOK_SHOTS=landing-shots/after node scripts/browser-box-job.mjs scripts/check-landing.mjs --strict --no-shots "$@"
    ;;
  build) exec npm run build ;;
  *) echo 'Use red, syntax, prepare, render, browser-smoke or build' >&2; exit 2 ;;
esac
