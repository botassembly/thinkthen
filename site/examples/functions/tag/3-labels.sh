question="Which labels fit this message?"
labels=(
  --label "bug=Something is broken."
  --label "praise=Says something kind."
  --label "request=Asks for something new."
)

cat <<'EOF' |
The export button crashes the app.
Great support, thank you!
Please add a dark mode.
EOF
thinkthen tag "$question" "${labels[@]}" \
  --lines \
  --threshold 0.9 |
jq .
