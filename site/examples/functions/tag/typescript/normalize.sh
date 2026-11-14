set -eu
python3 -B - "$1" "$SITE_ANSWERS_FILE" <<'PYANSWERS'
import ast
import json
from pathlib import Path
import sys

answers = [sorted(ast.literal_eval(Path(sys.argv[1]).read_text().strip()))]
Path(sys.argv[2]).write_text(json.dumps(answers))
PYANSWERS
