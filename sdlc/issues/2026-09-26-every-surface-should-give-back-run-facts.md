# Every surface should give back what Jev tells us about each run

Status: open for five parts: full detail on every verb, backend time and request IDs past ticket 0302, caller-priced cost on the remaining hosts (ticket 0300), SQL per-call facts, and the docs. Item 5, the `meta.usage` name, is settled. Shortened 2026-09-30. It absorbs the closed `closed/2026-09-23-record-the-backends-own-time-for-each-call.md`.

Priority: rank 9 of 25 in `../planning/issue-priorities-2026-09-30.md`. Owner: tickets 0300 and 0302 for items 2 and 3, ticket 0314 slice 4 for item 1, marketing for item 6.

Ian ruled on 2026-09-26 that library results carry `facts` on every call, with no setting and no second call. `--facts` controls only what the command line prints. Ian can overturn this ruling.

## Done

- Every library call returns its value with `facts`: Rust `Call<T>` (0212), C JSON (0230) and the eight typed `*_with_facts` C forms (0255), Python (0214), Ruby (0234), TypeScript (0236), R (0237), and every C-door language's typed routes (0277, 0279, 0280, 0282). ADR 0101 keeps the eight old bare C symbols as compatibility forms without facts.
- `--facts` prints one `thinkthen.run/1` line with whole-run totals, including rows `filter` drops and `rank --top` cuts (0170).
- `--details` carries the full result line on every verb (0151). DuckDB's `thinkthen_details` now returns the full result JSON. The Rust and TypeScript typed views expose `confidence` and `url`. `conformance/cases.json` checks `usage`, `requests_sent`, `cached` and `confidence`.
- Caller prices give `estimated_cost_usd` in `facts` and in `--facts` on the command, Rust and C (ticket 0300, ADR 0108; `specification/result.md`, `specification/settings.md`).
- Every surface adds to the usage totals `status` reads (0322 slice 1, ADR 0113).

## 1. Full detail on every verb

Ask 1's second half is unproved: request digests and every probability with `confidence` on every verb, bulk verbs and frames included, on every host. `public/results/observation.rs` supplies ordered question and row detail in Rust, and each binding converts it. No record shows each host's full-detail and failure parity at its boundary. `sdlc/records/2026-09-28-run-accounting-remainder-preparation.md` names that check.

## 2. Backend time and request IDs

Ticket 0302 (ADR 0109) landed an opt-in attempt record for Rust, the `--details` rows of `decide`, `choose`, `filter`, `rank`, `score` and `tag`, and the C JSON door. Each attempt carries `wall_ms`, `server_ms` from `x-envoy-upstream-service-time`, and `request_id` from `x-typesafe-request-id`, read from a fixed header allowlist.

Still open from Ian's 2026-09-23 ruling to keep the times:

- Attempt rows for `find`, `recognize`, `relate` and `annotate`.
- Attempts on a failed run whose result row never prints.
- A durable record of the times beside the recording, keyed by the entry's digest. A recording entry holds bodies and never headers (`specification/recording.md`), so the times cannot live inside it.
- `command_ms`: the whole run's time minus the exchanges.
- The attempt record on every typed host beyond Rust and C JSON.
- A chat adapter's `server_s`, from the body's `created` and the time in its `id`, if a chat adapter ships.

## 3. Caller-priced cost on the remaining hosts

Ticket 0300 stays open for the strict readers and the other typed hosts. SQLite refuses caller prices until its host adopts them. The price comes from the caller and never from a guess. With no price set, a run shows tokens and no money.

## 4. SQL per-call facts

DuckDB, PostgreSQL and SQLite keep `thinkthen_details` and `thinkthen_usage()`. A per-call or per-query facts shape waits for its own design, because a scalar returns one value a row.

## 5. The `meta.usage` name: settled

Register 61 asked that per-record token shares carry a name saying they are even shares. On 2026-09-30 the coordinator settled it: the shares keep the name `meta.usage`, as `sdlc/planning/work-plan-2026-09-27.md` decided. `specification/result.md` already says each share is even. The generated `specification/result.schema.json` pins the name, so a rename would change every binding. Ruling 8 asks for less pedantry. This is a coordinator decision; Ian can overturn it. Ticket 0337 recorded it.

## 6. Docs

Each surface's README names `facts`. The site shows no raw HTTP exchange with its `usage`, and no site page says where a library or SQL call's run facts come from. Both belong to marketing, which owns `site/`.

Target-package and runner qualification belongs to `2026-09-25-release-and-install-for-0-1.md`.
