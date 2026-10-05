billing=$(thinkthen rank \
  'Does this document discuss a billing dispute?' \
  --input documents --unit file --model local-1)
printf '%s\n' "$billing"
