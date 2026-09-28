question="Is this urgent?"

cat <<'EOF' |
Newsletter: our autumn catalog is here. No reply needed.
Our checkout page is down and customers cannot pay
Reminder: your invoice is due in 30 days
Please send the signed quote by 5 pm today
EOF
thinkthen rank "$question" --batch 1
