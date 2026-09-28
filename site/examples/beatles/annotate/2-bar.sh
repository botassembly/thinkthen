jq '.questions[].threshold = 0.5' \
  annotate-cold-card.json > card-0.5.json

cat <<'EOF' |
Blackbird
Octopus's Garden
EOF
thinkthen annotate card-0.5.json \
  --lines \
  --replay recording |
jq .
