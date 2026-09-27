---
flow: build
priority: 159
opens: crates/thinkthen/src/core/adapters/systemone.rs crates/thinkthen/src/cli/failure.rs crates/thinkthen/src/public/error.rs crates/thinkthen/src/cli/audit/write.rs crates/thinkthen/src/cli/audit crates/thinkthen/tests crates/thinkthen/src/core/backend.rs crates/thinkthen/src/core/plan_document.rs crates/thinkthen/src/core/question_file/resolve/tests.rs crates/thinkthen/src/cli/conformance_tests.rs crates/thinkthen/src/cli/conformance_tests/mutations.rs crates/thinkthen/src/cli/conformance_tests/runner.rs crates/thinkthen/src/cli/conformance_tests/command.rs crates/thinkthen/src/cli/conformance_tests/profile_cases.rs specification/backends.md specification/settings.md specification/question-file.md specification/check.md specification/recognize.md specification/recording.md specification/result.md specification/audit.md specification/decide.md specification/choose.md specification/score.md specification/rank.md specification/filter.md specification/find.md specification/annotate.md specification/relate.md specification/channels.md specification/fixtures spec demos transforms/rows probes conformance/cases.json databases/postgresql/fixtures/runner-excuse.json libraries CHANGELOG.md sdlc/scripts/rekey-model sdlc/scripts/fixtures/rekey-model sdlc/scripts/lint sdlc/scripts/README.md sdlc/issues sdlc/planning/backlog-0-1-2026-09-26.md sdlc/ratchet.json sdlc/records sdlc/tickets
---

# 0159: The default model is a pinned version

Status: landed 2026-09-26 (`sdlc/records/0159-build-pin-the-default-model.md`). A fresh read-only code review accepted it after one fix. The coordinator accepted it on 2026-09-26 after five fresh read-only reviews and a coordinator fix to two lines. Owner: Claude.

Review route: a fresh read-only Claude session reviews this design and the final diff. Codex does not review this ticket unless Ian routes it.

## Outcome and authority

A user who names no model gets answers from `jev-1.13.0`, and every request, cache key and recording says so. The vendor can move its `jev-latest` alias without moving a tuned cut or a cached answer. `jev-latest` still works when the user types it. A cut that `audit --write` tunes also records the model it was tuned on, so a later default change cannot move it silently. A run that stops because one record's replies came from two model versions says that a cache or recording may hold the older version, and it no longer blames the backend alone.

Ian approved the pin on 2026-09-26. That answers question 1 of `sdlc/planning/backlog-0-1-2026-09-26.md` with option A. It carries rulings 2 and 8 of `sdlc/issues/closed/2026-09-22-what-the-vendors-founder-said-about-where-the-model-goes.md`. Ruling 8 reads: "the default the tool writes into a new question file and into every cache key is the current pinned version (today `jev-1.13.0`), and `jev-latest` is accepted only when the user types it."

## Which version, and why

The pin is `jev-1.13.0`. The repository's own evidence names no other model that answered.

- Every live reply the repository keeps names `jev-1.13.0`. A scan of every tracked `thinkthen.recording/1` entry on `origin/main` `40eae81e` found 2,465 live entries. 2,463 asked for `jev-latest` and were answered by `jev-1.13.0`. Two, in `demos/15-find-the-line/recording/`, asked for `jev-1.13.0` by name and were answered by it. The only other name is the loopback's `local-1`, in four entries.
- So the backend accepts the pinned name as a request model. The two `demos/15` entries prove it with no new call.
- Every measurement behind a ruling names the same model. Experiment 212 (ADR 0010, amendment of 2026-09-25) saw `jev-1.13.0` answer all 100 messages twice. Experiment 259 found every repeated benchmark digest answered by `jev-1.13.0`. Local experiment 273, report 09, found every Beatles Bench recording answered by it.
- `specification/result.md` already shows `meta.model` as `jev-1.13.0` in every example.
- The vendor's founder named `jev-1.13.0` as the version that may get temporary long-term support (issue of 2026-09-22, point 2).
- The only other version names in the repository, `jev-1.14.0`, `jev-2.0.0`, `jev-1.2` and `jev-1.3`, come from fake backends in tests and reviews.

Pinning to a version that no recorded answer came from would re-key the fixtures onto a model the gates never measured.

## What happens today

Read from `origin/main` `40eae81e`, the base of this branch.

