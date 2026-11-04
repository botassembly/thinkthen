echo 'I was charged twice and my parcel is late again.' |
  thinkthen choose --threshold 0.9 \
    --choice billing --choice shipping --choice other \
    'What is the message about?'
