# Runs show.go against this commit's Go module, linked to the C package scripts/prepare-packages.sh staged.
# The records come from the function's case.json, staged as the file show.go reads.
# The run fails when show.go's question or options differ from case.json, then
# turns the printed answers into the JSON every language's example must match.
set -e
root=$(cd ../../../../.. && pwd)
shown=$(pwd)/show.go
work=$(mktemp -d)
trap 'rm -rf "$work"' EXIT
jq -r '[.question?, .choices[]?, .labels[]?, .levels[]?, .kinds[]?,
        (.rules[]? | .source, .target, .relation),
        (.questions? // {} | keys[]),
        (.questions[]? | .question, .choices[]?, .levels[]?)]
       | .[] | select(. != null)' "$SITE_CASE_FILE" |
  while IFS= read -r wording; do
    grep -qF "\"$wording\"" "$shown" || { echo "show.go does not use \"$wording\" from case.json" >&2; exit 1; }
  done
jq -r '.records[]' "$SITE_CASE_FILE" > "$work/messages.txt"
cp "$shown" "$work/main.go"
printf 'module example\n\ngo 1.23\n\nrequire github.com/botassembly/thinkthen/sdk/go/thinkthen v0.2.0\n\nreplace github.com/botassembly/thinkthen/sdk/go/thinkthen => %s\n' \
  "$root/sdk/go/thinkthen" > "$work/go.mod"
export PKG_CONFIG_PATH="$root/target/site-packages/c/lib/pkgconfig"
export GOFLAGS=-mod=mod
# pkg-config leaves runtime lookup to the program; -rpath records where the library is.
export CGO_LDFLAGS="-Wl,-rpath,$(pkg-config --variable=libdir thinkthen-c)"
(cd "$work" && go run -tags thinkthen_system .) > "$work/output.txt"
cat "$work/output.txt"
jq -R -s -c --slurpfile case "$SITE_CASE_FILE" 'split("\n") | map(select(length > 0)) | map(split(" ") | {refund: (.[0] | fromjson), topic: .[1], urgency: (.[2] | fromjson)})' \
  "$work/output.txt" > "$SITE_ANSWERS_FILE"
