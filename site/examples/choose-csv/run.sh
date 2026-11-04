# cut keeps the first five columns: id, text, the answer, its status
# and its probability. The error, detail and source columns follow.
thinkthen choose --column text --into topic \
  --choice billing --choice shipping --choice other \
  --from csv 'What is the message about?' < messages.csv |
  cut -d, -f 1-5
