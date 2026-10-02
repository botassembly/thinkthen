awk 'NR % 10 != 1 { sub(/^\[[0-9:]+\] /, "") } 1' \
  transcript.txt |
paste -d' ' - - - - - - - - - - > passages.txt
wc -l < passages.txt
cut -c1-60 passages.txt | head -2
