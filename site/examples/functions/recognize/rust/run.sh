# --prepare is forwarded to the shared language launcher.
set -eu
shown="$PWD/show.rs"
SITE_CASE_FILE=${SITE_CASE_FILE:-$PWD/../case.json}
export SITE_CASE_FILE
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
SITE_FUNCTION_NORMALIZER="$PWD/normalize.sh"
export SITE_FUNCTION_NORMALIZER
. ../../../lang-rust/run.sh
