# Runs in a scratch folder, so kept.txt never lands in the example folder.
here=$PWD
work=$(mktemp -d)
trap 'rm -rf "$work"' EXIT
cp notes.txt "$work/" && cd "$work" && bash "$here/show.sh"
