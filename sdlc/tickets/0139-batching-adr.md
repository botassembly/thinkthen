---
flow: build
priority: 139
opens: sdlc/planning/adr/0048-records-batch-into-full-requests.md sdlc/planning/adr/0007-flat-verbs-bare-values-and-one-threshold.md sdlc/planning/adr/0008-an-eval-is-annotate-plus-report.md sdlc/planning/adr/0010-one-wire-shape-two-variables-and-a-smaller-version-one.md sdlc/planning/adr/0032-explicit-backend-profiles-carry-limits-and-calibration.md sdlc/planning/adr/0040-split-requests-under-backend-limits.md sdlc/planning/adr/0009-what-the-vendors-how-to-pages-change.md specification/roadmap.md specification/annotate.md specification/records.md specification/result.md specification/channels.md specification/question-file.md specification/backends.md sdlc/records sdlc/tickets
---

# 0139: Record the batching rulings in one ADR

Status: COMPLETE.

Opened as: 2026-10-11. 2026-09-26 (`sdlc/records/0139-build-batching-adr.md`). A fresh code review accepted it after one round of fixes. Owner: Claude.

Review route: a fresh read-only Claude session reviews this design and the final diff. Codex does not review this ticket unless Ian routes it.

## Outcome and authority

Ian's batching rulings of 2026-09-26 become one accepted ADR, and the Settled pages they contradict say so. Every later batching ticket, and recognize ticket R0, then cites one ADR section in place of the design issue.

The authority is Ian's eleven rulings in the batching design, sent to the main builder on 2026-09-26, with review findings applied at `9b667c09`. Ruling 9 puts B0 first. The design's B0 row lists the amendments. Ian can overturn each ruling, as the design's "Open items" list says.

B0 writes no code. It changes no behavior, no help text, no fixture, and no schema. The command still sends one record a request after it lands.

## What happens today

Five accepted ADRs and six Settled pages forbid what Ian ruled. Each line below is at `origin/main` `d410ef4a`.

| Where | Line | What it says today |
| --- | --- | --- |
| ADR 0007 | 103 | "Each record is its own request, and records never share model context." |
| ADR 0008 | 41 | "Two pieces of evidence never share a request, because records must not see each other" |
| ADR 0008 | 43 to 50 | The request table: N records make N requests |
| ADR 0008 | 52 | "Every request inside one command is independent of every other" and the round runs up to `jobs` |
| ADR 0010 | 34 | "`--jobs N` sets how many requests are in flight at once." It stays true, because a batch is one request. B0 says so |
| ADR 0032 | 18 | "Calibration identity enters the question or question-set digest." |
| ADR 0040 | 20 | "Records, evidence text, options, and unrelated calls are never combined or divided." |
| ADR 0040 | 24 | "Other plans and other addresses keep no ceiling. The accuracy size of a request belongs to the packing ADR." |
| `specification/roadmap.md` | 27 | `--context FILE` is held |
| `specification/roadmap.md` | 35 | Packing many records into one request is held for isolation |
| `specification/records.md` | 81 | "Records never share model context, except that `find` …" |
| `specification/records.md` | 85, 91 | One request normally carries one piece of evidence. N records make N requests |
| `specification/records.md` | 109 | The stop line names one record. "A run that finishes prints nothing there." |
| `specification/records.md` | 133 | The output buffer holds at most `jobs` rows |
| `specification/result.md` | 38, 99 to 108 | `meta` has no batch, share, context or batch-warning field. "The other fields are always present." |
| `specification/channels.md` | 14 | Standard error is "Never parsed by a script" |
| `specification/channels.md` | 32 | The advanced option list has no `--batch`, `--context` or `--facts` |
| `specification/channels.md` | 99 | Record-mode `--dry-run` plans the first record |
| `specification/question-file.md` | 92 to 100 | No `batch` row. Precedence is "the command line, then the file, then the default" |
| `specification/backends.md` | 21 | "Every other plan and every other address has no ceiling." |

## Design

### The ADR

