question="Which answer says how to reset a password?"

cat <<'EOF' |
{"id": "Q1", "text": "Open Settings and choose Reset password."}
{"id": "Q2", "text": "Invoices are emailed on the first of the month."}
{"id": "Q3", "text": "We ship to the US and Canada."}
EOF
thinkthen find "$question" \
  --jsonl \
  --field /text |
jq .
