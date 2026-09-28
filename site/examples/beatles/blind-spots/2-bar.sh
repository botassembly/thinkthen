is_a_song="The text is the title of a song by the Beatles."
question="$is_a_song Who sings the lead vocal on it?"
singers=(
  --option "john=John Lennon"
  --option "paul=Paul McCartney"
  --option "george=George Harrison"
  --option "ringo=Ringo Starr"
)

printf '%s' "Nothin' Shakin'" |
thinkthen choose "$question" "${singers[@]}" \
  --threshold 0.5 \
  --replay recording