- `crates/thinkthen/src/core/adapters/systemone.rs:35` sets `DEFAULT_MODEL` to `jev-latest`. The command, `status`, `check`, every library through `public/settings.rs:229`, and the SQL extensions read it.
- `specification/settings.md` line 52 gives the default as `jev-latest` with "No reason recorded for the alias over a pinned version", and line 78 lists it under "Defaults with no recorded reason". `backends.md` line 57, `question-file.md` line 91, `check.md` line 16 and the "Backend options" row of seven verb pages name `jev-latest` as the default.
- A recording's digest covers the request body, and the body names the model (`recording.md`, "An entry"). Changing the default changes every default digest, so every replay of a default request misses.
- Recordings replayed by a gate that ask for `jev-latest`: 40 entries under `crates/thinkthen`, 131 under `demos/`, 80 under `transforms/rows`, the default request bodies in `conformance/cases.json`, and 879 under `probes/01` to `probes/09`, which the `spec` rung replays through `probes/replay-check.sh`. `site/recordings` holds 193 and `site/examples` 54. The marketing lead owns `site/`, and its hand-run build replays them (`.github/workflows/pages.yml`).
- `audit --write` writes only a threshold (`specification/audit.md`, "Writing the bar"). The question digest leaves the model out (`question-file.md` canonical rule 1). A file tuned under one model runs under the next with no sign. Architect review 05, finding 2.1, and review 08, item 1, found it.
- A record whose replies name two model versions stops at exit 4 with "the backend returned different model versions for one record; pin --model and rerun with --record or --cache" (`cli/failure.rs:344-351`, `public/error.rs:220`, `result.md` line 154). Local experiment 273, report 03, finding 2-5, showed that a cache causes it. After the alias moves, unchanged groups replay from the cache under the old version and an edited group answers live under the new one. The advice to use a cache is then wrong (`sdlc/issues/2026-09-26-a-mixed-model-cache-fails-every-annotate-record.md`).

## Design

### The pin

`DEFAULT_MODEL` becomes `jev-1.13.0`. Nothing else in the resolution order changes: `--model`, then the question file's `model`, then the configuration file's `model`, then the default. `jev-latest` stays a valid name that a user can type or write.

Ruling 2 also asks that a run with a model other than the question file's either refuse or take an explicit flag. `--model` is that explicit flag. It already outranks the file's `model`, and a user types it on purpose. So this ticket adds no refusal and no new flag, and `question-file.md` line 101 gains: "`--model` beside a file that names a model is the explicit way to run that file on another model."

The pages change with it: `settings.md` lines 18, 52 and 78 (the default leaves the "no recorded reason" list and cites this ticket), `backends.md` line 57, `question-file.md` line 91 and its two examples (lines 12 and 20), `relate.md`'s file example on line 19, `check.md` line 16 and its examples, `recognize.md` line 56, `channels.md` lines 92 and 102, and the "Backend options" row of `decide.md`, `choose.md`, `score.md`, `rank.md`, `filter.md`, `find.md` and `annotate.md`. `backends.md` gains one sentence: "The default is a pinned version, so a vendor's move of its alias moves no default answer. A later release that changes the default says so in the changelog, and every default cache entry then misses once." `CHANGELOG.md` gets one line under 0.1.

### Re-keying the recordings

The replies behind the queue owner's recordings came from `jev-1.13.0`. A new Python 3 script, `sdlc/scripts/rekey-model FROM TO FOLDER...`, moves each such entry to the key the pinned default now asks for. Its rules:

1. It works only inside a Git work tree. It refuses an entry unless Git tracks the file and the file matches the index and `HEAD`, so it never moves an untracked or edited entry. It reads only digest-named files, 64 lowercase hex characters and `.json`, whose `schema` is `thinkthen.recording/1`. It skips every other name. It refuses a digest-named symlink or other non-regular file and writes nothing.
2. It checks every entry before it writes any. For each entry it recomputes the old digest from the adapter name, a newline, the stored URL, a newline, and the stored request bytes, as `recording.md` defines it, and refuses when the file's name does not match. It refuses an entry whose request asks for FROM and whose reply names any model other than TO. It refuses when the new name already exists. One refusal stops the run before any write, names the file, and prints nothing from inside it.
3. It replaces the bytes of the request's model value in place. It never parses and re-serializes the JSON, so every other byte of the entry stays as the recorder wrote it. The reply is not touched.
4. It computes the new digest from the new request bytes, renames the file with `git mv`, and then writes the new bytes to the new name. Every old file was tracked and clean in `HEAD`, so it can be undone until the commit. `git checkout HEAD -- FOLDER` restores the old names, but it leaves the new names staged, so `git rm -f` on each new name must follow. After the commit, the parent commit keeps the old entries.
5. It prints each old digest beside its new one, so pages, rows and tests that print a digest can be updated from that list.

