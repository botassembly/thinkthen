# Runs show.java against the jars scripts/prepare-packages.sh staged, with Java 22.
# The records come from the function's case.json, staged as the file show.java reads.
# The run fails when show.java's question or options differ from case.json, then
# turns the printed answers into the JSON every language's example must match.
set -e
packages=$(cd ../../../../../target/site-packages && pwd)
shown=$(pwd)/show.java
work=$(mktemp -d)
trap 'rm -rf "$work"' EXIT
jq -r '[.question?, .choices[]?, .labels[]?, .levels[]?, .kinds[]?,
        (.rules[]? | .source, .target, .relation),
        (.questions? // {} | keys[]),
        (.questions[]? | .question, .choices[]?, .levels[]?)]
       | .[] | select(. != null)' "$SITE_CASE_FILE" |
  while IFS= read -r wording; do
    grep -qF "\"$wording\"" "$shown" || { echo "show.java does not use \"$wording\" from case.json" >&2; exit 1; }
  done
jq -j '.records | if length == 1 then .[0] else error("expected one record") end' "$SITE_CASE_FILE" > "$work/message.txt"
(cd "$work" && "$THINKTHEN_BUILD_TOOLS/jvm/jdk-22.0.2+9/bin/java" --enable-native-access=ALL-UNNAMED \
  -cp "$packages/jvm/lib/*" "$shown") > "$work/output.txt"
cat "$work/output.txt"
jq -R -s -c --slurpfile case "$SITE_CASE_FILE" 'split("\n") | map(select(length > 0)) | map(if . == "no line answers" then null else . end)' \
  "$work/output.txt" > "$SITE_ANSWERS_FILE"
