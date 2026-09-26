---
flow: build
priority: 144
opens: crates/thinkthen/src/core/batch.rs crates/thinkthen/src/core/batch crates/thinkthen/src/core/mod.rs crates/thinkthen/src/core/records.rs crates/thinkthen/src/core/backend.rs crates/thinkthen/src/engine/prepared_request.rs specification/fixtures/systemone specification/fixtures/batching sdlc/ratchet.json sdlc/records sdlc/tickets
---

# 0144: The engine plans batches

Status: ready. The coordinator accepted it on 2026-09-26 after a fresh read-only review. Owner: Claude. Lane: `worktrees/thinkthen-lane-3`.

Review route: a fresh read-only Claude session reviews this design and the final diff. Codex does not review this ticket unless Ian routes it.

## Outcome and authority

The pure core turns a stream of records into batches. Each batch is one request: its plan, its exact body, its digest, the records it holds, and why it closed. A batch of one record without a context is today's request, byte for byte. Nothing calls the planner outside its tests yet. Ticket B4 puts it on the command.

This is ticket B3 in `sdlc/issues/2026-09-26-batching-design.md`. Its row reads: "The engine plans batches. Fill to the limit, content cuts, size, profile limits, ceiling, quote prefix, evidence object, duplicates, batch of one, digests. No surface change. Proof: tests 1, 2 and 3. Depends on B0. Covered by B0." ADR 0048 is B0. Its ticket table gives items 1 and 2 to B3 for the plan and to B4 for the command. Ian's ruling 9 in the design orders B3 after B0, C1, B1, B2 and J1, and before S1 and B4.

B3 builds alongside 0141 (B1) and 0143 (J1), because no file collides. It lands only after both have landed. The coordinator ruled this within ruling 9, and Ian can overturn it.

## What happens today

Each record is its own request. `Engine::judge` in `engine/facade.rs` builds a `Plan` of one evidence and one question and asks it. `built_in::encode` in `core/adapters/systemone/request.rs` writes the body. `PreparedRequest::with_profile` in `engine/prepared_request.rs` checks the exact body against a profile through `BackendProfile::check` in `core/backend_profile.rs`, and digests it through `recording::Exchange::digest`.

`PreparedRequests::with_profile` already splits one plan's questions under a profile and a byte ceiling. It splits questions over fixed evidence. A batch grows its evidence and its questions together, so that splitter does not fit. `Backend::relation_ceiling` in `core/backend.rs` returns the 96,000-byte ceiling at the built-in address. Only `SettledRelation::settle` calls it.

## Design

One new core module, `core/batch.rs`, holds the planner. It is pure. It reads no file, clock or process, and it takes every value from its caller.

**Inputs.** A `Batcher` takes the backend, the profile if any, the model, the question, the batch setting, and an optional context. The batch setting is `Max` or `Records(NonZeroUsize)`. B4 adds the parser for `max` and `N`. The context is an `Evidence`. B7 reads the file and names the option.

**The batch record.** A `BatchRecord` carries two things:

- `evidence`: today's `Evidence`, which the batch-of-one form sends unchanged.
- `value`: the JSON value the batch quotes, lists in the evidence object, hashes for the content cut, and compares for copies. For a whole JSONL, CSV or TSV record it is the record's own value, an object, as the design's section 1 asks. For a pointer selection it is the selected value. For a text record, such as a line, it is that text as a JSON string.

Today `Reading::evidence` (`core/records.rs:410`) turns a whole JSON record into `Evidence` text of its compact line. Quoting that evidence would quote a CSV row as a string. So B4 builds each `BatchRecord` through a new `Reading::batch_record(&Record)` in `core/records.rs`. `BatchRecord` lives in `core/batch.rs`, and `core/records.rs` imports it.

The match in `Reading::evidence` (`core/records.rs` lines 407 to 422) moves into a private `Reading::selected(&Record) -> Result<Selected, RecordError>`. `Selected` holds what the match found: the whole text, the whole JSON value, or the selection. `evidence` converts it to `Evidence` exactly as it does today. `batch_record` calls `selected` once and builds both the evidence and the value from it. The existing tests in `core/records/tests.rs` guard `evidence` through the move.

