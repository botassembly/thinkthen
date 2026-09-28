sentence="Ringo Starr wrote Octopus's Garden"
sentence+=" on a boat off Sardinia, and the band"
sentence+=" recorded it at Abbey Road Studios"
sentence+=" for the album Abbey Road."
kinds=(person song album place)

printf '%s' "$sentence" |
thinkthen recognize "${kinds[@]}" \
  --relation wrote=person:song \
  --relation recorded_at=song:place \
  --relation on_album=song:album \
  --replay recording |
jq -c '.relations[] | [.source.text, .relation,
  .target.text, .probability]'
