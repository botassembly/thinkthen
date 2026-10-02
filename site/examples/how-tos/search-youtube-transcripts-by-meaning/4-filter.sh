asks="Does this passage explain"
question="$asks what problem Jev solves that LLMs do not?"

thinkthen filter "$question" < passages.txt |
cut -c1-60
