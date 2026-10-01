kinds=(
  person
  organization
  place
)
text="Maria Chen joined Northwind Freight, a company"
text+=" in Chicago."

printf '%s' "$text" |
thinkthen recognize "${kinds[@]}" |
jq -c '.entities[] | [.text, .kind]'
