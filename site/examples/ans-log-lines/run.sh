thinkthen choose --from lines --json \
  --choice disk --choice network --choice login --choice other \
  'Which system does this log line concern?' app.log |
  jq '{position, answer}'
