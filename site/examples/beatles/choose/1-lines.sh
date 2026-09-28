is_a_song="The text is the title of a song by the Beatles."
question="$is_a_song Who sings the lead vocal on it?"
singers=(
  John
  Paul
  George
  Ringo
  "John and Paul duet"
)

cat <<'EOF' |
Come Together
Yesterday
Something
Octopus's Garden
She Loves You
EOF
thinkthen choose "$question" "${singers[@]}" \
  --lines \
  --replay recording
