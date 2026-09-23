# Sourced by scripts/check_surfaces.sh and scripts/test_gate_lib.sh: the
# gate's counting and its wire verdict. The caller owns the counters
# (green, skipped, diverged, failed, wire_ran, wire_lost), step_log, and
# wire_expected (a space-padded list of ports whose stub must answer).
# `skip` and `diverge` are defined once, in conformance/skiptable.py.

# Count one step's own result lines into the running totals.
count_lines() {
  local file=$1
  # A TAP `ok N - name # SKIP` or `# TODO` line is a skip, never green
  # (surfaces-review-5: node's skipped tests were counted green).
  local tap_skip='^ok[[:space:]].*#[[:space:]]*(SKIP|TODO)'
  green=$((green + $(grep -E '^(ok[[:space:]]|ok:|OK:)' "$file" | grep -cvE "$tap_skip" || true)))
  # `# skip` is node --test, which prints a test's stdout as TAP comments
  # (surfaces-review-4: the covered cases never reached the totals and
  # node `# SKIP` lines were counted by no one).
  skipped=$((skipped + $(grep -cE "^(skip[[:space:]]|# skip )|$tap_skip" "$file" || true)))
  diverged=$((diverged + $(grep -cE '^(diverge[[:space:]]|# diverge )' "$file" || true)))
  # `not ok` is TAP, which node --test prints; a node failure must land in
  # the failed total and not only in the step's exit status. `# fail N` is
  # node's own summary line, counted only when it names a failure.
  failed=$((failed + $(grep -cE '^(FAIL|not ok|# fail [1-9])' "$file" || true)))
}

# One surface's wire verdict: a counted skip line where the stub was
# expected is a lost wire suite, and a lost suite fails the gate. A wire
# suite counts as run only when its surface passed: a surface that failed
# before its wire section never ran it (surfaces-review-5).
wire_verdict() {
  local surface=$1 port=$2 status=${3:-0}
  if grep -qE "^skip[[:space:]]+wire-$surface:" "$step_log"; then
    if wire_expected_on "$port"; then
      echo "FAIL     wire-$surface: the stub on $port was expected and the wire suite did not run"
      failed=$((failed + 1))
      wire_lost=$((wire_lost + 1))
      return 1
    fi
  elif [ "$status" -eq 0 ]; then
    wire_ran=$((wire_ran + 1))
  fi
  return 0
}

# Whether this run expects the stub on a port (the gate started it or
# found it up with the asked delay).
wire_expected_on() {
  case "$wire_expected" in *" $1 "*) return 0 ;; *) return 1 ;; esac
}

# Start one stub on a port, or reuse a running one that carries the asked
# delay. The caller owns stub_bin, stub_pids, failed, fail, and
# wire_expected.
start_stub() {
  local port=$1 delay=$2 page
  if page=$(curl -sf --max-time 1 "http://127.0.0.1:$port/v1/stats" 2>/dev/null); then
    # Reuse a running stub only when it carries the asked delay: the
    # interrupt and cancel proofs need it (surfaces-review-5).
    local held
    held=$(printf '%s' "$page" | sed -n 's/.*"delay_ms":\([0-9]*\).*/\1/p')
    if [ "$held" != "$delay" ]; then
      echo "FAIL     stub on $port: it answers with delay '${held:-unreported}' ms and this run needs $delay ms; stop it or free the port"
      failed=$((failed + 1))
      fail=1
      return
    fi
    echo "stub already up on $port with delay ${delay}ms; using it"
    wire_expected="$wire_expected$port "
    return
  fi
  if [ ! -x "$stub_bin" ]; then
    echo "no stub binary; wire suites on $port will skip"
    return
  fi
  STUB_PORT=$port STUB_DELAY_MS=$delay "$stub_bin" >/dev/null 2>&1 &
  stub_pids+=($!)
  local _
  for _ in $(seq 1 30); do
    if curl -sf --max-time 1 "http://127.0.0.1:$port/v1/stats" >/dev/null 2>&1; then
      echo "stub up on $port (delay ${delay}ms)"
      wire_expected="$wire_expected$port "
      return
    fi
    sleep 0.1
  done
  echo "stub on $port did not answer; its wire suites will skip"
}
