# Source from the repository root after scripts/native-tools-env.
# The candidate gate owns the default pin; explicit prepared runtimes must match the site manifest.
if [ -z "${THINKTHEN_NODE_HOME:-}" ]; then
THINKTHEN_NODE_HOME=$(python3 - <<'PY'
import ast
from pathlib import Path
module = ast.parse(Path('scripts/public_export_gate.py').read_text())
version = next(ast.literal_eval(node.value) for node in module.body
               if isinstance(node, ast.Assign) and any(isinstance(name, ast.Name) and name.id == 'NODE_VERSION' for name in node.targets))
print(Path.home() / '.local/share/thinkthen-build-tools/gate-node' / version)
PY
)
  if [ ! -x "$THINKTHEN_NODE_HOME/bin/node" ]; then
    thinkthen_node=$(command -v node || true)
    if [ -x /opt/zerobrew/opt/node@22/bin/node ]; then
      thinkthen_node=/opt/zerobrew/opt/node@22/bin/node
    fi
    if [ -n "$thinkthen_node" ]; then
      THINKTHEN_NODE_HOME=$(dirname "$(dirname "$thinkthen_node")")
    fi
    unset thinkthen_node
  fi
fi
if [ ! -x "$THINKTHEN_NODE_HOME/bin/node" ]; then
  echo "Prepare the existing candidate gate Node runtime at $THINKTHEN_NODE_HOME, or set THINKTHEN_NODE_HOME to a supported prepared runtime" >&2
  exit 1
fi
python3 - "$THINKTHEN_NODE_HOME/bin/node" <<'PY'
import json
from pathlib import Path
import re
import subprocess
import sys
required = json.loads(Path('site/package.json').read_text())['engines']['node']
minimum = re.fullmatch(r'>=(\d+)\.(\d+)\.(\d+)', required)
version = subprocess.check_output([sys.argv[1], '--version'], text=True).strip()
actual = re.fullmatch(r'v(\d+)\.(\d+)\.(\d+)', version)
if not minimum or not actual or tuple(map(int, actual.groups())) < tuple(map(int, minimum.groups())):
    raise SystemExit(f'Node {version} does not match site/package.json {required}')
print(f'Docs runtime: Node {version} (requires {required})')
PY
PATH="$THINKTHEN_NODE_HOME/bin:$PATH"
export THINKTHEN_NODE_HOME PATH
