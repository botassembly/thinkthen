#!/bin/sh
set -eu
python3 "$(dirname -- "$0")/../../sdlc/scripts/check_verdict.py" platform libraries/objective-c
tools_group=objective-c
. "$(dirname -- "$0")/../../sdlc/scripts/resolved-tools"
ROOT=$(CDPATH= cd -- "$(dirname -- "$0")" && pwd)
REPO=$(CDPATH= cd -- "$ROOT/../.." && pwd)
python3 "$REPO/sdlc/generators/results/generate.py" --target objc --check
case "$(uname -s)" in
  Darwin) exec python3 "$ROOT/checks/foundation_installed.py" ;;
  *) echo 'Objective-C Foundation execution requires macOS; GNU Objective-C support has ended' >&2; exit 1 ;;
esac
