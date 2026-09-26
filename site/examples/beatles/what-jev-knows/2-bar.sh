about_a_member="The text names a member of the Beatles."
question="$about_a_member Which of these songs"
question+=" by the Beatles has this member"
question+=" as its only lead singer?"
songs=(
  --option "a=I'm a Loser"
  --option "b=Girl"
  --option "c=Sweet Little Sixteen"
  --option "d=I've Just Seen a Face"
)

printf '%s' "Paul McCartney" |
thinkthen choose "$question" "${songs[@]}" \
  --threshold 0.7 \
  --replay recording
