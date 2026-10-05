policy=$(thinkthen find \
  'Which line gives the refund policy?' \
  --input documents --unit line --model local-1)
printf '%s\n' "$policy"
