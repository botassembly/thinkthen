# ADR 0036: Call stored-result metadata cached

- Status: Accepted by ticket 0068 after independent design review under Ian's authorized engine plan
- Date: 2026-09-22

## Decision

Ticket 0068 executes the planned `meta.replayed` to `meta.cached` rename from the architect's reviewed plan. It supersedes the field spelling in ADR 0010 and ADR 0017's sends amendment, not their behavior. The architect selects a pure output rename to avoid silently redefining stored answers. Ian can overturn the spelling or compatibility choice.

Every newly emitted `thinkthen.result/1` detailed result has the boolean `meta.cached` in the former `meta.replayed` position, immediately after `requests_sent`. It emits no `meta.replayed` alias. The schema identifier remains unchanged in this unreleased interface. Bare answers, errors, exit codes, requests, counters, and recordings do not change.

`cached` is true when the result came entirely from stored exchanges, including explicit read-only replay, paired record/replay, named/default caches, and stored answers read after waiting on a cache lock. It is false for a live result, including a live result shared by in-flight request coalescing. Annotation keeps its existing all-groups rule: one live group makes the row false. This boolean is not a substitute for `requests_sent` or the usage counters; explicit replay still does not increment cache-answer usage.

Keep private Rust replay bookkeeping and the `--replay` option unchanged. The production serializers own the new field name. Language/database rehearsal code remains with its owners until integration adopts the shared contract.

## Historical readers and evidence

The cost and trials transforms and the find-probe row reader accept either spelling in input. Presence of `cached` takes precedence, even when its value is false; only an absent key falls back to `replayed`. Existing validation remains: trials and the find reader reject a nonboolean canonical field rather than falling back to a valid legacy field. Cost retains its existing policy of treating only boolean true as stored. Existing aggregate output names such as `trials.replayed`, cost's `replayed` bucket, and probe summary fields remain unchanged.

Historical captured exchanges, recording files, recorded probe rows, and experiment measurements are never rewritten for this rename. The find probe's preregistered `fixture.py`, `cases.json`, and `hashes.sha256` also remain byte-identical. Its runner normalizes new metadata into a transient legacy-shaped input for the original reader; the frozen reader still owns validation and derived results. The replay checker sets aside both provenance spellings during old/new comparison, still compares all other existing normalized content, and requires freshly replayed rows to carry `cached:true`. Current executable examples and output expectations adopt `cached`; historical fixtures remain useful compatibility tests.

## Proof

Serializer tests pin the new field and absence of the old field. Counted-listener tests preserve live/retry, explicit replay, named/default cache, cache-waiter, and mixed annotation behavior. Consumer tests cover new and legacy true/false values, mixed inputs, canonical-false precedence, and invalid canonical values wherever validation already exists. Replay, transform, and how-to gates run without paid calls. The implementation diff must contain no recording or captured probe-row edits.
