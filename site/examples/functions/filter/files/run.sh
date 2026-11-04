#!/bin/bash
set -euo pipefail
# Keep generated spreadsheets out of the authored example folder.
work=$(mktemp -d)
trap 'rm -rf "$work"' EXIT
cp show.sh messages.csv "$work/"
cd "$work"
bash -e -o pipefail show.sh
# Read fresh semantic answers from the spreadsheet the shown command wrote.
python3 - <<'ANSWERS'
import csv
import json
import os

with open("messages.thinkthen.csv", newline="", encoding="utf-8") as source:
    rows = list(csv.DictReader(source))
answers = [row["message"] for row in rows if row["is_refund"] == "TRUE"]
with open(os.environ["SITE_ANSWERS_FILE"], "w", encoding="utf-8") as output:
    json.dump(answers, output)
ANSWERS
