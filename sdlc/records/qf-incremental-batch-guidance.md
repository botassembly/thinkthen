# Incremental batch cache guidance

Experiment 284's `63-incremental-reruns-resend.md` asks the batching page to quantify an inserted record's cache effect and name `--batch 1` for append-heavy work. ADR 0048 item 5 keys a cached answer by the whole batch request. Item 2 describes content cuts and other limits, but its rule alone is not a measured current-workload result. Experiment 273's architect review, `02-batching.md` question 7, inferred a stretch to the next content cut; it did not compare current request digests. This Quick Fix changes only `specification/records.md` and this record, not batching behavior.

The current planner builds each closed request and hashes the adapter name, URL and exact body (`crates/thinkthen/src/core/batch.rs:324-347`; `crates/thinkthen/src/core/recording.rs:94-112`). Content cuts, repeated members, numeric batch size and request/profile limits enter in `crates/thinkthen/src/core/batch/questions.rs:22-56`; end and pause close the open batch in `crates/thinkthen/src/core/batch.rs:224-241`. The cache therefore reuses an unchanged request identity, not an unchanged record in an otherwise changed request. `specification/records.md:81-87,129` already explains batch construction, the live-pipe pause and batch-sized cache entries.

## One bounded local comparison

The unpushed local artifact is `$HOME/workspace/experiments/299-thinkthen-batch-insertion-identity/`: `run.py`, `summary.json`, and four raw command outputs. It used the existing compiled command while this branch was at main `60a66e45`; production command, batch and recording source had not changed since that binary was built. Input was 40 distinct ASCII lines, `item 001` through `item 040`; the second run inserted `item NEW` at position 2. The script checked that neither input had a content cut. Each arm reused its own new cache folder, ran at `--jobs 1`, and used the same question, model and loopback URL for both runs. The local server returned one valid fixed-probability answer per wire question. The script compared actual detailed-row `meta.requests` digests and counted live posts and cache-hit rows. All 43 local posts were sequential; no provider was called.

| Setting | First pass | Inserted rerun | Existing rows whose request digest stayed the same |
| --- | --- | --- | --- |
| Default `max` | One live request for 40 records | One live request for 41 records; zero cached rows | 0 of 40 |
| `--batch 1` | 40 live requests for 40 records | One live request for the inserted record; 40 cached rows | 40 of 40 |

For this input the `max` baseline body was 3,402 bytes and the inserted body 3,485 bytes. The single new `--batch 1` body was 111 bytes. Those are wire sizes, not token or money estimates. The local response reported artificial usage, so it cannot measure provider billing. The comparison establishes one cache-identity consequence of an early insertion in a short no-cut list. It does not establish that every insertion invalidates all later batches: content cuts, size and profile caps can move the boundary, and a timed pipe may pause at a different place. The `--batch 1` alternative preserves old identities only while the question, selected record, backend address, model, context and other request inputs remain unchanged. Its first-pass request overhead is real; broader B6/D1 accuracy work remains separate.

`sdlc/scripts/pages`, `sdlc/scripts/tickets`, and `git diff --check` passed. No source test was added or run for this page correction. The local compiled-command comparison above is the only new behavior proof.

## What the build taught us

The old review's “all tail” wording is too broad. The directly observed unit is a changed request digest, and a cache hit follows that identity. Counting requests alone would hide the difference here because both reruns sent one request: one covered 41 records, while the other covered only the new record. A small compiled-command comparison with real cache folders made that distinction visible without a provider call or a permanent test matrix. No source tests were added for this page correction.

## Independent review and landing

Fresh independent Sol Medium reviewer `01a0e74e-7721-7ac1-9170-74504bf8dfe9` accepted candidate `288bfaa5`. It read the raw outputs independently of the summary, checked the cache entries against observed digests, request sizes, question counts, model and loopback address, and verified the bounded runner and 43 local posts. The example satisfies register 63's documentation criterion; the broader B6/D1 accuracy work remains open.

The reviewer checked that the binary timestamp precedes the experiment and that later Git history changed no production source. The artifacts do not cryptographically attest the binary's build commit. That is an explicit provenance limit on this local workload observation, not a claim of a release or cross-platform measurement. No experiment or build was repeated for review. Pages, tickets and diff checks pass on the final merge; only documentation and status records changed.
