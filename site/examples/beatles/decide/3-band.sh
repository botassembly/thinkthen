is_a_song="The text is the title of a song by the Beatles."
question="$is_a_song It is a love song."

cat <<'EOF' |
She Loves You
Michelle
Yesterday
Taxman
EOF
thinkthen decide "$question" \
  --lines \
  --threshold 0.3:0.7 \
  --replay recording
