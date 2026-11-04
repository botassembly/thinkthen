# Works on a copy, so the example leaves no output file behind.
work=$(mktemp -d)
trap 'rm -rf "$work"' EXIT
cp survey.csv show.sh "$work/" && cd "$work"
bash show.sh
