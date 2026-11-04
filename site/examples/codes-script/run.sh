echo 'I was charged twice for one invoice.' |
  thinkthen decide --check --low 0.2 --high 0.8 \
    'Does the customer request a refund?' > /dev/null
status=$?
case "$status" in
  0) echo 'send to the refunds queue' ;;
  1) echo 'send to the inbox' ;;
  3) echo 'send to a person' ;;
  *) echo 'stop: read the error above'
     exit "$status" ;;
esac
