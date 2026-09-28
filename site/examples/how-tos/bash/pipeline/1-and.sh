about_the_item="Is this about the item itself?"
item_means="Its build, size, color, or parts."
other_means="Delivery, price, or service."
complaint="Is this a complaint?"

cat <<'EOF' |
Arrived a day early. Thank you!
The zipper broke the first time I used it.
Does this come in blue?
The strap snapped on day two.
EOF
thinkthen filter "$about_the_item" \
  --batch 1 \
  --true "$item_means" \
  --false "$other_means" |
thinkthen filter "$complaint" --batch 1
