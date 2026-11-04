# Saves the rows the way the previous step does, then counts them from the file.
here=$PWD
work=$(mktemp -d)
trap 'rm -rf "$work"' EXIT
cp ../recipe-survey-rows/survey.txt ../recipe-survey-rows/show.sh "$work/" && cd "$work"
bash show.sh > /dev/null
bash "$here/show.sh"
