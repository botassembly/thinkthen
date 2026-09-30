# Docs, how-tos, and spec claims owed

Status: Open for its remaining criteria. Quick Fix qf-h1-h3-h6 settles claims 5 and 6 and page 19. Reviewed reconciliation `b23f1a18` confirms accepted ticket 0241 supplies pages 6–10. Remaining pages 11–18 retain their later, held or measurement-dependent criteria.

This issue merges the open documentation work from twelve older issues: `2026-09-25-docs-how-tos-and-spec-claims-owed.md`, `2026-09-25-docs-how-tos-and-spec-claims-owed.md`, `2026-09-25-docs-how-tos-and-spec-claims-owed.md`, `2026-09-25-docs-how-tos-and-spec-claims-owed.md`, `2026-09-25-docs-how-tos-and-spec-claims-owed.md`, `2026-09-25-docs-how-tos-and-spec-claims-owed.md`, `2026-09-25-docs-how-tos-and-spec-claims-owed.md`, `2026-09-25-docs-how-tos-and-spec-claims-owed.md`, `2026-09-25-docs-how-tos-and-spec-claims-owed.md`, the how-to and verb-hint parts of `2026-09-25-release-and-install-for-0-1.md`, item 8 of the closed `closed/2026-09-21-where-a-user-could-lose-trust-a-first-list.md`, and the one remaining row of `2026-09-25-docs-how-tos-and-spec-claims-owed.md`. They belong together because each asks for words a user reads: a spec sentence, a help line, a message, or a page. Each item was checked against `demos/`, `specification/`, `README.md`, `site/`, and the planning pages on 2026-09-25 at main `44de5c8b`. Work that already landed is listed at the end. ADR 0018 fixes the `demos/` list, so a new page either amends that list or joins the site how-tos.

## Spec or doc claims that are wrong

Fix these first. A reader who trusts them is misled today.

### 1. The spec and ADR 0010 say a repeated request returns the same answer

Fixed by ticket 0126, landed 2026-09-25 from branch `ticket/0126-wording-help-and-doc-claims`.

### 2. The interface audit says three sent fields never reach the wire

Fixed by ticket 0126, landed 2026-09-25 from branch `ticket/0126-wording-help-and-doc-claims`.

### 3. Two printed numbers name no measuring record

Fixed by ticket 0126, landed 2026-09-25 from branch `ticket/0126-wording-help-and-doc-claims`.

### 4. `find --details` keys probabilities by ids the spec never explains

Fixed by ticket 0125, landed 2026-09-25 from branch `ticket/0125-audit-complete`.

### 5. Three more spec numbers name no measuring record

Fixed by Quick Fix qf-h1-h3-h6, 2026-09-26. `annotate.md` names the closed live-probe issue for 20.8 and `probes/annotate-0015/mixed-summary.json` for 915 and 371. `threshold.md` names section 5 of `sdlc/planning/design-study.md`. Demo 28 says the block above it measured 294.6 tokens a case. The word "unresolved" on `threshold.md` stays with item 16 of the wording issue.

### 6. recognize says an empty text dry run prints zero requests

Fixed by Quick Fix qf-h1-h3-h6, 2026-09-26. The page now says an empty or blank text exits 2 as a live run does. `spec/recognize.md` pins both texts under `--dry-run`.

## Pages owed for 0.1

### 5. A guessed verb, a CSV file, or a second path gets no hint

Fixed by ticket 0153 on 2026-09-27, code `3e8ad635`. The exact command hints, JSONL advice and second-argument refusal pass compiled-CLI tests. The `choose` help points to `tag`.

**Gap.** The headline sells semantic `if`, `grep`, and `sort`. Checked against the debug build on 2026-09-25: `thinkthen grep` and `thinkthen classify` print clap's `unrecognized subcommand` and the usage line. A CSV file piped into `filter --jsonl` prints `thinkthen: the record is not valid JSON` and exits 2, with no mention of `--csv`. `thinkthen decide 'Q?' README.md` prints `unexpected argument 'README.md' found`. Only `annotate` has a second-path hint (`crates/thinkthen/src/cli/annotate.rs:139`). `thinkthen choose --help` never points to `tag` for many labels. Rows 3, 5, and 6 of `2026-09-20-new-user-stumble-register.md` track the same three stumbles.

**Fix.** Add a hint table for unknown verbs: `grep` names `filter`, `if` names `decide`, `classify` and `switch` name `choose`, `sort` names `rank`, and `summarize` and `rewrite` say the tool writes no text. Add a hint to the JSONL parse failure on the first record that names `--csv` and `--lines`. Give every function the second-path hint `annotate` has. Add one line to `choose --help` that names `tag` for zero or more labels. Close rows 3, 5, and 6 of the register with the commit.

