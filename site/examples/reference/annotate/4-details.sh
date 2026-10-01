cat <<'EOF' |
{
  "id": "B-7",
  "steps": "none given",
  "report": {
    "page": "/login",
    "body": "Steps: click Log in. Nobody gets in."
  }
}
EOF
thinkthen annotate form.json \
  --field /report/body \
  --details |
jq '{input, value}'
