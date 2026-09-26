buying="Is this a buying inquiry?"
ready="Is this buyer ready to pay now?"
team="Which team should take this?"
teams=(
  enterprise
  smb
)

cat <<'EOF' |
We need 200 seats next quarter. Please send a quote.
Please remove me from this list.
Our team of six wants to buy today. How do we pay?
Loved your talk at the conference last week.
EOF
thinkthen filter "$buying" |
thinkthen rank "$ready" |
thinkthen choose "$team" "${teams[@]}" --lines |
jq .