`--self-test` runs the script in a scratch folder, and `sdlc/scripts/lint` runs it as it runs `tickets --self-test`. Its fixture, `sdlc/scripts/fixtures/rekey-model/`, holds two entries whose names and bytes the Rust recorder wrote, copied before the re-key: one `jev-latest` entry from `demos/01-refund-gate/recording/`, and one of the two `demos/15-find-the-line/recording/` entries that asked for `jev-1.13.0`. The self-test runs these steps.

1. It copies both entries into a scratch folder, runs `git init` there, and commits them.
2. It turns the second copy's request model value back to `jev-latest`, recomputes that copy's digest, renames it to the `jev-latest` digest, and commits again. The scratch folder now holds two clean `jev-latest` entries.
3. It re-keys the folder and checks that the second entry's new file equals the Rust-written `demos/15` original byte for byte, name and contents both. It checks that the first entry kept every byte except its model value.
4. It runs one plant at a time in a fresh scratch copy: a reply naming another model, an entry whose name does not match its bytes, a symlink, an existing target name, an untracked entry, and an edited entry. It checks that each is refused, that the refusal sentence is the one pinned, and that no file changed.

`sdlc/scripts/README.md` lists the script, its rules and its self-test.

Where the script and the rule apply:

- `crates/thinkthen/tests`, `demos/` and `transforms/rows` are re-keyed. Every old digest that a page, a saved row or a test prints is replaced by its new digest in the same commit.
- Request bodies stored as strings follow the same rule by hand, because they are not entries. The rule: a body that the default model produced changes its model value to `jev-1.13.0`, and a body that names a model on purpose keeps it. A reply stored beside it keeps its model, because it is what the fake backend answers.
  - `conformance/cases.json`: no case names a model today, so every case runs on the default, and every request body changes. A case that names `--model jev-latest` would keep its body, and none exists. Each `provenance.path` that names a re-keyed demo entry takes the entry's new name. The file's expected `details.requests` lists hold 56 distinct digests in 93 places. Each is SHA-256 of `systemone`, a newline, the file's `backend_url`, a newline, and its exchange's `request` string, which today matches every one of them. A short one-off computation replaces each old digest with the digest of the changed request string, and the `test` rung's conformance runner fails any digest left stale. The in-process runners build their backends with the literal `jev-latest` to match those bodies: `cli/conformance_tests.rs` lines 147 and 181, `cli/conformance_tests/runner.rs` lines 77 and 177, and `cli/conformance_tests/command.rs` line 405. Each moves to the default constant. The `command.rs` literal matters most. Its `counters` runner serves case `40-decide-counters` through `serve_once`, which waits for the case's exact request bytes. If the backend still sent `jev-latest`, the bytes would never match, and the case would fail when the client gave up. `cli/conformance_tests/profile_cases.rs` line 52 moves to the default constant too. It compares no request bytes, because `facade::split` checks only the profile's limits, so it would pass either way. It moves so that no conformance runner names the alias by accident.
  - `databases/postgresql/fixtures/runner-excuse.json` copies case `01-decide-yes-captured` for the PostgreSQL runner's self-test, so its request body changes with that case. Its one expected `details.requests` digest takes case 01's new digest by the same computation.
  - `specification/fixtures/check/requests.jsonl` changes, because `check` sends the default model. `check.md`'s copy of those bodies changes with it.
  - `specification/fixtures/systemone/*` keeps `jev-latest`. The adapter tests build their plans with that literal (`core/adapters/systemone.rs` test plans) and pin the wire shape, not the default.
  - Three copies of the fixture's `decide-urgent` exchange keep `jev-latest` with it. `backends.md` lines 107 and 113 show its request and response. `core/recording.rs` holds it in `plan()`, `RESPONSE` and `FILE` (lines 250 to 262), and its `PINNED` digest, `bd370a64…4a78.json`, is that exchange's name. The example entry in `recording.md` (lines 53 and 54) is the same exchange, and neither its request line nor its response line changes. One rule covers all of them: a reply keeps the model its backend named, and the re-key moves only an entry whose reply names the pinned version. This exchange's reply names `jev-latest`, so the script would refuse it, and it stays whole. These copies pin the adapter's wire shape and the entry format, not the default. `core/recording.rs` does not change, and it is not in `opens`.
