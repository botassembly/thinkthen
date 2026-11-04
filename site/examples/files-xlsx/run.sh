# Work on a copy, so the new workbook stays out of here.
work=$(mktemp -d)
trap 'rm -rf "$work"' EXIT
cp messages.xlsx show.sh "$work" && cd "$work"
bash show.sh
