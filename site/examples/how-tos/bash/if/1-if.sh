question="Does the customer ask for a refund?"
ticket=$(cat <<'EOF'
I renewed once this morning, but my card shows two charges.
Please refund the duplicate.
EOF
)

asks_for_refund() {
  printf '%s\n' "$1" |
  thinkthen decide "$question" --quiet
}

if asks_for_refund "$ticket"; then
  queue="refunds"
else
  queue="support"
fi

test "$queue" = "refunds"
