asks="Does this passage explain"
question="$asks why people are excited about Jev?"

thinkthen rank "$question" --top 3 < passages.txt |
cut -c1-60
