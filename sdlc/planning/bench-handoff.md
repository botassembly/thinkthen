# Handoff to the Beatles Bench team, 2026-09-30

Status: current. This page answers the Beatles Bench team's feedback of 2026-09-30 in one place. The Beatles Bench is public. The private release QA suite is named only as that. Ian can overturn each coordinator default below.

## Three places for tests

thinkthen's gate keeps mechanics against recordings, with no network. The public Beatles Bench publishes accuracy, cost and speed. The private release QA suite checks releases live. `test-split-2026-09-30.md`, "Three places", holds the full table, and ticket 0335 carries the work. Error paths stay in thinkthen's loopback tests, because the release QA suite does not fake the service: retries, server errors, status 520, oversized replies and dropped connections.

## Answers

### 1. `status --json` fields the bench reads

`status --json` moves from `thinkthen.status/1` to `thinkthen.status/2` in ticket 0334 (ADR 0114 section 6). On main, `thinkthen.status/1` already prints `.usage.total.requests_sent` and `.usage.total.input_tokens`: the count-only totals of every surface, all months (ADR 0113). Version 2 keeps both with the same meaning. That promise is on ticket 0334's branch, pending until 0334 lands; the 0334 builder was told.

### 2. Checkpoint and release-candidate tags

The convention is `worktrees.md`, "Landing commits and tags". Under it, only `phase/cleanup-2026-09-29` exists so far. Older `surfaces-wave7-*` tags predate the convention.

- `checkpoint/surfaces/YYYY-MM-DD-N` marks a main commit on which every surface check passed. The first comes after the first full surface pass on one main commit, expected after 0304 slice 3b lands. Its annotated message lists each check and its result.
- `rc/0.1.0-rc.1` marks the first release candidate. It comes when the 0.1 blockers in `issue-priorities-2026-09-30.md` clear. Ruling 10 holds every public release until 0.1.

The bench can pin its per-function summaries (bench ticket 0022) to these tags.

### 3. How to send a hard case as a recording

One line: a hard case is a folder holding one sorted `thinkthen.jsonl` beside its input and key files, and a test replays it through the real command with `--replay DIR`, the recorded `--url` and `--model`, and no key.

- **The file.** `thinkthen.jsonl` holds one JSON object per line: every shared state as `{"sha256":…,"state":…}` sorted by digest, then every answer sorted by question key (ADR 0111, "Fixtures"). The repository commits no `thinkthen.sqlite`. Recordings hold request and response bodies, never headers or keys.
- **Making one.** Run the case live into a scratch folder with `--record DIR`, then `thinkthen cache convert DIR` writes `DIR/thinkthen.jsonl` (`specification/recording.md`, "Converting a folder to the question fixture"). Old `DIGEST.json` recordings convert the same way. Send the folder with public text only.
- **Where it goes.** Cases that pin a function's behavior live under `specification/fixtures/<function>/`, with a README that names the source of each file. `specification/fixtures/relate/recording/` and `specification/fixtures/recognize/recordings/<case>/` are the pattern. Demo pages keep theirs in `demos/NN-name/recording/`. A bench case may name the Beatles Bench as its source. Coordinator default: bench cases go under `specification/fixtures/<function>/bench-<case>/`.
- **How a test replays one.** The command runs with the environment cleared, `--url` and `--model` equal to the recorded address and model (both are part of each question's key), and `--replay DIR`. The test pins exact output. `crates/thinkthen/tests/backend/relate_edge.rs` replays `demos/44-recognize-names/recording/thinkthen.jsonl`; the README of `specification/fixtures/relate/` replays its recording in a block that the `spec` rung runs and pins with `mustmatch`. A question the folder lacks exits 5 and names its key, and nothing is sent.

### 4. The bench's summaries, sample runs and repeat pass

The bench adds a JSON summary per function pinned to tags (bench ticket 0022), a one-at-a-time sample run (bench ticket 0023), and a repeat pass that lists near-cut and flipping answers. `../issues/2026-09-27-nothing-lists-the-uncertain-hard-or-flip-flopping-cases.md` now links this plan. That issue stays deferred past 0.1.

## The relate decision run

`../issues/2026-09-30-relate-pair-planner-loses-precision-on-the-beatles-bench.md`, "The decision run", records the bench's three conditions: the none-of-these rate beside precision when the right album is missing, an audit-tuned cut beside 0.5, and Liquid d1 beside the default backend. The run waits for the thinkthen commit that adds the single-answer menu, after 0304 slice 4.

## Issues from this feedback

Filed on 2026-09-30:

- `../issues/closed/2026-09-30-command-question-file-has-no-size-cap.md`
- `../issues/2026-09-30-live-batching-flake-and-unexplained-usage-calls.md`
- `../issues/2026-09-30-spec-no-calls-edges-need-a-real-send.md`

Merged into open issues: the Liquid d1 first-`check` timeout went into `../issues/2026-09-29-docs-page-naming-supported-providers.md`, section 2 and row 19 of `../issues/2026-09-20-new-user-stumble-register.md`. A longer default timeout on the `liquid` built-in waits for ticket 0334's deferred per-backend limits.

Already on main, not filed again: the Objective-C header collision (closed by ticket 0337), PostgreSQL on macOS (closed by ticket 0336), the recording page's retired backend marker (open), and relate precision (open).

Closed: `../issues/closed/2026-09-30-split-tests-between-the-gate-and-release-qa.md`, into ticket 0335.
