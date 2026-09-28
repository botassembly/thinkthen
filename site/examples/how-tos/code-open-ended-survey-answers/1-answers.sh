mood="How happy is this answer?"
moods=(
  "Unhappy."
  "Neutral."
  "Happy."
)
problem="What is wrong here?"
problems=(
  price
  speed
  bugs
)

cat <<'EOF' |
Too expensive for what it does.
The app crashes when I upload a file.
Reports take a full minute to load.
Everything works and the reports are quick.
EOF
thinkthen score "$mood" "${moods[@]}" --batch 1 --lines |
jq -r 'select(.value < 1) | .input' |
thinkthen tag "$problem" "${problems[@]}" \
  --batch 1 --lines |
jq .
