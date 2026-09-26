cat <<'EOF' |
{"name": "John Lennon", "kind": "person"}
{"name": "Paul McCartney", "kind": "person"}
{"name": "George Harrison", "kind": "person"}
{"name": "Ringo Starr", "kind": "person"}
{"name": "Octopus's Garden", "kind": "song"}
{"name": "Something", "kind": "song"}
{"name": "Come Together", "kind": "song"}
{"name": "Here Comes the Sun", "kind": "song"}
{"name": "Yesterday", "kind": "song"}
{"name": "Eleanor Rigby", "kind": "song"}
{"name": "Taxman", "kind": "song"}
{"name": "Abbey Road", "kind": "album"}
{"name": "Help!", "kind": "album"}
{"name": "Revolver", "kind": "album"}
EOF
thinkthen relate @relate.json \
  --jsonl \
  --threshold 0.8 \
  --replay recording |
jq -c '[
  .relation,
  .source.name,
  .target.name,
  .probability
]'