B0 writes `sdlc/planning/adr/0048-records-batch-into-full-requests.md`. No branch or worktree claims 0048. Main's last ADR is 0047, and 0044 to 0046 stay behind at the branch tag, by `sdlc/planning/one-line-plan-2026-09-24.md` line 48.

Status line: accepted 2026-09-26 on Ian's rulings, built by the batching tickets it names, Ian can overturn each item. Its sections:

1. **Context.** Per-request overhead of about 250 input tokens (experiment 271). A median round trip of 135 to 151 ms, of which Jev's own time is 57 to 69 ms (experiment 268). Jev takes 7,000 questions in one request and refuses only past about 65,536 input tokens (experiment 271). Each number names its section of `sdlc/records/2026-09-26-batching-and-recognize-evidence.md`.
2. **Decision.** One numbered item per rule, each copied from the design section it cites:
   1. The batch shape: the quote prefix, the evidence object `{"records":[…]}` without a context, the context as evidence with one, equal evidence asked once, and a batch of one without a context sending today's bytes (design section 1).
   2. Where a batch closes: fill to the limit, the content cut at SHA-256 mod 4,096 with its exact byte rule, the size, the profile limits and the 96,000-byte ceiling, the 50 ms pause and when it is off, and end of input. What an insert moves (section 2).
   3. The setting: `max` or a whole number of at least 1, `--batch 0` and `--batch fill` a usage error at exit 2 (sections 2 and 6).
   4. Precedence, in four tiers: the typed value, then the environment, then the question file, then the default. Only a per-call value counts as typed: `--batch` or a per-call `batch=`. The environment tier holds `THINKTHEN_BATCH`, the engine setting and the SQL `SET`. The four tiers are design section 6, exactly. The read-only configuration file (ADR 0033) holds no `batch` key, so it adds no tier. The ADR states these tiers directly and cites no older order. ADR 0007's profile order is stale: nothing reads `THINKTHEN_PROFILE`, and ADR 0010 and ticket 0007 removed the profile map. Ticket C1 owns `specification/settings.md`. C1 lists `batch`, `context` and `facts` in its "Settings on the way" list below the table, each with what it does and its designed default. Each moves into the table when the ticket that builds it lands: `batch` with B4, `context` with B7, and `facts` with B5. B8, B9, B10 and the library and SQL tickets B12a to B13e update the row's surface cells as they land.
   5. Order, jobs, cache and replay: `--jobs N` means N batches in flight, 1 to 32, default 4. The cache key is the batch's request digest. `--jobs` changes no batch (section 3).
   6. Failure: one failed request fails its batch. The stop line names the range and echoes no record. A retried status resends the whole batch. No batch is split and resent (section 4).
   7. Which functions batch, and the one-record-a-request cases (section 5 table).
   8. The batch setting is calibration identity. It stays out of the question digest, unlike ADR 0032's `profile` (section 7, ruling 5 of "Open items").
   9. Row metadata: even shares with the remainder to the earliest records, `meta.batch` and its fields, `meta.context_sha256`, `meta.batch_warning`, and an unreported usage field staying absent ("Output and run facts").
   10. Run facts: `--facts` prints one `thinkthen.run/1` line on standard error, finished or stopped. The command is silent there without it. The seven fields. Library results carry `facts` on every call (ruling 7). The run-facts ADR, which ticket B12a writes, picks how a bare-value call carries them. B0 does not amend ADR 0017.
   11. `--context FILE`: a reference text sent once a request. It enters the request digest and not the question digest.
   12. Speed ahead of accuracy: the default fills to the limit. Test 9 measures and reports the cost and gates nothing (rulings 1 and 3). The speed target and its owner, ticket S1 (ruling 4).
   13. `--dry-run` plans the first batch and never waits on a pause.