**Stream.** `Batcher::push(BatchRecord)` returns the batches that closed, zero, one or two. `Batcher::finish()` closes the open batch at the end of input. B4 adds the pause as one more close. Records join the open batch in input order.

**Close rules,** in ADR 0048 item 2's order:

1. Content cut, after the record. The content hash is the SHA-256 of the compact JSON of the record's `value`, the bytes its question quotes. Its first 8 bytes, read as a big-endian unsigned integer, give the value taken mod 4,096. A cut holds when that value is 0. A context changes nothing.
2. Size, after the record. The setting is `Records(N)` and the batch holds `N` records.
3. Limit, before the record. Adding the record would put the request over `max_questions`, `max_evidence_bytes` or `max_request_bytes`. Without a profile `max_request_bytes`, `Backend::ceiling` stands in for it. The open batch closes, and the record starts the next batch.
4. End, from `finish`.

A record that is both a cut and the Nth record closes as `content`. A push can close two batches: the open one at the limit, then the new one when its record is a cut.

**The batch shape,** from ADR 0048 item 1:

- Each distinct record gets one question. Its text is `The text is `, the compact JSON of the record's `value`, `. `, then the user's question text unchanged. `--true`, `--false`, options, levels and labels stay as today. A `tag` record expands to one wire question per label, as today.
- Without a context the evidence is the object `{"records":[R1,…,RN]}`, each distinct record once, in first-seen order. With a context the evidence is the context, and records appear only in their questions.
- Without a context, a batch whose records are all one distinct record sends today's request of that record. It carries the record as evidence and the question unquoted. So a batch of one is today's request, and `--batch 1` keeps every recording, cache entry and fixture valid.
- With a context, a batch of one sends the context and one quoted question.
- A question whose text is a JSON object or list cannot take the quote prefix. Without a context every batch then holds one record in today's form, closed `Size`. Copies do not share there. Each goes alone, as today. With a context, `Batcher::new` refuses it, as the next paragraph says.

**Duplicates.** Two records are equal when the compact JSON bytes of their `value` are equal. A record equal to one already in the open batch adds no question, no evidence and no bytes. It maps to the first copy's question. It still counts as a record toward `N`, because `--batch N` counts records a request.

**Two sizes per record, without quadratic work.** A batch of `k` records must not be encoded `k` times. Each record keeps two sizes, each from one encode of that record alone:

- Its single form: today's body length, evidence bytes and wire questions. Without a context, these apply while the open batch holds one distinct record, alone or with its copies.
- Its batched share: its bytes in the evidence object and in its quoted question, plus its separator and the extra digits its wire names need. The batcher keeps running totals of these. The limit test uses them once a second distinct record would join.

A context batch has one form, the quoted form, even for one record. It is sized by the batched totals from its first record. The single form never applies to it.

When a batch closes, the batcher encodes the form the batch actually sends, once. Without a context, a batch of one distinct record sends the single form. Any other batch, and every context batch, sends the batched form. The batcher compares the encoded lengths with the count for that form. A mismatch is a defect, `BatchError::Defect`, which a test would catch. Planning is linear in the input bytes.

**Profile check.** The batcher calls `BackendProfile::check` directly on each encoded body, with the compact evidence text, as `engine/prepared_request.rs` does today. `prepared_request.rs` changes only by the renamed ceiling call.

**Profile failures.** Without a context, a record whose single form fails the profile gets today's refusal, `BatchError::Profile(ProfileLimit)`. That covers evidence over `max_evidence_bytes`, a `tag` over `max_questions`, and a choice over `max_options`. The ceiling never refuses a batch of one, so a record whose own request passes 96,000 bytes goes alone as today's request.

**Context refusals.** `Batcher::new` checks the context alone, before any record. It refuses with `BatchError::StructuredQuestionWithContext` when the question text is a JSON object or list. It refuses with `BatchError::ContextOverLimit { limit, actual }` when the context passes `max_evidence_bytes`, or when the request of the context and the question with no record passes `max_request_bytes` or the ceiling. Neither variant holds text. B7 turns both into exit 2.

