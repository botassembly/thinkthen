question="How urgent is this?"
levels=(
  "Routine."
  "Soon."
  "Immediate."
)

cat <<'EOF' |
Our checkout page is down and customers cannot pay.
EOF
thinkthen score "$question" "${levels[@]}"
