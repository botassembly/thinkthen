# Start a decide that waits on an input that stays open, then interrupt it.
dir=$(mktemp -d)
trap 'rm -rf "$dir"' EXIT
mkfifo "$dir/held"
thinkthen decide --from lines 'Is this urgent?' "$dir/held" &
pid=$!
# Opening the pipe waits until thinkthen opens it to read,
# which it does after it installs its Ctrl-C handler.
exec 3>"$dir/held"
kill -INT "$pid"
wait "$pid"
