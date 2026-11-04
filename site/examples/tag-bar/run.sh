echo 'I was charged twice and my parcel is late again.' |
  thinkthen tag --threshold 0.85 \
    --label billing --label shipping \
    'Which topics appear?'
