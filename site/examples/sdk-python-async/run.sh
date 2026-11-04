set -eu
root=$(CDPATH= cd -- "$(dirname -- "${BASH_SOURCE[0]}")/../../.." && pwd)
packages=${SITE_PACKAGES:-$root/target/site-packages}
# Uses the Python package staged by scripts/prepare-packages.sh.
# show.py makes its own client, which reads THINKTHEN_BACKEND when no backend is
# passed, so routine runs use the backend the site runner sets.
PYTHONPATH="$packages/python" python3 -B show.py
