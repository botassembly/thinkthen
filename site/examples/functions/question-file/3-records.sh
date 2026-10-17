cat <<'EOF' |
{"body":"Refund my order.","policy":"30-day refunds."}
EOF
thinkthen decide @records/refund.json \
  --jsonl \
  --field /body \
  --context-field /policy \
  --plan |
  head -n 1 |
  jq '.request | {state} + .questions.q1.criteria'
