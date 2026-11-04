if thinkthen filter --check \
    'Does the customer request a refund?' \
    messages.txt > /dev/null
then
  echo 'refund requests are waiting'
else
  status=$?
  case "$status" in
    1|3) ;;
    *) exit "$status" ;;
  esac
fi
