if action=$(thinkthen find --check \
    'Which line states the requested action?' \
    message.txt)
then
  echo "to do: $action"
else
  status=$?
  case "$status" in
    1|3) echo 'send to a person' ;;
    *) exit "$status" ;;
  esac
fi
