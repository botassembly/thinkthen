windows=$(thinkthen filter \
  'Does this line describe a refund?' \
  --input documents/01-policy.txt \
  --input documents/01-policy.txt --window 2 \
  --model local-1)
printf '%s\n' "$windows"
