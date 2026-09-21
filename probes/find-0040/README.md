# Long-document find gate

Status: prepared and not run against a paid backend.

Ticket 0040 preregisters eleven made-up documents and two stages. `cases.json` fixes their sizes and trusted answer places. `fixture.py` fixes every unit, question, id, and exact compact JSON state. `hashes.sha256` pins both files before the first call. No case comes from a person, customer, or private project.

The free self-test drives the real binary's dry-run path at both choice ceilings, then uses a strict fake binary to prove the eight feasibility calls, the 18 find comparison calls, the 1,050 rank judgments, retries disabled, one shared cache, six visible feasibility replays, early stop, failure records, quality gates, safe summaries, and secrecy. The probe derives find ties and rank order locally from all probabilities. It does not trust `choose`'s selected value or use `rank --top`.

```sh
probes/find-0040/self-test
```

The first authorized paid command is:

```sh
sdlc/scripts/live --max-tokens 120000 probes/find-0040/run feasibility
```

After it succeeds, `analyse.py reservation` derives the comparison reservation from observed usage. The exact result must be committed before the second call and may not exceed 525,000:

```sh
python3 probes/find-0040/analyse.py reservation \
  probes/find-0040/results/feasibility-summary.json
sdlc/scripts/live --max-tokens N probes/find-0040/run comparison
```

Both maxima become durable ledger charges. The public `find` command remains blocked until every acceptance gate in ticket 0040 passes.
