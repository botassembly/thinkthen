cat <<'EOF' |
Blackbird
Octopus's Garden
EOF
thinkthen annotate annotate-cold-card.json \
  --batch 1 \
  --lines \
  --replay recording |
jq .
