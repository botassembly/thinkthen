set -o pipefail
seq 100000 |
  thinkthen decide --from lines 'Is this an even number?' |
  head -n 1
