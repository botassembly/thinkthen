# probes/

Ticket 0011 ran six measurements against the hosted decider model, and ticket 0017 ran four more. Each folder holds its cases, the job that judged them, the rows the job wrote, the recorded exchanges, and the analysis that reads the rows. `sdlc/records/0011-the-live-probe.md` holds the numbers and what each one changes.

Every case is made-up text written for its ticket, with a trusted answer fixed and committed before any call went out. No case is real customer text and no case names anything private.

Two checks cannot be replayed, and their folders say so at the top of the script. `09-evidence-shape/object.sh` posts a request shape the tool cannot send, and `token-budget/` posts three questions in one request, which the tool never does. Both keep the backend's answers as files instead of a recording. `token-budget/` carries no leading number, so the `spec` rung's replay check passes it by.

| Folder | Asks |
| --- | --- |
| `01-find-vs-rank/` | Does one request over many lines pick as well as one request per line |
| `02-confidence/` | Does the backend's `confidence` separate right from wrong better than the winning probability |
| `03-score/` | How well `score` places a text on five levels, beside `choose` over the same five labels |
| `04-option-order/` | Does reversing or shuffling the options move the pick |
| `05-irrelevant-option/` | Does one added label that fits nothing move the pick |
| `06-hostile-text/` | Does an instruction aimed at the judge, inside the evidence, move the answer |
| `07-true-and-false-texts/` | Does saying what true and what false mean move the answer |
| `08-option-descriptions/` | Does a description under each option pick better than a bare label |
| `09-evidence-shape/` | Does evidence from several pointers judge better as a JSON object than as the string the tool flattens it into |
| `find-0040/` | Does the accepted one-request find shape hold from 100 through 255 choices before the public command is built |
| `token-budget/` | Which of the vendor's two published request budgets is real. Written and not yet run, because a request that tests a 32,000 token ceiling costs more than 32,000 tokens |

## Running one again

A job records through the one door for a paid call.

```sh
sdlc/scripts/live --max-tokens 1000000 probes/01-find-vs-rank/job.sh
```

A job replays from its own recording with no network and no key. `probes/replay-check.sh` does that and compares every row against the committed one. It sets `meta.cached` and its older spelling `meta.replayed` aside and drops `meta.tool`, `meta.question_sha256`, `meta.requests_sent`, and `meta.requests` from both sides. Those fields arrived after the historical probe rows were written, and a fresh replayed row must carry `cached:true`. Every other byte must match. The `spec` rung runs the check over every numbered replayable probe, so it cannot rot again.

```sh
sh probes/replay-check.sh probes/01-find-vs-rank
```

Record mode arrived with ticket 0012, after these probes ran, so every loop is the shell's, one call per case, with `--details` and a record folder. Each row is the record row of `specification/result.md`, with the case under `input`, which is the shape `transforms/` reads.
