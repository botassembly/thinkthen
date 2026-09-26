is_a_song="The text is the title of a song by the Beatles."
question="$is_a_song It is a love song."

printf '%s\n' "Yesterday" |
thinkthen decide "$question" \
  --lines \
  --details \
  --replay recording |
jq .
