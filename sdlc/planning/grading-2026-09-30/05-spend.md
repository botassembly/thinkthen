# Area 5: Spend accounting and never sending twice

Commit `58014776c`. Reviewer: Sonnet 5.5, fresh, read only. Rubric: `README.md` in this folder.

## Scope

Every attempt is counted and capped before it goes, tokens and cache answers are added to count-only monthly totals, the run facts and call facts report them, and a transport failure is never sent again because the backend may bill it.

All paths below are under `crates/thinkthen/src/` unless they start with `crates/`, `specification/`, `sdlc/` or `probes/`.

| Kind | Paths (nonblank lines) |
| --- | --- |
| Code, counts | `engine/usage.rs` (421), `engine/usage/attempt.rs` (88), `counts.rs` (42), `facts.rs` (117), `lock.rs` (37), `storage.rs` (285) |
| Code, caps | `engine/budget.rs` (159), `engine/send_budget.rs` (169), `core/budget.rs` (54), `core/price.rs` (98) |
| Code, reporting | `engine/call_facts.rs` (140), `cli/facts.rs` (70), `cli/status.rs` (297), `public/results/tally.rs` (215) |
| Code, never twice | `engine/http.rs` (496, shared with area 3), `engine/error.rs` `retryable` (`:190`) |
| Code, live ledger | `sdlc/scripts/live` (285) |
| Code total | about 2,480 lines without `http.rs` |
| Tests | About 53 tests. Unit: `engine/usage/tests.rs` (14), `core/price.rs` (2), `engine/budget.rs` (1), `engine/send_budget.rs` (1), `public/results/tally.rs` (1). Integration: `crates/thinkthen/tests/backend/resend.rs` (10), `tests/backend/facts.rs` (9) with `facts/priced.rs` (3) and `facts/usage_lock.rs` (1), `tests/backend/default_cache/usage.rs` (7), `tests/public_cap.rs` (1), `tests/public_estimated.rs` (1, 443 lines), `tests/library/public_tally.rs` (3). Unit, never twice: `engine/http/tests.rs:144` |
| Contract | `specification/settings.md` rows Retries, Request limit, Process request total, Estimated input admission total, Caller prices, Run facts, Status format; `specification/result.md` `meta` and "The run facts line"; `specification/backends.md` "The request"; `sdlc/scripts/README.md` live ledger; ADRs 0022, 0034, 0049, 0089, 0097, 0101, 0108, 0111 section 7, 0113 |

## Complexity: 4 of 5

| Driver | Score | Measured fact |
| --- | ---: | --- |
| Code size | 3 | About 2,480 nonblank lines in 16 files, of which 285 are a Python script |
| States and concurrency | 5 | A write-behind thread with a queue, a condvar and a failure latch (`engine/usage.rs:29-51`, `:177-215`), a cross-process advisory file lock retried with a doubling pause and a one-second finish deadline (`engine/usage/lock.rs:24-43`), a fork rebuild (`engine/budget.rs:13`), CAS counters with refund on drop (`engine/budget.rs:86-176`), and a live ledger under `flock` (`sdlc/scripts/live:115-123`) |
| Rules and refusals | 4 | About 50. Reserve before send and refund if unmarked (three counters), three send-budget denials and three estimated-input denials, the `908/1000` estimate, every retried send counted, a transport failure never resent, a reply bound of 1 MiB plus 8 bytes a byte, usage absent when any share is missing, cost only when every attempt reported, tokens only when every reply reported, month files at mode 0700 and 0600, symlink refusal, two overflow refusals, a write failure that stops all later writes, and 25 `refuse(` calls in `sdlc/scripts/live` |
| Surfaces touched | 5 | 22 of 22. Every surface adds to the totals since ADR 0113, and the C door and SQL hosts carry facts |
| Settings | 4 | Seven rows: Retries, Request limit, Process request total, Estimated input admission total, Caller prices, Run facts, Status format |
| Contract weight | 5 | Four spec pages with sections, nine ADRs (0022, 0034, 0049, 0089, 0097, 0101, 0108, 0111, 0113) and ADR 0111's amendment |
| Churn and debt | 4 | 35 commits on the paths since 2026-09-23, 9 of them on 2026-09-30. At least nine fix commits, among them `2cfe3086c` (overflow before migrating files), `cebe945fc` (bound the usage lock at exit), `74fffb9e0` (restore old usage readers) and the usage-leak fix `87602e2ad`. Two open items: `sdlc/issues/2026-09-26-every-surface-should-give-back-run-facts.md` (five parts) and item 1 of `sdlc/issues/2026-09-25-public-library-api-gaps.md` |

