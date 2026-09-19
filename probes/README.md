# probes/

Ticket 0011 ran six measurements against the hosted decider model. Each folder holds its cases, the job that judged them, the rows the job wrote, the recorded exchanges, and the analysis that reads the rows. `sdlc/records/0011-the-live-probe.md` holds the numbers and what each one changes.

Every case is made-up text written for this ticket, with a trusted answer fixed before any call went out. No case is real customer text and no case names anything private.

| Folder | Asks |
| --- | --- |
| `01-find-vs-rank/` | Does one request over many lines pick as well as one request per line |
| `02-confidence/` | Does the backend's `confidence` separate right from wrong better than the winning probability |
| `03-score/` | How well `score` places a text on five levels, beside `choose` over the same five labels |
| `04-option-order/` | Does reversing or shuffling the options move the pick |
| `05-irrelevant-option/` | Does one added label that fits nothing move the pick |
| `06-hostile-text/` | Does an instruction aimed at the judge, inside the evidence, move the answer |

## Running one again

A job records through the one door for a paid call.

```sh
sdlc/scripts/live probes/01-find-vs-rank/job.sh
```

A job replays from its own recording with no network and no key. `probes/replay-check.sh` does that and compares every row against the committed one, setting `meta.replayed` and `meta.tool` aside. Ticket 0012 added `meta.tool` after these rows were written, so a row older than the field is compared without it.

```sh
sh probes/replay-check.sh probes/01-find-vs-rank
```

Record mode arrived with ticket 0012, after these probes ran, so every loop is the shell's, one call per case, with `--details` and a record folder. Each row is the record row of `specification/result.md`, with the case under `input`, which is the shape `transforms/` reads.
