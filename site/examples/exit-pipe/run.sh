set -o pipefail
# head prints the first answer and exits. The last ten numbers wait
# until head has gone, so thinkthen's next write finds the pipe closed.
dir=$(mktemp -d)
trap 'rm -rf "$dir"' EXIT
mkfifo "$dir/gone"
{ seq 50; cat "$dir/gone"; seq 51 60; } |
  thinkthen decide --from lines 'Is this an even number?' |
  { head -n 1; exec <&-; : >"$dir/gone"; }
