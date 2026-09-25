# recognize and relate: scale and shape

Status: Open.

This issue merges seven files: `2026-09-25-recognize-and-relate-scale-and-shape.md`, `2026-09-25-recognize-and-relate-scale-and-shape.md`, `2026-09-25-recognize-and-relate-scale-and-shape.md`, `2026-09-25-recognize-and-relate-scale-and-shape.md`, `2026-09-25-recognize-and-relate-scale-and-shape.md`, `2026-09-25-recognize-and-relate-scale-and-shape.md`, and one leftover from the deleted `closed/2026-09-23-relate-call-math-efficiency-and-real-jobs.md`. Each item was checked against main at `71ea0a84`, the two specifications, `sdlc/planning/relate-design.md`, and the tickets. They belong together because they share one engine path. The relation planner and the request splitter serve both commands. Every open item is about how that path grows with the input, or how its output and its fixed wording read to a user. Ian can overturn any fix below.

Items 1 and 2 block 0.1 users or ride with a blocker. Items 3 through 8 are later features or documentation.

## 1. relate sends a request over Jev's input token limit

Blocks 0.1. The backend refuses the request.

What happens today. The Beatles run (`thinkthen relate @graph.json --jsonl --kind-field /kind --field /name`, 184 songs, 13 albums, 4 people, no `--profile`) plans two requests. Request 1 (`sung_by`, 85,458 bytes, 184 questions) answers. Request 2 (`appears_on`, 161,252 bytes, 184 questions, 14 options each) comes back 400 with body `{"detail":{"error_type":"max_tokens_exceeded"}}`. The tool prints only "the backend refused the request; check --model and the request size" (`crates/thinkthen/src/cli/failure/status.rs:6`). Eight live calls on 2026-09-24 found the cap. 142 album questions passed at 65,423 input tokens and 143 failed. The cap fits 65,536 input tokens. Bytes, question count, and option count are not the limit. `PreparedRequests::with_profile` in `crates/thinkthen/src/engine/prepared_request.rs` splits only on a profile limit. With no profile, every question goes in one request. The ticket 0059 example profile for Jev (`max_request_bytes` 250,000, `max_evidence_bytes` 120,000, `max_questions` 64) names no source for its byte numbers. Only its `max_questions` would split request 2.

What the design says. `specification/backends.md` says a profile estimates no tokens, and a backend whose limit is only tokens needs a tokenizer or a verified byte ceiling. `relate-design.md` routes all splitting through the one 0079 splitter. Ticket 0082 item 26 chose a fixed phrase for status 400 and forbids printing the body, because a body can quote the evidence back. The vendor docs still say "around 32,000 tokens". Experiment 260 found packed accuracy falls from 0.92 near 2,000 tokens a request to 0.79 near 27,000 (`2026-09-25-packing-and-batching-what-they-buy-what-they-cost-and-the-setting.md`).

The fix.

- Give the default backend a built-in request ceiling that the splitter enforces with no `--profile`. Base it on a measured byte ceiling under the 65,536-token cap, with margin. The Beatles runs measured about 0.53 input tokens a byte. Record the source in the profile and in `backends.md`. A user profile still overrides it.
- Weigh a lower ceiling for accuracy against experiment 260 before picking the number. Record the choice.
- On status 400, read only `detail.error_type` from the body. Print it when it matches a closed list of known values such as `max_tokens_exceeded`. Print nothing else from the body. This keeps the ticket 0082 secrecy rule.
- Correct the ticket 0059 Jev example to the measured limit and name this issue as its source.

Done when: the Beatles dry run with no profile plans `appears_on` in requests under the measured ceiling, and a replayed 400 with `max_tokens_exceeded` prints that word.

## 2. Planning a full entity set encodes every request prefix

Not a crash or a refusal. Land it with item 1, because both change the same loop, and item 1 makes more chunks.

