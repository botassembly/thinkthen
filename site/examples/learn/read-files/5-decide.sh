contracts=$(thinkthen decide \
  'Does this document contain a support contract?' \
  --input documents --unit file --model local-1)
printf '%s\n' "$contracts"
