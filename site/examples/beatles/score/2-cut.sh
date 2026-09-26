is_a_song="The text is the title of a song by the Beatles."
question="$is_a_song How long is the recording?"
minutes=(
  "about 0 minutes"
  "about 1 minute"
  "about 2 minutes"
  "about 3 minutes"
  "about 4 minutes"
  "about 5 minutes"
  "about 6 minutes"
  "about 7 minutes"
  "about 8 minutes"
  "about 9 minutes"
)

cat <<'EOF' |
Her Majesty
Yesterday
Octopus's Garden
Something
Come Together
A Day in the Life
Hey Jude
Revolution 9
EOF
thinkthen score "$question" "${minutes[@]}" \
  --lines \
  --replay recording |
jq -c 'select(.value >= 5)'