What happens today. The splitter loop in `crates/thinkthen/src/engine/prepared_request.rs` (`for count in 1..=remaining`) encodes and preflights every prefix length. Planning cost grows with the square of the question count. A same-kind directed rule over n entities asks n(n-1) questions, so planning grows with the fourth power of n. The ticket 0088 final review (`sdlc/records/0088-review-final.md`, F1) measured a release dry run at 0.47 s for 40 entities and 7.65 s for 80. A debug build took 81 s for 80. 255 entities extrapolate to about 13 minutes of CPU before the first send. A regressed 256th-entity refusal would hang its test instead of failing it.

What the design says. `relate-design.md` requires one shared splitter. It sets no time bound.

The fix. Try the whole remainder first and take it when it passes. When it fails on a limit that permits a split, binary-search the largest passing count. Limits only tighten as a chunk grows, so the search is sound.

Done when: a release dry run over 255 line entities finishes in under a second, and a test pins the chunk boundaries the old loop chose.

## 3. A both-ways edge prints a direction it does not have

Later feature.

What happens today. Ticket 0088 made each edge carry both names and kinds. An `--either` edge still prints `source` and `target`, normalized to input order (`specification/relate.md` line 44). The reader cannot tell a both-ways pair from a one-way edge without the rule.

What the ruling says. Ian's words, from his review of the relate examples on 2026-09-22: "relate source and target should be more obvious too. im worried that function isnt obvious or useful for real needs." The 2026-09-22 issue asked that a both-ways rule print a pair with no `source` and `target`, and that a one-way rule keep its direction and may print the sentence the rule implies from `reads`.

The fix. Give `--either` edges an unordered shape, for example `{"relation":"duplicates","pair":[{…},{…}],"probability":…}`. Keep the one-way shape. Update the specification, the Option A details, and every surface's edge type in one change.

Done when: an `--either` edge prints no `source` or `target` key on the command and on every surface.

## 4. relate takes one set, and a grown table re-asks every question

Later feature. The item 3 and item 4 asks of the 2026-09-23 issue merge here, because both need one input marked as new or as the other side.

What happens today. `relate` reads one complete entity set (`specification/relate.md` line 28). Ticket 0088 added the kind column, and cross-kind rules ask one choice per member of the larger side. That covers the cost half of the two-set job: payments and invoices in one file with a kind column no longer pay for same-side pairs. Two more things are missing.

- No form takes two inputs. The job "which invoice does this payment settle" still needs the user to merge the sets and add a kind.
- No form marks new rows. Every relate request state carries the complete entity array (`relate-design.md`, "Exact relation request state"). Adding one song changes the state of every request, so every question misses the cache and bills again. A new album also changes every song's choice options.

What the design says. `relate-design.md` defines only the complete-set form and says nothing about growth. The 2026-09-23 issue called new rows "the most likely thing a database user needs after the first run", and asked the manual to say that a new option bills every old question again.

The fix.

- Say now, in `specification/relate.md` and the database pages, that adding any entity re-bills the whole run.
- Design one form that marks a subset as new, from a second input or a column. It asks only questions that touch a new entity: a choice for each new cross-kind member, and yes/no pairs that hold a new entity within a kind. Build the request state so old questions keep their digests where the method allows. A two-set run is the same form with every row of one side marked new.
- Measure the bill for "ten new songs against 184 old ones" before and after.

Done when: a second run that adds ten entities sends only questions touching those ten, and the specification states what it re-bills.

## 5. Block before pairing when the set is large

Later feature. The 2026-09-23 item 4 (links to a known entity table) merges here.

What happens today. Same-kind yes/no pairs grow with N². At 255 entities an either rule asks 32,385 questions. Experiment 225 counted 5, 14, and 43 false edges at 10, 50, and 200 records. A cross-kind choice past 255 options falls back to yes/no pairs, which is costly at table scale. Nothing narrows candidates first.

What the design says. The deleted call-math issue recommended: "A `choose` or `tag` pass puts records in coarse groups, and pairs are asked only within a group." The 2026-09-23 issue asked for a cheap first cut (the `find` family or a plain text match) when mapping names to a table past the option ceiling. `relate-design.md` has no blocking step.

The fix. Add an optional blocking step to the shared planner. A key column, a text match, or one cheap question per entity puts entities in groups. Pairs and choices are asked only within a group. Dry run reports questions with and without blocking. Recognize keeps its current path, since one sentence holds few names.

