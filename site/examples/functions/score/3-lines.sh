question="How urgent is this?"
levels=(
  "Routine."
  "Soon."
  "Immediate."
)

cat <<'EOF' |
Please update my mailing address when you can.
Can you send the signed contract by Friday?
Nobody can log in to the site right now.
EOF
thinkthen score "$question" "${levels[@]}" \
  --batch 1 --lines |
jq .
