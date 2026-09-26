kinds=(
  person
  song
  album
  place
)
text="Ringo Starr wrote Octopus's Garden on a boat off"
text+=" Sardinia, and the band recorded it at Abbey Road"
text+=" Studios for the album Abbey Road."

printf '%s' "$text" |
thinkthen recognize "${kinds[@]}" \
  --threshold 0.01 \
  --replay recording |
jq -c '.entities[] | {name, kind, strength}'
