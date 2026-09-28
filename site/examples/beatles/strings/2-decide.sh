is_a_song="The text is the title of a song by the Beatles."
on_abbey_road="It appears on the album Abbey Road."
question="$is_a_song $on_abbey_road"

cat <<'EOF' |
Octopus's Garden
Hey Jude
Penny Lane
EOF
thinkthen decide "$question" \
  --batch 1 \
  --lines \
  --threshold 0.5 \
  --replay recording
