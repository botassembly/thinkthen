refund_files=$(thinkthen filter \
  'Does this line describe a refund?' \
  --input documents --unit line --files-only \
  --model local-1)
printf '%s\n' "$refund_files"
