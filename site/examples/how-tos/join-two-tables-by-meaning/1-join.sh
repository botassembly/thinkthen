question="Do these two describe the same problem?"
gateway="INC-1 payment gateway returns 500"
export_queue="INC-2 nightly export queue backed up"

cat <<'EOF' |
Card charges fail at checkout.
Export finished hours late again.
EOF
while read -r ticket; do
  printf '%s / %s\n' "$ticket" "$gateway"
  printf '%s / %s\n' "$ticket" "$export_queue"
done |
thinkthen filter "$question" --lines
