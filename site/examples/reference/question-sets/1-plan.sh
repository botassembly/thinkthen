cat <<'EOF' |
{"id": "T-1", "subject": "Refund", "body": "Please refund the fee."}
EOF
thinkthen annotate triage.json \
  --jsonl \
  --plan |
jq 'select(.on) | .on,
  (.request.questions | map_values(.instructions))'
