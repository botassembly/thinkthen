REJECT

Reviewer: fresh read-only Claude (Opus) session, 2026-09-24. Checked: ticket at ticket/0082-close-command-contract (eaa4cbce), its prior review (sdlc/records/0082-design-review.md), main at ab72203c (0088 landed), the 0089 ticket at 7b3f5830, the merged 45-item issue, the 2026-09-21 vocabulary issue, main's help doc comments in cli/args.rs, cli/args/command.rs, cli/args/find.rs, specification/relate.md, sdlc/scripts/pages, and every green how-to.

## Prior findings

All nine are resolved. F1: row 3 now pins main's two 0088 sentences, which version.rs:55 and :59 already carry. F2: an exact relate sentence is given (but see B3). F3-F7: item 44 moved whole to 0090, so the stop rule has no exception and core is no longer edited. F8: the baseline is main after 0088 and 0089, and the shared 0089 files are named. F9: Claude owns it, and the review route follows the one-line plan.

## Blocking findings

B1. The vocabulary check cannot reach zero hits as written, and the prose cap cannot absorb the sweep. The rules "`row` where the contract means record", "`document` where a new user means text", and "`label` where `choose` means option" need a human to judge each hit. A script cannot apply them. On main, `row`/`rows` (header row excluded) appears 111 times across 14 of the README and how-to pages. Most of those uses are the owned term for saved `--details` rows: `transforms/rows/`, `$rows` in fenced code, and "forty committed detailed `decide` rows" in demo 13. `document` in help names the one-document framing ("available for one document, --lines, and --jsonl", args.rs:341, command.rs:85). Green how-to 02 is titled "Branch on a label with `choose` and `case`". `sdlc/scripts/pages` cross-checks that title against demos/README.md, `sdlc/planning/documentation-plan.md` (not in `opens`), and ADR 0018's list. Under the 13-file, +120-line prose cap, "zero unsanctioned hits" is either out of reach or undefined.
Smallest change: split the check into two lists. (a) Fixed banned phrases apply everywhere it scans: `decider model`, `decision model`, `rating`, `the mark`, `judgment(s)`, each with the sanctions already listed. (b) The context words `row`, `document`, `unit`, and `choose`'s `label` apply only to built help. Sanction `rows` for saved detailed result rows, and skip fenced code and paths. Decide demo 02's title in the ticket: keep it with a sanction, or rename it and add documentation-plan.md and the ADR 0018 list to `opens` and the budget.

B2. Row 1 names no exact sentence. The acceptance says "Exact assertions pin the four-outcome teaching copy to yes, no, not sure, and broken", but main has no such sentence. `decide` long help carries only "3 for unresolved" (command.rs:23). Also, "Specification text that uses `unresolved` names it as the formal term" conflicts with "introduce once": ten specification pages use the word. Smallest change: write the exact teaching sentence and the help and README locations that carry it. Replace the specification bullet with "exactly one sentence, in <named page>, defines `unresolved` as the formal term for not sure", pinned by a fixed-string test.

B3. The relate exit-4 sentence overclaims. The ticket's sentence is "A run that answers none prints nothing and exits 4." relate.md:34 says empty line and JSONL streams succeed with no output. relate.md:62 ties exit 4 to "no valid logical answer remains" after questions were asked. Smallest change: "A run whose relation questions all fail prints nothing and exits 4." Pin that exact sentence.

## Follow-up (non-blocking)

- Budgets: "the likely set is `cli/args/command.rs` alone" is wrong. Row 6 text sits in args.rs: choose `label` at 315-341, `document` at 79, 154, and 341, `row` at 38-40, and `judgments` at 446. It also sits in find.rs (`unit`, six lines). That makes three files and still fits the four-file cap. Correct the sentence.
- Tests: find_edge.rs:89 pins "Every unit leaves together and sees every other unit". Name it beside tag_edge.rs and version.rs. The five-file test cap is tight but reachable.
- Order and shared files with 0089 and 0083 are correct.
