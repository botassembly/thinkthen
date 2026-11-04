# Uses the Python package staged by scripts/prepare-packages.sh. Runs show.py once,
# then writes the answers it named for the check that every language agrees.
# Fail when show.py's question or options differ from the shared case.json.
jq -r '[.question?, .choices[]?, .labels[]?, .levels[]?, .kinds[]?,
        (.rules[]? | .source, .target, .relation),
        (.questions? // {} | keys[]),
        (.questions[]? | .question, .choices[]?, .levels[]?)]
       | .[] | select(. != null)' "$SITE_CASE_FILE" |
  while IFS= read -r wording; do
    grep -qF "\"$wording\"" show.py || { echo "show.py does not use \"$wording\" from case.json" >&2; exit 1; }
  done || exit 1
PYTHONPATH="${SITE_PACKAGES:-../../../../../target/site-packages}/python" python3 -B - <<'PY'
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
answers = [[{"source": relation["source"]["text"], "relation": relation["relation"], "target": relation["target"]["text"]} for relation in found] for found in shown["relations"].to_list()]
with open(os.environ["SITE_ANSWERS_FILE"], "w", encoding="utf-8") as out:
    json.dump(answers, out)
PY
