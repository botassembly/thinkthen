question="Which line gives the refund deadline?"

cat <<'EOF' |
Returns need the original receipt.
Refunds are issued within 30 days of purchase.
Shipping is free on orders over $50.
Gift cards cannot be exchanged for cash.
EOF
thinkthen find "$question" --details |
jq .
