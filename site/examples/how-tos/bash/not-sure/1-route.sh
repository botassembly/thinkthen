question="Does the customer ask for a refund?"

cat <<'EOF' |
Please refund my order. It arrived broken.
Thanks for the quick help yesterday!
I want to send this back.
EOF
while read -r message; do
  printf '%s' "$message" |
  thinkthen decide "$question" \
    --quiet \
    --threshold 0.2:0.8
  refund_code=$?
  case $refund_code in
    0) echo "refund: $message" ;;
    1) echo "reply: $message" ;;
    3) echo "a person: $message" ;;
  esac
done
