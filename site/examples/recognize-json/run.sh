printf '%s' 'Morgan works for Example Group.' |
  thinkthen recognize --json --kind person --kind organization |
  jq '.answer'
