question="Is this a complaint?"

cat <<'EOF' |
Arrived a day early. Thank you!
The zipper broke the first time I used it.
Does this come in blue?
The strap snapped on day two.
EOF
while read -r review; do
  if printf '%s' "$review" |
     thinkthen decide "$question" --quiet
  then
    echo "Open a case: $review"
  fi
done
