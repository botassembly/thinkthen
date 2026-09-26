question="Which team owns this?"
message="I cannot reset my password."
teams=(
  --option "billing=Invoices, fees, and refunds."
  --option "shipping=Parcels and delivery."
  --option "account=Logins and passwords."
)

team=$(
  printf '%s' "$message" |
  thinkthen choose "$question" "${teams[@]}" --raw
)

case "$team" in
  billing) queue="finance" ;;
  shipping) queue="warehouse" ;;
  account) queue="identity" ;;
esac

test "$team" = "account"
test "$queue" = "identity"
