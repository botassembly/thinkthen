---
flow: build
priority: 158
opens: crates/thinkthen/src/engine/request.rs crates/thinkthen/src/engine/recorder.rs crates/thinkthen/src/core/reply.rs crates/thinkthen/tests/backend/cache_partial.rs crates/thinkthen/tests/backend/main.rs specification/records.md specification/recording.md sdlc/ratchet.json sdlc/records sdlc/tickets
---

# 0158: A cache keeps no reply that failed a question

Status: ready for review. The coordinator drafted it on 2026-09-26 from ADR 0053 item 6. A fresh read-only review must accept it before it builds. Owner: Claude. It lands before ticket 0146 (B4) lands, and it may build at the same time.

Review route: a fresh read-only Claude session reviews this design and the final diff. Codex does not review this ticket unless Ian routes it.

## Outcome and authority

A user runs `annotate`, or after ticket 0146 a batched `filter`, and the backend's reply leaves one question unanswered. Today the default cache installs that reply. Every rerun reads it and fails the same question without asking again. After this ticket a cache installs only a reply that answered every question, and the next run asks again. `--record DIR` alone still writes the reply, so `--replay` gives the live run's result.

ADR 0053 item 6 is the ruling. `specification/records.md` already says each digest keeps "the first complete response". The coordinator ruled the order on 2026-09-26: this ticket lands before ticket 0146 lands. Batching makes a partial reply common, because every record of a batch is a question and the default cache is on. Ian can overturn each ruling.

## What happens today

Read from `origin/main` `c490f082`.

- `engine/request.rs::ask_prepared` installs the reply through `permit.finish` whenever it decodes (lines 147 to 160). A reply that fails to decode cancels the permit and writes nothing.
- `core/adapters/systemone/response.rs` decodes a reply when at least one question succeeds (lines 122 to 128). A failed question becomes `AnswerOutcome::Failed` in `core/reply.rs`. A reply where every question failed is a whole-reply decode error.
- `engine/recorder.rs` treats `--cache DIR` as recording and replaying on one folder. The default cache and `THINKTHEN_CACHE` are the same with a private folder. `--record DIR` alone records and does not replay.
- `specification/recording.md` line 84 says "A recorded partial reply replays the same good answers, failed markers, failure count, and exit 6." That rule is about `--record` and `--replay`.
- Local experiment 273 reproduced the defect with `annotate` and a hand-built cache entry whose reply answered `q1` and omitted `q2`. Two runs printed the same failure and exited 6. The loopback received 0 requests.
- The conformance backend's `/arm/malformed/missing_answer/v1` fails the last question of a reply and answers the rest (`tests/backend/loopback_arms.rs`).

## Design

`Recorder` gains `caches()`, true when it both records and replays, which is `--cache DIR`, `THINKTHEN_CACHE` or the default cache. `Reply` gains `failed_any()`, true when any outcome is `AnswerOutcome::Failed`.

In `ask_prepared`, after a live reply decodes, the engine checks both. When the recorder caches and the reply failed a question, it cancels the permit in place of finishing it. It still returns the reply, so the run prints and exits exactly as today. Every other case finishes the permit as today. A cancelled permit releases the digest's lock, so a waiter on the same digest sends its own request, as it does after a failed owner today.

The engine is shared, so the libraries and SQL extensions follow the same rule under their caches. `relate` edges and `annotate` question entries follow it too.

### Pages

- `records.md`, the cache paragraph: after "Each digest keeps the first complete response installed in the folder." add "A reply that failed a question is not complete. A cache does not install it, and the next run asks again. `--record` alone still writes it."
- `recording.md`, line 84: after the first sentence add "A cache, typed or default, installs no partial reply, so a cached run asks again. ADR 0053 item 6 rules it."
- `recording.md`, line 59, "Only an exchange that succeeded and decoded is recorded. A failure is never recorded.": after it add "A reply that failed a question beside a good answer counts as decoded. `--record` writes it. A cache does not install it."
- `recording.md`, line 13, the row "Both, with the same `DIR`": its cell becomes "A cache. An entry that exists is replayed. A request that is absent goes to the backend, and its reply is recorded when it answered every question."
- `relate.md`, line 70, stays as it is. It speaks of record and replay, and a recorded partial reply still reproduces there.

## Decisions

Each is the ticket author's call unless marked. Ian can overturn any of them.

1. **A cache never installs a reply that failed a question.** ADR 0053 item 6 ruled it. The coordinator ruled it.
2. **`--record` alone still writes a partial reply.** A recording exists to replay a run as it happened, and `recording.md` line 84 promises that.
3. **The run's output does not change.** The fix touches only what the folder keeps. The same reply prints the same rows, markers and exit code.
4. **The check lives in `ask_prepared`, the one place a live reply is installed.** No second write path exists.

