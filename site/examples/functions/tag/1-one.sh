question="Which labels fit this message?"
labels=(
  praise
  bug
  billing
)

cat <<'EOF' |
Love the new dashboard, but export crashes the app,
and I was charged twice.
EOF
thinkthen tag "$question" "${labels[@]}"
