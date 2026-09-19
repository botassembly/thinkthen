#!/bin/sh
# Read probe 8: how often each arm picked the trusted label, and where they
# disagreed.
set -eu
cd -- "$(dirname -- "$0")"

rows=${1:-runs}

printf '=== each arm against the trusted label ===\n'
for arm in bare described; do
	printf '%s: ' "$arm"
	jq -n '[inputs | {right: (.value == .input.label), label: .input.label, value}]
	       | {rows: length,
	          right: (map(select(.right)) | length),
	          accuracy: ((map(select(.right)) | length) / length * 10000 | round | . / 10000),
	          wrong_by_label: (map(select(.right | not))
	                           | group_by(.label)
	                           | map({label: .[0].label, wrong: length})
	                           | sort_by(- .wrong))}' "$rows/$arm.jsonl"
done

printf '\n=== where the two arms disagreed ===\n'
jq -n --slurpfile bare "$rows/bare.jsonl" '
  ($bare | INDEX(.input.id)) as $before
  | [inputs
     | {id: .input.id, label: .input.label,
        bare: $before[.input.id].value, described: .value}]
  | {rows: length,
     same: (map(select(.bare == .described)) | length),
     changed: (map(select(.bare != .described)) | length),
     changed_to_right: (map(select(.bare != .label and .described == .label)) | length),
     changed_to_wrong: (map(select(.bare == .label and .described != .label)) | length),
     changed_wrong_to_wrong: (map(select(.bare != .described and .bare != .label
                                         and .described != .label)) | length),
     rows_that_changed: (map(select(.bare != .described)))}' \
	"$rows/described.jsonl"

printf '\n=== the winning probability, arm by arm ===\n'
for arm in bare described; do
	printf '%s: ' "$arm"
	jq -n '[inputs | {right: (.value == .input.label),
	                  p: (.answer.probabilities | to_entries | map(.value) | max)}]
	       | {mean_winning_p: ((map(.p) | add) / length * 10000 | round | . / 10000),
	          mean_when_right: ((map(select(.right) | .p) | add)
	                            / (map(select(.right)) | length) * 10000 | round | . / 10000),
	          mean_when_wrong: (if (map(select(.right | not)) | length) == 0 then null
	                            else (map(select(.right | not) | .p) | add)
	                                 / (map(select(.right | not)) | length) * 10000 | round | . / 10000
	                            end)}' "$rows/$arm.jsonl"
done