- `probes/01` to `probes/09` are measurements of what the tool sent then. Their recordings keep their bytes. Each `probes/0N/job.sh` gains `--model jev-latest` on every command it runs, so its replay sends the request that was recorded, and the committed rows do not change.
- `probes/annotate-0015` and `probes/find-0040` keep their bytes. No rung replays them against the command.
- `crates/thinkthen/tests/backend/support.rs` line 12 sets the tests' `DEFAULT_MODEL` constant. It becomes `jev-1.13.0`, because every test that reads it builds a default request. One test needs the alias on purpose. `prune_refuses_the_alias_and_keeps_the_upgrade` in `tests/backend/default_cache.rs` builds its planted request and its echoed reply from that constant, and its rows 1, 2 and 7 prune by `jev-latest`. That test names the literal `jev-latest` for its planted request and its echoed reply, so those rows still plant a request that asked for the alias.
- Files that name `jev-latest` on purpose keep it: the alias tests in `cache prune`, `check`'s "an alias answering as a version" sentence, and test plans that name the model.
- `site/` is the marketing lead's. The ticket files `sdlc/issues/2026-09-26-site-recordings-rekey-for-the-default-model-pin.md` for the marketing lead. It gives the exact `rekey-model jev-latest jev-1.13.0 site/recordings site/examples/...` command and says that the site build replays against the old key until that commit lands. The Pages workflow runs only by hand, so the gap breaks no deploy that nobody starts.

The re-keyed entries say what happened to them. `recording.md` gains, under "An entry": "Ticket 0159 re-keyed the repository's recordings that asked for `jev-latest` and were answered by `jev-1.13.0`. `sdlc/scripts/rekey-model` re-keyed them to `jev-1.13.0`, the version every one of their replies named, and left each reply unchanged." The ticket's record lists every re-keyed folder and the count of entries in each.

The script stays until the site issue closes. A Quick Fix then removes it, its fixture and its lint line.

### Every `jev-latest` in `crates/`, `spec/` and `demos/`

`git grep -n jev-latest -- crates spec demos` on this branch, merged with `origin/main`, finds each hit below. Each one moves or stays for the reason given. Paths under `crates/thinkthen` drop that prefix.

Product code names the version once, at `src/core/adapters/systemone.rs:35`. The unit tests under `src` read `DEFAULT_MODEL`, so no other line there names `jev-1.13.0`. The tests that drive the binary and the pages under `spec/` pin the literal `jev-1.13.0`, so plant (a) turns them red.

These unit tests in product code move to the default constant.

- `src/core/backend.rs:261` asserts the model that `Backend::resolve` returns when nothing names one.
- `src/core/question_file/resolve/tests.rs:205` asserts the model that `resolve.rs:321` takes from `DEFAULT_MODEL` when no source names one.
- `src/core/plan_document.rs` lines 137, 156 and 158 all move. Line 137 builds `plan()` with `DEFAULT_MODEL`, the constant line 150 already passes to the backend. Lines 156 and 158 build their expected text with `format!` from the same constant. The test pins the order of the plan document's fields. It does not pin the default.
- The conformance runners named above move too: `src/cli/conformance_tests.rs` lines 147 and 181, `runner.rs` lines 77 and 177, `command.rs` line 405, and `profile_cases.rs` line 52.

These pins in tests that drive the binary, and in `spec/`, move to `jev-1.13.0`.

- `tests/backend/support.rs:12` and `tests/decide_edge.rs:11` are the two test constants for the default. The dry-run pins at `decide_edge.rs` lines 82, 84 and 112 move with them.
- `tests/choose_and_score_edge.rs` lines 60, 63, 80 and 83 pin default dry-run plans.
- `tests/backend/check.rs` lines 89 and 374 pin `model sent jev-latest` in the report's opening lines.
- `tests/backend/streaming.rs` lines 394 and 396 pin a default dry-run plan.
- `tests/status.rs` lines 42 and 53 pin `model jev-latest` beside `model_source built_in`, in the JSON and the human form.
- `tests/backend/profile.rs:77` builds the default request body to set a byte limit. It passes today only because `jev-latest` and `jev-1.13.0` are both 10 bytes long. It reads `support::DEFAULT_MODEL` in place of the literal.
- `spec/decide.md` lines 24, 44, 72 and 122 pin default plans. Line 72 shows that the environment's model is ignored. Line 122 is a default `--jsonl` plan.
- `spec/check.md:16` pins `model sent`.

The script re-keys the 131 entries under `demos/*/recording/` and the 40 under `tests/fixtures/recognize-225/`. No demo page names the alias.

These hits stay, because each is a reply that a fake backend gives, and a stored reply keeps its model.

