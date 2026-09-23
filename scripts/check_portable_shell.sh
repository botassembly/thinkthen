#!/usr/bin/env bash
# Shell spellings that break on macOS stay out of the tracked scripts
# (surfaces-review-7 R3-32, R4-10). Three spellings fail:
#   sed -i with no attached suffix: BSD sed reads the next word as the
#     suffix. Use perl -pi, or sed -i.bak and remove the .bak file.
#   %N in a date format: BSD date has no nanoseconds. Use perl's
#     Time::HiRes.
#   a bare timeout command: macOS ships it only as gtimeout. Resolve it
#     once into $TIMEOUT.
# Comment lines are skipped. A line that names a spelling as data (this
# checker's own summary, a test's planted fixture) ends with the comment
# "# portable-shell: data" and is skipped too. The mark covers its own line
# only, so a real use elsewhere in the same file still fails.
# With file arguments, only those files are
# read; the gate passes none and reads every tracked shell script.
# Usage: bash scripts/check_portable_shell.sh [FILE...]
set -euo pipefail
cd "$(dirname "$0")/.."
if [ $# -eq 0 ]; then
  set -- $(git ls-files '*.sh' 'sdlc/scripts/*' | while IFS= read -r f; do
    case $f in *.sh) echo "$f" ;; *) head -n 1 "$f" | grep -qE '^#!.*\b(ba)?sh\b' && echo "$f" ;; esac
  done)
fi
bad=0
for f in "$@"; do
  hits=$(grep -nE "(^|[^-\$[:alnum:]_./])(sed -i([[:space:]]|$)|timeout[[:space:]]+[0-9])|date [^|;]*%N" "$f" \
    | grep -vE '^[0-9]+:[[:space:]]*#' | grep -vE '#[[:space:]]*portable-shell: data$' || true)
  if [ -n "$hits" ]; then
    printf '%s\n' "$hits" | sed "s|^|FAIL     $f:|"
    bad=1
  fi
done
[ "$bad" -eq 0 ] && echo "ok:      $# shell scripts use no GNU-only sed -i, date %N, or bare timeout"  # portable-shell: data
exit "$bad"
