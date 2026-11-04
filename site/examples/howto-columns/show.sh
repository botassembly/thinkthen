thinkthen annotate --column text --details \
  --output messages.triage.csv questions.json messages.csv
python3 - <<'PY'
import csv
with open("messages.triage.csv", newline="") as table:
    for message in csv.DictReader(table):
        print(message["id"], message["refund"], message["topic"],
              message["urgency"], message["urgency_status"])
PY
