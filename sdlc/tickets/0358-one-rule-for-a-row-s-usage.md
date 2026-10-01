# 0358: One rule for a row's usage

Status: in progress. Lane claude-1. Branch `ticket/0358-one-row-usage-rule`. Plan: `sdlc/planning/cleanup-2026-09-30.md`. Serves top-ten cleanup item 4 of `sdlc/planning/grading-2026-09-30/README.md` and cleanup item 3 of its batching report `01-batching.md`.

## Outcome

Every row sums its usage by one rule, written once beside `pipeline::Answered` in `engine/pipeline/receipt.rs`. The rule follows ADR 0111 section 7 and `specification/annotate.md:129`:

- A row's usage is the sum of its question shares, live or stored.
- It is absent when any share lacks counts, or when the row has no share.
- When every share has counts and the sum does not fit, the row fails with `the backend reported token counts whose total is too large` (backend kind; exit 4 on the command). A missing share outranks an overflow, so the order of the answers never changes the result.

The command's one-question rows, the public Rust API, the C door and the hosts over it, `annotate`, `relate` and `recognize` all use it. A row's `meta.usage` then agrees with the run totals and the usage file whenever every live reply reported usage. Those two keep counting live replies, by ADR 0111 section 7 and ADR 0113, and this ticket does not touch them.

## Evidence

- Starts from: main `056a20b6c`.
  - The batching report's maintainability row names four copies of the answers-to-receipt block: `public/asking.rs:93-126`, `cli/asking/judged.rs:167-231`, `engine/facade/each.rs:206-228` and `engine/facade/annotate.rs:59-78`. Each reads the stored answers, sums usage, names the model and collects the keys.
  - Read on main, the copies disagree in two ways:
    - `public/asking.rs`, `cli/asking/judged.rs` and `annotate.rs`'s `QuestionAnswer::read` fold with `Usage::checked_plus(answered.usage?)`. A missing share gives no usage, which is right. An overflow also gives no usage, silently, while `annotate`'s pinned test `usage_overflow_fails_safely` (`tests/backend/annotate/scheduling.rs:462`) fails the row on overflow.
    - `engine/facade/each.rs` folds with `summed`, which keeps the shares that are present when one is missing. Its own doc says "absent when either is". It fails on overflow, which is right.
  - Two row folds over whole replies share `summed`'s partial sum: `relate`'s `add_meta` (`engine/facade/relate.rs:205`) and `recognize`'s `Aggregate::add_answered` (`engine/facade/recognize.rs:447`). A `relate` or `recognize --details` row whose requests include one reply without usage reports the other replies' counts. ADR 0111 section 7 says that row has no usage. `specification/recognize.md:96` says the row "sums send counts and usage".
  - `annotate`'s `assemble` (`engine/facade/annotate.rs:131`) stops at the first missing share, but fails on an overflow met before a later missing share. Its result depends on answer order.
  - A one-question row has more than one share only for `tag`, whose labels each take one wire question (`core/pack.rs` `wire_count`), so the silent overflow is reachable on `tag` when labels land in different requests.
  - `engine/facade/each.rs`'s fold serves `find`, `relate` and `recognize` through `ask_each`. Their questions are all one wire question (`core/recognize/questions.rs:135,158,235`), so that fold never sees two shares today. The visible defect is the partial sum in `relate` and `recognize` above.
  - `relate` and `recognize` also run for the public Rust API (`public/relate.rs:264-300`) and the C door (`libraries/c/src/door.rs:177,232`). Neither reports a row's usage there, but both fail on an overflow met among the present shares, even when another share is missing.
  - `specification/result.md:142` says "Absent when the backend reports none" and keeps a batched-row share sentence that ADR 0111 retired. `result.md:226` already says "`meta.usage` is the sum of the record's question shares." The contract names no overflow rule for a row.
