jq '.questions.abbey_road.threshold = "0.2:0.8"
  | .questions.big_hit.threshold = "0.2:0.8"' \
  questions.json > band.json

cat <<'EOF' |
{"title":"Yesterday"}
{"title":"Octopus's Garden"}
EOF
thinkthen annotate band.json \
  --jsonl \
  --field /title \
  --replay recording |
jq .
