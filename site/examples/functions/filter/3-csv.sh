question="Is this a complaint?"

cat <<'EOF' |
id,comment
R-1,Arrived a day early. Thank you!
R-2,The zipper broke the first time I used it.
R-3,Does this come in blue?
EOF
thinkthen filter "$question" \
  --batch 1 \
  --csv \
  --field /comment |
jq .
