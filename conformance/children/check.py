"""The Python helper's proof, run by test.sh with sentinels in this process."""

import subprocess
import sys

from children import child_env

PROBE = ('test -z "${THINKTHEN_SENTINEL+x}" && test -z "${FAKE_SERVICE_API_KEY+x}" && '
         'test -z "${ABSENT_0127+x}" && test "$KEPT_0127" = kept && test "$SET_0127" = set')
REFUSED = ("THINKTHEN_BASE_URL", "OPENAI_API_KEY", "GITHUB_TOKEN", "db_password", "AWS_SECRET_ACCESS_KEY")
SENTENCE = "a test child may not keep {} from the parent: set a THINKTHEN_ value or a fake key explicitly"

env = child_env(keep=("KEPT_0127", "ABSENT_0127"), SET_0127="set")
bad = [] if subprocess.run(["sh", "-c", PROBE], env=env, check=False).returncode == 0 else ["the child"]
for name in REFUSED:
    try:
        child_env(keep=(name,))
        bad.append(name)
    except ValueError as error:
        bad += [] if str(error) == SENTENCE.format(name) else [str(error)]
print(f"children python: {'ok' if not bad else 'FAIL ' + ', '.join(bad)}")
sys.exit(1 if bad else 0)
