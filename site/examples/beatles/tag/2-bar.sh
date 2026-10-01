is_a_song="The text is the title of a song by the Beatles."
question="$is_a_song Which of these describe it?"
labels=(
  "love song"
  sad
  psychedelic
  "about a place"
  "about the sea"
)

cat <<'EOF' |
Yesterday
Penny Lane
Octopus's Garden
EOF
thinkthen tag "$question" "${labels[@]}" \
  --lines \
  --threshold 0.7 \
  --replay recording |
jq .
