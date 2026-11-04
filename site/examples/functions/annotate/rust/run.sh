# Compile the unchanged example against the shared staged rust package.
set -eu
root=$(cd ../../../../.. && pwd)
packages=${SITE_PACKAGES:-$root/target/site-packages}
shown=$(pwd)/show.rs
work=$(mktemp -d)
trap 'rm -rf "$work"' EXIT
# Keep the question and options aligned with the shared capture case.
jq -r '[.question?, .choices[]?, .labels[]?, .levels[]?, .kinds[]?,
        (.rules[]? | .source, .target, .relation),
        (.questions? // {} | keys[]),
        (.questions[]? | .question, .choices[]?, .levels[]?)]
       | .[] | select(. != null)' "$SITE_CASE_FILE" |
  while IFS= read -r wording; do
    grep -qF "\"$wording\"" "$shown" || { echo "show.rs does not use \"$wording\" from case.json" >&2; exit 1; }
  done
# Rust examples carry their records as string literals.
jq -c '.records[]' "$SITE_CASE_FILE" |
  while IFS= read -r record; do
    grep -qF "$record" "$shown" || { echo 'show.rs does not use a record from case.json' >&2; exit 1; }
  done
rustc --edition=2024 --extern thinkthen="$packages/rust/libthinkthen.rlib" \
  -L "dependency=$packages/rust/deps" "$shown" -o "$work/example"
(cd "$work" && ./example) > "$work/output.txt"
cat "$work/output.txt"
jq -R -s -c --slurpfile case "$SITE_CASE_FILE" 'split("\n") | map(select(length > 0)) | map(capture("^(?<refund>true|false) \"?(?<topic>[^\" ]+)\"? (?<urgency>[0-9.eE+-]+)$") | {refund: (.refund | fromjson), topic, urgency: (.urgency | tonumber)})' \
  "$work/output.txt" > "$SITE_ANSWERS_FILE"
