names=$(thinkthen recognize person organization \
  --input documents --unit file --model local-1)
printf '%s\n' "$names"