3. **What this amends.** The table under "What happens today", with the amendment each line gets.
4. **Which ticket builds each item.** One row per decision item naming the design's ticket label: B3 for items 1 and 2, B4 for 3 to 6 and 13, B5 for 9 and 10's command line, B16 for 8, B7 for 11, B8 to B10 and R7 for item 7's rows, B10 for the question set's `batch` under item 4, B12a to B13e for the libraries and SQL surfaces. A later ticket's "covered by B0" row then points at one item.
5. **What recognize R0 may rely on.** The list under "What R0 may rely on" below.
6. **What Ian can overturn.** The design's fifteen "Open items", in its order, plus this ticket's decisions 3, 4 and 6.

The ADR copies each rule. It does not re-argue it. The design issue stays the argument, and the evidence record stays the measurement.

### The amendments

Accepted ADRs keep their text. Each named line gains the marker `(Amended by ADR 0048, below.)`, and the ADR gains one section `## Amendment, 2026-09-26: ADR 0048 batches records` of at most four sentences. This is the form ADR 0007 already uses at line 103 for ticket 0137.

Settled pages keep today's sentence and gain the new rule after it, in one fixed form: `Not built yet, by ADR 0048 item N: …`. N is the ADR's decision item. The page stays true today. The reader sees what changes and finds the rule in one place. The ticket that builds item N deletes the old sentence and the marker in the same commit, and the ADR's item table names that ticket. `grep -rn "Not built yet, by ADR 0048" specification` lists every leftover. The last batching ticket to land requires that grep to come back empty. Each page's status line adds `amended by ADR 0048`.

| Page | Line | Amendment |
| --- | --- | --- |
| `roadmap.md` | 27, 35 | Both rows leave the held table. One sentence under the table says ADR 0048 brought in `--context FILE` and batching |
| `records.md` | 3 | Status adds ADR 0048 |
| `records.md` | 81 | Item 1: records of one batch share one request |
| `records.md` | 85, 91 | Item 2: the default fills each request to the limit. N records make N requests only at `--batch 1`. The table row gains that condition |
| `records.md` | 109 | Item 6: a batch failure names the range. Item 10: `--facts` prints one line on a finished run |
| `records.md` | new paragraph after 133 | Item 5: `jobs` counts batches in flight, and the buffer holds at most `jobs` batches of rows. Line 129 does not change, because ticket J1 edits it |
| `result.md` | 3, 38 | Status adds ADR 0048. Item 9: line 38 names the three fields that can be absent |
| `result.md` | table after 108 | Item 9: rows for `batch`, `batch_warning` and `context_sha256`. The `usage` and `requests_sent` rows say a batched row carries its share |
| `channels.md` | 3, 14 | Status adds ADR 0048. Item 10: standard error gains one exception, the `--facts` line, which is for a script |
| `channels.md` | 32 | Items 3, 10 and 11: `--batch N`, `--facts` and `--context FILE` join the advanced options |
| `channels.md` | 99 | Item 13: record-mode `--dry-run` plans the first batch |
| `question-file.md` | 3 | Status adds ADR 0048 |
| `question-file.md` | new row after 92 | Item 3: the batch setting, `--batch N`, key `batch`, default `max`, refusals `0`, a fraction, and any text but `max` |
| `question-file.md` | new paragraph after 100 | Items 4 and 8: `--batch` replaces the file's `batch`, as the single values on line 100 replace theirs. `batch` takes four tiers: the typed value, then the environment, then the file, then the default. Only a per-call value counts as typed. The read-only configuration file (ADR 0033) holds no `batch` key. It names the setting the threshold was tuned at, as `profile` names the backend. It stays out of the digest. A question set carries at most one top-level `batch`, as it carries one `profile`. Line 98's ruling stands for every other setting |
| `backends.md` | 3, 21 | Status adds ADR 0048. Item 2: line 21's last sentence gains that a batched record plan at the built-in address also closes at the ceiling |
| `annotate.md` | 3, 108 | Status adds ADR 0048. Item 7: the records of one `on` group share a batch's request, and `--batch 1` keeps them apart. Added after code review |
| `backends.md` | new paragraph after 21 | Items 2 and 6: profile limits close batches, a retried status resends the whole batch, and a batch is never split and resent |

The design issue section 6 says B0 adds `batch` to `question-file.schema.json`. Decision 4 moves that to the building tickets.

