# Reconcile remaining library and SQL outcomes

Read-only preparation checked accepted implementations against main `db542da4`. No product code changed, no build or provider run was needed, and this factual refresh adds no numbered prepared ticket. The coordinator checked the original register criteria and obtained a fresh independent closure review for register 78.

## Register 78: accepted closure

Status: **ACCEPT closure** after a fresh independent Sol Medium source and retained-evidence review on `db542da4`. The original finding asks for bulk choose, score and tag across libraries. Landed code now supplies all three; host spellings need not be identical.

| Host | Public implementation | Retained reviewed proof |
| --- | --- | --- |
| Rust | `crates/thinkthen/src/public/bulk.rs` typed many-record methods and public delegates | `tests/public_batches/contracts.rs`; [0212 review](0212-code-review.md) |
| C | The public JSON door accepts `records` for all four judgments through `libraries/c/src/call/records.rs`; its typed bulk symbol remains decide-specific | Real installed-header driver and `tests/door/batching.rs`; [0230 build](0230-build.md) and [review](0230-code-review.md) |
| Python | `choose_many`, `score_many`, `tag_many` and delegates in the runtime and stub | `tests/test_call.py`; [0214 build](0214-build.md) and [review](0214-code-review.md) |
| TypeScript | The three list methods and delegates in `index.js` and `index.d.ts` | `tests/verbs.test.mjs` with values, descriptions, null choice and empty tags; [0236 review](0236-code-review.md) |
| Ruby | The three list methods and delegates in `lib/thinkthen.rb` | `tests/test_batch_facts.rb`; [0234 build](0234-ruby-batching-build.md) and [review](0234-ruby-batching-code-review.md) |
| R | `tt_choose`, `tt_score`, `tt_tag` accept columns through the existing details-many route | `tests/verbs.R` including missing cells; [0237 build](0237-r-batching-build.md) and [review](0237-code-review.md) |

Rust Polars retains its three series methods; Python columns and pandas retain the corresponding verbs. These are implemented bulk paths, not a documentation-only workaround. Register 78 is done because the batching tickets fixed its code. Broader E2 recognition and SQL work retains its own criteria. The closure reviewer rebuilt nothing and made no source edits.

## Remaining outcomes and the next useful work

| Outcome | What is already evidenced | What remains |
| --- | --- | --- |
| Register 73 | B13d PostgreSQL and B13e SQLite warm batching have landed with installed proof | Original numeric documentation and warm criteria passed independent closure review below. Ordinary scalar one-row behavior is retained, not a new required feature. |
| Every surface gives run facts | Rust, C, Python, TypeScript, Ruby, R and frame call carriers have landed; command facts and uniform details also landed | Per-request server timing and request ID, user-supplied pricing, and exact remaining documentation criteria stay separate. SQL per-call facts were explicitly deferred. Do not confuse call elapsed seconds with server timing. |
| Persisted spend and SQL totals | Current DuckDB warm reads session settings; actual-attempt caps and PostgreSQL's per-backend pool multiplier have reviewed code and docs | Library/SQL/frame spend is still absent from command status and needs the existing persistence design. Preserve process-local semantics and the expected billing of uncached repeat calls. |
| SQL find | All three Linux installed extensions passed shared cases18/19 and focused edges; native Apple Silicon DuckDB proof passed independent review at37b57014 | Linux ARM64 package acceptance passed at aab8baad; Intel DuckDB still uses the old surface. Keep platform qualification and final release runner proof distinct. |
| Shared conformance | PG filter/rank, SQLite rank/relation and R named-file proof landed; internal failures have named equivalent boundary evidence | Accepted0244 must implement C/TypeScript/Ruby named-file forms and update selected coverage. DuckDB Q5 member-file forms and broader recognize/relate forms remain distinct; no new injection API or duplicate frame54-case suite is required. |

## Preparation correction

The preparer initially proposed a new scalar concurrency design for register73 because its problem statement describes serialized scalar calls. The coordinator read the original success criterion: it explicitly assigns numeric in-flight documentation and warm-path batching to B13d/B13e, and says no new decision is needed. Review the accepted success criteria before turning retained behavior into new mandatory work. The next audit should check the missing numeric documentation, not invent a scalar scheduler.


## Register73: accepted closure against its actual criterion

The same independent closure reviewer checked this adjacent outcome on main `7c797ce8` and returned **ACCEPT**. The equivalence page states the retained scalar model of one request in flight, the shared throttle range1–32 and warm routes. SQLite warm calls `decide_many_with`; its held test observes eight in flight at throttle eight and nine two-row requests for18 inputs. Accepted0219 installed proof covers that path. PostgreSQL warm calls the same shared bulk route per question group under one deadline; accepted0217 proves exact packed warm bodies and22 selected installed checks. PostgreSQL's held-eight measurement belongs to the array path, not a distinct warm measurement.

Earlier ticket statements kept register73 open because scalars remained serial. That extrapolation exceeds the original register criterion, which explicitly asks for numeric documentation and the B13d/e warm paths. The coordinator closes the original finding, corrects current equivalence prose and preserves serial scalar behavior. This does not create a new scalar concurrency promise or erase the historical records.