A later record whose own batch of one with the context passes a limit makes `push` return `BatchError::ContextOverLimit { limit, actual }`, with no text. B3 builds that refusal. B7 only chooses the policy, deferred gap 5.

**The batch.** Each closed batch holds its `Plan`, its body, its digest from `Exchange::new(url, body).digest()`, the index of each record's first wire question in input order, and its close reason: `Content`, `Size`, `Limit` or `End`. B4 adds `Pause`. B4 maps a reply's answers back to records through the index. `Batch` and `BatchError` have hand-written `Debug` that prints counts, the digest and the reason, and never record, context or body bytes.

**The ceiling.** `Backend::relation_ceiling` becomes `Backend::ceiling`, with its doc saying batches use it too. Its one caller in `engine/prepared_request.rs` follows the rename. Its value and rule do not change.

**Not yet used.** The module carries one `#[cfg_attr(not(test), expect(dead_code, reason = "ticket B4 puts batches on the command"))]`. B4 removes it.

**No page changes.** B3 changes no surface, so no rule on a specification page becomes true for a user. Every "Not built yet, by ADR 0048 item N" marker stays. B4 removes the markers for items 1 and 2 when the command batches. B3 adds only fixtures under `specification/fixtures/`.

## Decisions

Each is the agent's decision. Ian can overturn any of them.

1. **The planner lives in the pure core.** Encoding, the profile check and the digest are already core. Placing the planner there keeps it pure and table-testable, and it touches no engine file that 0141 or 0143 owns.
2. **The evidence lists each distinct record once.** ADR 0048 asks equal evidence once. Listing a copy twice would add bytes and give the model a second copy to weigh.
3. **Copies count toward `N`.** `--batch N` reads as records a request. A batch can then hold fewer than `N` questions.
4. **A batch of copies of one record sends today's request.** The request depends only on its distinct content, so a repeated line replays from an old recording.
5. **A structured question with a context is refused in `Batcher::new`.** The quote prefix needs a text question, and a context request has no other place for the record. Checking the context before any record means no request is planned before the refusal.
6. **One ceiling function, renamed.** The rule and value stay. The name stops saying it serves relations only.
7. **Markers stay until B4.** A marker comes off when the page's rule is true for a user.
8. **Two sizes per record, with one exact check at close against the form sent.** This keeps planning linear. The check keeps the counts honest.
9. **The batch record carries its own JSON value.** Whole JSONL, CSV and TSV records quote as objects, as the design asks, while the batch of one still sends today's evidence text.

## Edge cases

