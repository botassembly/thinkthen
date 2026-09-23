#!/usr/bin/env bash
# The interrupt-and-hook proof, parent half (surfaces-review-7, R7-13).
# A Ctrl-C during a call must meet a user's options(error = ...) hook the
# way a Ctrl-C during plain R code meets it. The parent runs the child
# with each body, signals half a second after "ready", and requires the
# thinkthen run's lines to equal the plain run's lines. Uncaught, both
# run the hook once and continue. Caught by tryCatch(interrupt = ...),
# neither runs the hook (the third review's item 20c). Offline; check.sh
# calls it.
set -euo pipefail
cd "$(dirname "$0")"
export ENGINE_NULL=1

run_child() { # body catch
  local out child
  out=$(mktemp)
  HOOK_BODY=$1 HOOK_CATCH=$2 Rscript interrupt_hook_child.R >"$out" 2>&1 </dev/null &
  child=$!
  for _ in $(seq 1 600); do
    grep -q '^ready$' "$out" && break
    sleep 0.1
  done
  sleep 0.5
  kill -INT "$child"
  local status=0
  wait "$child" 2>/dev/null || status=$?
  echo "exit $status"
  grep -E '^(HOOK RAN|CAUGHT|COMPLETED|AFTER)$' "$out" || true
  rm -f "$out"
}

fail=0
for catch in 0 1; do
  plain=$(run_child plain "$catch")
  engine=$(run_child thinkthen "$catch")
  if [ "$plain" != "$engine" ]; then
    echo "FAIL: with catch=$catch a Ctrl-C during a call gave"
    echo "$engine" | sed 's/^/  /'
    echo "and during plain R code gave"
    echo "$plain" | sed 's/^/  /'
    fail=1
  else
    echo "ok    catch=$catch: a Ctrl-C during a call meets the hook as plain R does ($(echo "$plain" | paste -sd ' '))"
  fi
done
exit "$fail"