Section 6 also says B0 adds `--batch` to line 100's list. B0 does not edit line 100, because that line lists overrides the command accepts today, and `--batch` does not exist yet. The new paragraph after it carries `--batch` under the marker. The ticket that builds item 4 moves `--batch` into line 100's list when it deletes the marker.

**`batch` on an `annotate` question set.** A set takes one top-level `batch`, assigned to ticket B10. The design's section 7 says the file's `batch` "names the setting its threshold was tuned at, as its `profile` names the backend". ADR 0032 line 18 lets a question set name one top-level `profile`, and nested questions cannot name another. `batch` follows the same rule. B10 batches `annotate` per `on` group (design section 5 and the B10 row), so B10 adds the set key with its parser and schema, as decision 4 gives.

### What R0 may rely on

Recognize ticket R0 lands after B0 and builds on it. R0 may cite these as settled:

- ADR 0048 by number, with decision items 1 to 13 at the numbers above.
- The batch shape (item 1) and the close rule (item 2). R0's recognize batch shape extends them for many short texts.
- The setting, its spelling `max`, and its four-tier precedence (items 3 and 4).
- `meta.batch` and its fields in `result.md` (item 9). R0 adds recognize's use of it.
- `jobs` as batches or requests in flight (item 5). R0 amends `records.md` `jobs` for `recognize` on one document.
- ADR 0040's amendment reaching batched record plans. R0 amends ADR 0040 again for word and `confirm` questions.
- The canonical question form unchanged. `batch` and `context` stay out of the question digest, so `question-file.md` line 130's key order is R0's to amend alone.

B0 leaves these for R0: `backends.md` line 17, `recognize.md`, `relate.md`, the recognition keys in `question-file.md`, and `result.md`'s `answer.confirm`.

## Decisions

Each is the owner's call under Ian's rulings. Ian can overturn any of them.

1. **One ADR, 0048, for every batching rule.** The design row asks for one. Later tickets each cite an item, so no second batching ADR is needed until the run-facts spelling (B12a).
2. **Accepted ADRs are amended by marker and a dated section.** `CLAUDE.md` says to amend an accepted ADR only where history matters. The marker keeps the old sentence readable beside its replacement.
3. **Pages keep today's sentence and add the rule under one fixed marker, `Not built yet, by ADR 0048 item N: …`.** The marker points at the ADR item and not at a design ticket label, so it stays right when tickets are renumbered or split. One fixed phrase lets one `grep` list every leftover. The other choice rewrites the pages to the new rule now. That makes the Settled contract false until the rules are built, and a reader cannot tell.
4. **The schema waits for the ticket that parses `batch`.** `specification/fixtures/question-file/README.md` holds the parser and `question-file.schema.json` to one verdict over `corpus.json`. A schema that accepts `batch` while the parser refuses it breaks that agreement, and a corpus case would turn the parser test red. B4 adds `batch` to the `decide` entry with its parser code and corpus cases. B8 adds it to `choose`, B9 to `tag` and `score`, and B10 to the `annotate` question set.
5. **`records.md` line 129 does not change.** Ticket J1 amends the `jobs` paragraph for `relate`. B0 adds its own paragraph after line 133, so the two tickets touch different lines.
6. **`--batch`, `--context` and `--facts` are advanced options.** Batching is automatic, so a new user needs none of them. They go in the long help with `--jobs`. Short help stays as it is.
7. **B0 closes no issue.** The batching design stays open until its last ticket lands. B0 does not edit the design issue, which the other in-flight batching tickets also cite.

## Edge cases

