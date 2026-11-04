message='Please refund my order.'
if echo "$message" |
    thinkthen decide --check \
      'Does the customer request a refund?' > /dev/null
then
  echo 'send to the refunds queue'
else
  status=$?
  case "$status" in
    1|3) ;;
    *) exit "$status" ;;
  esac
fi
