thinkthen backends check \
  --url http://localhost:8080/v1 \
  --plan |
grep '^request' |
cut -d ' ' -f 1-2
