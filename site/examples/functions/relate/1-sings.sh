sings="sings=singer:song"

cat <<'EOF' |
[
  {"name": "Paul McCartney", "kind": "singer"},
  {"name": "Ringo Starr", "kind": "singer"},
  {"name": "Yesterday", "kind": "song"},
  {"name": "Octopus's Garden", "kind": "song"}
]
EOF
thinkthen relate "$sings" |
jq -c '[.source.name, .target.name, .probability]'