| Input | Expected |
| --- | --- |
| No records | No batch |
| One record, `Max` or `Records(1)` | One batch, today's bytes, closed `End` or `Size` |
| Three records, `Records(1)` | Three batches, each today's request of its record, each closed `Size` |
| Three plain records, `Max`, no profile, another address | One batch, evidence `{"records":[…]}`, questions `q1` to `q3` quoted, closed `End` |
| A record with a quote mark and a newline | JSON escapes inside the quote, and again inside the instructions string |
| Whole CSV, TSV or JSONL records | Each quoted and listed as its compact JSON object, keys in input order. The batch of one still sends today's text |
| A pointer selecting a string or a number | Quoted as that JSON string or number |
| Two equal records in one batch | One question and one evidence entry. Both records map to question 1 |
| A batch of three copies of one record | Today's request of that record |
| `choose` over three records | Each instructions quoted, options unchanged |
| `tag` with 3 labels, profile `max_questions` 8 | 2 records a batch, 6 wire questions, closed `Limit` |
| `decide` with `--true` and `--false` | Each question carries the same `criteria` |
| A context, three records | Evidence is the context string. Three quoted questions |
| A context, `Records(1)` | Each batch holds the context and one quoted question |
| A question whose text is a JSON object, no context | Every batch holds one record in today's form, closed `Size`. Copies go alone |
| The same, with a context | `BatchError::StructuredQuestionWithContext` from `Batcher::new` |
| Two records whose `value` differs only in key order | Not equal. Two questions |
| A record on a content cut | Its batch closes after it, `Content` |
| A cut that is also the Nth record | Closed `Content` |
| Profile `max_request_bytes` equal to the byte length of `batch-three.request.json` without a trailing newline, over its three records | The three share one batch |
| The same limit less one byte | The batch closes at two, `Limit` |
| Profile `max_evidence_bytes` equal to the compact length of that fixture's `state` member, without a trailing newline, then less one | The same two outcomes |
| Three copies of the `decide-urgent` record, profile `max_request_bytes` equal to that fixture's compact length without a trailing newline. That limit is at or above today's body and below the batched form | One batch, sent in today's form, closed `End` |
| A context, and a later record whose batch of one with the context passes a limit | `push` returns `BatchError::ContextOverLimit`, naming the limit and size only |
| Built-in address, no profile, records of 20,000 bytes | Two a batch, closed `Limit` |
| Built-in address, records of 40,000 bytes | One a batch, today's request. Each record's bytes appear twice, in the evidence and in its quote, so two pass 96,000 |
| Built-in address, one record of 100,000 bytes | Sent alone as today's request. The ceiling never refuses a batch of one |
| Built-in address, profile with `max_request_bytes` | The profile's limit replaces the ceiling |
| Another address, no profile | No byte limit. Batches close at a cut, `N` or the end |
| One record over `max_evidence_bytes` alone | `BatchError::Profile`, today's refusal |
| A choice over `max_options` | `BatchError::Profile`, today's refusal |
| A context over `max_evidence_bytes`, or whose request with no record passes the ceiling | `BatchError::ContextOverLimit` from `Batcher::new`, naming the limit and size only |
| The same records and settings twice | The same batches, bodies and digests |
| A line inserted before the first cut | Its batch and the later batches of its stretch change. Batches after the next cut keep their digests |
| An inserted line that is itself a cut | Its stretch splits. Batches re-form up to the next cut. Earlier batches and those after the next cut keep their digests |
| `Debug` of a batch or an error | No record, context or body bytes |

## Proof

All three tests are table-driven unit tests beside the pure planner in `core/batch/tests.rs`, as `sdlc/planning/rust-standards.md` asks for core code. B3 changes no surface, so the planner's API is the boundary it builds. B4 adds the command tests, 4, 5, 7 and 8. Every expected value comes from a fixture file or a hand-worked README, never from the code under test.

| Test | Proof | Planted faults that turn it red |
| --- | --- | --- |
| `a_batch_of_one_is_todays_request`, design test 1 | For each of the four request fixtures under `specification/fixtures/systemone/`, the record and question go through the planner at `Records(1)` and at `Max` as a stream of one record. Each run gives one batch. Its body equals the fixture's compact bytes, read through the order-keeping `Json` tree and written compact. Its digest equals `Exchange::new(url, compact_bytes).digest()` over those same compact bytes. `find-two` counts too, since its body is byte-pinned and holds escapes | (a) Drop the batch-of-one branch, so one record goes in the batched form. (b) Quote the question at a batch of one. (c) Digest the plan rather than the body |
| `each_batch_body_matches_its_fixture`, design test 2 | Five new byte-pinned fixtures, one compact line each: `batch-three` (three plain records, one holding a quote mark and a newline), `batch-context`, `batch-choose`, `batch-csv` (whole CSV records built through `Reading::batch_record`, so they quote as objects) and `batch-duplicate`. The builder writes each body by hand from ADR 0048 item 1 and checks it with `jq -c`. The README gives one worked question | (a) Drop the `. ` after the quote. (b) Write the quote with the record's text instead of its JSON. (f) Quote a whole CSV record from its `evidence` text instead of its `value`. (c) Send the records as a plain list. (d) Ask a duplicate twice. (e) Put a context batch's records in the evidence too |
| `batches_close_where_the_readme_says`, design test 3 | `specification/fixtures/batching/grouping.txt` holds 25 lines. Two of them are content cuts. `specification/fixtures/batching/README.md` lists each line's first 16 hex digits from `printf '"%s"' LINE \| sha256sum`. A cut is a line whose 14th to 16th hex digits read `000`. The `printf` rule holds only for lines that need no JSON escapes, so every fixture line is plain ASCII without quote marks or backslashes. The README works out every batch by hand, and the test holds the ranges and close reasons as literals: at `Max` under a profile with `max_questions` 8, and at `Records(5)`. It names a third cut line for inserting. Rows insert a plain line and then the cut line, and assert which batch digests change and which stay. More rows cover the exact-limit rows, which take their limits from `batch-three.request.json`'s byte length and its `state` member's compact length, and the 20,000-byte, 40,000-byte and 100,000-byte cases from the edge table | (a) Read the 8 bytes little-endian. (b) Hash the record's text without its JSON quotes. (c) Close a cut batch before its record. (d) Compare a limit with `<` where `<=` belongs. (e) Ignore the ceiling. (f) Let batches fill across a cut. (g) Check a batch of one distinct record with copies against its batched size: the three-copies row wrongly refuses with `BatchError::Profile` |

