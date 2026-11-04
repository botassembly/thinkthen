. ../../../lang-typescript/run.sh
if [ "${1:-}" = --prepare ]; then
  mv "$work/run.sh" "$work/execute.sh"
  cat > "$work/run.sh" <<'RUN'
#!/bin/sh
set -eu
entry=$(dirname "$0")
work=$(mktemp -d)
trap 'status=$?; trap - EXIT; set +e; rm -rf "$work"; exit "$status"' EXIT
sh "$entry/execute.sh" > "$work/output.txt"
cat "$work/output.txt"
jq -R -s -c --slurpfile case "$SITE_CASE_FILE" 'split("\n") | map(select(length > 0) | fromjson)' \
  "$work/output.txt" > "$SITE_ANSWERS_FILE"
RUN
else
  jq -R -s -c --slurpfile case "$SITE_CASE_FILE" 'split("\n") | map(select(length > 0) | fromjson)' \
    "$work/output.txt" > "$SITE_ANSWERS_FILE"
fi
