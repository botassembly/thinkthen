expense='Client dinner, $60 a head'
covers="Which rule covers: $expense"
allows="Does this rule allow: $expense"

cat <<'EOF' |
Meals with a client: up to $75 a head.
Flights: economy only, booked 14 days out.
Software: needs a manager's approval.
EOF
thinkthen find "$covers" |
thinkthen decide "$allows" --lines |
jq .
