question="Does the customer ask for a refund?"

cat <<'EOF' |
I renewed once this morning, but my card shows two charges.
Please refund the duplicate.
EOF
thinkthen decide "$question" \
  --replay recording \
  --facts 2> facts.json
jq 'del(.seconds, .call_id, .command_ms)' facts.json