- `tests/backend/check.rs` lines 21 to 39, 146 and 361.
- `src/cli/failure/tests.rs:199`.
- `src/core/adapters/systemone/response.rs` lines 325, 435, 455, 494, 498, 515 and 530.
- `src/core/adapters/systemone/response_distribution_tests.rs` lines 13, 25 and 37.
- `src/core/adapters/systemone/response_partial_tests.rs` lines 29, 44 and 96 to 116.
- `src/engine/facade_tests.rs` lines 387, 396 and 475, and `src/engine/facade_tests/contract_tests.rs` lines 121 and 170.
- `src/engine/width_tests.rs:419`.
- `tests/public_batches.rs:25`, `tests/public_controls.rs:23` and `tests/public_members.rs:215`.

These hits stay, because each test names its model and never reads the default.

- `src/core/adapters/systemone.rs` lines 169, 171 and 186 feed names to the model-name diagnostic rule. Lines 213 and 223 build the adapter's test plans.
- `src/core/adapters/systemone/request.rs:453` pins the adapter's wire shape. `response_partial_tests.rs:9` builds a plan.
- `src/core/batch/tests.rs` lines 63 and 390, `src/core/batch/tests/refusals.rs:76` and `src/core/plan.rs:80` build backends and plans.
- `src/core/question_set/tests.rs:48` plants a `model` member that a question set refuses. Any name would do.
- `src/engine/deadline_tests.rs` lines 334 and 338, `src/engine/facade_tests.rs` lines 35, 87, 121 and 130, `src/engine/request.rs` lines 218 and 222, `src/engine/width_tests.rs` lines 382 and 398, and `src/cli/failure/tests.rs:206` build backends and plans.
- `src/core/recording.rs` lines 250, 260, 262, 273 and 316 hold the `decide-urgent` exchange, which keeps `jev-latest` with the adapter fixture.

These hits stay, because the command types `--model jev-latest` on purpose.

- `tests/backend/annotate.rs` lines 317 and 381 to 383.
- `tests/backend/annotate/scheduling.rs:311`.
- The alias rows of `tests/backend/default_cache.rs` at lines 275, 276, 281 and 336.

The builder runs the same `git grep` again before review. A hit that no list above names stops the build until the ticket names it.

The build's `git grep` on 2026-09-26, merged with `origin/main` `19ca8302`, found six hits that the lists above miss or place wrongly. The builder names them here, and each follows the rule of its list.

- `tests/backend/cache_partial.rs:160` stays. It is a reply a fake backend gives.
- `tests/backend/check.rs` lines 21 to 39 and 146 move to `jev-1.13.0`, and so do `check.md`'s four `reply` lines. The conformance backend's arms echo the model the request sent (`conformance/backend/src/arms.rs`), so these replies follow the default. Line 361 stays, because it is a canned reply.
- `src/core/adapters/systemone/request.rs:453` moves to the default constant. Its plan resolves the model through a question file, so it reads the default.
- `src/engine/facade_tests.rs:35` moves to the default constant. The facade's contract tests replay the request bodies of `conformance/cases.json` through it.
- `tests/find_edge.rs` replays `probes/find-0040/recording`, so a rung does replay that probe. The test names `--model jev-latest` on both runs, as each `probes/0N/job.sh` does, and the probe keeps its bytes.
- `tests/backend/relate/ceiling.rs:81` pins the digest of a default `relate` dry-run request. It takes the new digest.

### `audit --write` records the model

When `audit --write` writes a bar into a single question file, it also writes the file's `model` member with the one model that every graded line names in `meta.model`. It writes `model` by the same byte rule it uses for `threshold`. It changes a present member's value bytes, or it appends an absent member. Every results line audit reads counts, including a line whose answer failed and a line the key does not label, because each line names the model that answered its request. It writes no model when those lines name more than one model, and it prints `thinkthen: audit: kept the model for TARGET; the results name more than one model` beside the bar line. It writes no model when any line lacks `meta.model`, and it prints `thinkthen: audit: kept the model for TARGET; a result names no model`. It writes no model when it writes no bar. When the file already names a model, such as a typed `jev-latest`, and the lines name one version, audit replaces the file's value with that version, because the bar was tuned on the version and not on the alias. A question set holds no model (`annotate.md` line 32), so a set gets no model member.

`audit.md`, "Writing the bar", states the rule and the new line. `demos/41-tune-a-question-file` shows the written member and adds one sentence: "A file tuned by `audit --write` names the model its bar was tuned on. Rerun the labeled set, and tune again, whenever that model changes." That is ruling 2's how-to sentence.

### The two-version message

The command's message tells the user how to keep one version. It names no version to keep, because the command cannot tell which version the fresh answer comes from.

