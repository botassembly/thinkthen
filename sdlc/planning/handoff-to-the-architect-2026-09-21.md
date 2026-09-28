# Handoff to the architect

Written 2026-09-21 by the product side for the architect Ian is standing up. Until today one agent held both the product shape and the marketing. From here the architect drives the build to the 0.1 release and the product side goes back to marketing. This page is the punch list: what is in flight, what is decided, what is left, and where everything lives. Every claim names the file that backs it. Check the file before trusting the sentence.

## The one rule for your first day

Ticket 0055 is done. The move to one package passed independent review after repairing the rejected first pass. The architect can now coordinate the next queue. The library team remains mid-wave on the `surfaces` branch (`worktrees/thinkthen-surfaces`); let that wave finish before merging it.

## What ThinkThen is, in four lines

Ten functions ask a classifier model (Jev, from TypeSafe) a bounded question about a text and return an answer with a probability: `decide`, `choose`, `tag`, `score`, `filter`, `rank`, `find`, `annotate`, `recognize`, `relate`. One Rust engine does all the work. It is reached through a command, six libraries (Rust, Python, TypeScript, Ruby, R, C), Polars and pandas through Python, and three database extensions (DuckDB, SQLite, PostgreSQL). Ian ruled that the first release is 0.1.0 on every one of those at once. The lens for all of it is `notes/ideal-state/thinkthen.md` in the workspace.

## Read in this order

1. `CLAUDE.md` and `README.md` in this repo. The gate ladder, the pure core, and the live-call rules are not optional.
2. `sdlc/planning/build-queue-2026-09-21.md`. The one ordered queue, with the quality findings folded in and Ian's latest rulings at the bottom. **This is your map.**
3. `sdlc/planning/build-team-response-to-handoff-2026-09-21.md` and `go-ahead-for-the-build-team-2026-09-21.md`. The sixteen proposed tickets and their approval.
4. `sdlc/planning/adr/0017` (one engine, many surfaces) and `sdlc/planning/polars-plan.md`.
5. `sdlc/planning/recognize-design.md` and `relate-design.md`. The public shapes of the two special functions.
6. `sdlc/planning/quality-plan.md` and `experiments/218-thinkthen-release-qa/wave1.5/FINDINGS.md`.
7. `sdlc/issues/closed/2026-09-21-triage-of-the-open-issues-by-layer.md`. About ninety issues are open. This is the map of them.

## Where things stand, observed 2026-09-21

- Tickets 0001 to 0055 landed. 0053 put request identity on every result. 0054 keeps good answers when one question fails, with exit 6. 0055 put the private core, engine, and command in one package without changing command behavior.
- Eight of the ten functions exist in the command. `recognize` and `relate` exist only as experiments.
- No library and no extension is shipped. All nine surfaces exist as a rehearsal on the `surfaces` branch over a stand-in engine, with 72 shared conformance cases. Main has its own 27 cases. The merge must make one file.
- Nothing is published anywhere. The package names are not claimed yet. That is Ian's todo, and it blocks every upload.

## The two special functions: what you inherit

The method is final and measured. The recognize team's package is `experiments/225-recognize-harvest-package/` (start at its README, then `rules/rules.md`, then the words files). The master report is `experiments/THINKTHEN-RECOGNIZE-MASTER-REPORT.md`.

- **How it works.** Code splits the text into words. The model answers one small pick-one question per word (is it part of a name) and one for its kind, in the same request. Code joins the yes-words into names. For relations, code lists the legal pairs from the caller's rule table and the model answers one pick-one question per pair, with "no relation" always an option.
- **The exact question wordings are the product.** Seven attempts to improve them failed. A port that rewords them is a new, unmeasured method.
- **No word list, dictionary, or template exists anywhere in the pipeline.** Ian ruled the last one deleted. Read the tombstone in rule four before anyone proposes one.
- **Settled names.** A name carries `strength` (ours, computed: the lowest word probability times the mean kind probability, every word counting). A relation carries `probability` and its two ends are `source` and `target` on every host, in the JSON, and in the question file. The vendor's `confidence` field is never used, because it measured backwards as a gate. The rule for every number word is `sdlc/issues/closed/2026-09-21-one-rule-for-every-number-the-tool-prints.md`.
- **The measured quality, as the team reported it.** Names: about 0.72 on one public set and 0.76 on the other, strict scoring. Kinds given the right boundaries: 0.93 to 0.98. Relations given the right names: 0.73. Relations end to end: 0.47, so `relate` and relations in `recognize` ship marked as a preview. Cost is about 0.06 cents a sentence today and falls about five-fold once shared instructions are packed once per request (queue item A5).
- **Your tests are written.** Forty recorded cases replay with no key and no network. The labeled divergences in them are evidence, not bugs to fix.
- **Limits you inherit.** One model version exists, so pin its explicit name in recordings. The 2026-09-23 probes accepted choices with 101 and 255 options. A 165,154-byte request failed; request size is the likely cause, not a proved diagnosis, so no Jev byte ceiling is named until a measurement supports one. The tool's separate 255-option ceiling remains unchanged. The vendor's probability totals sometimes miss 1.0 by a hair, and the repo's tolerance fix must be verified (A5).
- **Quality questions still open on these two:** `sdlc/issues/2026-09-25-recognize-and-relate-scale-and-shape.md`, and the library team's finding that the relations table function cannot run as drawn on any database.

