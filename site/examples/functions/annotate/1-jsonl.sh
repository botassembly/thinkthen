cat <<'EOF' |
{"id": "B-7", "body": "Steps: click Log in. Nobody gets in."}
{"id": "B-8", "body": "The Pay button on billing is too blue."}
{"id": "B-9", "body": "Steps: click Export. It is very slow."}
EOF
thinkthen annotate form.json \
  --jsonl \
  --field /body \
  --jobs 8 |
jq .
