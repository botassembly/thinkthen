# One line, one owner

Historical. [cleanup-2026-09-30.md](cleanup-2026-09-30.md) sets the current order of work.

Written 2026-09-24 by Claude, which now owns the whole ThinkThen queue at Ian's direction: main line, surfaces, QA, and launch. Every ticket is reviewed by a fresh Claude session; no other vendor reviews this queue. The ticket flow is the workspace `AGENTS.md` flow. This page supersedes the order in `build-queue-2026-09-21.md` where they differ. Ian can overturn any placement here.

## Why

Four threads ran at once and split. `surfaces-wave7` holds 446 commits main lacks, and main holds 31 the branch lacks. They plan two different ways to ship the libraries. ADR numbers collide from 0041. The same resend bug is filed twice. Marketing claims features that exist only on the branch. From now on, one queue on main.

## The decision: main is the spine

Main's path (0084 contract, 0085 engine façade, 0086 public Rust API) ships the libraries. Every surface becomes a binding over that public API inside the one `thinkthen` landing zone. `surfaces-wave7` is not merged. It is ported piece by piece, each piece in its own ticket with its tests.

Why: ADR 0017 (Ian: no `thinkthen-core` is ever published), Ian's "one landing zone" line in `planning/libraries/README.md`, and 0084's ban on a connector trait all point this way. Main also carries request splitting, SIGINT handling, `recognize`, and the relate planner, which the stand-in lacks. The branch's own `MERGE-NOTE.md` §8 names binding the real public API as its planned follow-on.

Cost: surfaces sit idle until 0086, each binding shim is rewritten against different types, and the 152 closed error-index rows were proven only against the stand-in. Each surface ticket re-runs its rows against the real engine. Ian's 2026-09-20 ruling stands: 0.1 ships on every surface together.

## The queue

Revised 2026-09-24 after three design reviews of the spine (`sdlc/records/2026-09-24-spine-review-{contract,engine,controls}.md` on the ticket branches). Every ticket is built by a Claude builder (Opus subagent) and reviewed by a fresh Claude session. Ian can overturn the order and the splits.

Landed: 0088 public `relate` (`71841025`) and 0089 no resend after a transport failure (`29578528`).

Three lanes run at once. Each lane is serial inside.

- **Lane A, engine controls and the public API:** 0082 command contract → 0083 transform catalog → 0076 whole-call deadlines → 0077 one process width cap → 0078 host signals → 0084 and 0095 contract (design only) → 0085 engine façade → 0097 interrupt check → 0096 fork recovery → 0086 public Rust API → 0098 binding members → 0119 mutation audit of the engine tests. 0078 may build beside 0077 once 0076 lands. 0119 builds before 0.1 ships, may build beside a surface ticket, and never builds beside 0113 or 0114 (`sdlc/issues/2026-09-24-red-green-scaffold-tests-outlive-their-purpose.md`).
- **Lane B, cases:** 0090 `tuned_for` rename → 0091 conformance union. It lands before 0085.
- **Lane C, test backend:** 0092 loopback backend, after 0089 and 0091, and before 0076's tests are written.
- **Any time before 0086:** 0099 ports branch ADRs 0041–0043.

After Lane A:

