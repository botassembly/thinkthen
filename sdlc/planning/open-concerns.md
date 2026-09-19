# Open concerns

**Ruled by Ian on 2026-09-19. ADR 0010 records the rulings.** He accepted every recommendation below with two changes: `segment` leaves the plan, and `jq` recipes come before any `report` command. The text below is kept as it was written.

Written 2026-09-19 at Ian's request. He asked for the most contested points across every ADR and the plan, with options, a recommendation, and the reason. Nothing here is decided. Each item names the record it would change.

The test for every item is Ian's goal: the simplest grammar for intelligent control inside a Bash script, the most value out of the backend's three question types, and primitives for processing data on one machine.

## 1. `report` is the largest invented thing, and it sits furthest from the goal (ADR 0008 item 4)

`report` calls no model. It is ordinary arithmetic over saved rows. Its JSON shape was invented by a reviewer, and three demos now lean on it. Ian's own central requirement says ordinary code handles metrics and comparison. The saved row is the real contract. `report` is a convenience on top.

- A. Build it as proposed: truth labels, a sweep, a calibration table, a baseline run, and averaged trials.
- B. Build it in two steps. First the summary, `--truth`, `--threshold`, and the sweep. Then `--baseline`. The calibration table and the averaging of repeated trials wait for a benchmark that needs them.
- C. Ship no `report` in version one, and publish a page of `jq` recipes.

Recommendation: B. The sweep answers the one question every script writer has: what threshold goes in my script. F1 in `jq` is ugly enough to earn a command. The rest can wait. Accept the row parts of ADR 0008 now (items 1, 2, 3, 5, 6, 7), because a saved row is hard to change after people keep files of them.

One unknown sits under repeated trials. The pages never say whether the judge returns the same numbers for the same request. If it does, a trial only means something when the candidate's output changes. A live call settles this.

## 2. A failed request reads as "no" inside `if` (ADR 0007)

`if thinkthen decide ...; then` sends exit 1 (no), exit 3 (unresolved), and exit 4 (backend down) to the same branch. An outage turns every gate to "no" with only a line on standard error. Under `set -e`, a bare `decide` that answers "no" ends the script.

- A. Keep it. This is `grep`'s convention, and `grep` has the same two traps.
- B. Add an option that names what a failure means.

Recommendation: A. Every shell user knows the convention, and a new rule costs more than it saves. The help for `decide` shows two patterns: a `case $?` block, and the advice to word the question so that yes permits the action. A failure then never permits anything.

## 3. `find` is a tenth command, and its units see each other (ADR 0009 item 3)

`find` is the only verb that would use the choice type's 255 options at scale. It makes one request where `rank` makes one per record. It also breaks the rule that records never share a request, and how it says "nothing fits" is open.

- A. Accept it as a verb now.
- B. Fold it into `choose`. This fails, because `--lines` on `choose` already means one record per line.
- C. Keep the page Draft and build nothing until a live run compares it with `rank --top 1` on the same units.

Recommendation: C. The measurement costs cents. If `find` picks as well as `rank --top 1`, it is the largest unused value in the vendor's interface. If it picks worse, it goes to the roadmap.

## 4. `segment` refuses the documents that most need it (ADR 0009 item 1)

The whole document rides in one request, so a document over the evidence limit is refused. A 300-line document also means 299 questions in one request, and the vendor publishes no cap on questions.

- A. Whole document only, as proposed.
- B. Keep `--window` as well.
- C. Hold `segment` out of version one.

Recommendation: A, built last. `segment` is the least general verb, and it is the first one to cut if the surface must shrink. A live call finds the question cap before the ticket is written.

## 5. `score` prints a decimal, and Bash cannot compare decimals (ADR 0007, and ADR 0009 "left out")

`[ "$(thinkthen score ...)" -ge 2 ]` fails on `2.37`. The answer today is a pipe into `jq -e '. >= 2'`.

- A. Keep it, and show the `jq` line in the help.
- B. Let `--threshold N` on `score` set the exit code.

Recommendation: A, held loosely. `--threshold` means a probability on every other verb, and a second meaning costs the grammar its one rule. The Bash way to branch on levels is `case "$(thinkthen choose ... low medium high)"`. `score` gives a number for `sort -n`, `jq`, and reports. It stays in version one because the goal names all three types, and its measured weakness gets a fresh live check.

## 6. Structured questions carry the vendor's shape, and no demo needs them (ADR 0009 item 5)

The agent that proposed this now recommends striking it from version one. No demo uses it. The local adapter has no meaning for it. It can be added later without breaking any file.

## 7. The cut uses the winning probability, and the vendor cuts on `confidence` (ADR 0009 item 2)

Recommendation: keep it as proposed. Store both numbers, cut on the one that exists on every backend, and let a live sweep against labels say whether `confidence` separates better.

## 8. Resuming a stopped run takes two options that name one folder (ADR 0007)

`--record DIR --replay DIR` is the testing form. Resuming a long run is an everyday need.

Recommendation: add `--cache DIR` with the meaning of both, when slice 7 is specified. Ian can strike it.

## 9. Smaller points

- **Evidence keys (ADR 0008 item 2).** The last part of each pointer names the member, and a clash is a usage error. Keep it. `jq` renames a field before the run.
- **The configuration command.** It serves a second backend that nobody uses yet. Keep it, and build it after the verbs.

## The plan

- **Nothing has been measured live.** The vendor still answered 402 on 2026-09-19: "Your organization has no available TypeSafe API credits. Please add more credits and/or set up auto-reload". The lever is the vendor's billing page. Until a call succeeds, slice 5 stays early, because a local server is the only free live path.
- **Money is no constraint.** The price is $0.042 for a million input tokens. Ian's limit of $20 buys about 476 million tokens.
- **The design writing is far ahead of the code.** Eighteen pages, nine ADRs, and fifteen red demos stand on one built verb. The review found six errors in demos that have never run. Recommendation: no new pages until slice 4 lands. Code and live answers push back on the design from here.
- **A live probe comes first once calls work.** It checks the real shape of all three answers, whether the same request returns the same numbers, the cap on questions in one request, `find` against `rank --top 1`, and `confidence` against the winning probability.
