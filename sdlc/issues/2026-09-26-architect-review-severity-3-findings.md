# Architect review severity 3 findings

Status: Open until ticket 0321 lands, then closed. Filed 2026-09-26 by the queue owner from local experiment 273, reports 01 and 03 to 12. Triaged against `origin/main` at `267efff53` on 2026-09-30.

Every line below carries its disposition. Fixed names the landed work or the page that now says it. Obsolete names the ADR that removes the cause. Tracked names an open issue that carries it. Won't fix follows Ian's ruling 8 of 2026-09-29 or ADR 0005's rule that a feature waits for a demo. Ticket 0321 carries the one small fix left. Nothing else stays open here.

## Report 01, `filter` and streams

- I3. One request per record. Fixed by 0146.
- I4. One slow record stalls ordered output. Fixed: `specification/records.md` states the stall bound (`b753f1d2d`).
- I5. Retries move in lockstep. Fixed by 0155 and 0315 (jitter).
- I6. No machine-readable stop point. Fixed by 0170, the `thinkthen.run/1` line.
- I7. No default record size guard. Fixed by 0154.
- I8. `filter` cannot serve as a coprocess. Fixed: `README.md` sends long-lived loops to `decide --lines --batch 1` and says `filter` prints only kept records.
- I9, I10. Ctrl-C stop line and second-signal escape. Fixed by 0169.

## Report 03, `annotate`

- 3-1. Name collision under `--details`. Fixed by ADR 0104: the member stays under `input` (`specification/annotate.md`).
- 3-2. A missing pointer stops the run. Fixed as the opt-in `--on-error continue`. The default stop is the contract of `specification/records.md`.
- 3-3. Row shape varies. Fixed: `annotate.md` recipe "Read a mixed record stream".
- 3-4. Failure isolation depends on grouping. Fixed: documented in `annotate.md`. ADR 0111 stores the good answers of a partial reply.
- 3-5. No request ceiling. Fixed by 0154.
- 3-6. Libraries refuse sets with `on`. Fixed by 0150.
- 3-7. Document mode sniffs JSON. Fixed: `annotate.md` states the rule and the flags that force framing.

## Report 04, libraries and databases

- I5. One failing SQL row fails the statement. Fixed: `thinkthen_try_details` on DuckDB, SQLite and PostgreSQL returns a per-row failure.
- I6. A deadline bounds one call, not a query. Fixed: the DuckDB query-hook issue closed under 0201 and 0231.
- I7. SQLite and PostgreSQL hold one request in flight. Deferred to ADR 0111 slice 3, SQL hosts on the one batching path.
- I8. DuckDB's warm pass ignores session settings. Fixed: `thinkthen_warm` is removed.
- I9. DuckDB refuses calls after 16 engines. Fixed: the least recently used idle engine retires (`databases/duckdb/src/engines.rs`).
- I10. The throttle belongs to the process. Fixed in part by 0308's per-process, per-address pacer. The rest is the design.
- I11. Libraries cannot set timeout, retries, profile or replay. Fixed by 0148, 0149 and 0157.
- I12. A SQLite cancel holds its permit. Fixed by 0168.
- I13. DuckDB relate's uncommitted-table message. Fixed: the message hints at the open transaction.
- I14. List calls refuse `choose`, `score` and `tag` over many texts. Deferred to ADR 0111 slice 3.
- I15. Missing-key error kind differs by surface. Deferred to 0314 slice 3: ADR 0112 gives every port the same named error codes. The command's exit 4 stays.
- I16. Packaging and loading sharp edges. Tracked: `2026-09-25-release-and-install-for-0-1.md`.

## Report 05, the answer contract

- 3.1. Tie policy. Fixed: `score.md`, `choose.md`, `find.md` and `threshold.md` state each tie rule.
- 3.2. Only `decide` has a not-sure region. Won't fix: documented design in `threshold.md` and `find.md`.
- 3.3. `--threshold` cuts different quantities. Fixed: `threshold.md` names the quantity per function.
- 3.4. Details line parse traps. Fixed: `result.md`; ADR 0112 generates the schema.
- 3.5. A single question file carries no version. Won't fix: `question-file.md` refuses it on purpose and says where a version goes.
- 3.6. A record run stops at the first failure. Fixed in part by `--on-error continue`. The rest waits on a demo under ADR 0005.
- 3.7. Contract pages disagree. Fixed by 0152.

## Report 06, backends and configuration

- I-6. No request-size setting. Fixed by 0154 and 0157.
- I-7. A broken configuration file names no field. Fixed: `crates/thinkthen/src/config.rs` names the field.
- I-8. The model has no environment tier. Won't fix: design in `specification/settings.md`, pinned by `tests/decide_edge.rs`.
- I-9, I-11, I-12, I-13. Fixed by Quick Fix qf-command-edges-and-prune.
- I-10. The default cache serves one address. Obsolete under ADR 0111: the address sits in every key and the marker goes in slice 5.
- I-14. Refusal phrases misfit. Fixed: `cli/failure/status.rs` gives 302, 413, 429 and `max_tokens_exceeded` their own sentences.

