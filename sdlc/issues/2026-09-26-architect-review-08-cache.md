Status: open. Only the remaining model freshness, run-wide checking and refresh work in item 1 stays open. Ticket 0159 pinned the default, 0158 completed item 2, and reviewed 0163 source `0292cba2` completes item 3. Proof: `sdlc/records/0163-code-review.md`.

# Architect review 08: the answer cache

A fresh reviewer tested the default answer cache as an architect who relies on it for cost, sharing and traceability. The review ran against main `9d652bed` and the release binary. It rated the topic fair and found 0 severity 1, 5 severity 2 and 8 severity 3 issues. The full detail sits in the architect review report 273, 08. Work file names in that report refer to its local work folder, which stays unpushed.

Severity 1 means a wrong answer, data loss, a security problem or a hang. Severity 2 means a broken guarantee or a misleading document. Severity 3 means a sharp edge or a missing feature an integrator needs.

The storage mechanics held up well. Ten processes asking one new question sent one request. The three items below are policy problems around that storage.

## 1. Nothing ties an answer, a cache entry, a recording or a tuned cut to the model version that answered (severity 2)

Reviews 08 (issue 1), 05 (issues 2.1 and 2.2) and 09 (issue 8) found this root cause. This file carries it. Review 08 notes that a reader could rate it 1 for high-stakes use, because the tool returns a different decision than the current model would.

Evidence from review 08. The key holds the alias `jev-latest`. A fake backend moved to `jev-1.14.0` and a flipped answer, and plain runs kept printing the `jev-1.13.0` answer. `--no-cache` does not refresh the entry. The alias default has "no recorded reason" (`specification/settings.md`).

Evidence from review 05. The default model is `jev-latest` (`core/adapters/systemone.rs:35`, and every dry run shows `"model":"jev-latest"`). The question digest excludes the model (`question-file.md` canonical rule 1). The calibration check fires only when `--profile` is also given (`core/result/profile_warning.rs:17-21`). A file tuned at 0.42 ran on two other model names with the same digest and no warning. Bench cuts vary from 0.31 to 0.72 by question on one model, so a model change can move the right cut. A second probe ranked two records answered by `jev-1.13.0` and `jev-2.0.0` into one order at exit 0 with no warning. The version check covers only the replies behind one row (`result.md` last paragraph).

Evidence from review 09. `--model jev-1.13.0` misses a recording made under `jev-latest`. No setting warns when the replayed `meta.model` differs from what the alias now resolves to.

What an integrator hits. After a vendor model update, a pipeline returns old decisions indefinitely for every text it has seen, while new texts get the new model. One run mixes model versions, and `rank` and `filter` compare them across two probability scales, without the bare output saying so. Every tuned gate follows the alias to the new model with no signal. CI stays green on answers from an older model. `cache prune --answered-by-other-than MODEL` exists but is manual.

Done by ticket 0159: the default model is the pinned version `jev-1.13.0`, and `audit --write` records the model beside a bar it writes in a single file. Still open: a freshness rule, a run-wide model check and a refresh mode.

Direction. Pin the default model to a version, or put a freshness rule in the lookup, such as treating an entry as stale when the alias resolves to a newer version. Record the answering model version with a tuned threshold, as `audit --write` records the cut, and warn or refuse when `meta.model` differs. Fail or warn a record run whose rows name more than one model version. Add a refresh mode that sends and replaces.

## 2. A cached partial reply replays the failure forever, and the cache is on by default (severity 2)

Five reviews found this: 02 (issue 4), 03 (issue 2-4), 06 (I-1) and 08 (issue 2) at severity 2, and 11 (issue 5) at severity 3. This file carries it.

Evidence from review 08. `recording.md:59` says "A failure is never recorded", and `recording.md:84` says a partial reply replays with exit 6. Removal is only by age, model or size (`recording.md:92`).

Evidence from review 06. With `06-work/fake06_server.py` in mode `flip`, the first run returned `{"a":true,"b":{"failed":...}}` at exit 6. The backend then recovered. Reruns on `--cache pc2` and on the default cache returned the same failure at exit 6, and `status` counted a cache answer. `--no-cache` gave both answers.

Evidence from review 11. `annotate` against a server whose second answer carried probability 1.7 exited 6 three times in a row with `--cache`. The server saw one request. A whole refused reply (exit 4) is not stored.

Evidence from review 03:

```text
$ ./run.sh annotate triage.json --jsonl --cache cache-pf < issues.jsonl
{…,"kind":{"failed":{"kind":"backend","cause":"missing_probability"}},…}
{…}
exit 6
```

Review 02 found the same effect under the batching design, where one failed answer would fail a whole batch of records on every rerun.

What an integrator hits. The standard retry after exit 6 returns the same failure with zero requests. The failed marker says `kind: backend`, which reads as transient. The fix needs the digest from `meta.requests` and a hand delete, a blunt prune, or `--no-cache`, which pays for everything again.

Direction. Do not install replies that hold a failed answer in the answer cache. Keep them for explicit `--record`, or treat them as a miss on the next cached lookup. Fix the "never recorded" sentence. Add per-digest removal to `cache prune`.

## 3. One malformed or foreign-schema entry stops `status` and `cache prune` for the whole folder, without naming it (severity 2)

Evidence. `cache_prune.rs:200-240` returns `CacheEntry` for the first bad file, and `status` calls the same scan (`cli/status.rs:126`). The messages name no file.

What an integrator hits. A restored backup, a sync tool, a disk error, or a newer ThinkThen writing `thinkthen.recording/2` into the shared default cache leaves the folder unprunable and `status` broken. A nightly prune fails every night while the cache grows. This breaks the only mechanism that bounds the cache.

Direction. Let prune skip and report bad entries, or quarantine them, and still trim valid ones. Name the offending digest. Let `status` report a bad-entry count rather than fail.

## Carried in other files

- The settings page says the configuration file's `cache` names a folder (issue 4, severity 2). The architect review 06 file carries it.
- `cache_bytes` does nothing on any library or SQL surface (issue 5, severity 2). Already filed in `2026-09-26-settings-some-surfaces-cannot-reach.md` item 3 and `2026-09-25-public-library-api-gaps.md` item 4. This review adds that ADR 0017 line 87 says a warm pass "evicts its own answers". That sentence needs a correction, or a mark that ADR 0033 supersedes it.

## Severity 3 titles

- The key is exact bytes, so input framing changes it. Carried in the architect review 09 file.
- Prune evicts oldest-written, not least recently used.
- A typo in `--answered-by-other-than` deletes every entry at exit 0, and prune has no dry run. Quick Fix qf-command-edges-and-prune refuses a model no reply in the folder names. The dry run is still open.
- A cached result repeats the stored token usage, so a dashboard that sums `meta.usage` overstates spend.
- The default cache binds to one backend address. Carried in the architect review 06 file.
- `Engine::builder()` ignores the configuration file's `cache: false` (`public/settings.rs:255-266`).
- A crashed write leaves hidden temporary files that nothing removes or counts. Carried in the architect review 11 file.
- DuckDB's warm pass ignores `SET thinkthen_cache`. Carried in the architect review 04 file.
