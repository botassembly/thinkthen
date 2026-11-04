printf '%s' 'I was charged twice and my parcel is late again.' |
  thinkthen tag --json --label billing --label shipping \
    'Which topics appear?' |
  jq .
