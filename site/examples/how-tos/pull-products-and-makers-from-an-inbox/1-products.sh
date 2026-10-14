kinds=(
  person
  product
  place
)
text="Sam Patel says his Kindle arrived broken"
text+=" in Denver."

printf '%s' "$text" |
thinkthen recognize "${kinds[@]}" |
jq -r '.entities[]
  | select(.kind == "product")
  | .text' |
tee products.txt
