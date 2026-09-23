# Sourced by scripts/check_surfaces.sh and scripts/test_gate_lib.sh: the
# gate's counting and its wire verdict. The caller owns the counters
# (green, skipped, diverged, failed, wire_ran, wire_lost), step_log, and
# wire_expected (a space-padded list of ports whose stub must answer).
# `skip` and `diverge` are defined once, in conformance/skiptable.py.

# Count one step's own result lines into the running totals.
count_lines() {
  local file=$1
  green=$((green + $(grep -cE '^(ok[[:space:]]|ok:|OK:)' "$file" || true)))
  # `# skip` is node --test, which prints a test's stdout as TAP comments
  # (surfaces-review-4: the covered cases never reached the totals and
  # node `# SKIP` lines were counted by no one).
  skipped=$((skipped + $(grep -cE '^(skip[[:space:]]|# skip )' "$file" || true)))
  diverged=$((diverged + $(grep -cE '^(diverge[[:space:]]|# diverge )' "$file" || true)))
  # `not ok` is TAP, which node --test prints; a node failure must land in
  # the failed total and not only in the step's exit status. `# fail N` is
  # node's own summary line, counted only when it names a failure.
  failed=$((failed + $(grep -cE '^(FAIL|not ok|# fail [1-9])' "$file" || true)))
}

# One surface's wire verdict: a counted skip line where the stub was
# expected is a lost wire suite, and a lost suite fails the gate.
wire_verdict() {
  local surface=$1 port=$2
  if grep -qE "^skip[[:space:]]+wire-$surface:" "$step_log"; then
    if case "$wire_expected" in *" $port "*) true ;; *) false ;; esac; then
      echo "FAIL     wire-$surface: the stub on $port was expected and the wire suite did not run"
      failed=$((failed + 1))
      wire_lost=$((wire_lost + 1))
      return 1
    fi
  else
    wire_ran=$((wire_ran + 1))
  fi
  return 0
}
