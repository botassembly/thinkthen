question="Does the customer ask for a refund?"

cat <<'EOF' |
Please refund my order. It arrived broken.
Thanks for the quick help yesterday!
I want to send this back.
EOF
thinkthen decide "$question" \
  --batch 1 \
  --lines \
  --threshold 0.2:0.8 |
jq .value
