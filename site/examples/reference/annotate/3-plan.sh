cat <<'EOF' |
{"id": "B-7", "body": "Steps: click Log in. Nobody gets in."}
{"id": "B-8", "body": "The Pay button is too blue.", "area": "billing"}
EOF
thinkthen annotate form.json \
  --jsonl \
  --field /body \
  --plan
