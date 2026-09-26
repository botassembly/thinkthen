question="Does the customer ask for a refund?"
yes="The customer asks for money back."
no="Anything else, such as a cancellation or a question."

cat <<'EOF' |
Please cancel my plan at the end of this month.
EOF
thinkthen decide "$question" \
  --true "$yes" \
  --false "$no"
