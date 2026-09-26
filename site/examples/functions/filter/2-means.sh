question="Is this about the item itself?"
yes="Its build, size, color, or parts."
no="Delivery, price, or service."

cat <<'EOF' |
Arrived a day early. Thank you!
The zipper broke the first time I used it.
Does this come in blue?
The strap snapped on day two.
EOF
thinkthen filter "$question" \
  --true "$yes" \
  --false "$no"
