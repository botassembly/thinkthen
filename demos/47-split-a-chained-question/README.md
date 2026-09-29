# How to split a chained question into two calls

Status: green

Verbs: `choose`

Use this when one question asks for a fact through another fact. In this public music-catalog example, the first call finds a song's album. The second asks for that album's year. The two `choose` calls and their options are in [run.sh](run.sh).

```bash
set -euo pipefail
bash run.sh 'I Me Mine' | mustmatch 'I Me Mine -> Let It Be -> 1970'
```

## Input

`run.sh` asks both questions with `thinkthen choose` and explicit options. Its three saved exchanges in `recording/` contain two first-hop album answers and one album-year answer. The script uses `--replay recording/`, so this example requires no key and sends no request. The saved text names public songs, albums and years. The first-hop exchanges come from the public Beatles Bench run of 2026-09-26; the second comes from experiment 297's recorded pipeline against the same backend and model alias. Their exact request bodies, not just their answers, match this script.

## Step 1: keep the first answer

The first `choose` returns an option such as `d`. The shell reads its exit code before using the option. It maps the chosen label to the album description attached to that option. If the top choices tie, `choose --raw` prints nothing and exits 3; the script prints `needs review` and stops before asking for a year.

```bash
set -euo pipefail
result=$(bash run.sh "It Won't Be Long" 2>&1) && code=0 || code=$?
printf '%s | %s\n' "$code" "$result" | mustmatch '3 | needs review: first album not sure'
```

## Step 2: pass that album into the next call

The script writes the selected album as the second call's entire text input and supplies every candidate year as a `choose` option. “Let It Be” returns `1970` from the saved exchange. A different album needs its own matching recording or a live call; replay never invents an answer. Repeated albums can share a complete cached exchange only when the question, options, model, backend and encoded request bytes match.

The measured album-year cohort in experiment 297 answered 22 of 60 chained questions directly. The two-call pipeline answered 51 of the 59 cases with a usable first hop; one tied first hop had no second call. Counting that tied case as wrong gives 51 of 60 for the full cohort. Those numbers describe one saved run, not a general accuracy guarantee. Its second-hop cache already held repeated album requests, so its near-equal cost does not promise equal cost for a cold run.

## What can go wrong

- **A not sure first answer needs review.** Exit 3 and an empty raw answer stop this script before the second call. Do not turn that missing answer into a blank album or drop its case from a cohort silently.
- **A failed call is not an answer.** Usage or input errors exit 2, a backend failure exits 4, and a missing or damaged replay entry exits 5. The script does not continue to the next hop after any of them.
- **A cache hit has an exact identity.** Changing the album spelling, options, model, address or request framing can miss. The first successful replay here returns the saved answer, but a new cold run may send two requests.

## Related how-tos

- [How to branch on a label with `choose` and `case`](../02-route-a-ticket/)
- [How to test a script with no network](../27-test-with-no-network/)
- [How to tune a question file and use the same file in the gate](../41-tune-a-question-file/)
