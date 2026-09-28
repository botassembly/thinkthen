question="Does this stop customers from buying?"

cat <<'EOF' |
{"id": "B-1", "title": "Typo on the About page"}
{"id": "B-2", "title": "Checkout crashes for every customer"}
{"id": "B-3", "title": "Logo looks blurry on large screens"}
EOF
thinkthen rank "$question" \
  --batch 1 \
  --jsonl \
  --field /title \
  --top 1 |
jq .
