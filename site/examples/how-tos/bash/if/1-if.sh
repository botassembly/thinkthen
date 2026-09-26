question="Does the customer ask for a refund?"
ticket=$(cat <<'EOF'
I renewed once this morning, but my card shows two charges.
Please refund the duplicate.
EOF
)

if printf '%s\n' "$ticket" |
   thinkthen decide "$question" --quiet
then
  queue="refunds"
else
  queue="support"
fi

test "$queue" = "refunds"
