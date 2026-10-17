makes="makes=company:product"

cat <<'EOF' |
[
  {"name": "Apple", "kind": "company"},
  {"name": "Amazon", "kind": "company"},
  {"name": "iPhone", "kind": "product"},
  {"name": "Kindle", "kind": "product"}
]
EOF
thinkthen relate "$makes" |
jq -c '[.source.name, .target.name, .probability]'
