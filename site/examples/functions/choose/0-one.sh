question="Which team owns this?"
teams=(
  --option "billing=Invoices, fees, and refunds."
  --option "shipping=Parcels and delivery."
  --option "account=Logins and passwords."
)

printf '%s' "My parcel went to the wrong address." |
thinkthen choose "$question" "${teams[@]}"