1. 0093 Rust examples as the first binding crate, with draft ADR 0047.
2. 0094 the C interface (ADR 0037). It closes R7-1 (G3, the C-door churn crash) with the tag's committed churn probe. 0086 runs a Rust churn probe first and claims no fix.
3. One ticket per remaining surface onto the public API: Python with Polars, TypeScript, DuckDB, Ruby, R, SQLite, PostgreSQL. Each brings its `check.sh`, tests, notes, and error-index rows, and follows ADR 0047's surface checklist.
4. 0113 `thinkthen audit`. Ian's 2026-09-24 ruling (`sdlc/issues/closed/2026-09-24-audit-and-diff-move-into-0-1.md`) puts both in 0.1, after the ten functions are done and before the release build. Both wait on 0086. audit lands before diff, and 0114 reuses 0113's code. Neither touches a binding, so either may build beside a surface ticket whose files it does not share.
5. 0114 `thinkthen diff`, after 0113 under the same ruling.
6. New: release build and installers (archives, checksums, Homebrew line, download script; Ian's 2026-09-21 ruling). Nothing tickets this today. Its release checklist counts a surface check that reports "not run" as a failure (ticket 0111, decided 2026-09-24).

Changes from the first version of this queue:

- 0078 split. 0078 keeps host signals. Fork recovery became 0096 and moved after 0085, because the retained pools it rebuilds do not exist before the façade, and the crate cannot call `fork()` without `unsafe`. The real-fork proofs live in 0086's outside consumer crate.
- The interrupt check (old item 7) is designed in 0095 and built in 0097, between 0085 and 0086. Ticket 0086 exposes `interrupt`, `deadline_seconds`, and `deadline_millis` from 0095. Ticket 0098 builds the other 0095 members after 0086, so 0086 fits its budget.
- Rust examples (0093) run before C (0094). They have no FFI and no host toolchain, so they prove the workspace, lint, ratchet, and surface-rung pattern alone. C then carries only its door and still precedes every other surface.
- Every engine row a ticket carries needs a planted-bug proof in its record: the row's test turns red on the planted bug.

## Housekeeping

- ADRs 0041–0043 come over from the branch at their numbers. 0044–0046 cover only stand-in lanes and stay behind. Main's next new ADR is 0047.
- Tag `surfaces-wave7` at `f6a7faea` before any branch is retired. The w6 and w7 branches and worktrees retire once their surface ticket lands. Only `workspace sweep` deletes worktrees.
- Fix stale ticket status lines: 0074 and 0087 are landed, 0035 says "done".
- `.github/workflows/gate.yml` still runs on push against Ian's 2026-09-22 ruling. Close that issue with a change.

## QA

Experiment 218 keeps its method. Its wave 2 conditions assumed a branch merge that will not happen. Wave 2 now runs per surface ticket against the real engine. Cheap Pi workers (GLM-5.3 through `pi-job`) run the example, input, exit-code, vocabulary, and probe re-run batteries. Claude verifies each finding before it is filed. Judgment calls, the blind seat, and anything through `sdlc/scripts/live` stay with a strong model and Ian's authorization. The release checklist and waves 3 and 4 wait on queue item 11. Experiment 252 holds a one-time Mac run of the branch surfaces.

## Launch

Ian moved marketing and Beatles Bench to a separate agent on 2026-09-24. Claude keeps the engineering gates above and does not edit launch copy.

Launch waits on the queue. The engineering gates are 0088, the release build, published crates, and every surface at 0.1. Until then, marketing copy that shows unbuilt features (deck slides 16–18 and 21, the site's "comes with 0.1" tabs, the article's "shipped" section) stays marked planned. Ian moved audit and diff into 0.1 on 2026-09-24. Tickets 0113 and 0114 hold them, and diff covers cut movement. Backend time per call becomes a ticket after item 11 unless Ian moves it.

## Needs Ian

1. Ian accepted this plan on 2026-09-24, including main as the spine.
2. Claim the package names, the Homebrew tap, and thinkthen.dev DNS. The site deploys through GitHub Actions, which Ian paused, so the site needs a deploy path he approves.
3. Whether arXiv endorsement is held or pending: `notes/todos/2026-09-09-arxiv-endorsement-status.md` and the marketing repository's `products/thinkthen/go-live.md` disagree.

## Ian's rulings, afternoon of 2026-09-24

- Everything is in 0.1. Nothing waits: audit and diff build now, beside the spine.
- Rust Polars and Python Polars are both in 0.1. A Rust Polars surface ticket joins the surfaces.
- The width setting is named the throttle. The public library setting is `throttle`, and the command keeps `--jobs`. ADR 0017's amendment records the scope.
- Package names, the tap, the site, and papers are Ian's. Claude's job is the code: the main line and every surface.
- Surfaces move as fast as possible once 0086 lands. Risk spikes 253 to 256 on the Beelink retire surface risks before then, and 0086 builds beside 0096.
- The repo's Rust toolchain moves from 1.93.1 to 1.95 in one Quick Fix. It lands right after 0086 and before any surface build. Spike 257 showed nothing breaks, and current Polars 0.55 needs 1.95. Ian can overturn this.
- A churn probe is a one-time measurement. It never runs in the ladder, a `check.sh`, a review, or a rerun, because it overloads the machine. 0086's Rust churn run stopped at the count its record gives. 0094 runs the C-door churn once to close R7-1, under the heavy lock at low load. Ian ruled this on the evening of 2026-09-24.
- Mutation testing waits until the end. Ticket 0119 builds after every surface lands and before the release build. This replaces its earlier place beside a surface ticket. Ian ruled this on the morning of 2026-09-25.
