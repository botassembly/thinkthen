question="Does the customer ask for a refund?"

printf '%s\n' "I want to send this back." |
thinkthen decide "$question" \
  --quiet \
  --threshold 0.2:0.8

case $? in
  0) echo "refund it" ;;
  1) echo "answer it as usual" ;;
  3) echo "send it to a person" ;;
esac