Backlog beyond them (linking, coreference, decomposition) is recorded in `sdlc/issues/closed/2026-09-21-three-next-language-problems-linking-coreference-decomposition.md`. None of it is in 0.1.

## The other eight functions: known quality debt

Quality wave 1.5 ran every function against every flag that adds information (30 cells). Only `filter` was wrong: `--details` printed the records `filter` should have dropped. Ian ruled that `filter` always filters. The same wave filed nine issues, three of them serious: a corrupt saved entry that bills every retry and never repairs itself, record mode always exiting 0 with no word in the help, and exit 6 told two ways in the specification. Each has a product ruling beside it in the queue, lanes A1 and A4. The wave's standing method stays: a matrix for membership bugs, the pages against each other for contradictions, and a stranger with only the binary and `--help` for everything else.

## The tracks you will coordinate

Roughly twelve can now run side by side where their files and dependencies do not overlap. The queue page has the detail and the order.

| Track | What | Depends on |
| --- | --- | --- |
| 1 | Command-layer fixes (queue A1) | 0055 only where files overlap |
| 2 | Backend profiles, size check, records returned with their answers (A2, A3) | 0055 |
| 3 | The default cache, `status` with spend counts, and the money bugs (A4) | 0055 |
| 4 | Request packing and tolerance (A5), then `recognize` and `relate` in the engine (A6) | Track 3 for cost claims |
| 5 | The public Rust library, then width, cancel, fork (A7, A8) | Tracks 2 to 4 |
| 6 | The C door (A9) | Track 5 |
| 7 | Python with plain lists, then the Polars column form, then pandas | Track 5. The Polars plugin expression is ruled out of 0.1 |
| 8 | TypeScript | Track 5 |
| 9 | Ruby and R | Track 5 |
| 10 | DuckDB, SQLite, PostgreSQL | Track 5. DuckDB and SQLite each have a filed blocker |
| 11 | Release and install: Homebrew tap plus download script copied from BioMCP, CI, packaging for every registry (A10) | Ian claiming the names |
| 12 | Quality waves 2 to 4 and the release pass (A11, A12) | Everything |

Tracks 7 to 10 do not start from nothing. The `surfaces` branch already holds all nine, and lane B of the queue lists eight jobs the library team does while main is busy: adopt main's new shapes early, port `recognize` and `relate` to every surface, prove cancel on a fast backend, and build one examples file per surface. The merge of that branch into main is a build ticket that comes after the public Rust library exists. The rulings that govern its cleanup are in `sdlc/issues/closed/2026-09-21-product-rulings-on-the-surfaces-adversarial-review.md`.

## Experiments worth knowing

All under the workspace's `experiments/` folder. 205 (libraries) and 207 (databases): the first rehearsals and zero-copy proofs. 206: accuracy and calibration tables. 211: the blocking engine and its width numbers. 218: release quality waves. 220: a second, open backend. 225 and 227: the recognize harvest and the name-number comparison. 226: the relation graph demo. 228: Polars, with the verdict to ship the column form first. 229-thinkthen-spreadsheets: Excel and Google Sheets, both paused by Ian. 230: splitting a packet of pages by composing `choose` and `decide`, the proof that a split function is not needed.

## Rulings that are settled. Do not reopen them without Ian.

- Ten functions. No eleventh (`sdlc/issues/closed/2026-09-21-is-there-an-eleventh-function-a-sweep-of-the-three-primitives.md`).
- 0.1.0 everywhere at once. Builds before it are 0.0.N.
- Polars is an optional extra, column form only. pandas rides the same door.
- Spreadsheets, a serve mode, HTTP bridges, and Windows are paused.
- `thinkthen status` shows spend counts. No spending limit in 0.1.
- The command installs by Homebrew and a download script.
- `filter --details` still filters. Record mode exits 0 and the help says so.
- The tool judges and never acts. It runs no command, starts no service, and writes no file the user did not name, apart from the default cache Ian approved.

## Guard rails

- A paid call runs only through `sdlc/scripts/live` under a token cap. Never edit the live ledger by hand. Never print, log, hash, or commit a key.
- Every ticket keeps its independent review. 0055 shows why: green tests, real gaps.
- Publish nothing and claim no name. This repository will go public, so never name a private project or a customer.
- `yellow.local` is the build and gate host, run from exact pushed commits.
- Decide by default and record it in `sdlc/`. Bring Ian only what is hard to reverse, outward-facing, costs money or someone's time, contradicts his ruling, or turns on something only he knows. Never send him a clinical, biological, or configuration question.
- Ian dictates by voice and wants plain words, no jargon, and no time estimates.

## What marketing needs from the build

The product side keeps owning the deck, the site, and the words (the marketing repository's `products/thinkthen/`, with `vocabulary.md` as the word list). It needs five things from you, and it will ask for nothing else without filing an issue here:

1. **One examples file per surface, keyed by function, run by that surface's tests.** Every tab on every site page is drawn from these (the marketing repository's `products/thinkthen/site.md`).
2. **The record-with-answer row (A3).** Two deck slides wait on it.
3. **The field `meta.replayed` renamed to `meta.cached`** before the shapes freeze. Ian dislikes the word. The `--replay` flag keeps its name.
4. **The real install lines**, the day they work.
5. **A word whenever a printed output changes.** The deck rebuilds from recorded real runs and its build fails when an output moves.

If a public name, flag, or output wording is in doubt, `vocabulary.md` rules, and a change to it goes through the product side.