The edge-case table's other rows sit in the same three tests as rows. The `Debug` row asserts that no record text appears in a batch's or an error's `Debug`, over the escape fixture's records.

The four questions:

- **Test 1.** It protects `--batch 1` and every stream of one: every recording, cache entry and fixture stays valid. A planner that quotes or wraps a lone record fails it. No existing test encodes a record through a batch planner, because none exists. It needs no test-only hook.
- **Test 2.** It protects the wire shape ADR 0048 item 1 fixes, which later tickets, the libraries and recordings depend on. A changed prefix, lost escape, plain-list evidence or repeated duplicate fails it. The existing fixture test covers one-record plans only. No hook.
- **Test 3.** It protects where batches close, which decides the digests every cache and replay reads. A wrong byte order, a hash over the wrong bytes, an off-by-one limit or a missed ceiling fails it. Nothing tests grouping today. No hook: the test gives the planner records, a profile and a backend, as B4 will.

## Budgets

Nonblank lines, measured with `grep -c .`.

- `crates/thinkthen/src/core/batch.rs`: at most 240.
- `crates/thinkthen/src/core/batch/tests.rs`: at most 340.
- `crates/thinkthen/src/core/mod.rs`: at most 3 net added.
- `crates/thinkthen/src/core/backend.rs`: at most 2 net added, for the rename and its doc.
- `crates/thinkthen/src/core/records.rs`: at most 20 net added, for `Reading::selected`, `Selected` and `Reading::batch_record`, with `evidence` rebuilt on `selected`. The file stays under 500.
- `crates/thinkthen/src/engine/prepared_request.rs`: the one renamed call. No net lines.
- `specification/fixtures/systemone/`: five one-line fixtures, and at most 15 lines added to its README.
- `specification/fixtures/batching/`: `grouping.txt` of 25 lines and a README of at most 90 nonblank lines.
- `sdlc/ratchet.json` moves to the measured total, at most 605 above today. That is 240, 340, 3 and 2 for the first four Rust budgets, plus 20 for the `core/records.rs` helper. The commit says what grew.
- No dependency. `sha2` is already a core dependency. No public library type, method or message changes, so the `surfaces` rung is not required.

## Stop rules

1. Stop before crossing a budget or adding a dependency.
2. Stop if a batch of one differs from any request fixture by one byte.
3. Stop if the running byte count disagrees with an encoded body in any row.
4. Stop if a batch of one distinct record without a context, alone or with copies, is checked or refused by its batched size rather than today's size.
5. Stop if the planner needs to encode a whole open batch for each record it adds.
6. Stop if any plant stays green.
7. Stop if the change needs a file another in-flight ticket owns: `engine/usage.rs` and `cli/mod.rs` (0141), `engine/facade.rs`, `engine/facade/relate.rs`, `engine/workers.rs` and `cli/relate` (0143), or the release scripts and surface checks (0128).
8. Stop if the design needs a surface change or a changed rule on a specification page.
9. Stop if the build needs a live call. None is authorized here.

## Scope and exclusions

