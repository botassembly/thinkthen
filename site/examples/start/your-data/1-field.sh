question="Is this a complaint?"

cat <<'EOF' |
id,comment
R-1,Arrived a day early. Thank you!
R-2,The zipper broke the first time I used it.
EOF
thinkthen filter "$question" \
  --csv \
  --field /comment \
  --plan |
head -1 |
jq '.request'
