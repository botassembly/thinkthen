question="Is this a complaint?"

cat <<'EOF' |
Arrived a day early. Thank you!
The zipper broke the first time I used it.
Does this come in blue?
The strap snapped on day two.
EOF
thinkthen filter "$question" --batch 1
