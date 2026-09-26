these_songs="These are songs by the Beatles."
question="$these_songs Which one did they release first?"

cat <<'EOF' |
She Loves You
I Saw Her Standing There
From Me to You
All My Loving
Love Me Do
I Want to Hold Your Hand
Please Please Me
Can't Buy Me Love
A Hard Day's Night
I Feel Fine
EOF
thinkthen find "$question" \
  --details \
  --replay recording |
jq .
