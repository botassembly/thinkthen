# A chained question hides a better pipeline

Status: Open. Filed 2026-09-28 from the GEPA tuning experiments 296 and 297. Evidence lives in the workspace at `experiments/297-gepa-loop-tests/`, `pipeline.py` and `runs/pipeline/`; the full write-up is `notes/2026-09-27-optimization-lessons.md`.

## What happens today

A user who needs two facts chained writes one question that asks for both, as the bench's album-year question does: find the first album, then give its year. The tool answers it in one call, and nobody measures the two smaller questions.

## Why it matters

Splitting the chain nearly tripled the right answers at the same cost.

| Form | Right |
| --- | --- |
| Direct chained question | 22/60 = 0.367 |
| Step one alone, song to album | 48/60 = 0.800 |
| Two-question pipeline | 51/60 = 0.850 |
| Pipeline ceiling, the true album | 52/60 = 0.867 |

The second hop repeats across cases: thirteen albums carry sixty songs, so the album-year answers are thirteen distinct requests and the recording cache serves the rest. The shipped `audit` confirms the pipeline at 0.864 over 59 rows. Experiment 243 saw the same effect on a smaller sample, 11 of 22 against 3 of 22.

## What to change

A how-to that shows the two-command pipeline: the first `choose` prints the album, and the second call takes that album as its input. A line in `audit`'s guidance telling a user to compare a chained question with its split before tuning wording. The help and the how-to name the reuse: a hop with a small, repeated answer space costs nothing after its first call.

ThinkThen sequences nothing new. The host already runs two commands. The gap is that nothing tells the user to try it.

## Factual preparation refresh, 2026-09-28

At source `9b766cf9`, demo 16's `annotate | jq` pipeline makes one model call per record and then applies local policy; it does not feed one model answer into a second model question. The original two-call how-to and audit comparison therefore remain open. Experiment 297's `pipeline.py` is the existing host sequence and reused saved second-hop answers. Its 60-case summary and `runs/pipeline/` audit's 59 saved rows have different denominators: the script omits a live second-hop row without a usable first-hop album. The `22/60`, `48/60`, `51/60` and `52/60` results and thirteen repeated album inputs describe that cohort and cache state, not a general accuracy or cost promise. A future replay proof must run both displayed calls against matching saved request bodies, assert the passed answer value and compare the actual direct/split audit sets. No engine sequencer is proposed. See `sdlc/records/2026-09-28-tuning-evidence-refresh.md`.
