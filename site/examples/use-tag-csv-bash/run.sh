# Work on a copy so the example can run again.
work=$(mktemp -d)
trap 'rm -rf "$work"' EXIT
cp messages.csv show.sh "$work" && cd "$work"
bash show.sh
