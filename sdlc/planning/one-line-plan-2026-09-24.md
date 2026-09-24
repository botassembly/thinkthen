# One line, one owner

Written 2026-09-24 by Claude, which now owns the whole ThinkThen queue at Ian's direction: main line, surfaces, QA, and launch. Codex (Astra) reviews as the other vendor. The ticket flow is the workspace `AGENTS.md` flow. This page supersedes the order in `build-queue-2026-09-21.md` where they differ. Ian can overturn any placement here.

## Why

Four threads ran at once and split. `surfaces-wave7` holds 446 commits main lacks, and main holds 31 the branch lacks. They plan two different ways to ship the libraries. ADR numbers collide from 0041. The same resend bug is filed twice. Marketing claims features that exist only on the branch. From now on, one queue on main.

## The decision: main is the spine

Main's path (0084 contract, 0085 engine façade, 0086 public Rust API) ships the libraries. Every surface becomes a binding over that public API inside the one `thinkthen` landing zone. `surfaces-wave7` is not merged. It is ported piece by piece, each piece in its own ticket with its tests.

Why: ADR 0017 (Ian: no `thinkthen-core` is ever published), Ian's "one landing zone" line in `planning/libraries/README.md`, and 0084's ban on a connector trait all point this way. Main also carries request splitting, SIGINT handling, `recognize`, and the relate planner, which the stand-in lacks. The branch's own `MERGE-NOTE.md` §8 names binding the real public API as its planned follow-on.

Cost: surfaces sit idle until 0086, each binding shim is rewritten against different types, and the 152 closed error-index rows were proven only against the stand-in. Each surface ticket re-runs its rows against the real engine. Ian's 2026-09-20 ruling stands: 0.1 ships on every surface together.

## The queue

1. 0088 public `relate`: Claude reviews Codex's uncommitted diff, then gates and lands.
2. New: stop resending a delivered request after a transport failure (money; `issues/2026-09-23-the-command-sends-a-delivered-request-again-after-a-transport-failure.md`). Port the branch fix `dd8a383` idea, not the code. Built as ticket 0089 at 3686414f and awaiting review.
3. 0082 command contract. Ian ruled `tuned_for` on 2026-09-23 per the 0082 ticket branch, so it is not blocked.
4. 0083 transform catalog.
5. 0076 whole-call deadlines, 0077 one process width cap, 0078 fork recovery and host signals.
6. New: merge the branch's conformance cases into main's `cases.json`. It lands before 0085 because 0085 runs every case.
7. 0084 amended: add a public interrupt-check hook for bindings. 0086 says bindings add it over the private façade, and a binding crate cannot reach private code.
8. 0085, then 0086.
9. New: the C interface, porting `contract/include/thinkthen.h` and `libraries/c/DESIGN.md` (ADR 0037).
10. One ticket per surface onto the public API: Python, Polars, TypeScript, DuckDB, Ruby, R, SQLite, PostgreSQL, Rust examples. Each brings its `check.sh`, tests, notes, and error-index rows.
11. New: release build and installers (archives, checksums, Homebrew line, download script; Ian's 2026-09-21 ruling). Nothing tickets this today.

0078, 0085, and 0086 are drafts whose designs are not reviewed. Each gets a design review before its turn.

## Housekeeping

- ADRs 0041–0043 come over from the branch at their numbers. 0044–0046 cover only stand-in lanes and stay behind. Main's next new ADR is 0047.
- Tag `surfaces-wave7` at `f6a7faea` before any branch is retired. The w6 and w7 branches and worktrees retire once their surface ticket lands. Only `workspace sweep` deletes worktrees.
- Fix stale ticket status lines: 0074 and 0087 are landed, 0035 says "done".
- `.github/workflows/gate.yml` still runs on push against Ian's 2026-09-22 ruling. Close that issue with a change.

## QA

Experiment 218 keeps its method. Its wave 2 conditions assumed a branch merge that will not happen. Wave 2 now runs per surface ticket against the real engine. Cheap Pi workers (GLM-5.3 through `pi-job`) run the example, input, exit-code, vocabulary, and probe re-run batteries. Claude verifies each finding before it is filed. Judgment calls, the blind seat, and anything through `sdlc/scripts/live` stay with a strong model and Ian's authorization. The release checklist and waves 3 and 4 wait on queue item 11. Experiment 252 holds a one-time Mac run of the branch surfaces.

## Launch

Launch waits on the queue. The engineering gates are 0088, the release build, published crates, and every surface at 0.1. Until then, marketing copy that shows unbuilt features (deck slides 16–18 and 21, the site's "comes with 0.1" tabs, the article's "shipped" section) stays marked planned. The measurement requests (backend time per call, cut movement, audit and diff) become tickets after item 11 unless Ian moves them.

## Needs Ian

1. Ian accepted this plan on 2026-09-24, including main as the spine.
2. Claim the package names, the Homebrew tap, and thinkthen.dev DNS. The site deploys through GitHub Actions, which Ian paused, so the site needs a deploy path he approves.
3. Whether arXiv endorsement is held or pending: `notes/todos/2026-09-09-arxiv-endorsement-status.md` and `repos/mktg/products/thinkthen/go-live.md` disagree.
