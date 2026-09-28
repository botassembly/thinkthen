rule='def ok:
  (try fromjson catch null)
  | type == "object" and (.body | type) == "string";'
jq -Rr "$rule select(ok | not)" queue.jsonl > aside.jsonl
jq -Rr "$rule select(ok)" queue.jsonl > good.jsonl
printf 'aside:\n'
cat aside.jsonl
printf 'judged:\n'
thinkthen decide 'Is this a complaint?' \
  --jsonl --field /body --batch 1 < good.jsonl |
  jq -c '{id:.input.id, complaint:.value}'
