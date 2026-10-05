refunds=$(thinkthen filter \
  'Does this line describe a refund?' \
  --input documents --unit line --model local-1)
printf '%s\n' "$refunds"
