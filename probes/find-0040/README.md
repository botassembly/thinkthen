# Long-document find gate

Status: feasibility passed; comparison is prepared and has not run.

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

The comparison remains unrun. Its exact next command is:

```sh
sdlc/scripts/live --max-tokens 519000 probes/find-0040/run comparison
```

The 120,000-token feasibility maximum is already a durable ledger charge. The 519,000-token comparison maximum becomes one when that command runs. The public `find` command remains blocked until every comparison gate in ticket 0040 passes.
