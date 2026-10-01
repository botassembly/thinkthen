cat <<'EOF' |
{"id": "B-7", "body": "Steps: click Log in. Nobody gets in."}
{"id": "B-8", "text": "The Pay button on billing is too blue."}
{"id": "B-9", "body": "Steps: click Export. It is very slow."}
EOF
thinkthen annotate on-body.json \
  --jsonl \
  --details \
  --batch 1 \
  --on-error continue \
  > rows.jsonl
skipped_code=$?
jq -c 'if .schema == "thinkthen.record-error/1"
  then .
  else {id: .input.id, value}
  end' rows.jsonl
test "$skipped_code" = 7
