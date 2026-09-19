# 15 Find the line

Status: red

Verbs: `find`

A desk keeps a one-page policy and answers customers from it. Somebody asks when the money comes back. The answer is one line of the policy, and a person reads the whole page to find it. `find` reads the page once and points at the line.

`find` is Draft. ADR 0009 item 3 plans it and leaves one question open, and this page argues that question.

Every number in an expected output on this page is illustrative until a recording exists. No block asserts on a probability.

## Input

`policy.txt` is a twelve-line shop policy, one rule per line.

## One question, one request

`find` reads up to 255 lines, sends them together with an id on each, and asks which one best answers. That is one request where `filter` and `rank` would make twelve.

```bash
set -euo pipefail

thinkthen find 'When does a refund reach the customer?' \
  --lines --input policy.txt --replay recording/ \
  | mustmatch "Refunds reach the original payment method within five working days."
```

The line comes back as it arrived, the way `filter` returns a record. Nothing is rewritten and nothing is summarised.

The units see each other, and that is the reason to choose this verb. `filter` asks about each line alone and would keep two lines. `find` is asked for the best one present.

```bash
set -euo pipefail
work=$(mktemp -d)
trap 'rm -rf -- "$work"' EXIT

env -u TYPESAFE_API_KEY thinkthen find 'When does a refund reach the customer?' \
  --lines --dry-run --input policy.txt > "$work/plan.json"

jq -r '.request.questions | length' "$work/plan.json" | mustmatch "1"
jq -r '.request.state | length' "$work/plan.json" | mustmatch "12"
```

One question, twelve units, one request. The plan is how a user checks that the whole page left the machine, because it did.

## The three best lines

`--top 3` prints three lines, best first. The request is the same one.

```bash
set -euo pipefail

thinkthen find 'What does a customer have to do to return something?' \
  --lines --top 3 --input policy.txt --replay recording/ \
  | mustmatch "Returns are accepted within 30 days of delivery.
Items must be unused and in the original packaging.
The customer pays return postage unless the item arrived damaged."
```

## When the page does not answer

The policy says nothing about warranties. A tool that always returns its best line would hand the desk a wrong answer with no warning.

```bash
set -euo pipefail
work=$(mktemp -d)
trap 'rm -rf -- "$work"' EXIT

thinkthen find 'How long is the manufacturer warranty?' \
  --lines --input policy.txt --replay recording/ \
  > "$work/hit.txt" && rc=0 || rc=$?

printf 'rc=%s\n' "$rc" | mustmatch "rc=3"
wc -c < "$work/hit.txt" | tr -d ' ' | mustmatch "0"
```

Nothing on standard output and exit 3. A desk reads the code and falls back to a person.

```bash
set -euo pipefail

if answer=$(thinkthen find 'How long is the manufacturer warranty?' \
              --lines --input policy.txt --replay recording/)
then
  printf 'quote: %s\n' "$answer"
else
  printf 'ask a person\n'
fi | mustmatch "ask a person"
```

The recording under `recording/` does not exist yet.

## What this demo decides

- **Nothing fits should be a "none" option, and the demo is written that way.** ADR 0009 leaves three choices open. A `none` option costs no second request, and the tool already has a word for the outcome: the answer is unresolved, standard output is empty, and the exit code is 3, exactly as `choose` behaves. A second yes/no question doubles the request and asks about the set rather than about a unit, and it can disagree with the pick it is meant to guard. A cut on the vendor's `confidence` rests on a formula ADR 0009 item 2 calls unpublished, so nobody could say what the number meant. The demo argues for the `none` option and asks that `find` inherit `--threshold` and exit 3 from `choose` rather than growing a rule of its own.
- **The demo could not say what `find` prints.** ADR 0009 says `find` reads lines or records and never says what comes out. This page prints the line byte for byte, as `filter` does. Under `--jsonl` it would print the record. The surface has to fix it, and `--details` needs an answer kind too.
- **`find` sends the whole page and the plan is the only warning.** `--field` narrows a record and nothing narrows a page. A user who runs `find` over a private document sends all of it in one request. The demo asks that the `find` help lead with that fact, because the verb is the one place in the surface where the whole input leaves in one go.
- **The 255-unit ceiling has no visible edge.** A thirteen-line policy works and a three-hundred-line one does not. ADR 0009 does not say whether the run is refused or the file is cut. The demo asks for a refusal before any request, naming the count, in the same words `segment` uses for a document too large.
- **`--top 3` is free here and that is worth saying.** One request answered the whole page, so printing three lines costs nothing more than printing one. The `--top` row in the `rank` help says the opposite for `rank`, and the two verbs should say so plainly next to each other.
