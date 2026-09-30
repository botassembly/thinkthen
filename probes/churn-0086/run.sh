#!/bin/sh
# Run the churn probe RUNS times, PARALLEL at a time (8 by default) at nice
# 19, and count how each run ended. `run.sh main` builds against this
# checkout's public API.
# `run.sh tag CHECKOUT` builds against the stand-in library of a checkout of
# tag surfaces-wave7-final, such as one made with `git archive`.
# Before each batch it waits while the one-minute load is above 10.
set -eu

MODE=${1:?main or tag}
HERE=$(CDPATH= cd -- "$(dirname -- "$0")" && pwd)
RUNS=${RUNS:-300}
PARALLEL=${PARALLEL:-8}
TARGET=${CARGO_TARGET_DIR:-${TMPDIR:-/tmp}/churn-0086-$MODE}
if [ "$MODE" = tag ]; then
	CHECKOUT=${2:?the tag checkout}
	mkdir -p "$CHECKOUT/probes/churn-0086"
	cp -R "$HERE/src" "$HERE/Cargo.toml" "$CHECKOUT/probes/churn-0086/"
	cp "$CHECKOUT/libraries/rust/Cargo.lock" "$CHECKOUT/probes/churn-0086/"
	sed -i 's#path = "../../crates/thinkthen", default-features = false, features = \["bundled-sqlite"\]#path = "../../libraries/rust"#' \
		"$CHECKOUT/probes/churn-0086/Cargo.toml"
	cd "$CHECKOUT/probes/churn-0086"
	CARGO_TARGET_DIR=$TARGET cargo build --release --offline --features standin
else
	cd "$HERE"
	CARGO_TARGET_DIR=$TARGET cargo build --release --offline --locked
fi

LOG=$TARGET/ends.txt
: >"$LOG"
done_runs=0
while [ "$done_runs" -lt "$RUNS" ]; do
	while [ "$(cut -d. -f1 /proc/loadavg)" -ge 10 ]; do sleep 5; done
	batch=0
	while [ "$batch" -lt "$PARALLEL" ] && [ "$done_runs" -lt "$RUNS" ]; do
		(
			set +e
			env -u THINKTHEN_API_KEY -u THINKTHEN_BASE_URL RUST_BACKTRACE=1 nice -n 19 \
				"$TARGET/release/churn-0086" >"$TARGET/run-$$-$done_runs.out" 2>&1
			ended=$?
			printf '%s\n' "$ended" >>"$LOG"
			# Keep the output of a run that did not end cleanly.
			[ "$ended" -ne 0 ] || rm -f "$TARGET/run-$$-$done_runs.out"
		) &
		batch=$((batch + 1))
		done_runs=$((done_runs + 1))
	done
	wait
done
printf '%s runs: ' "$RUNS"
sort "$LOG" | uniq -c | awk '{printf "%s ended %s; ", $1, $2}'
printf '\n'