- When both names are safe to print: ``the replies for one record named model versions `A` and `B`; a cache or recording folder may hold answers from the other version, so rerun with --no-cache or prune it with `thinkthen cache prune DIR --answered-by-other-than VERSION`, naming the version a --no-cache run returns``.
- When a name is not safe to print: `the replies for one record named different model versions; a cache or recording folder may hold answers from the other version, so rerun with --no-cache or prune it with thinkthen cache prune DIR --answered-by-other-than VERSION, naming the version a --no-cache run returns`.

`DIR` stays a literal word, because the command does not know which folder answered. The library message at `public/error.rs:220` becomes `the replies for one call named different model versions; a cache may hold answers from the other version, so turn the cache off or prune it with thinkthen cache prune DIR --answered-by-other-than VERSION, naming the version a call with the cache off returns`. `result.md` line 154 says the same. The refusal itself does not change. The mixed-model cache fix waits, as "Deferred gaps" says.

## Decisions

Each is the ticket author's call unless marked. Ian can overturn any of them.

1. **The pin is `jev-1.13.0`.** Ian approved a pin. Every recorded reply and every measurement names this version. ADR 0054 item 5 asks for a comparison with the simplest alternative, which is keeping the alias. The alias and the pin both resolve to `jev-1.13.0` today, so they cost the same and answer the same. The recording scan in "Which version, and why" is that comparison. A comparison against another model is a deferred gap.
2. **The fixtures are re-keyed, not re-recorded.** Their replies came from `jev-1.13.0`, so the re-keyed request names the model that answered. A re-record would cost paid calls and would move probabilities on every demo page. The record lists every re-keyed folder.
3. **The probes' recordings keep their bytes, and each `probes/0N/job.sh` gains `--model jev-latest`.** A probe is a measurement of what was sent, so its replay sends the recorded request.
4. **`audit --write` records the model with the bar, for single files only.** It carries ruling 8's "the default the tool writes into a new question file". It replaces a `model` the user typed, such as `jev-latest`, with the version the results name. A set's grammar holds no model, and adding one is deferred.
5. **The two-version message names the cache.** Experiment 273's run shows the cache as the usual cause.
6. **No new setting and no new flag.**

## Edge cases

| Input | Expected |
| --- | --- |
| `decide "Q" --dry-run` with no model anywhere | The request carries `"model":"jev-1.13.0"` |
| `--model jev-latest` | The request carries `jev-latest`, as today |
| A question file with `"model":"jev-latest"` | The request carries `jev-latest` |
| A configuration file `model` | It outranks the default, as today |
| `thinkthen status` | `model jev-1.13.0`, with `model_source` as today |
| `--help` | `[default: jev-1.13.0]` |
| A library engine with no model | Its requests carry `jev-1.13.0` |
| `audit --write` over lines that all name `jev-1.13.0` | The file gains or keeps `"model":"jev-1.13.0"` beside the new bar |
| `audit --write` over lines naming two models | The bar is written, the model is kept, and the "more than one model" line prints |
| `audit --write` over a file whose `model` is `jev-latest` and lines that name `jev-1.13.0` | The member becomes `jev-1.13.0` |
| `audit --write` over lines where one lacks `meta.model` | The bar is written, the model is kept, and the "names no model" line prints |
| `audit --write` over lines where one answer failed and one is unlabeled, all naming `jev-1.13.0` | Every line counts, and the member becomes `jev-1.13.0` |
| `audit --write` that writes no bar | The model is not touched |
| `audit --write` over a question set | No model member is written |
| `annotate` with a `--cache` folder whose unchanged groups hold `fake-1` replies and an edited group answered live as `fake-2` | Exit 4 and the new two-version sentence |

## Proof

Every test drives the compiled binary. The backend rows use the in-process loopback in `tests/backend/harness`.

