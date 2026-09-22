#!/bin/sh
# Probe 9, the object arm: the same request with `state` as a real JSON object.
#
# The tool sends this shape itself now — the two pointers make one ordered
# object — so the body is the tool's own `--dry-run` plan unchanged. The arm
# is still posted by hand, so it opens its own connection, keeps no recording,
# and lives outside runs/. Each answer is saved under answers/ as the backend
# sent it.
#
# The key is never an argument and never printed. It is written into curl's
# configuration on standard input, which is the one channel curl reads it from
# without putting it in the process list.
#
#   sdlc/scripts/live --max-tokens 1000000 probes/09-evidence-shape/object.sh
set -eu

cd -- "$(dirname -- "$0")"

base=${THINKTHEN_BASE_URL:-https://api.typesafe.ai/v1}
question=$(cat question.txt)
mkdir -p answers
: >bodies.jsonl

while IFS= read -r case; do
	id=$(printf '%s\n' "$case" | jq -r .id)
	body=$(printf '%s\n' "$case" | thinkthen decide "$question" \
		--jsonl --field /subject --field /body --dry-run |
		jq -c '.request')
	printf '%s\n' "$body" >request.json
	printf 'header = "Authorization: Bearer %s"\n' "${THINKTHEN_API_KEY}" |
		curl --silent --show-error --fail-with-body --config - \
			--header 'content-type: application/json' \
			--data-binary @request.json \
			--output "answers/$id.json" \
			"$base/systemone" || {
		printf 'object: %s was refused, and its answer file holds the reply\n' "$id" >&2
		exit 4
	}
	jq -c -n --arg id "$id" --slurpfile sent request.json \
		'{id: $id, input_bytes: ($sent[0] | tojson | length)}' >>bodies.jsonl
done <cases.jsonl
rm -f request.json
printf 'wrote %s answers to answers/\n' "$(ls answers | wc -l | tr -d ' ')"
