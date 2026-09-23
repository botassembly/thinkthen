#!/bin/sh
# Ask and record the method-H relate question sets that arms.py wrote.
#   Live:   sdlc/scripts/live --max-tokens N conformance/relate-h/job.sh
#   Replay: conformance/relate-h/job.sh replay   (no key, no network)
# Each qset-NN.json is one annotate run, recorded under the arm's cache/.
# A second live run answers from cache/ and sends nothing.
# RELATE_H_RUNS names another runs folder, for a rehearsal against a stub.
set -eu
here=$(cd -- "$(dirname -- "$0")" && pwd)
runs=${RELATE_H_RUNS:-$here/runs}
mode=live
case "${1:-}" in replay) mode=replay; shift ;; esac
for arm in "$runs"/*/H; do
	cd -- "$arm"
	mkdir -p cache
	for q in qset-*.json; do
		n=${q#qset-}; n=${n%.json}
		if [ "$mode" = replay ]; then
			thinkthen annotate "$q" --jsonl --field /e --details --replay cache --input records.jsonl > "replay-$n.jsonl"
		else
			thinkthen annotate "$q" --jsonl --field /e --details --cache cache --input records.jsonl > "run-$n.jsonl.part"
			mv "run-$n.jsonl.part" "run-$n.jsonl"
		fi
	done
done
