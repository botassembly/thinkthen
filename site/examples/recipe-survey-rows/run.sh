# Runs in a scratch folder, so themes.jsonl never lands in the example folder.
here=$PWD
work=$(mktemp -d)
trap 'rm -rf "$work"' EXIT
cp survey.txt "$work/" && cd "$work" && bash "$here/show.sh"
