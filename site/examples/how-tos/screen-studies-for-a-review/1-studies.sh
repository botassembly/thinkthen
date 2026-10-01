question="Does this report measured results on remote work?"

cat <<'EOF' |
Survey of 900 remote workers: output rose 4%.
Opinion essay on open-plan offices.
Title only: Remote work and output.
EOF
thinkthen decide "$question" \
  --lines \
  --threshold 0.1:0.9 |
jq .
