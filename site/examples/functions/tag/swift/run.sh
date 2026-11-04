# Compile the unchanged example against the shared staged swift package.
set -eu
root=$(cd ../../../../.. && pwd)
packages=${SITE_PACKAGES:-$root/target/site-packages}
shown=$(pwd)/show.swift
work=$(mktemp -d)
trap 'rm -rf "$work"' EXIT
# Keep the question and options aligned with the shared capture case.
jq -r '[.question?, .choices[]?, .labels[]?, .levels[]?, .kinds[]?,
        (.rules[]? | .source, .target, .relation),
        (.questions? // {} | keys[]),
        (.questions[]? | .question, .choices[]?, .levels[]?)]
       | .[] | select(. != null)' "$SITE_CASE_FILE" |
  while IFS= read -r wording; do
    grep -qF "\"$wording\"" "$shown" || { echo "show.swift does not use \"$wording\" from case.json" >&2; exit 1; }
  done
jq -j '.records | if length == 1 then .[0] else error("expected one record") end' "$SITE_CASE_FILE" > "$work/message.txt"
build=$(cat "$packages/swift/module-path")
compiler=$(cat "$packages/swift/compiler-path")
# SwiftPM's resource accessor looks beside the consumer executable.
for resources in "$build/ThinkThen_ThinkThen.resources" "$build/ThinkThen_ThinkThen.bundle"; do
  if [ -d "$resources" ]; then ln -s "$resources" "$work/"; fi
done
"$compiler" -swift-version 6 -I "$build/Modules" -I "$build/CThinkThen.build" \
  "$shown" "$build"/ThinkThen.build/*.o "$build"/CThinkThen.build/*.o -o "$work/example"
(cd "$work" && ./example) > "$work/output.txt"
cat "$work/output.txt"
jq -R -s -c --slurpfile case "$SITE_CASE_FILE" 'split("\n") | map(select(length > 0)) | map(fromjson | sort)' \
  "$work/output.txt" > "$SITE_ANSWERS_FILE"