Mean 4.3, rounded to 4.

## Grades

| Dimension | Grade | Evidence |
| --- | :---: | --- |
| Quality | C | The send path does what the contract says. The count is marked before transport and a failed mark refunds every reservation (`crates/thinkthen/src/engine/http.rs:272-286`, `engine/send_budget.rs:91-138`, `engine/budget.rs:131-137`). Only a retried status is sent again (`engine/error.rs:190-192`, `engine/http.rs:322`). Three contract divergences hold it at C. First, `Engine::usage()` says "This process's totals" (`public/engine.rs:176`) while every built engine owns its own counters (`public/settings.rs:414`, issue item 1 in `sdlc/issues/2026-09-25-public-library-api-gaps.md`), so a second engine starts at zero and a caller reads the wrong spend. Second, `engine/facade/each.rs:219-221` and `:296-304` keep a partial usage sum when one share is missing, where ADR 0111 section 7 says a row's usage is absent when any question lacks a share and `public/asking.rs:108-113` follows the ADR. The helper's own comment says "absent when either is". Third, `specification/result.md:157` lists six statuses as `retryable:true` (429, 500, 502, 503, 504, 529) while the code marks eleven (`engine/error.rs:105`, used at `cli/failure/facts.rs:42`) and `specification/backends.md` lists eleven |
| Reliability | C | Never-resend has a loopback-count test for each transport class that can be scripted: close, reset, stall, cut body and oversize reply each arrive once (`crates/thinkthen/tests/backend/resend.rs:102-179`), and a retried status is still sent again (`:193`). Usage failures are injected at eight stages (`engine/usage.rs:217-249`, 14 tests in `engine/usage/tests.rs`). The caps refund on a cancel and on a denied estimate (`engine/send_budget.rs:154-182`, `engine/http/tests.rs:248`). Weak points: one usage-leak defect landed and was fixed on 2026-09-30, when stress runs wrote the real totals (`87602e2ad`); the open ureq issue says a legitimate send can fail with "the backend closed the connection" when a short keep-alive races, and the rule forbids the healing resend (`sdlc/issues/2026-09-30-ureq-reuses-a-connection-after-an-http-1-0-reply.md`, "The idle case", choice for Ian); the usage-lock tests use real clocks (`tests/backend/facts/usage_lock.rs`); and the owner files changed on 7 of the last 7 days. `Counters::add` silently skips a count when the queue lock is poisoned (`engine/usage.rs:128-130`) while `snapshot` reads through poison (`:124`) |
| Maintainability | C | The same spend rules live in three places. The process counters keep `token_sum_valid`, `live_with_usage`, `live_without_usage` and `mixed_models` (`engine/usage/facts.rs:10-19`, `:83-128`), the per-call facts repeat the token and cost rules with different fields and no mixed-model flag (`engine/call_facts.rs:19-30`, `:74-118`), and the tally repeats them again (`public/results/tally.rs:20-21`, `:108`). Four askers sum a row's usage by two rules (`public/asking.rs:108-113` against `engine/facade/each.rs:219-221`). `engine/usage.rs` mixes counters with nine `cfg(test)` hooks and ten file-safety helpers in one 421-line file (`:229-425`). `engine/usage/tests.rs` holds 497 of 500 lines and `tests/public_controls/call_facts.rs` 494. A lint suppression sits on `Engine::usage` with a stale reason (`engine/facade.rs`) |

## Strengths

- Attempts are counted before transport and refunded only when usage could not mark them, so a crash never hides a send (`engine/http.rs:272-286`, `engine/usage/attempt.rs:57-94`).
- The process total and the estimate reserve together and refund together, which a test pins (`engine/send_budget.rs:154-182`).
- Usage files are count-only, private (0700 and 0600), symlink-refusing and lock-bounded at exit (`engine/usage.rs:298-409`, `engine/usage/lock.rs:9-43`, ADR 0097).
- Rounding is exact integer arithmetic with one rounding for the whole amount, and its decimal grammar has a refusal table (`core/price.rs:24-63`, tests `:70-110`).
- The live ledger precharges durably under one lock, refuses a malformed or legacy ledger, and hands the key only to the charged job, refusing a blank or multi-line key first (`sdlc/scripts/live:115-145`, `:230-240`, `:265-303`).

## Cleanup

