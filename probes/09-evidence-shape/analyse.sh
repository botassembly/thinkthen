#!/bin/sh
# Read probe 9: does the object shape judge better than the flattened string?
#
# Run collect.sh first, which turns the object arm's saved answers into rows.
set -eu
cd -- "$(dirname -- "$0")"

rows=${1:-runs}

printf '=== the text arm, through transforms/score/score.jq at the default cut ===\n'
jq -n --argjson cut 0.5 -f ../../transforms/score/score.jq "$rows/text.jsonl" |
	jq -c '{rows, accuracy, true_positive, false_positive, true_negative, false_negative}'

printf '\n=== the two arms side by side ===\n'
jq -n --slurpfile text "$rows/text.jsonl" '
  def round4: . * 10000 | round | . / 10000;
  ($text | INDEX(.input.id)) as $string_arm
  | [inputs
     | .input.id as $id
     | {id: $id,
        label: .input.label,
        object_p: .p,
        string_p: $string_arm[$id].answer.probability}
     | .object_value = (.object_p >= 0.5)
     | .string_value = (.string_p >= 0.5)
     | .move = (.object_p - .string_p | round4)]
  | {rows: length,
     string_right: (map(select(.string_value == .label)) | length),
     object_right: (map(select(.object_value == .label)) | length),
     answers_that_differ: (map(select(.string_value != .object_value)
                               | {id, label, string_p, object_p})),
     unmoved: (map(select(.move == 0)) | length),
     mean_absolute_move: ((map(.move | fabs) | add) / length | round4),
     max_absolute_move: (map(.move | fabs) | max | round4)}' object-rows.jsonl

printf '\n=== what each arm cost in input tokens ===\n'
printf 'text:   '
jq -s -c 'map(.meta.usage.input_tokens) | {calls: length, input_tokens: add}' "$rows/text.jsonl"
printf 'object: '
jq -s -c 'map(.usage.input_tokens) | {calls: length, input_tokens: add}' object-rows.jsonl
