# Help and warning gaps from the transcript how-to

Status: open for the refresh-cache warning choice and remaining record/accounting help. Filter-versus-rank guidance already appears in the current site examples.
Milestone: 0.2

The site's transcript how-to ran on 0.1.0 and found four gaps. None changes an answer.

1. **`--record` help.** `--record DIR` writes `thinkthen.sqlite`, but site recordings need `thinkthen.jsonl`. The help never names `thinkthen cache convert`. The `--record` help line should name it.
2. **The `--refresh-cache` warning.** Each command prints the "may incur a charge" warning, so a script that runs ten commands prints it ten times. Each command is its own process, so "once per process" is what happens today. The choice is a way to silence it, such as an environment setting a script sets once. A ticket weighs the spend warning against the noise.
3. **`filter` against `rank`.** `filter` at its default threshold of 0.5 kept 4 of 292 transcript passages, and a clearly relevant passage scored 0.42. A help line should say that `rank` suits exploring and `filter` suits gating, and how to pick `--threshold` from a few labeled cases.
4. **`requests_sent` on batched rows.** Under `sdlc/scripts/live`, a row printed `meta.requests_sent: 0` and `cached: false` beside a live attempt that returned 200. That is correct by ADR 0048 item 9: each batched row carries an even share of its batch's requests, with the remainder on the earliest rows, so one request over many rows counts 1 on the first row and 0 on the rest. `--facts` gives the run's total. `result.md` says so, but the field's name misleads a reader of one row. The help or the how-to should say it.

## Reconciliation, 2026-10-08

The current transcript example in `site/src/data/catalog.mjs` says to use rank to explore and filter to keep. Item 3 no longer asks for that missing distinction; labeled-case threshold advice remains an editorial obligation. Items 1, 2 and 4 retain their asks. `cli/asking.rs` still emits the spend warning per process. Result/2 carries observed request counts and call facts separately; the [result contract](../../specification/result.md) governs current names, rather than the old 0.1 row excerpt. This issue authorizes no spend-warning suppression or paid rerun.