| Test | What it proves | Planted faults that turn it red |
| --- | --- | --- |
| The existing literal pins that "Every `jev-latest`" moves to `jev-1.13.0`: `spec/decide.md` lines 24, 44, 72 and 122, `spec/check.md:16`, `tests/decide_edge.rs`, `tests/choose_and_score_edge.rs`, `tests/backend/check.rs` lines 89 and 374, `tests/backend/streaming.rs` lines 394 to 396, `tests/status.rs` lines 42 and 53, every test that reads `tests/backend/support.rs:12`, including `tests/backend/profile.rs:77`, and `specification/fixtures/check/requests.jsonl` | The default request, the default plan, the `check` report and `status` name the pin | (a) `DEFAULT_MODEL` back to `jev-latest`: every one of these pins fails. The unit tests in `src` that read `DEFAULT_MODEL` stay green by design. They pin resolution and field order |
| `rekey-model --self-test` in `lint` | The five script rules against a digest the Rust recorder produced | (h) Re-serialize the JSON: the second entry's file differs from the Rust original. (i) Skip the old-name check: the misnamed-entry plant passes. (m) Skip the Git check: the untracked or edited plant moves. (j) Write before checking all: a planted refusal leaves a written file. (k) Follow a symlink or overwrite a target: its plant passes |
| Every replay rung (`test`, `spec`, `surfaces`) after the re-key | Every re-keyed recording replays, and every page and row prints its new digest | (b) Skip one folder in the re-key: that folder's replay misses at exit 5 |
| `audit_writes_the_model_it_tuned_on`, new in `tests/audit_model.rs` | The seven `audit --write` rows of the edge table, each pinning the file's bytes and standard error | (c) Write the model when lines disagree: the two-model row fails. (d) Write the requested alias in place of `meta.model`: the alias row fails. (e) Write the model with no bar: the no-bar row fails. (l) Skip lines without `meta.model`: the no-model row writes a model |
| `a_cache_that_mixes_versions_stops_the_record`, new in `tests/backend/annotate` | The last edge row. A cache folder is bound to its endpoint URL, port included, so a second loopback would get a new port and the second run would stop at exit 5. One listener therefore stays up across both runs. The test starts one `Listener::serving` over a list of `Canned::ok` replies: one `fake-1` reply for each group of the first run, then one `fake-2` reply. `serving` answers one scripted reply per connection, in order. The first run fills the `--cache` folder with `fake-1` answers. The test edits one question. The second run replays the unchanged groups from the cache and sends only the edited group, which gets `fake-2`. Exit 4, the whole new sentence, and no row | (f) Skip the version check for cached chunks: the row prints with exit 0. (g) Restore the old message: the sentence differs |

The existing two-version tests in `tests/backend/annotate.rs` and `annotate/splitting.rs` update to the new sentence.

The four questions:

- **What behavior does it protect?** The pinned default on every surface, a tuned file that records its model, and a two-version stop that names the cache.
- **What credible regression fails it?** A default that slips back to the alias, a digest left stale by the re-key, a re-key that rewrites more than the model value, an audit write that records the alias or a mixed model, and a cache that mixes versions without a stop.
- **Why does no existing test catch it?** The literal pins in the first row already exist and pin `jev-latest`, so they move with the default and need no new test. Nothing tests audit's model, nothing tests the re-key script, and the two-version tests use live chunks only.
- **Does it need a test-only hook?** No. The loopback, the cache folder and the question file are the real boundaries.

## Budgets

Nonblank lines, measured with `grep -c .`.

- `core/adapters/systemone.rs`: 1 changed line.
- `cli/failure.rs` and `public/error.rs`: at most 6 net together.
- `cli/audit/write.rs` and the rest of `cli/audit`: at most 45 net.
- `core/plan_document.rs`: at most 4 net, for the `format!` of the expected text.
- `tests/audit_model.rs`: at most 120, new. The new annotate test: at most 70.
- `sdlc/scripts/rekey-model`: at most 160 with its self-test, new. Its fixture: two entries. `sdlc/scripts/lint` and `sdlc/scripts/README.md`: at most 6 net together.
- Pages under `specification/`, `demos/`, `CHANGELOG.md`: at most 30 net, beside the replaced model names and digests.
- Renamed recordings change only the model value and their file names.
- `sdlc/ratchet.json` moves to the measured total, at most 265 above main. The ratchet counts `.rs` files under `crates` and `conformance`. The Rust budgets above add up to 246 lines, and about 15 more lines fall outside them: the `DEFAULT_MODEL` imports in `resolve/tests.rs` and the four conformance runner files, and the `mod` line for the new annotate test. The Python script falls outside it. The commit says what grew.
- No dependency. The `surfaces` rung runs.

## Stop rules

1. Stop before crossing any budget by more than a tenth, or before adding a dependency.
2. Stop if a re-keyed entry's reply names a model other than `jev-1.13.0`. Report the folder.
3. Stop if a probe's recording bytes or committed rows change after its `job.sh` names `--model jev-latest`.
4. Stop if any change needs a file under `site/`. File it for the marketing lead.
5. Stop if the build needs a live call. None is authorized, and the `demos/15` entries already show the backend accepts the pinned name. Never run `sdlc/scripts/live`.
6. Stop if any plant stays green.
7. Stop if another ticket is building at the same time. This ticket re-keys files that every ticket's tests read.

## Build order

It builds after tickets 0150 and 0158 land and before ticket 0146 builds, and it builds alone. The order follows the files.

