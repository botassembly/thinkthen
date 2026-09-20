#!/bin/sh
# The token budget: which of the vendor's two published numbers is real?
#
# `sdlc/planning/interface-audit.md` row 66 records the disagreement. One page
# gives about 32,000 tokens for the whole request. Another gives 64,000 for the
# whole request with 32,000 for the state plus the longest question. With one
# question the two numbers cannot be told apart, so this check sends three
# questions: the state plus the longest question stays well under 32,000 while
# the whole request passes it. An accepted request settles it for the larger
# number, and a 422 settles it for the smaller one.
#
# The tool sends one question per request, so this arm is posted by hand. The
# key is never an argument and never printed. It goes into curl's configuration
# on standard input, which keeps it out of the process list.
#
#   sdlc/scripts/live --max-tokens 40000 probes/token-budget/job.sh
set -eu

cd -- "$(dirname -- "$0")"

base=${THINKTHEN_BASE_URL:-https://api.typesafe.ai/v1}

# Made-up depot text, repeated with a line number so no two lines are equal.
# About four characters to a token, so 80,000 characters is near 20,000 tokens
# for the state and 18,000 is near 4,500 for each question.
filler() {
	awk -v want="$1" -v tag="$2" '
	  { line[NR] = $0 }
	  END {
	    printed = 0
	    for (n = 1; printed < want; n++) {
	      text = tag " note " n ": " line[(n - 1) % NR + 1] "\n"
	      printf "%s", text
	      printed += length(text)
	    }
	  }' filler.txt
}

state=$(filler 80000 shift)
rubric=$(filler 18000 rule)

jq -n --arg state "$state" --arg rubric "$rubric" '
  def ask($text): {type: "noul", instructions: ($text + "\n\n" + $rubric)};
  {state: $state,
   model: "jev-latest",
   questions: {
     q1: ask("The notes above describe a shift that ended without an open problem."),
     q2: ask("The notes above describe a delivery that was chased and never arrived."),
     q3: ask("The notes above describe a battery that was swapped rather than charged.")
   }}' >request.json

printf 'request: %s bytes, about %s tokens at four bytes to a token\n' \
	"$(wc -c <request.json | tr -d ' ')" \
	"$(($(wc -c <request.json) / 4))"

printf 'header = "Authorization: Bearer %s"\n' "${THINKTHEN_API_KEY}" |
	curl --silent --show-error --config - \
		--header 'content-type: application/json' \
		--data-binary @request.json \
		--output answer.json \
		--write-out 'status %{http_code}\n' \
		"$base/systemone"

rm -f request.json
printf 'the backend answered:\n'
jq -c 'if .usage then {model, questions: (.answers | keys), usage} else . end' answer.json
