is_a_song="The text is the title of a song by the Beatles."
question="$is_a_song Who sings the lead vocal on it?"
singers=(
  John
  Paul
  George
  Ringo
  "John and Paul duet"
)

printf '%s' "Octopus's Garden" |
thinkthen choose "$question" "${singers[@]}" \
  --threshold 0.8 \
  --replay recording
