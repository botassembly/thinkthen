. ../../../lang-ruby/run.sh
python3 -B - "$work/output.txt" "$SITE_CASE_FILE" "$SITE_ANSWERS_FILE" <<'PYANSWERS'
from collections import Counter
import json
from pathlib import Path
import re
import sys

case = json.loads(Path(sys.argv[2]).read_text())
rows = []
for line in Path(sys.argv[1]).read_text().splitlines():
    match = re.fullmatch(r'([0-9]+) #<data ThinkThen::RankRow item=("(?:[^"\\]|\\.)*"), row=.*>', line)
    if match is None:
        raise ValueError("Invalid printed rank row")
    rows.append((json.loads(match[2]), int(match[1])))
if Counter(text for text, rank in rows) != Counter(case["records"]):
    raise ValueError("Printed rank records differ from case records")
rows.sort(key=lambda row: case["records"].index(row[0]))
Path(sys.argv[3]).write_text(json.dumps([rank for text, rank in rows]))
PYANSWERS
