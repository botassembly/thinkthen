#!/bin/sh
# Prove that a probe's job reproduces its committed rows from its recording
# alone, with no network and no key.
#
#   sh probes/replay-check.sh probes/01-find-vs-rank
#
# It runs the job with --replay into a fresh folder and compares every row
# against the committed one. Two fields are set aside, and every other byte
# must match. `meta.replayed` is true where the live row is false. `meta.tool`
# arrived with ticket 0012, after these rows were written, so the field is
# dropped from both rows and a row older than it still compares.
set -eu

probe=${1:?name the probe folder}
probe=$(CDPATH= cd -- "$probe" && pwd)
repo=$(CDPATH= cd -- "$(dirname -- "$0")/.." && pwd)

# The job calls the built binary by name. No rung builds it, so this check runs
# after a build, which is what sdlc/scripts/live already does before a job.
cargo build --locked --quiet --manifest-path "$repo/Cargo.toml" --package thinkthen
PATH="$repo/target/debug:$PATH"
export PATH
scratch=$(mktemp -d)
trap 'rm -rf -- "$scratch"' EXIT

unset THINKTHEN_API_KEY THINKTHEN_BASE_URL

OUT=$scratch sh -- "$probe/job.sh" --replay >/dev/null

status=0
for committed in "$probe"/runs/*.jsonl; do
	name=$(basename -- "$committed")
	replayed=$scratch/$name
	if [ ! -f "$replayed" ]; then
		printf 'replay-check: %s was not written\n' "$name" >&2
		status=1
		continue
	fi
	jq -c 'del(.meta.tool) | .meta.replayed = "set aside"' "$committed" >"$scratch/committed.norm"
	jq -c 'del(.meta.tool) | .meta.replayed = "set aside"' "$replayed" >"$scratch/replayed.norm"
	if diff -q "$scratch/committed.norm" "$scratch/replayed.norm" >/dev/null; then
		printf 'replay-check: %s reproduced %s rows\n' "$name" "$(wc -l <"$committed" | tr -d ' ')"
	else
		printf 'replay-check: %s differs\n' "$name" >&2
		diff "$scratch/committed.norm" "$scratch/replayed.norm" | head -5 >&2
		status=1
	fi
	if [ "$(jq -s 'map(select(.meta.replayed != true)) | length' "$replayed")" != 0 ]; then
		printf 'replay-check: %s holds a row that was not replayed\n' "$name" >&2
		status=1
	fi
done
exit "$status"
