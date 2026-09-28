is_a_song="The text is the title of a song by the Beatles."
on_abbey_road="It appears on the album Abbey Road."
question="$is_a_song $on_abbey_road"

printf '%s' "A Day in the Life" |
thinkthen decide "$question" \
  --details \
  --replay recording |
jq .
