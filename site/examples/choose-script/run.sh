message='Please refund my order.'
if topic=$(echo "$message" |
    thinkthen choose --check \
      --choice billing --choice shipping \
      --choice other 'What is the message about?')
then
  echo "send to the $topic queue"
else
  status=$?
  case "$status" in
    1|3) echo 'send to a person' ;;
    *) exit "$status" ;;
  esac
fi
