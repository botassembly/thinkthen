work=$(mktemp -d)
trap 'rm -rf "$work"' EXIT
cp messages.csv questions.json show.sh "$work/" && cd "$work"
bash show.sh
