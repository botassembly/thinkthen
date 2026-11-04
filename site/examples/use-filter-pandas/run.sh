set -eu
root=$(CDPATH= cd -- "$(dirname -- "${BASH_SOURCE[0]}")/../../.." && pwd)
packages=${SITE_PACKAGES:-$root/target/site-packages}
# Uses the Python package staged by scripts/prepare-packages.sh.
# show.py calls the module functions, which use the shared default client, as a
# reader's program would. This sets that client's backend from THINKTHEN_BACKEND,
# which the site runner sets to stub for routine runs.
PYTHONPATH="$packages/python" python3 -B - <<'PY'
import os
import runpy

import thinkthen


backend = os.environ.get("THINKTHEN_BACKEND", "systemone")
thinkthen.set_default_client(thinkthen.Client(backend=backend))
runpy.run_path("show.py")
PY