- Ticket 0150 opens `conformance/cases.json`, `cli/conformance_tests.rs` and the library conformance runners. This ticket rewrites the request bodies in that file and the runner's model.
- Ticket 0158 opens `specification/recording.md` and `crates/thinkthen/tests/backend/main.rs`. This ticket adds the re-key sentence to `recording.md`, and adds a test module beside 0158's.
- Ticket 0146 opens `cli/failure.rs`, `cli/audit/write.rs`, `specification/backends.md`, `question-file.md`, `settings.md`, `result.md`, `decide.md`, `filter.md`, `rank.md`, `demos` and `crates/thinkthen/tests`. This ticket edits each of them. Landing first lets 0146 rebase onto re-keyed fixtures once, instead of this ticket re-keying 0146's new recordings.

S1 live run 1 may run before or after this ticket. Its record notes that the alias resolved to `jev-1.13.0`.

## Scope and exclusions

Excluded: the mixed-model cache fix itself, a run-wide model check, a refresh mode, a model member in a question set, and `site/`.

## Routing

Builder: Claude (Opus subagent) in the lane the coordinator names. Reviewer: a fresh read-only Claude session for the design and for the code.

## Complexity

Contract 2; state and timing 1; reach 3; proof 1; cost of error 2; total 9. Final level: 3. The reach is every recording a gate replays. The re-key script and the replay rungs guard it.

## Deferred gaps

- The mixed-model cache fix: treat a cached group answered by another version as a miss, or allow mixed versions and list them. It waits on this pin by the backlog's placement, and the pin makes the fault appear only when a user types an alias. `sdlc/issues/2026-09-26-a-mixed-model-cache-fails-every-annotate-record.md` keeps it.
- Architect review 05, finding 2.2: a warning or refusal when one run's rows name more than one model version. The pin removes the default path. The run loop belongs to ticket 0146, so the check waits for it.
- Architect review 08, item 1: a freshness rule for an alias in the cache key, and a mode that sends again and replaces. Both matter only for a typed alias after this ticket.
- A `model` member in a question set, so `audit --write` can record a set's model.
- The site's recordings, which the marketing lead re-keys.
- A measured comparison of `jev-1.13.0` against another model, under ADR 0054 item 5. No other model answers today, so the recording scan compares only the alias with the pin.

## What Ian can overturn

- Decision 1: the pin is `jev-1.13.0`.
- Decision 2: the fixtures are re-keyed rather than re-recorded.
- Decision 3: the probes keep their recordings, and each `job.sh` names `--model jev-latest`.
- Decision 4: `audit --write` records the model.
- The build order: after 0150 and 0158, and before 0146.

## Closes

Ruling 8 and the pin half of ruling 2 in `sdlc/issues/closed/2026-09-22-what-the-vendors-founder-said-about-where-the-model-goes.md`, which then has no open item. Question 1 of the 0.1 backlog. The message half of `sdlc/issues/2026-09-26-a-mixed-model-cache-fails-every-annotate-record.md`. Finding 2.1 of `sdlc/issues/2026-09-26-architect-review-05-answer-contract.md` for single files, and the default half of item 1 in `sdlc/issues/2026-09-26-architect-review-08-cache.md`.

## Evidence

- Starts from: Ian's approval of 2026-09-26. Rulings 2 and 8 of the 2026-09-22 vendor issue. Local experiment 273, report 03 finding 2-5, report 05 findings 2.1 and 2.2, report 08 item 1 and report 09 item 8, as filed in the issues above. The recording scan at `origin/main` `40eae81e`. Experiments 212 and 259 as ADR 0010's amendment records them. `systemone.rs:35`, `cli/failure.rs:344-351`, `public/error.rs:220` and `audit.md`, "Writing the bar".
- Keeps: The resolution order of `--model`, the question file, the configuration file and the default. `jev-latest` as a name a user may type. Every answer, exit code and row except the model name and request digests. Every reply byte in every re-keyed entry. The two-version refusal. The probes' recording bytes and measured rows, replayed through `--model jev-latest` in each `job.sh`. The adapter fixtures under `specification/fixtures/systemone` and the three copies of their `decide-urgent` exchange.
- Changes: The default model. Re-keyed recordings and digests in queue-owned folders. `audit --write` records the model beside a written bar in a single file. The two-version message. Pages that name the default.
- Proof: Updated literal pins, the replay rungs over every re-keyed folder, the re-key script's self-test in `lint`, two new outside-in tests, thirteen plants in all, and the `install`, `lint`, `test`, `spec` and `surfaces` rungs.
- Defers: The mixed-model cache fix, a run-wide model check, a cache freshness rule and refresh mode, a set's model member, the site's re-key, and a comparison against another model.
