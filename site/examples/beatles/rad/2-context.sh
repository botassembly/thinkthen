about_an_entry="The text gives a catalog entry"
about_an_entry+=" and then names a song by the Beatles."
on_abbey_road="It appears on the album Abbey Road."
question="$about_an_entry $on_abbey_road"

album="Sgt. Pepper's Lonely Hearts Club Band (1967-05-26)"
entry="A Day in the Life (lead: Lennon;"
entry+=" written: Lennon–McCartney; 5:38;"
entry+=" released 1967-05-26; first album:"
entry+=" Sgt. Pepper's Lonely Hearts Club Band)"

printf 'Catalog:\n%s\n%s\nText: %s' \
  "$album" \
  "$entry" \
  "A Day in the Life" |
thinkthen decide "$question" \
  --details \
  --replay recording |
jq .
