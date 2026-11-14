set -eu
root=$(CDPATH= cd -- "$(dirname -- "${BASH_SOURCE[0]}")/../../../../.." && pwd)
packages=${SITE_PACKAGES:-$root/target/site-packages}
# Uses the Python package staged by scripts/prepare-packages.sh. Runs show.py once,
# then writes the answers it named for the check that every language agrees.
PYTHONPATH="$packages/python" python3 -B - <<'PY'
import json
import os
import runpy

import thinkthen

# show.py calls the module functions, which use the shared default client, as a
# reader's program would. This sets that client's backend from THINKTHEN_BACKEND,
# which the site runner sets to stub for routine runs.
backend = os.environ.get("THINKTHEN_BACKEND", "systemone")
thinkthen.set_default_client(thinkthen.Client(backend=backend))

shown = runpy.run_path("show.py")
answers = [rank.answer for rank in sorted(shown["by_urgency"], key=lambda rank: rank.position)]
with open(os.environ["SITE_ANSWERS_FILE"], "w", encoding="utf-8") as out:
    json.dump(answers, out)
PY
