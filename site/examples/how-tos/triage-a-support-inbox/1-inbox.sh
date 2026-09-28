question="Does this need a reply?"

cat <<'EOF' |
Please fix the wrong amount on my invoice.
Just saying thanks, no reply needed.
Order never arrived and the party is tonight.
Do you ship to Canada?
EOF
thinkthen filter "$question" |
thinkthen annotate inbox-form.json --lines |
jq .
