question="What kind of page is this?"
kinds=(
  invoice
  notice
)

cat <<'EOF' |
Invoice 7 for Northwind. Page 1 of 2.
Invoice 7 for Northwind. Page 2 of 2. Total due $1,200.
Invoice 8 for Contoso. Page 1 of 1. Total due $310.
Notice to all customers. Page 1 of 1. New terms from May 1.
EOF
thinkthen choose "$question" "${kinds[@]}" \
  --lines \
  --raw