Excluded: the `--batch` option, `THINKTHEN_BATCH`, the question file's `batch`, precedence, and the parser for `max` (B4). The pause. Jobs over batches, order, failure lines, reply splitting, record, replay and cache on the command (B4). Shares, `meta.batch` and `--facts` (B5). `--context` and its message (B7). Calibration identity (B16). Library and SQL batching (B12a to B13e). `site/`.

## Routing

Builder: Claude (Opus subagent) in lane 3. Reviewer: a fresh read-only Claude session for design and for code. The change raises the ceiling, so the code review names what it checked.

## Complexity

Contract 2; state and timing 0; reach 1; proof 1; cost of error 2; total 6. Final level: 2. The risks are a batch of one that drifts from today's bytes, which voids every recording, and a wrong cut or limit, which moves every digest. Tests 1 and 3 guard each.

## Deferred gaps

1. Cross-language hash identity. The content hash reads serde_json's compact spelling: raw UTF-8, escapes only for quote marks, backslashes and control characters, keys in input order, and numbers as the `Json` tree holds them. B12a's shared conformance cases pin it for the other libraries.
2. The design's test 3 says records of 40,000 bytes close batches at the ceiling. They do, but each goes alone, because each record's bytes appear twice. The ticket adds 20,000-byte records to show two sharing a batch. D1's page should say a batch carries each record twice.
3. Demo recordings under B4's default. The demos under `demos/` were recorded one record a request. Under the default `max`, a demo over several records misses its recording. B4 must pass `--batch 1` in those demos or record them again.
4. Reply splitting per record, including duplicates, belongs to B4 with the failure rule of ADR 0048 item 6.
5. The policy for a late overflow with a context. `Batcher::new` checks the context with no record. A record whose own batch of one with the context passes a limit is found only when it arrives, after earlier batches may have gone. B3's `push` then returns `BatchError::ContextOverLimit { limit, actual }`. ADR 0048 item 11 asks for exit 2 before any request. B7 chooses the policy: check the first record before any request, or fail at that record.

## What Ian can overturn

- Decision 2: each distinct record appears once in the evidence.
- Decision 3: copies count toward `N`.
- Decision 4: a batch of copies of one record sends today's request.
- Decision 5: a structured question with a context is refused in `Batcher::new`.
- Decision 7: the markers stay until B4.
- The coordinator's order: B3 builds alongside 0141 and 0143 and lands after both.

## Closes

No issue. `sdlc/issues/2026-09-26-batching-design.md` stays open until its last ticket lands.

## Evidence

- Starts from: The B3 row and sections 1, 2, 5 to 7, the edge cases and acceptance tests 1 to 3 of `sdlc/issues/2026-09-26-batching-design.md`. ADR 0048 items 1 and 2 and its ticket table. `sdlc/records/2026-09-26-batching-and-recognize-evidence.md` section 9, which simulated content cuts over the 306 titles and found none at 4,096, and section 5, the wire probe's silent answer to plain-list evidence. The code at `origin/main` `a14d959e`: `core/adapters/systemone/request.rs`, `core/backend_profile.rs`, `core/backend.rs`, `core/recording.rs`, `core/records.rs` and `engine/prepared_request.rs`. A search over `line N` strings found 11 content cuts below 40,000, so a 25-line fixture with named cuts is easy to build.
- Keeps: Every request today's code sends. The request fixtures, recordings and cache entries. The 96,000-byte ceiling's value and rule for relations. The profile's refusals and their messages. Every surface, page and marker.
- Changes: A pure `Batcher` in `core/batch.rs` plans batches from records by the close rules and batch shape of ADR 0048 items 1 and 2, with exact sizes, profile limits, the ceiling, the quote prefix, the evidence object, duplicates, the batch of one and digests. `Backend::relation_ceiling` becomes `Backend::ceiling`. `Reading::batch_record` gives each record its JSON value. Five batch fixtures and a grouping fixture join `specification/fixtures/`.
- Proof: Design tests 1, 2 and 3 as the three table tests under "Proof", each with its plants.
- Defers: The command, the setting's parser and precedence, the pause, reply splitting and failure (B4). Shares and facts (B5). The context option (B7). Cross-language hash identity (B12a). The late context overflow (B7). The demo recordings under the default (B4).
