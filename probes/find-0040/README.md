# Long-document find gate

Status: both paid stages passed. The public command is not built yet.

Ticket 0040 preregisters eleven made-up documents and two stages. `cases.json` fixes their sizes and trusted answer places. `fixture.py` fixes every unit, question, id, and exact compact JSON state. `hashes.sha256` pins both files before the first call. No case comes from a person, customer, or private project.

The free self-test drives the real binary's dry-run path at both choice ceilings, then uses a strict fake binary to prove the eight feasibility calls, the 18 find comparison calls, the 1,050 rank judgments, retries disabled, one shared cache, six visible feasibility replays, early stop, failure records, quality gates, safe summaries, and secrecy. The probe derives find ties and rank order locally from all probabilities. It does not trust `choose`'s selected value or use `rank --top`.

```sh
probes/find-0040/self-test
```

The feasibility stage ran once through the authorized command:

```sh
sdlc/scripts/live --max-tokens 120000 probes/find-0040/run feasibility
```

All eight requests were accepted. They used 70,265 billed input tokens across 1,559 unit appearances. The backend accepted 255 units without `none` and 254 units with it. The observed rate was 45.07055805003207 input tokens per unit appearance. The preregistered calculation estimates 451,157.32217573223 tokens and fixes the comparison reservation at 519,000.

The comparison ran once through the committed command:

```sh
sdlc/scripts/live --max-tokens 519000 probes/find-0040/run comparison
```

Both find policies hit 6 of 6 answerable documents, with 2 of 2 at each size. `none` found all 3 blank documents and falsely refused none of the 6 answerable documents. Rank hit the same 6 of 6. Find used 12 live requests and 6 feasibility replays, billing 95,002 input tokens. Rank used 1,050 live requests and billed 322,935 input tokens. Every correct find winner had probability at least 0.99; there was no wrong result from which to measure an error floor.

The 120,000- and 519,000-token maxima are durable ledger charges. The public `find` command may now be built under ticket 0040, but it is not present yet.
