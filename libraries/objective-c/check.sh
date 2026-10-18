#!/bin/sh
set -eu
ROOT=$(CDPATH= cd -- "$(dirname -- "$0")" && pwd)
REPO=$(CDPATH= cd -- "$ROOT/../.." && pwd)
case "$(uname -s)" in
  Darwin) ;;
  *) echo 'Objective-C Foundation execution requires macOS; GNU Objective-C support has ended' >&2; exit 77 ;;
esac
python3 "$REPO/conformance/children/privacy.py" --sources "$ROOT" --public --artifact-env
python3 "$REPO/sdlc/generators/results/generate.py" --target objc --check
exec python3 "$ROOT/checks/foundation_installed.py"
