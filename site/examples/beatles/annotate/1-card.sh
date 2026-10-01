cat <<'EOF' |
{"title":"Yesterday"}
{"title":"Octopus's Garden"}
EOF
thinkthen annotate questions.json \
  --jsonl \
  --field /title \
  --replay recording |
jq .