Done when: each of the four commands above prints a hint that names the right option or function, and a test holds each hint.

### 6. The refusals page names no neighbor tool and never says text only

Met by accepted 0241 (`db7e2418`) and the current `site/src/pages/refusals.astro`: text-only input and neighbors for writing, redaction, clustering, sampling and extraction/graphs. Reviewed reconciliation: [first-hour criteria](../records/2026-09-29-first-hour-criterion-reconciliation.md).

### 7. A tool-call guard for a coding agent

Met by [ticket 0241's site guard](../../site/examples/how-tos/bash/agent-tool-guard/1-guard.sh), accepted at `db7e2418`. It maps the three replayed outcomes from demo 19 to Claude Code `PreToolUse` `allow`, `ask`, and `deny`, and denies judge failures. The [0241 build record](../records/0241-site-and-sample-build.md) says the host mapping was checked against the official hooks reference on 2026-09-28. The issue allows one page; a second green demo is not required.

### 8. A long-lived loop from one process

Met by accepted 0241 (`db7e2418`). The `long-lived-loop` recipe holds one `choose` coprocess, uses `--batch 1`, feeds three steps and reads each answer before the next. Its replayed example and saved output are covered by the [0241 build record](../records/0241-site-and-sample-build.md).

### 9. Text split into paragraphs before a function reads it

Met by accepted 0241 (`db7e2418`). The `judge-paragraphs` recipe uses `awk -v RS=`, `jq -Rc` and `filter --field /text`; the catalog names sentence and code splitters. The [0241 build record](../records/0241-site-and-sample-build.md) records its replay. No new splitting flag is owed by this criterion.

### 10. Skipping a bad record has no decision

The decision half was fixed by Quick Fix qf-h1-h3-h6 on 2026-09-26. The roadmap keeps `--on-error continue` on hold, dated, and says Ian can overturn it. [Ticket 0241's site recipe](../../site/examples/how-tos/bash/set-aside-bad-records/1-split.sh) already sets a malformed line and a non-text body aside, then judges three valid records. Its recorded output shows both aside lines and the judged records. Ticket 0241 passed independent review at `db7e2418`; the original criterion expressly allowed a site recipe. This page criterion is met.

## Pages that can follow 0.1

### 11. A first cut before a two-list job over 255 candidates

**Gap.** `find` takes 2 to 255 units (`demos/15-find-the-line/README.md:68`), and `relate` takes at most 255 entities (`specification/relate.md`). Normalize, join, and dedupe against a longer list need a cheap first cut upstream. No page shows one.

**Fix.** Write a recipe that narrows a long reference list with `grep`, a database query, or another index, then runs `find` or `relate` on the survivors.

Done when: one page shows a first cut feeding a job that would otherwise pass 255.

### 12. Grep a directory by meaning and print `file:line`

**Gap.** A shell user expects `grep -rn`. Today it takes `jq -Rc '{file: input_filename, n: input_line_number, text: .}'`, then `filter --jsonl --field /text`, then a `jq -r` line. No page shows `input_filename`. The coverage pass held a line-number flag until a page shows whether people copy the recipe.

**Fix.** Write the recipe and print `file:line: text`.

Done when: one page greps a folder by meaning and prints file and line.

### 13. Diagnose a failed agent trace

**Gap.** The 2026-09-20 survey found this the one software-delivery use case in Ian's notes with no page. No demo or site how-to mentions a trace.

**Fix.** Write a how-to where `find --none` picks the step where the run went wrong and `choose` names the kind of error.

Done when: a green page names the failing step of a recorded trace.

### 14. Ask a question about the answers

**Gap.** Candidate 3 of the 2026-09-21 how-to list: `annotate` fills a form per ticket, `jq` groups answers by customer or minute into one text, and a second `decide` asks whether the group describes one outage. No page shows a second-level question.

**Fix.** Write the three-command how-to.

Done when: a green page feeds grouped answers into a second question.

### 15. Watch a live log, and state the streaming promise

**Gap.** The 2026-09-21 probe showed `filter --lines --replay` prints a kept line while the pipe stays open: a line sent at 1.0 s printed at 1.00 s. `specification/records.md` says output keeps input order and `jobs` holds finished rows until earlier rows print (lines 81 and 133). It never says output does not wait for the end of input. A live backend was not measured.

**Fix.** Add one sentence to `records.md`: each record prints as soon as its answer and every earlier record's answer are ready, except under `rank` and `find`. Then write the `tail -f app.log | thinkthen filter ... --lines` how-to, with one line that ThinkThen holds no state, no windows, and no delivery promise.

Done when: `records.md` states the promise and a page shows a live-log filter.

### 16. Map a week of incident reports

**Gap.** Candidate 5 of the 2026-09-21 list uses `filter`, `recognize`, `relate`, and `score`, then plain SQL. Both functions it waited on are now green in `demos/44-recognize-names/` and `demos/45-map-relationships/`. No page combines them.

**Fix.** Build a made-up set of about twenty reports with a known answer. Print the `--dry-run` request count for the recognize and relate steps on the first screen. Show the bar, what it dropped, and the middle sent to a person, the way the relate demo in `experiments/225-relate-demo/` needs.

Done when: a green page runs the four-function flow under `--replay` and ends in SQL.

### 17. A library examples folder with River Run as the first Python example

**Gap.** Ian ruled on 2026-09-23: "we probably should start pulling together an examples folder in ThinkThen, and this can be one of the Python examples." The issue waited on the library surfaces reaching main. Python landed on main on 2026-09-25 (`a95474be`). No `examples/` folder exists. No file in the repo mentions River Run.

**Fix.** Open a ticket. Add `examples/` with a README that says an example is a program a developer reads end to end, and a how-to is one shell job. Keep `sdlc/scripts/pages` green by not counting `examples/`. Port `game.py` and its question files from `experiments/250-thinkthen-plays-a-card-game`, hold one engine per run, and run under replay with no key. Report requests against the experiment's recorded runs: 32 one decision per request against 13 one per screen, plus the warm-connection number.

Done when: the River Run example runs under replay from a clean checkout and its README states the request counts.

### 18. No profile ships a measured limit, and no case tests a backend's own edge

**Gap.** Ian said on 2026-09-21 that other models will be supported "as long as they support the system one API, or we can adapt to whatever their API is". ADR 0032 gives `--profile FILE` byte and question limits. `profiles/README.md` says the folder "carries no claimed backend limits yet". `conformance/backend-profiles.json` tests the local profile parser at synthetic byte edges. No case records a backend's own reply just under and just over its stated limit. Experiment 219 bracketed the first backend at 31,826 tokens answered and about 33,150 refused for one text, and 61,819 answered and about 66,000 refused for a whole request. The refusal carries no size and no limit. The tool counts no tokens, so a token bracket is not yet a byte ceiling.

**Fix.** Measure a byte ceiling for the first backend from the experiment 219 bracket, and ship one profile that states it with its record. Add conformance cases at a second backend's stated edges when one is chosen. Each adapter page states its limits, packing, question kinds, and price, each with the check that measured it.

Done when: `profiles/` holds one measured profile that names its record, and the conformance file holds one backend-edge case.

### 19. Two held flags need a written answer

Fixed by Quick Fix qf-h1-h3-h6, 2026-09-26. The `--invert` row answers the survey: a reworded question is a different measurement, so the hold stands and `decide --details` with `jq` keeps the other side of the same question. The third example in `specification/filter.md` shows that detour. ADR 0048 item 11 settles where reference text goes: `--context FILE`, which the batching tickets build.

## Already done

- **Split a file into piles by a `choose` label.** `demos/02-route-a-ticket/` step 2 moves each note into a folder by label.
- **Many labels read back as a list.** `tag` landed. `demos/39-screen-a-message/` prints `["urgent"]`.
- **Several questions in one request.** `annotate` packs one group per evidence. `specification/annotate.md:106` records 20.8 times fewer billed tokens with no answer changed.
- **Evidence as a structured object and the models listing.** Record 0017 settled the string. `specification/roadmap.md:42` holds the listing with its trigger.
- **CSV input.** `--csv` and `--tsv` landed. `specification/records.md:31-39` defines them.
- **A size and cost page.** `site/src/pages/trust.astro` "How big the evidence can be" and "What it costs", `site/src/pages/reference.astro:177`, and the `FACTS` table in `site/src/data/catalog.mjs`, where each number names its record.
- **A comparison floor and the fair-pair rule.** The `compare` transform takes a 0.08 tolerance. `demos/41-tune-a-question-file/README.md:104` says never to pair a live run with a replay.
- **`--dry-run` checks `--jobs`.** The binary refuses `decide --jobs 1 --dry-run` on one text at exit 2, the same as a live run.
- **Join by meaning, group alerts into incidents, and split where the topic changes.** The site how-tos `join-two-tables-by-meaning`, `group-alerts-into-incidents`, and `split-a-scanned-packet-into-documents` in `site/src/data/examples/_howtos.json`.
- **Fill a form by selection, a prose linter, and a semantic diff.** The site recipes `fill-a-form-by-selection`, `lint-prose-for-hedging`, and `review-a-diff-by-what-it-does` in `site/src/data/examples/_recipes.json`.
- **Judge pairs inside one set.** `relate` judges every pair in a complete entity set, and `demos/45-map-relationships/` is green.
- **Filter database rows in plain English.** The database surfaces put `thinkthen_decide` in a `WHERE` clause. The site's `filter__postgresql` example shows it.
- **Say what happens to a bad record today.** `specification/records.md:107` and `demos/12-keep-going/`.

## Handed to marketing

- The vocabulary ruling leaves one break: "buckets" beside band talk. It sits in the marketing repository at `products/thinkthen/deck.md:78` and `products/thinkthen/objections.md:30`. The fixed words are "band" and "middle range". Marketing owns the fix.
- The cost slide says 3.6 cents and names no record. It sits in the marketing repository at `decks/2026-09-21-thinkthen-semantic-commands/slides/34-cost/slide.html:31`. Marketing owns the citation.

## Marketing notes for 0.1, 2026-09-29

- Ian ruled that 0.1 waits for the SQL and data frame redesign (proposed ADR 0105, workspace experiment 2038). A page in this issue that shows SQL or a data frame call should wait for ADR 0105, so it is written once.
- Page 7 (the tool-call guard) and page 10 (skipping a bad record) are the ones a new user reaches first. Marketing asks that they land before 0.1 over items 11 to 19.
- The site mirrors how-tos under Ian's site rulings: examples of 10 to 25 lines including output, and no `--details`. A how-to that needs `--details` keeps it in `demos/`, and marketing's site copy leaves it out.

## Build-side help gate follow-up, 2026-09-29

The0300 builder's selected demo28 run stopped before executing that page because built `annotate -h` and `annotate --help` contain three occurrences of `row` rejected by the existing help vocabulary rule. Reproduction with candidate `d005fb664` and its CLI artifact `7ce82e71929e`: `sdlc/scripts/demos demos/28-what-a-run-cost` reports `thinkthen annotate -h:9`, `thinkthen annotate --help:3` and `thinkthen annotate --help:13`, then exits1. The changed demo's direct saved replay passes. `cli/args/annotate.rs` already contains the error-row wording on main; pricing does not change it. This limits the whole runner receipt, not the direct replay proof.

Codex's coordinator owns this residual under the existing documentation issue, with a small command-help Quick Fix after the active core review frees the relevant files. Read the actual help sentences and preserve their meaning while reconciling the settled vocabulary rule; do not weaken the scanner to hide the failure. Prove the changed built help and the selected demo, without a full surfaces run. This belongs to the command's build team and needs no site edit. The earlier build-record sentence saying it was already owned elsewhere was unsupported; this entry establishes the owner and remaining criterion explicitly.

The annotate-help Quick Fix candidate replaces only “safe row” and “error row” with “safe error record” and “error record.” Its rebuilt `annotate -h` and `--help` retain the flag conditions, exits 6/7 and examples; the only remaining whole-word `row` hits are the allowed CSV/TSV `header row`; the unchanged built-help scanner passes when run with a page-free staging root. The selected demo runner now reaches an independent page-rule failure: demo 28 is 129 lines and 940 words, above ADR 0016's 120-line/900-word limits. Direct `mustmatch` on that page passes six saved-replay assertions. A page-folder argument to `sdlc/scripts/demos` by itself reports `0 green, 0 red` and is not a runner proof; the actual one-page staging root is described in [the build record](../records/qf-annotate-help-records.md). This issue remains open for the unrelated owed pages; the following expanded Quick Fix candidate addresses both selected build-side failures without closing the umbrella.

The coordinator expanded the claim after comparing page history: before 0300 at `1e493eee8`, demo 28 was 120 lines/868 words; its new priced example raised it to 129/940. The corrected page is 119/759, with all six executable proof blocks preserved apart from blank lines. The source-matched CLI `f37203fb17e7` ran the whole one-page `sdlc/scripts/demos` selection from an indexed staging root: `running 28-what-a-run-cost/README.md`, `6 passed`, `1 green, 0 red`. Help wording and page form now pass together in this candidate. Other page criteria in this issue remain open until independently met.
