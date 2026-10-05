categories=$(thinkthen choose \
  'Which category fits this document?' billing support \
  --input documents --unit file --model local-1)
printf '%s\n' "$categories"
