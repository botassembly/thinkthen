cat <<'EOF' |
{"id": "R-1", "text": "We are on the team plan and cannot export to CSV."}
{"id": "R-2", "text": "Our enterprise account was billed twice. Call me today."}
{"id": "R-3", "text": "New here on the free plan. How do I reset my password?"}
EOF
thinkthen annotate form.json \
  --batch 1 \
  --jsonl \
  --field /text |
jq 'del(.text)'
