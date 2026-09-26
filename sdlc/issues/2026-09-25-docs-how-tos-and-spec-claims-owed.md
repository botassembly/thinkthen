# Docs, how-tos, and spec claims owed

Status: Open.

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

Ticket 0125 owns this item.

**Gap.** The core builds each unit id as `format!("u{:03}", place + 1)` (`crates/thinkthen/src/core/find.rs:172-174`). `specification/find.md:41` says only "the selected generated unit id". `specification/result.md:83` shows `u001` and `u002` with no mapping. `thinkthen find --help` says `--details` prints "the full result object". Only ADR 0030 line 10 states the rule. A user on 2026-09-25 had to rebuild the map from input order, and their own ids `u01` to `u10` looked almost the same as `u001` to `u010`.

**Fix.** State the rule in `specification/find.md` and in the `--details` help line: `uNNN` is the one-based input position, zero-padded to three digits. This changes no output. Adding an `index` or a `units` list to the result widens a settled schema and waits for a user who still misjoins after the rule is written. Ian can overturn that order.

Done when: `find.md` and `thinkthen find --help` both state the `uNNN` rule.

## Pages owed for 0.1

### 5. A guessed verb, a CSV file, or a second path gets no hint

**Gap.** The headline sells semantic `if`, `grep`, and `sort`. Checked against the debug build on 2026-09-25: `thinkthen grep` and `thinkthen classify` print clap's `unrecognized subcommand` and the usage line. A CSV file piped into `filter --jsonl` prints `thinkthen: the record is not valid JSON` and exits 2, with no mention of `--csv`. `thinkthen decide 'Q?' README.md` prints `unexpected argument 'README.md' found`. Only `annotate` has a second-path hint (`crates/thinkthen/src/cli/annotate.rs:139`). `thinkthen choose --help` never points to `tag` for many labels. Rows 3, 5, and 6 of `2026-09-20-new-user-stumble-register.md` track the same three stumbles.

**Fix.** Add a hint table for unknown verbs: `grep` names `filter`, `if` names `decide`, `classify` and `switch` name `choose`, `sort` names `rank`, and `summarize` and `rewrite` say the tool writes no text. Add a hint to the JSONL parse failure on the first record that names `--csv` and `--lines`. Give every function the second-path hint `annotate` has. Add one line to `choose --help` that names `tag` for zero or more labels. Close rows 3, 5, and 6 of the register with the commit.

Done when: each of the four commands above prints a hint that names the right option or function, and a test holds each hint.

### 6. The refusals page names no neighbor tool and never says text only

**Gap.** `site/src/pages/refusals.astro` says the tool writes no text. It never names the tool that does each refused job: summarize, rewrite, redact a span, cluster, pick a diverse sample, extract a free-form graph. No page, spec, or README says the tool reads text only. A search for "image" or "screen" in `site/src/pages/`, `README.md`, and `specification/` finds nothing. The computer-use projects in Ian's 2026-09-20 notes read a screen.

**Fix.** Add one short list to the refusals page: each refused job, the kind of tool that does it, and the pipe that hands off to or from ThinkThen. Add one line that the tool sends text and nothing else, so a screen or an image becomes text first.

Done when: the refusals page names a neighbor for each refused job and states text only.

### 7. A tool-call guard for a coding agent

**Gap.** Tool gating is the clearest pattern in the 2026-09-20 article: allow, deny, or ask maps to yes, no, and unresolved. `demos/19-no-or-could-not-ask/` gates a risky command with a `case $?`, and it maps to no host. One coding agent's hook contract blocks a call on exit 2, and this tool uses exit 2 for a usage error. No page shows the mapping.

**Fix.** Write a how-to that reads a proposed tool call, asks `decide` with a band, and maps the tool's exit codes to one host's hook contract in a short `case`. Check that host's current contract when the page is written, and say which version it matches.

Done when: a green page shows the guard and its exit mapping against one named host contract.

### 8. A long-lived loop from one process

**Gap.** `README.md:31` says record mode through a `coproc` serves a steady loop from one long-lived process. Ticket 0024 landed that loop. No page under `demos/` or `site/` shows it. `demos/21-options-from-the-record/` changes the options per step and starts a new process per run.

**Fix.** Write a how-to that holds one `thinkthen choose --jsonl --options POINTER` process open through `coproc`, feeds it one step at a time, and reads each answer before the next step. Name the library as the route when the loop must go faster.

Done when: a green page runs a step loop through one process under `--replay`.

### 9. Text split into paragraphs before a function reads it

**Gap.** The functions read lines, JSON lines, CSV, or TSV. Real text arrives as paragraphs. `demos/43-lint-a-change/` splits a diff with `awk`, and the site's prose-lint recipe reads lines. No page shows `awk -v RS=` or any paragraph split. The 2026-09-20 coverage pass names this the most likely first request for a new flag.

**Fix.** Write a short how-to or recipe that turns paragraphs into JSON lines with `awk -v RS=` and `jq`, then runs one function over them. Name a neighbor tool for sentences and for code functions.

Done when: one green page or site recipe splits a document into paragraphs and judges each one.

### 10. Skipping a bad record has no decision

**Gap.** Item 8 of the closed trust list: one malformed line ends a long run, and a user will ask to skip it, write it to a side file, and go on. `specification/records.md:107` and `demos/12-keep-going/` say plainly what happens today, and `--cache` is the resume. `specification/roadmap.md:28` holds `--on-error continue` until "a demo over a large file where one bad record must not end the run". ADR 0008 item 5 keeps it on the roadmap. No record accepts or rejects the skip, and no page shows how to pre-check a file.

**Fix.** The owner records the decision in the roadmap row. The recommendation: keep the hold, and add one step to `demos/12-keep-going/` or a site recipe that splits bad JSON lines to a side file with `jq -R 'fromjson? // empty'` before the run. Ian can overturn this and ask for `--on-error continue` in 0.1.

Done when: the roadmap row names the decision and its date, and one page shows how to set bad records aside before a run.

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

**Gap.** `specification/roadmap.md:25` holds `filter --invert` and tells users to word the question the other way. The 2026-09-20 survey found that a reworded question is a different measurement with a different threshold. Keeping records below a mark differs from keeping records above the mark of the opposite question. Outlier ranking and "more like these" want reference text beside the question. `--context` is held, and no page says whether `--true` and `--false` are the right home.

**Fix.** Add the survey's argument to the `--invert` row and either keep the hold with a reason or admit the flag. Show the detour on a page: `decide --details` and `jq 'select(.answer.probability < 0.5)'`. Write one page that settles where reference text goes.

Done when: the roadmap row answers the survey's argument, and one page shows where reference text goes.

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
