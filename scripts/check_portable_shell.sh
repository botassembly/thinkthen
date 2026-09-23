#!/usr/bin/env bash
# Shell spellings that break on macOS stay out of the tracked scripts
# (surfaces-review-7 R3-32, R4-10). Three spellings fail:
#   sed -i with no attached suffix: BSD sed reads the next word as the
#     suffix. Use perl -pi, or sed -i.bak and remove the .bak file.
#   %N in a date format: BSD date has no nanoseconds. Use perl's
#     Time::HiRes.
#   a bare timeout command: macOS ships it only as gtimeout. Resolve it
#     once into $TIMEOUT.
# Comment lines are skipped. A line that names a spelling as data ends
# with the comment "# portable-shell: data". The mark counts only in this
# checker and in scripts/test_gate_lib.sh, and only on its own line;
# anywhere else it is ignored (surfaces-review-7 verifier).
# With file arguments, only those files are read, and a missing one
# fails. With none, every tracked shell script is read; a tree without
# git, or one with no script, fails rather than passing empty.
# Usage: bash scripts/check_portable_shell.sh [FILE...]
set -euo pipefail
cd "$(dirname "$0")/.."
if [ $# -eq 0 ]; then
  listed=$(git ls-files '*.sh' 'sdlc/scripts/*' 2>/dev/null) \
    || { echo "FAIL     no git here to list the scripts; pass the files to read"; exit 1; }
  set -- $(printf '%s\n' "$listed" | while IFS= read -r f; do
    [ -n "$f" ] || continue
    case $f in *.sh) echo "$f" ;; *) head -n 1 "$f" | grep -qE '^#!.*\b(ba)?sh\b' && echo "$f" ;; esac
  done)
  [ $# -gt 0 ] || { echo "FAIL     no shell script to read"; exit 1; }
fi
forbidden="(^|[^-\$[:alnum:]_./])(sed[[:space:]]+(-i([[:space:]]|$)|--in-place)|timeout[[:space:]]+[-0-9\"'\$])|date [^|;]*%[0-9]*N"  # portable-shell: data
bad=0
for f in "$@"; do
  if [ ! -f "$f" ]; then
    echo "FAIL     $f: no such file"
    bad=1
    continue
  fi
  hits=$(grep -nE "$forbidden" "$f" | grep -vE '^[0-9]+:[[:space:]]*#' || true)
  case $f in
  scripts/check_portable_shell.sh | scripts/test_gate_lib.sh)
    hits=$(printf '%s\n' "$hits" | grep -vE '#[[:space:]]*portable-shell: data$' || true) ;;
  esac
  if [ -n "$hits" ]; then
    printf '%s\n' "$hits" | sed "s|^|FAIL     $f:|"
    bad=1
  fi
done
[ "$bad" -eq 0 ] && echo "ok:      $# shell scripts use no GNU-only sed -i, date %N, or bare timeout"  # portable-shell: data
exit "$bad"
