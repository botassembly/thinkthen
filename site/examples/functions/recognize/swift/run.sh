# Compile only in preparation; capture and replay execute this retained program.
set -euo pipefail
[ "${SITE_EXECUTE_ONLY:-}" != 1 ] || { echo 'Use the prepared function executable' >&2; exit 2; }
if [ "${1:-}" = --prepare ]; then program=$2; mkdir -p "$program"; else
  program=$(mktemp -d)
  trap 'status=$?; trap - EXIT; set +e; rm -rf "$program"; exit "$status"' EXIT
fi
bash ../../../lang-swift/run.sh --prepare "$program"
cat > "$program/run.sh" <<'RUN'
set -euo pipefail
program=$(CDPATH= cd -- "$(dirname -- "$0")" && pwd)
shown=$program/show.swift
work=$(mktemp -d)
trap 'status=$?; trap - EXIT; set +e; rm -rf "$work"; exit "$status"' EXIT
jq -r '[.question?, .choices[]?, .labels[]?, .levels[]?, .kinds[]?,
        (.rules[]? | .source, .target, .relation),
        (.questions? // {} | keys[]),
        (.questions[]? | .question, .choices[]?, .levels[]?)]
       | .[] | select(. != null)' "$SITE_CASE_FILE" |
  while IFS= read -r wording; do
    grep -qF "\"$wording\"" "$shown" || { echo "show.swift does not use \"$wording\" from case.json" >&2; exit 1; }
  done
jq -j '.records | if length == 1 then .[0] else error("expected one record") end' "$SITE_CASE_FILE" > "$work/message.txt"
(cd "$work" && "$program/example") > "$work/output.txt"
cat "$work/output.txt"
jq -R -s -c --slurpfile case "$SITE_CASE_FILE" '[split("\n") | map(select(length > 0)) | map(capture("^(?<kind>[^:]+): (?<text>.*)$") | {text, kind})]' \
  "$work/output.txt" > "$SITE_ANSWERS_FILE"
RUN
[ "${1:-}" = --prepare ] || bash "$program/run.sh"
