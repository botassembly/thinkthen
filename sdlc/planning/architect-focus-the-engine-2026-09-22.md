# The architect's focus: finish the engine

Written 2026-09-22 by the product side for the architect. It covers the Rust engine and the command only. The library team owns the languages and the databases on the `surfaces` branch, and their items are listed at the end so nothing is picked up twice. Ian can overturn anything here.

Read first: `handoff-to-the-architect-2026-09-21.md`, then `build-queue-2026-09-21.md` for the lanes, then `open-issues-for-the-architect-2026-09-22.md` for the map. This page is the short list.

## Where the engine stands

Checked 2026-09-23 at main `2b63c87`: the one-package move, backend profiles, returned records, cache/counts, bounded retries, backend-bound folders, help corrections, cached metadata, and structured descriptions/evidence are landed. Ticket 0072 added fast refused connections and 0073 added private cancellation. Ticket 0074 remains an uncommitted Ctrl-C draft. Its [completion checklist and current assessment](mainline-readiness-2026-09-23.md) are the next handoff. Private width, deadlines, fork recovery, and host signal ownership remain before recognition/relation integration and the public Rust API. The library team keeps `surfaces`; its code is not on main.

## Execution amendment, 2026-09-22

Ian authorized resumption of the architect's reviewed plan. The execution amendment in `build-queue-2026-09-21.md` now controls the order: 0065 is complete; surviving command corrections and shared settings/result decisions precede private engine controls; controls precede recognition integration and the public Rust API. Recognition policy and optional packing need settled evidence, and the relation-request conflict remains blocked. C and the surface merge follow the public API. ADR 0017 records the dependency amendment. The table below retains the original source map, not the current dispatch order. Command corrections may be split into bounded reviewed tickets after checking what still fails.

## Original ticket order and source map

| Order | Lane | Ticket | Sources | Why this order |
| --- | --- | --- | --- | --- |
| 1 | A4 | The remaining money bug: a cache resume under a different address re-bills every record | `issues/2026-09-21-a-cache-resume-under-a-different-address-silently-re-bills-everything` | P0. The cache is on by default now, so every user can hit it |
| 2 | A1 | Command wording and help, one ticket | `issues/2026-09-22-command-wording-and-help-fixes-for-0-1` (40 items). Item 1 needs a ruling first: the words for the third and fourth outcomes. The product side recommends "not sure" and "an error". Item 26 holds a conflict on the 400 body; pick one. Check each item against the binary, since 0057 may have landed some | P1. Every item is a promise a user reads |
| 3 | A5 | Probability tolerance verified, shared instructions packed once per request | build-team response, proposed ticket 8; `issues/2026-09-21-the-same-request-answers-differently-twice-measured` | Changes request bytes and the cost record. Lands before any `recognize` cost claim |
| 4 | A6 | `recognize` and `relate` | `experiments/225-recognize-harvest-package`, `recognize-design.md`, `relate-design.md`, `issues/2026-09-21-quality-review-of-the-recognize-and-relate-designs` (ten usability points to rule before code), `issues/2026-09-21-recognize-is-the-ninth-function-and-the-deck-needs-one-real-output` | The last two functions. The deck has one real output already and needs a fresh relate run once this lands |
| 5 | A7 | The public Rust library over all ten functions, with the shapes the libraries copy | `issues/2026-09-21-what-must-land-before-the-bindings-freeze` (eight shape changes), `issues/2026-09-21-what-a-procedure-runtime-asks-of-a-judgment`, `issues/2026-09-20-libraries-ruled-in-and-every-public-name-is-thinkthen` (crate naming), and the ten library settlements below | The libraries sit on this. The merge ticket comes after it |
| 6 | A8 | Width, cancellation, fork repair, fast failure on a dead address | build-team response, proposed ticket 11; `issues/2026-09-20-a-process-that-forks-after-its-first-call-hangs`; `issues/2026-09-21-a-run-stopped-by-sigint-prints-no-stopped-at-line` | The library team proves each fix on its own language after this lands |
| 7 | A9 | The C door: one JSON result, one free function | proposed ticket 12 | After the Rust result path exists |
| 8 | A10 | Install and release: the Homebrew tap and the download script, copied from BioMCP | `issues/2026-09-21-nothing-says-how-the-command-gets-installed` (ruled), `issues/2026-09-20-lessons-from-biomcp-for-release-install-ci-and-docs` (the checklist), `issues/2026-09-20-launch-gaps-found-in-marketing-prep` (P0: key on first run, no exit-1 page, second-backend owner), `issues/2026-09-20-what-the-launch-needs-from-the-build` | P0 items live here. Ian creates the tap repository himself |
| 9 | A11, A12 | The promise findings, the quality waves, the release pass | `issues/2026-09-20-accuracy-round-on-three-public-sets-and-a-speed-rerun` (a crash in `cost.jq`), `issues/2026-09-20-new-user-stumble-register`, and the P2 list in the map | Last before 0.1 |
| 10 | after 0065, before A6 (moved 2026-09-22) | Structured descriptions in the question file | `issues/2026-09-21-the-question-file-cannot-carry-typesafes-structured-fields` (approved, nine rulings at its top) and `issues/closed/2026-09-22-what-the-vendors-founder-said-about-where-the-model-goes` (why it moved) | Now ahead of recognize and relate. The vendor is optimizing the model for structured slots, and a 0.1 that sends strings only reads as behind on launch day. The measurement is done. Goes first if the recognize method needs array instructions. Typed builders for the libraries are their own issue for the library team |

