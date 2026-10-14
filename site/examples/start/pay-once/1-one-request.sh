question="Does the customer ask for a refund?"

cat <<'EOF' |
Please refund my order. It arrived broken.
Thanks for the quick help yesterday!
I want to send this back.
EOF
thinkthen decide "$question" \
  --lines \
  --plan |
jq -s '{
  in_first_request: (.[0].request.questions | length),
  input_tokens: .[1].estimated_input_tokens
}'
