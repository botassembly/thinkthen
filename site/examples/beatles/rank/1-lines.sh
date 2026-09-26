is_a_song="The text is the title of a song by the Beatles."
a_big_hit="It is one of the Beatles' biggest hits."
question="$is_a_song $a_big_hit"

cat <<'EOF' |
Something
Hey Jude
Blackbird
Help!
Octopus's Garden
She Loves You
Piggies
Yesterday
Penny Lane
Her Majesty
Can't Buy Me Love
Good Night
EOF
thinkthen rank "$question" \
  --replay recording