## Report 07, throughput, limits and cost

- I3, I5, I10. Fixed by 0155 and 0315.
- I4. `Retry-After: 0` has no floor. Fixed: `engine/http/retry.rs` raises it to one second.
- I6. Every retry opens a new connection. Fixed: one shared agent, and the error body is read before return.
- I7. One slow record stalls the run. Fixed with report 01 I4.
- I8. `--dry-run` cannot estimate cost. Fixed: `--plan` gives the token range; ADR 0108 puts price only in `--facts`.
- I9. No run total. Fixed by 0170.
- I11, I13. Fixed by 0148 and qf-command-edges-and-prune.
- I12. Nothing caps concurrency across processes. Won't fix: `records.md` says so, and the roadmap calls a cross-process budget a different tool.
- I14. The final 429 message hides the retries. Fixed: the sentence names the attempts and `--max-retries`.

## Report 08, the cache

- 6. Byte-exact keys. Fixed: `specification/recording.md` documents the framing rules.
- 7. Prune evicts by write time. Fixed: `recording.md` documents it; ADR 0111 prunes on `taken_at`.
- 8. A model typo prunes everything. Fixed: prune refuses an unknown model and has `--dry-run`.
- 9. Cached results repeat stored usage. Accepted cost; ADR 0111 stores each question's share.
- 10. Default cache binds to one address. Obsolete under ADR 0111.
- 11. `Engine::builder()` ignores `cache: false`. Fixed: documented; `EngineBuilder::from_env` honors it.
- 12. Crashed writes leave temporary files. Obsolete under ADR 0111 slice 5: SQLite commits replace them.
- 13. DuckDB warm pass ignores `SET thinkthen_cache`. Fixed: the warm pass is removed.

## Report 09, record, replay and testing

- 4. Reformatting a recording breaks it. Fixed: `recording.md` says so.
- 5. A replay miss does not say why. Obsolete under ADR 0111 slice 2: the miss names the key and its parts.
- 6. No process-wide strict replay switch. Won't fix: no demo reached for it (ADR 0005).
- 7. `--record` into a used folder. Fixed: the help and `recording.md` say it; ADR 0111 makes `--record` replace the entry.
- 8. Replay cannot tell a recording is stale. Won't fix: replay is keyless history by design (`recording.md`). ADR 0111 stores `answered_by` and `taken_at` for a later check.
- 9. Golden files break across versions. Won't fix: ADR 0112's generated schema is the stable contract.
- 10. A token in the base path lands in recordings. Fixed: user information and queries are refused (`core/backend.rs`); `backends.md` and `recording.md` warn about the path.
- 11. Byte-exact keys make recordings fragile. Fixed: `recording.md`.
- 12. No way to find unused fixture entries. Fixed: `thinkthen cache unused`.

## Report 10, `recognize` and `relate`

- 7. A lower name threshold adds no word. Fixed: `specification/recognize.md`.
- 8. Possessives, quotes and touching names. Tracked: `2026-09-25-recognize-and-relate-scale-and-shape.md`.
- 9. Mentions multiply relation cost. Tracked: `2026-09-26-relation-pairs-span-every-mention-and-the-whole-text.md`.
- 10. No entity cap on `recognize --relation`. Fixed: 255 names and 4,000 pairs, refused before sending.
- 11. Relation rules sent one after another. Fixed: the relation-requests issue closed.
- 12. `--details` hides relation probabilities. Fixed: `answer.pairs`.
- 13. Offset units by surface. Fixed: `recognize.md` table and shared case 41.
- 14. Split texts repeat the whole text. Tracked with item 8.

## Report 11, failure and scripting

- 3. SIGTERM. Fixed by 0169.
- 4. Killed runs leave temporary entries. Obsolete under ADR 0111 slice 5.
- 5. A stored partial reply replays forever. Fixed by 0158; ADR 0111 stores no failed answer.
- 6. A deterministic refusal blocks reruns. Same as report 05 3.6.
- 7. Absurd `--timeout` panics. Fixed by qf-command-edges-and-prune.
- 8. Ctrl-C blames the backend. Fixed by 0169.
- 9. Default cache binds to one address. Obsolete under ADR 0111.
- 10. No catalog of error sentences. Won't fix: ADR 0112 gives the ports six named error kinds; a command catalog waits on a demo.

## Report 12, security and the data boundary

- 3.1. No private TLS roots. Fixed: `THINKTHEN_CA_BUNDLE` and its own certificate sentence.
- 3.2. Local faults read as network faults. In 0321: a key holding a control character fails in the HTTP layer as unreachable. The 302 half is fixed.
- 3.3. Configuration file trusted whatever its mode. Fixed by qf-config-owner-warning (`53ba4edb`).
- 3.4. Planted-text guidance too narrow. Fixed: `specification/decide.md` bounds the claim.
- 3.5. Ruby result values print caller text. Fixed; closed issue.
- 3.6. No release to verify. Tracked: `2026-09-25-release-and-install-for-0-1.md`.
- 3.7. Nothing says what the default destination keeps. Fixed: `README.md` links the provider's terms and retention statement.
