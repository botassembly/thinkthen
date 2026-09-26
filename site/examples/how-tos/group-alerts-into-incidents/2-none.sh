alert="Disk is full on the backup server"
covers="Which incident covers: $alert"
same="Is this the same failure as: $alert"

incident=$(
cat <<'EOF' |
INC-1 checkout returns 500 at payment
INC-2 search results load slowly
INC-3 nightly export ran late
EOF
thinkthen find "$covers" --none
)

if [ -n "$incident" ]; then
  printf '%s\n' "$incident" |
  thinkthen decide "$same"
fi

test -z "$incident"