## Ten settlements the library team made that touch the engine

On 2026-09-22 the library team settled ten open points on the `surfaces` branch in their own contract crate. Each is a shape the engine will have to match in A7, or overturn now. Read their settlement issue on the branch when it lands on main, and rule on each before A7 starts:

1. Polars: the Series door ships behind a feature flag. The plugin does not ship.
2. `score` details gain a `nearest` field, the level closest to the value.
3. `find` and `rank` return a pair everywhere: the best unit's place and its probability.
4. SQLite drops the deterministic flag, so a paid function is never legal in an index.
5. SQLite keeps a shim cache until the engine swap, recorded as a named divergence.
6. A spent deadline is legal everywhere.
7. A built question plus members is refused as ambiguous.
8. DuckDB's load-time interrupt is that binding's named cancel channel.
9. The record row is a host-side rendering, so no surface emits it itself.
10. SQLite's `relate` takes the ruled rule grammar. DuckDB's relation columns are `source` and `target`.

Items 2 and 3 change result shapes in `specification/result.md`. The product side has not reviewed them. If the engine adopts them, the spec pages and the deck's `--details` slides change in the same commit.

## Rulings the architect owes

- The words for the third and fourth outcomes (item 1 above).
- The ten usability points on the recognize and relate designs.
- `find-in`: rule it out or into the backlog. `issues/2026-09-21-candidates-for-a-tenth-function-relate-and-find-in`.
- `windows-over-long-text`: A2 already refuses with a clear message. Close it or say what more is owed.
- Whether to adopt library settlements 2 and 3.

## Not the architect's

- Everything on the `surfaces` branch. The library team has seven hand-off items listed in `open-issues-for-the-architect-2026-09-22.md`, including the SQLite and DuckDB workarounds.
- The merge of the surfaces branch. It comes after A7 and is its own ticket.
- Ian's: registry names, the tap repository, the domain, any upstream report, and whether a throughput claim needs a paid packing probe.
- Marketing's: the two public-page issues, the deck, the site.
- Paused by Ian: Excel, Google Sheets, any serve mode.

## Guard rails, repeated

A live call runs only through `sdlc/scripts/live` under a cap. Tests replay recordings. The gate ladder runs before every hand-back. A public-surface change gets a second-agent review. Every decision lands in `sdlc/` with a note on whether Ian can overturn it.