Done when: a relate run with a blocking key asks only within-group questions, and dry run shows the saving.

## 6. The 255 refusal does not teach the pair math

Later. It is a local refusal that already works.

What happens today. More than 255 entities exits 2 with "relate takes at most 255 entities" (`crates/thinkthen/src/core/relate_file.rs:88`). Tests pin 255 and 256 in core (`relate_file/tests.rs`). No wave covers 254, 255, and 256 on every surface.

What the review says. Quality review item 8: the refusal names the limit, the pair arithmetic, and the narrowing move, and each surface tests 254, 255, and 256.

The fix. Word the refusal like "relate takes at most 255 entities; this set has N, and a same-kind rule would ask about N×(N-1)/2 pairs; split the set by kind or block it first". Add the three boundary cases to the shared conformance cases.

Done when: the 256 refusal names the count and the pair math on the command and every surface, and conformance pins 254, 255, and 256.

## 7. No page maps a recognize symptom to its dial

Later. Documentation.

What happens today. `specification/recognize.md` documents `--threshold` on the computed `strength`, `--relation-threshold`, and `--kind KIND=DESCRIPTION`. No page tells a user whose names drop out, or whose output is noisy, which dial to turn and what it costs.

What the review says. Quality review item 1 asks for one symptom-to-dial table and one line saying custom kinds are only as good as their descriptions, because the measured numbers cover the three stock kinds. The repository rule says every printed number names its record.

The fix. Add the table to the recognize manual page. Map names dropped, too much noise, and a kind that never appears to the shipped dials. Give only numbers measured on the shipped `strength` formula, each with its record. The review's span-gate numbers predate the 2026-09-23 baseline and do not carry over.

Done when: the recognize manual carries the table and every number in it names a record.

## 8. recognize hides its fixed news-document wording

Later. Documentation now, measurement only if a user needs it.

What happens today. `DETECTION_WORDS` and `KIND_WORDS` in `crates/thinkthen/src/core/recognize.rs` both begin "The snippet shows five consecutive words from a news document". The detection question lists fixed categories (person, organization, place, nationality, event, product, creative work) and says dates and numbers are not names. The kind question offers the caller's own kinds. Neither `specification/recognize.md` nor `recognize --help` mentions this. A user's benchmark on 2026-09-25 read the requests and took the wording for a bug. A kind outside the fixed list may be marked OUT before any kind question sees it. That risk is read from the code and not measured.

What the design says. Ticket 0080 orders the measured lineage-B words byte for byte (CoNLL04 three-class F1 0.7636, experiment 221). Experiment 225's `words/kind.md` says caller kinds "are new words and a new measurement".

The fix. Add a short section to `specification/recognize.md`: the wording is fixed, cites ticket 0080 and the measured lineage, and says kinds outside the listed categories are unmeasured. Add one sentence to the help. A generic wording needs a paid comparison and a ticket, because it changes every recognize request digest.

Done when: the specification and help name the fixed wording and its limits.

## Already fixed

- 2026-09-22 ask 1, edges carry the records. Ticket 0088 prints both endpoint names and kinds on every edge (`specification/relate.md` line 41). Name and kind form a unique identity, since duplicates are refused.
- 2026-09-23 item 1, every surface takes a kind column. Ticket 0088 added `--kind-field` and the `fields.kind` key. The public Rust relate entity is a name and a kind (`crates/thinkthen/src/public/relate.rs`). The surfaces build on it, and the planner lives in the core.
- 2026-09-23 item 5, graph queries stay in the host. This is a standing rule with no work. `relate-design.md` adds no graph function.
- Call-math waste 5, two-set jobs pay for same-side pairs. The ticket 0081 hybrid planner asks cross-kind rules as one choice per member of the larger side. The stand-in engine that refused the kind field is gone.
- Call-math waste 2, nothing splits the request. Ticket 0079's splitter splits on profile limits. The no-profile gap is item 1.
- Quality review items 2 through 7 and 9 through 11 were closed before this merge, as the caller reported. They are not repeated here.