1. **Make `Engine::usage()` true.** Where: `public/engine.rs:176`, `public/settings.rs:414`. Why: the rustdoc promises process totals and the engine returns its own counts. Either give engines built in one process one shared in-memory count, or change the sentence to "this engine's totals". Item 1 of the library-gaps issue asks for the same. Size: S for the sentence, M for the shared count. Blocks 0.1: yes.
2. **Correct the retryable status list in the run facts text.** Where: `specification/result.md:157`. Why: the contract lists six statuses, the code and `backends.md` eleven, so a script written from the contract expects `retryable:false` on a 520 and gets `true`. Size: S. Blocks 0.1: yes.
3. **Sum a row's usage by one rule.** Where: `engine/facade/each.rs:219-221`, `:296-304`, `public/asking.rs:108-113`. Why: ADR 0111 section 7 says absent when any share is absent. `each.rs` keeps a partial sum on recognize and relate rows, and errors on overflow where `asking.rs` drops the usage. One helper beside `pipeline::Answered` settles it. This is cleanup item 3 of the batching report. Size: S. Blocks 0.1: no.
4. **Keep one accounting rule for tokens, cost and model.** Where: `engine/usage/facts.rs`, `engine/call_facts.rs`, `public/results/tally.rs`. Why: three implementations of "every reply reported usage and the sum is exact", already different on mixed models. A single small value type with `add` would serve all three. Size: M. Blocks 0.1: no.
5. **Let the live wrapper carry its cap to the job (unconfirmed).** Where: `sdlc/scripts/live:298-303`. Why: the wrapper records `charge N` and then runs the job with no cap in the environment, and `rg` finds no `--max-estimated-input-tokens-total` in any `probes/*/job.sh` or `sdlc/live-test`. A job that sends more than `N` tokens is stopped by nothing. I read no job in full and the README says the wrapper "owns no completion line", so the intent may be that jobs police themselves. Setting `THINKTHEN_MAX_ESTIMATED_INPUT_TOKENS_TOTAL` bounds input only. It is a development tool and ships nowhere, so I do not block 0.1 on it; Ian should decide. Size: S. Blocks 0.1: no.
6. **Take the file-safety helpers out of `usage.rs`.** Where: `engine/usage.rs:229-425`. Why: nine `cfg(test)` hooks and ten helpers for modes and identity sit beside the counters, so a reader of the counters must skip 200 lines. Move them to `engine/usage/private.rs`. Size: S. Blocks 0.1: no.
7. **Make a poisoned queue count or fail, not skip (unconfirmed).** Where: `engine/usage.rs:127-130`. Why: `add` returns without counting when the lock is poisoned, while `snapshot` and `run_snapshot` read the same data through poison, so a count can vanish with no failure flag. The writer catches unwinds, so I found no path that poisons the lock. Size: S. Blocks 0.1: no.
8. **Decide the ureq keep-alive choice and record it.** Where: `sdlc/issues/2026-09-30-ureq-reuses-a-connection-after-an-http-1-0-reply.md`, `engine/http.rs:158`. Why: the default keeps the never-resend rule and accepts an exit 4 on a short keep-alive race. The one-second idle age cut the loopback failure from 60 of 60 to 0 of 60. Ian chooses between keeping the rule and a guarded single resend. Size: L with an ADR if he picks the resend. Blocks 0.1: no.
9. **Split the two test files at the cap.** Where: `engine/usage/tests.rs` (497), `crates/thinkthen/tests/public_controls/call_facts.rs` (494). Why: the next case fails the cap. Size: S. Blocks 0.1: no.

## Confidence: medium

What was read: `engine/usage.rs`, `attempt.rs`, `lock.rs`, `counts.rs`, the first 145 lines of `facts.rs`, `engine/budget.rs`, `engine/send_budget.rs`, `core/budget.rs`, `core/price.rs`, `engine/call_facts.rs`, `cli/facts.rs`, `engine/http.rs`, all of `sdlc/scripts/live`, ADR 0111 section 7, ADR 0113's decision, the run facts and `meta` text, and the names and clock use of every test file listed.

Not checked: `engine/usage/storage.rs`, `cli/status.rs` and `public/results/tally.rs` were skimmed for size and names only. ADRs 0022, 0034, 0049, 0089, 0097, 0101 and 0108 were not read beyond titles. No test and no script ran, so the `live` gap (item 5), the poisoned-lock gap (item 7) and the test flake rates are inferred from code and issue text. The SQL hosts' and other bindings' spend checks were not read. Nothing in `sdlc/scripts/live` was run, and no ledger or key file was opened.
