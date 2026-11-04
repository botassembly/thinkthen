# Run the unchanged example with the same-source Python package.
set -eu
root=$(CDPATH= cd -- ../../../../.. && pwd)
packages=${SITE_PACKAGES:-$root/target/site-packages}
PYTHONPATH="$packages/python" python3 -B - <<'PYRUN'
import json
import os
import runpy

shown = runpy.run_path("show.py")
with open(os.environ["SITE_ANSWERS_FILE"], "w", encoding="utf-8") as output:
    json.dump(shown["refund_requests"], output)
PYRUN
