is_a_song="The text is the title of a song by the Beatles."
on_abbey_road="It appears on the album Abbey Road."
question="$is_a_song $on_abbey_road"

cat <<'EOF' |
Octopus's Garden
Yellow Submarine
Something
Here Comes the Sun
Yesterday
Come Together
Hey Jude
Penny Lane
A Day in the Life
Her Majesty
Let It Be
Maxwell's Silver Hammer
EOF
thinkthen filter "$question" \
  --batch 1 \
  --threshold 0.7 \
  --replay recording
