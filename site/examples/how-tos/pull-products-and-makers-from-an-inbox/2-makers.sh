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
jq -c --rawfile named products.txt '
  select(.target.name as $p
    | $named | split("\n") | any(. == $p))
  | [.source.name, .target.name]'
