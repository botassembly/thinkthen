# Runs the shown command, then runs it again with JSON output and writes the
# answers that every other language's example must match.
set -e
bash -o pipefail show.sh
THINKTHEN_JSON=true bash -o pipefail show.sh |
  jq -s -c 'map(.answer | map({source: .source.text, relation, target: .target.text}))' > "$SITE_ANSWERS_FILE"
