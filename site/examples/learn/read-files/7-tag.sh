labels=$(thinkthen tag 'Which labels apply?' \
  refund contract --input documents \
  --unit file --model local-1)
printf '%s\n' "$labels"
