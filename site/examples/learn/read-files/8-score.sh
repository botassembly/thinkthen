urgency=$(thinkthen score 'How urgent is this document?' \
  a b --input documents --unit file --model local-1)
printf '%s\n' "$urgency"
