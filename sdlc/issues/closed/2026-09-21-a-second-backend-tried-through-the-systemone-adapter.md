# A second backend tried through the `systemone` adapter

Status: Closed on 2026-09-22. The run happened and this page is the record of what it settled.

On 2026-09-21 a second, local decider model answered the tool's exact request bytes with no change to this repository. The model is `laya-mlx` (Hugging Face `aac6fef/laya-mlx`, Apache-2.0, ModernBERT-large, Apple Silicon, English). A small shim server put its in-process API behind `POST BASE/systemone` on loopback, and `THINKTHEN_BASE_URL` alone pointed the released binary at it. No paid call was made.

The experiment folder is `experiments/220-thinkthen-second-backend/` in the workspace outside this repository. It holds the shim, the staged example trees, both run recordings, and `RESULTS.md` with every number below.

## What this settles

- **Another server accepts our exact request bytes.** The size-cost issue listed that as not known. It is now known, once.
- **ADR 0004 held as written.** A backend is a URL and an adapter. No code, no option, and no new adapter was needed. `chat-logprobs` and the subprocess adapter stay planned, not required.
- **The `systemone` adapter decoded a reply from a model built by people who never saw this tool.** That model's own library exposes a `system_one(state, questions)` call that takes and returns the shape in `specification/backends.md`. The shim translated nothing.
- **Unknown fields are ignored on decode.** A reply carrying `confidence` and `action` on a `noul` answer, and an unknown top-level key, decoded without complaint. A compatible server need not strip its extras.
- **Probability tolerance was never hit.** The model rounds to four decimals, so a distribution totals one to within 0.0002.

## What ran

All eight function examples from the published example set, plus `annotate` and the two question-file runs. Ten runs, exit code 0 on every one, empty standard error, twice over with a fresh `--cache` each time. Sixteen comparable answers were compared with the first backend: twelve agree on the value a script acts on, four differ, and two of the four are wrong.

## What this opens

1. **A threshold does not carry between backends.** This is the finding worth acting on. The published `@refund.json` pins a `0.2:0.8` band. Against the first backend the sample text landed in the middle and exited 3, which sends it to a person. Against the second it exited 0 and routed itself. The second model's probabilities sit nearer the middle across the board: it won a `find` with 0.11 and spread a three-way `choose` as 0.40 / 0.20 / 0.40. A question file that pins a threshold should name the backend it was calibrated against, or say plainly that it was not.
2. **A small backend cannot tell a user its limit.** The shim refused a 2,601-token text with status 422 and a body naming the 512-token budget and the 478 tokens left for text after the question. The user saw `the backend answered with status 422: the backend refused the request as malformed or too large` and exit 4. The rule that never repeats a response body is right, and its cost is that the most useful sentence a backend can send is discarded. A profile carrying the backend's stated limits, checked before the request leaves, is the fix the size-cost issue already proposes, and this is the first measured case for it.
3. **A backend that truncates in silence is the danger.** The model's own library trims a long text to fit and says nothing. The shim had to count tokens and refuse to avoid a confident answer about the first paragraph of a contract. Any adapter page for a small backend must state whether the backend truncates.

## Measured

| Measure | Value |
| --- | --- |
| Load from a local cache, fresh process | 0.18 s and 0.30 s on two starts |
| One request, short text, median of 34 | 0.012 s |
| One request, slowest of 34 (four questions) | 0.098 s |
| Peak resident memory of the shim | about 944 MiB |
| Request budget | 512 tokens for the text and the question together |
| Questions in one request | 16 |
| Price | none |

## Not done

The ten use-case flows were staged and not run. No accuracy work: the agreement count above is one pass of one example set and is a smoke signal, not a measurement.