## Edge cases

| Input | Expected |
| --- | --- |
| `annotate` under the default cache, a reply missing its last answer, run twice | Both runs print the same failure marker and exit 6. The loopback sees 2 requests |
| The same under `--cache DIR` | The same. The folder holds no entry for that digest |
| The same under `--record DIR`, then `--replay DIR` | The replay prints the recorded failure marker and exits 6, with no request |
| A reply that answers every question under the default cache, run twice | The second run sends nothing, as today |
| A reply that fails every question | A whole-reply decode error at exit 4, as today. No entry |
| Two concurrent runs on one `--cache DIR` with the same partial reply | Each sends once. The folder holds no entry |

## Proof

The test drives the compiled binary against the conformance backend's `/arm/malformed/missing_answer/v1` or the in-process loopback, and counts requests.

| Test | What it proves | Planted faults that turn it red |
| --- | --- | --- |
| `a_cache_keeps_no_failed_question`, new in `tests/backend/cache_partial.rs` | The first four edge rows. Each pins standard output, exit code and the backend's request count. The `--cache DIR` row also pins that the folder holds no entry file for the digest | (a) Install every decoded reply: the second cached run sends nothing. (b) Skip the write under `--record` too: the replay row exits 5. (c) Skip the write for every reply: the complete-reply row sends twice |

The four questions:

- **What behavior does it protect?** `records.md`'s "first complete response" under the cache, and `recording.md` line 84 under `--record`.
- **What credible regression fails it?** Installing a partial reply, or dropping the recorded one.
- **Why does no existing test catch it?** No test sends a partial reply twice through a cache today.
- **Does it need a test-only hook?** No. The loopback's reply is an ordinary reply, and the folder is a real folder.

## Budgets

Nonblank lines, measured with `grep -c .`.

- `crates/thinkthen/src/engine/request.rs`, `engine/recorder.rs` and `core/reply.rs`: at most 15 net together.
- `crates/thinkthen/tests/backend/cache_partial.rs`: at most 90, new, and one `mod` line.
- Pages under `specification/`: at most 8 net.
- `sdlc/ratchet.json` moves to the measured total, at most 110 above main. The commit says what grew.
- No dependency.
- The `surfaces` rung runs, because the engine that the libraries share changes.

## Stop rules

1. Stop before crossing any budget by more than a tenth, or before adding a dependency.
2. Stop if the change needs a file that ticket 0146, 0154 or 0155 opens.
3. Stop if `--record` alone stops writing a partial reply.
4. Stop if any run's standard output, standard error or exit code changes.
5. Stop if any plant stays green.
6. Stop if the build needs a live call. None is authorized. Never run `sdlc/scripts/live`.

## Scope and exclusions

Excluded: a recorded refusal entry. Any change to how a partial reply prints or exits. Batching itself, which ticket 0146 builds. `site/`.

## Routing

Builder: Claude (Opus subagent) in the lane the coordinator names. Reviewer: a fresh read-only Claude session for the design and for the code.

## Complexity

Contract 1; state and timing 1; reach 2; proof 1; cost of error 1; total 6. Final level: 2. The risk is a cache that drops complete replies too, which the complete-reply row guards.

## Deferred gaps

None.

## What Ian can overturn

- Decision 1: a cache never installs a reply that failed a question.
- Decision 2: `--record` alone still writes it.
- The coordinator's order: this ticket lands before ticket 0146 lands.

## Closes

Finding 4 of `sdlc/issues/2026-09-26-batching-design-review-before-0146.md`.

## Evidence

- Starts from: ADR 0053 item 6. Finding 4 of `sdlc/issues/2026-09-26-batching-design-review-before-0146.md`, reproduced in local experiment 273 with `annotate` and a hand-built cache entry. The code at `origin/main` `c490f082`: `engine/request.rs::ask_prepared`, `core/adapters/systemone/response.rs`, `core/reply.rs` and `engine/recorder.rs`. `specification/records.md`'s "first complete response" and `recording.md` line 84.
- Keeps: Every complete reply cached as today. `--record` and `--replay` as today, partial replies included. Every run's output, standard error and exit code.
- Changes: A cache, typed or default, no longer installs a reply that failed a question. Page sentences in `records.md` and `recording.md`.
- Proof: One outside-in test with three plants, and the `install`, `lint`, `test`, `spec` and `surfaces` rungs.
- Defers: Nothing.
