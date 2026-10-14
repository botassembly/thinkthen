asks="Does this report say what the person did"
question="$asks before the problem appeared?"

cat report.txt |
thinkthen decide "$question" \
  --replay recording
