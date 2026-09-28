person="PER=Part of a person's name."
org="ORG=Part of the name of an organization:"
org+=" a company, band, team, agency, government"
org+=" body, or media outlet."
place="LOC=Part of the name of a place: a country,"
place+=" region, city, or geographic feature."
other="MISC=Part of another named entity: a"
other+=" nationality, an event, a product, or the"
other+=" name of a creative work."
text="Maria Chen joined Northwind Freight in Chicago"
text+=" last spring."
printf '%s' "$text" |
thinkthen recognize \
  --kind "$person" \
  --kind "$org" \
  --kind "$place" \
  --kind "$other" |
jq .
