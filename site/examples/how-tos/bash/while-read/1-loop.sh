question="Is this a complaint?"

is_complaint() {
  printf '%s' "$1" |
  thinkthen decide "$question" --quiet
}

cat <<'EOF' |
Arrived a day early. Thank you!
The zipper broke the first time I used it.
Does this come in blue?
The strap snapped on day two.
EOF
while read -r review; do
  if is_complaint "$review"; then
    echo "Open a case: $review"
  fi
done
