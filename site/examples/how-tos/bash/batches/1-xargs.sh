asks_for_refund() {
  thinkthen decide @refund.json \
    --quiet \
    --input "$1" &&
  echo "$1"
}
export -f asks_for_refund

ls tickets/*.txt |
xargs -P 4 -I {} bash -c 'asks_for_refund {}' |
sort
