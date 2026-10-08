"""The Python helper's proof, run by test.sh with sentinels in this process."""

import subprocess
import sys
import tempfile

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
# The owned-home contract permits an intentional folder override and still
# excludes ambient keys. Reversing the helper's default/override order fails.
with tempfile.TemporaryDirectory(prefix="thinkthen-child-home-") as home:
    env = child_env(home=home, XDG_CONFIG_HOME=home + "/explicit", KEPT_0127="kept", SET_0127="set")
    probe = PROBE + ' && printf "%s\\n" "$HOME" "$XDG_CONFIG_HOME" "$APPDATA"'
    result = subprocess.run(["sh", "-c", probe], env=env, text=True, capture_output=True, check=False)
    if result.returncode != 0 or result.stdout.splitlines() != [home, home + "/explicit", home + "/config"]:
        bad.append("owned home and explicit override")
print(f"children python: {'ok' if not bad else 'FAIL ' + ', '.join(bad)}")
sys.exit(1 if bad else 0)