| Case | What B0 does |
| --- | --- |
| A line ticket 0138 also edits | 0138 changes `backends.md` lines 65 and 77 and `question-file.md` lines 59 and 90. B0 edits `backends.md` lines 3 and 21 and inserts after 21. It inserts in `question-file.md` after 92 and after 100. No hunk touches another. Whichever lands second merges `origin/main` first |
| `records.md` line 129, which J1 edits | Untouched. B0's `jobs` paragraph goes after line 133 |
| `specification/settings.md`, which C1 writes | Untouched. C1 lands after B0. The ADR says C1 lists `batch`, `context` and `facts` under "Settings on the way", and B4, B7 and B5 move them into the table when they land |
| `backends.md` line 17, `recognize.md`, `relate.md` | Untouched. R0 and J1 own them |
| `result.md`, which a test reads | `contract_pages_name_the_tuned_for_key_and_never_the_old_one` (`crates/thinkthen/tests/backend/profile.rs:495`) requires exactly one `{"tuned_for":NAME,"running":NAME}` and no `"calibrated"` or `calibrated:`. The `batch_warning` row writes `{"tuned_for":1,"running":"max"}`, which that shape does not match, and never the old key |
| An ADR number another branch takes first | Stop rule 5 |
| A number in the ADR | It names its section of the evidence record or its experiment |
| A rule the design states and Ian's rulings do not cover | The ADR copies it and lists it among the author's calls Ian can overturn |
| A page sentence the building ticket forgets to delete | `grep -rn "Not built yet, by ADR 0048" specification` lists it. The ADR's item table names the ticket that removes each marker. The last batching ticket requires the grep to come back empty |
| `batch` on an `annotate` question set | One top-level key, as `profile` is. Ticket B10 builds it. See "`batch` on an `annotate` question set" above |

## Proof

B0 adds no test. It changes no behavior, so a new test would check prose. The building tickets carry the design's tests 1 to 13, each against the real command.

The proof is the review plus the checks that already guard these pages:

- The reviewer reads the ADR against the design's sections and rulings, item by item. The reviewer reads each amended line in the diff against the amendment table above, and confirms every changed page keeps today's sentence beside the marked rule.
- `grep -rn "Not built yet, by ADR 0048" specification` lists one hit for each marked passage in the amendment table, and no other. The reviewer checks each hit names the right item.
- The reviewer confirms the diff touches only the files in `opens`. Nothing under `crates`, `libraries`, `databases`, `conformance`, `spec`, `site`, `demos`, or `specification/fixtures` changes, and `question-file.schema.json` does not change. This is a review item, not a plant.
- `sdlc/scripts/lint`: the ticket format check, the private-name check, and the rest of the rung.
- `contract_pages_name_the_tuned_for_key_and_never_the_old_one`, run alone as `flock -o /run/user/1000/thinkthen-heavy.lock env -u THINKTHEN_API_KEY cargo test -p thinkthen --test backend contract_pages_name_the_tuned_for_key`.
- `grep -rnF -e "never share model context" -e "its own request" -e "Never parsed by a script" -e "has no ceiling" -e "prints nothing there" -e "plan for the first record" -e "the command line, then the file, then the default" spec` comes back empty, before and after the build. No executable page quotes an amended sentence, so none can turn red. `sdlc/scripts/spec` is not run, because it never reads `specification/`.

| Guard | Planted fault that turns it red |
| --- | --- |
| The contract-page test | (a) Write a second `{"tuned_for":NAME,"running":NAME}` into the new `batch_warning` row. (b) Write the row's key as `"calibrated"` |
| The ticket check | (c) Drop the `- Defers:` item from this ticket's Evidence section. This plant tests the ticket file, not the ADR or the pages |

The four questions, for the guards B0 relies on and adds nothing to:

- **What behavior does each protect?** The contract-page test protects the one warning shape on `result.md`, which the new `batch_warning` row sits beside. The ticket check protects this ticket file's Evidence section.
- **What credible regression fails it?** Copying the profile warning's shape for the batch warning, or naming it with the old key. A ticket missing an Evidence part.
- **Why does no existing test catch it?** Each is an existing guard. B0 adds nothing a new test would reach.
- **Does it need a test-only hook?** No.

## Budgets

Nonblank lines, measured with `grep -c .` on the diff.

- ADR 0048: at most 170 nonblank lines.
- The five amended ADRs: at most 12 added or changed lines each, 50 in all.
- The six specification pages: at most 45 added or changed lines in all.
- The build record: at most 40 nonblank lines.
- Nothing else changes. No code, test, fixture, schema, ratchet, or dependency. The `surfaces` rung is not required.