- Keeps: every pinned token count, `requests_sent`, `cached`, `requests`, model and error sentence on every surface. `usage_overflow_fails_safely` and `partial_failures_keep_their_question_keys_and_missing_usage_removes_the_total` (`tests/backend/annotate/splitting.rs:332`) pass unchanged. The process totals, `--facts` and the usage files are untouched (`engine/call_facts.rs`, `engine/usage*`). Each row with every share present keeps its exact sum. The check order in each asker stays: read, then the asker's own refusals, then the model, then usage. `find` uses `read` and `receipt` through `each.rs` with one share, so its counts cannot change.
- Changes: one helper, its callers, and the contract row.
  - New `engine/pipeline/receipt.rs`: `read(question, answers)` reads one question's outcomes from its wire answers; `receipt(answers, outcomes)` builds the `facade::Answered` a row reports (model, outcomes, usage, cached, first key, sends); `RowUsage` adds shares and gives the total by the rule above.
  - `public/asking.rs`, `cli/asking/judged.rs`, `engine/facade/each.rs` and `annotate.rs`'s `QuestionAnswer::read` call `read` and `receipt` in place of their copies.
  - `relate`, `recognize` and `annotate`'s `assemble` add each reply's usage to a `RowUsage` and take its total once the row's answers are in. `summed` goes.
  - `specification/result.md:142` states the rule in place of the retired batched-row sentence, and `:226` points to it. `CHANGELOG.md` gains one paragraph. It names the two corrected command rows and the outcomes library and C-door callers see change:
    - A `tag` row whose label shares overflow together fails with the backend error, on the command, the Rust API's `tag` calls, the C door's `tag` and `annotate` with a `tag` question. It returned the row without usage before.
    - A `relate` or `recognize` call with one share missing and an overflow among the others succeeds without usage. It failed with the overflow error before.
    - `annotate` with an overflow before a missing share succeeds without usage. It failed before.
- Proof: an edge table, one regression per wrong copy, and the gates.
  - An edge table for `RowUsage` beside it: no share, all present, one missing, an overflow, and an overflow before and after a missing share.
  - One outside-in regression for each wrong copy, each failing on main:
    - `tag` on the command, two labels in two requests whose input counts overflow together: exit 4 with the overflow sentence (main printed the row without usage, exit 0). Covers `cli/asking/judged.rs`.
    - The same `tag` through the public Rust API's `tag_many`: a backend error with the same sentence (main returned the row without usage). Covers `public/asking.rs`.
    - `annotate --details` with one `tag` question whose two labels land in two requests that overflow together: exit 4 with the sentence (main printed the row without usage). Covers `QuestionAnswer::read`.
    - `annotate --details` with three questions one per request, where the first two overflow and the third reports no usage: the row prints without `meta.usage`, exit 0 (main failed with exit 4). Covers `assemble`'s order.
    - `relate --details` over two requests, one reply without usage: no `meta.usage` (main printed the other reply's counts). Covers `relate`'s `add_meta`. The edge table covers the `each.rs` fold, which no surface can reach with two shares today.
    - `recognize --details` with one reply without usage: no `meta.usage` (main printed a partial sum). Covers `recognize`.
  - Each regression pins the whole standard error. A real overflow also stops the usage counters, so the overflow cases expect the fixed `usage counters could not be updated` warning after any diagnostic, as `usage_overflow_fails_safely` does.
  - Checks before landing: `sdlc/scripts/test`, `spec`, workspace clippy with `-D warnings`, `policy.py`, `tickets`, lint in a clean checkout, and the C door tests. No binding's numbers change: no binding reports `relate` or `recognize` usage, and every other binding row keeps its sum when every share is present. Only the overflow outcomes above change, and no surface check pins them. The build confirms that by searching each binding's tests for overflow and missing-usage cases, and runs any surface check that has one.
- Defers: a library `tag` row with one failed label share and an overflow among the others returns the overflow error, while the command reports the failed share first; both orders follow the ticket's check order, and the case needs a hostile backend. Cleanup item 4 of the batching report (one dry-run planner) and top-ten item 10. The run totals keep their own per-reply sum in `engine/call_facts.rs`, because they count live replies, not rows.
