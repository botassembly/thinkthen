question="Which team owns this?"
teams=(
  --option "billing=Invoices, fees, and refunds."
  --option "shipping=Parcels and delivery."
  --option "account=Logins and passwords."
)

cat <<'EOF' |
Please refund the extra fee on my invoice.
My parcel went to the wrong address.
I cannot reset my password.
My parcel never came, and now I cannot log in to track it.
EOF
thinkthen choose "$question" "${teams[@]}" \
  --batch 1 \
  --lines \
  --threshold 0.9 |
jq -r .value