## Stop rules

1. Stop before crossing a budget.
2. Stop if an amendment needs a ruling Ian did not give and the design does not state. Report it with options.
3. Stop if a plant stays green, or a rung goes red for a reason page text cannot fix.
4. Stop before touching `specification/settings.md`, `relate.md`, `recognize.md`, `backends.md` line 17, `records.md` line 129, `question-file.schema.json`, `site/`, or the design issue.
5. Stop if another branch claims ADR 0048 before this one lands. The coordinator renumbers.
6. Stop if ticket 0138, C1, J1, or R0 edits a line B0 edits. The coordinator orders the two.

## Scope and exclusions

Excluded: any code. `specification/settings.md` (ticket C1). The schema (decision 4). `relate.md` and relate's `--jobs` (ticket J1). Recognize's rules and pages (ticket R0). ADR 0017 and the library `facts` spelling (ticket B12a). The D1 page and `site/`.

## Routing

Owner and builder: Claude (Opus subagent). Reviewer: a fresh read-only Claude session for design and for the diff. No ceiling moves and no public surface widens.

## Complexity

Contract 2; state and timing 0; reach 2; proof 0; cost of error 1; total 5. Final level: 1. The risk is an amendment that misstates a ruling. The item-by-item review guards it.

## Deferred gaps

- `question-file.schema.json` gains `batch` in B4, B8, B9 and B10 (decision 4).
- `specification/settings.md` lists `batch`, `context` and `facts` under "Settings on the way" from C1. B4, B7 and B5 move them into the table.
- The marked sentences stay on the pages until the tickets that build their items land. The last batching ticket empties the grep.
- `batch` on an `annotate` question set waits for B10.
- The library `facts` spelling on a bare-value call waits for the run-facts ADR in B12a.
- SQL per-call facts stay deferred by `sdlc/issues/2026-09-26-every-surface-should-give-back-run-facts.md`.
- The context-time figure stays unclaimed until S1 measures it.

## What Ian can overturn

- Every ruling and author's call the ADR copies. The design's "Open items" list names fifteen.
- Decision 3: marked sentences in place of rewriting the pages now.
- A question set taking one top-level `batch`, built by B10, in place of no `batch` on sets.
- Decision 4: the schema waiting for its parser.
- Decision 6: the three options in long help only.

## Closes

None. `sdlc/issues/closed/2026-09-26-batching-design.md` stays open until its last ticket lands.

## Evidence

- Starts from: Ian's rulings 1 to 11 and the B0 row in `sdlc/issues/closed/2026-09-26-batching-design.md` at `9b667c09`. `sdlc/records/2026-09-26-batching-and-recognize-evidence.md` sections 1 to 9, from experiments 208, 260, 261, 262, 268 and 271 and the 2026-09-22 wire probe. The closed packing issue `2026-09-25-packing-and-batching-what-they-buy-what-they-cost-and-the-setting.md`. The recognize design's R0 row. The page lines in "What happens today" at `origin/main` `d410ef4a`.
- Keeps: Every behavior, help line, fixture, schema and test. Every accepted ADR sentence, marked and not deleted. Every Settled page's current sentence, beside its marked replacement. The question digest's canonical form.
- Changes: A new ADR 0048 holds the batching rulings. ADRs 0007, 0008, 0010, 0032 and 0040 carry markers and dated amendment sections. `roadmap.md` drops two held rows. `records.md`, `result.md`, `channels.md`, `question-file.md` and `backends.md` state each new rule beside today's, under the marker.
- Proof: The item-by-item review against the design and the amendment table. The marker grep and the `spec/` grep. `lint` and the contract-page test on `result.md`, with plants (a) to (c) each turning one red. Plant (c) tests this ticket file.
- Defers: The schema key to B4, B8, B9 and B10. The "Settings on the way" lines to C1, and their table rows to B4, B5 and B7. The library `facts` spelling to B12a. SQL per-call facts. Removing each marker to the ticket that builds its item.
