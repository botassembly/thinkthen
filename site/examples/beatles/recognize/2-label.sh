sentence="Ringo Starr wrote Octopus's Garden"
sentence+=" on a boat off Sardinia, and the band"
sentence+=" recorded it at Abbey Road Studios"
sentence+=" for the album Abbey Road."
kinds=(person song album place)

printf '%s' "$sentence" |
thinkthen recognize "${kinds[@]}" \
  --replay recording |
jq -c '.entities[] | {text, kind}'
