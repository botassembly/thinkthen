---
flow: build
priority: 159
opens: crates/thinkthen/src/core/adapters/systemone.rs crates/thinkthen/src/cli/failure.rs crates/thinkthen/src/public/error.rs crates/thinkthen/src/cli/audit/write.rs crates/thinkthen/src/cli/audit crates/thinkthen/tests crates/thinkthen/src/core/recording.rs specification/backends.md specification/settings.md specification/question-file.md specification/check.md specification/recognize.md specification/recording.md specification/result.md specification/audit.md specification/decide.md specification/choose.md specification/score.md specification/rank.md specification/filter.md specification/find.md specification/annotate.md specification/relate.md specification/channels.md specification/fixtures spec demos transforms/rows probes conformance/cases.json databases/postgresql libraries CHANGELOG.md sdlc/scripts/rekey-model sdlc/issues sdlc/planning/backlog-0-1-2026-09-26.md sdlc/ratchet.json sdlc/records sdlc/tickets
---

# 0159: The default model is a pinned version

Status: ready for review. Written 2026-09-26 by Claude, the queue owner's planner, after Ian approved pinning the default model. A fresh read-only review must accept it before it builds. Owner: Claude.

Review route: a fresh read-only Claude session reviews this design and the final diff. Codex does not review this ticket unless Ian routes it.

## Outcome and authority

A user who names no model gets answers from `jev-1.13.0`, and every request, cache key and recording says so. The vendor can move its `jev-latest` alias without moving a tuned cut or a cached answer. `jev-latest` still works when the user types it. A cut that `audit --write` tunes also records the model it was tuned on, so a later default change cannot move it silently. A run that stops because one record's replies came from two model versions says that a cache or recording may hold the older version, and it no longer blames the backend alone.

Ian approved the pin on 2026-09-26. That answers question 1 of `sdlc/planning/backlog-0-1-2026-09-26.md` with option A. It carries rulings 2 and 8 of `sdlc/issues/2026-09-22-what-the-vendors-founder-said-about-where-the-model-goes.md`. Ruling 8 reads: "the default the tool writes into a new question file and into every cache key is the current pinned version (today `jev-1.13.0`), and `jev-latest` is accepted only when the user types it."

## Which version, and why

The pin is `jev-1.13.0`. The repository's own evidence names no other model that answered.

- Every live reply the repository keeps names `jev-1.13.0`. A scan of every tracked `thinkthen.recording/1` entry on `origin/main` `ebd28382` found 2,465 live entries. 2,463 asked for `jev-latest` and were answered by `jev-1.13.0`. Two, in `demos/15-find-the-line/recording/`, asked for `jev-1.13.0` by name and were answered by it. The only other name is the loopback's `local-1`, in four entries.
- So the backend accepts the pinned name as a request model. The two `demos/15` entries prove it with no new call.
- Every measurement behind a ruling names the same model. Experiment 212 (ADR 0010, amendment of 2026-09-25) saw `jev-1.13.0` answer all 100 messages twice. Experiment 259 found every repeated benchmark digest answered by `jev-1.13.0`. Local experiment 273, report 09, found every Beatles Bench recording answered by it.
- `specification/result.md` already shows `meta.model` as `jev-1.13.0` in every example.
- The vendor's founder named `jev-1.13.0` as the version that may get temporary long-term support (issue of 2026-09-22, point 2).
- The only other version names in the repository, `jev-1.14.0`, `jev-2.0.0`, `jev-1.2` and `jev-1.3`, come from fake backends in tests and reviews.

Pinning to a version that no recorded answer came from would re-key the fixtures onto a model the gates never measured.

## What happens today

Read from `origin/main` `ebd28382`.

- `crates/thinkthen/src/core/adapters/systemone.rs:35` sets `DEFAULT_MODEL` to `jev-latest`. The command, `status`, `check`, every library through `public/settings.rs:229`, and the SQL extensions read it.
- `specification/settings.md` line 52 gives the default as `jev-latest` with "No reason recorded for the alias over a pinned version", and line 78 lists it under "Defaults with no recorded reason". `backends.md` line 57, `question-file.md` line 91, `check.md` line 16 and the "Backend options" row of seven verb pages name `jev-latest` as the default.
- A recording's digest covers the request body, and the body names the model (`recording.md`, "An entry"). Changing the default changes every default digest, so every replay of a default request misses.
- Recordings replayed by a gate that ask for `jev-latest`: 40 entries under `crates/thinkthen`, 131 under `demos/`, 80 under `transforms/rows`, the default request bodies in `conformance/cases.json`, and 879 under `probes/01` to `probes/09`, which the `spec` rung replays through `probes/replay-check.sh`. `site/recordings` holds 193 and `site/examples` 54. The marketing lead owns `site/`, and its hand-run build replays them (`.github/workflows/pages.yml`).
- `audit --write` writes only a threshold (`specification/audit.md`, "Writing the bar"). The question digest leaves the model out (`question-file.md` canonical rule 1). A file tuned under one model runs under the next with no sign. Architect review 05, finding 2.1, and review 08, item 1, found it.
- A record whose replies name two model versions stops at exit 4 with "the backend returned different model versions for one record; pin --model and rerun with --record or --cache" (`cli/failure.rs:344-351`, `public/error.rs:219`, `result.md` line 154). Local experiment 273, report 03, finding 2-5, showed that a cache causes it. After the alias moves, unchanged groups replay from the cache under the old version and an edited group answers live under the new one. The advice to use a cache is then wrong (`sdlc/issues/2026-09-26-a-mixed-model-cache-fails-every-annotate-record.md`).

## Design

### The pin

`DEFAULT_MODEL` becomes `jev-1.13.0`. Nothing else in the resolution order changes: `--model`, then the question file's `model`, then the configuration file's `model`, then the default. `jev-latest` stays a valid name that a user can type or write.

The pages change with it: `settings.md` lines 18, 52 and 78 (the default leaves the "no recorded reason" list and cites this ticket), `backends.md` line 57, `question-file.md` line 91 and its two examples, `check.md` line 16 and its examples, `recognize.md` line 56, `recording.md`'s example entry, and the "Backend options" row of `decide.md`, `choose.md`, `score.md`, `rank.md`, `filter.md`, `find.md` and `annotate.md`. `backends.md` gains one sentence: "The default is a pinned version, so a vendor's move of its alias moves no default answer. A later release that changes the default says so in the changelog, and every default cache entry then misses once." `CHANGELOG.md` gets one line under 0.1.

### Re-keying the recordings

The replies behind the queue owner's recordings came from `jev-1.13.0`. A new script, `sdlc/scripts/rekey-model FROM TO FOLDER...`, rewrites each entry whose request asked for FROM and whose reply names TO. It replaces the request's model value and nothing else, recomputes the digest from the adapter name, the URL and the new request bytes, as `recording.md` defines it, and renames the file. It refuses an entry whose reply names another model, and it leaves every other file alone. It prints each old digest beside its new one.

- `crates/thinkthen/tests`, `demos/`, `transforms/rows` and `conformance/cases.json` are re-keyed. Every old digest that a page, a saved row or a test prints is replaced by its new digest in the same commit. `conformance/cases.json` stores request strings, not entries, so its default request bodies change by the same rule.
- `probes/01` to `probes/09` keep their bytes. Each measured what the tool sent then. Each `job.sh` names `--model jev-latest`, so its replay sends the recorded request. The committed rows do not change.
- `probes/annotate-0015` and `probes/find-0040` keep their bytes. No rung replays them against the command.
- Files that name `jev-latest` on purpose keep it: the alias tests in `cache prune`, `check`'s "an alias answering as a version" sentence, and fixtures whose plan names the model.
- `site/` is the marketing lead's. The ticket files `sdlc/issues/2026-09-26-site-recordings-rekey-for-the-default-model-pin.md` for the marketing lead. It gives the exact `rekey-model jev-latest jev-1.13.0 site/recordings site/examples/...` command and says that the site build replays against the old key until that commit lands. The Pages workflow runs only by hand, so the gap breaks no deploy that nobody starts.

The script stays until the site issue closes. A Quick Fix then removes it.

### `audit --write` records the model

When `audit --write` writes a bar into a single question file, it also writes the file's `model` member with the one model that every graded line names in `meta.model`. It writes `model` by the same byte rule it uses for `threshold`. It changes a present member's value bytes, or it appends an absent member. It writes no model when the graded lines name more than one model, and it prints `thinkthen: audit: kept the model for TARGET; the results name more than one model` beside the bar line. It writes no model when it writes no bar. A question set holds no model (`annotate.md` line 32), so a set gets no model member.

`audit.md`, "Writing the bar", states the rule and the new line. `demos/41-tune-a-question-file` shows the written member and adds one sentence: "A file tuned by `audit --write` names the model its bar was tuned on. Rerun the labeled set, and tune again, whenever that model changes." That is ruling 2's how-to sentence.

### The two-version message

The command's message becomes:

- With both names: ``the replies for one record named model versions `A` and `B`; a cache or recording folder may hold answers from the other version, so rerun with --no-cache or prune that folder with --answered-by-other-than``
- Without names: `the replies for one record named different model versions; a cache or recording folder may hold answers from the other version, so rerun with --no-cache or prune that folder with --answered-by-other-than`

The library message becomes `the replies for one call named different model versions; a cache may hold answers from the other version, so turn the cache off or prune it`. `result.md` line 154 says the same. The refusal itself does not change. The mixed-model cache fix waits, as "Deferred gaps" says.

## Decisions

Each is the ticket author's call unless marked. Ian can overturn any of them.

1. **The pin is `jev-1.13.0`.** Ian approved a pin. Every recorded reply and every measurement names this version.
2. **The fixtures are re-keyed, not re-recorded.** Their replies came from `jev-1.13.0`, so the re-keyed request names the model that answered. A re-record would cost paid calls and would move probabilities on every demo page. The record lists every re-keyed folder.
3. **The probes keep their bytes and name the alias.** A probe is a measurement of what was sent.
4. **`audit --write` records the model with the bar, for single files only.** It carries ruling 8's "the default the tool writes into a new question file". A set's grammar holds no model, and adding one is deferred.
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
| `audit --write` that writes no bar | The model is not touched |
| `audit --write` over a question set | No model member is written |
| `annotate` with a `--cache` folder whose unchanged groups hold `fake-1` replies and an edited group answered live as `fake-2` | Exit 4 and the new two-version sentence |

## Proof

Every test drives the compiled binary. The backend rows use the in-process loopback in `tests/backend/harness`.

| Test | What it proves | Planted faults that turn it red |
| --- | --- | --- |
| The existing dry-run pins in `spec/decide.md`, the `model sent` pin in `spec/check.md`, and the `status` and `--help` pins,, updated to the literal `jev-1.13.0` | The default request, status line and help name the pin | (a) `DEFAULT_MODEL` back to `jev-latest`: every updated pin fails |
| Every replay rung (`test`, `spec`, `surfaces`) after the re-key | Every re-keyed recording replays, and every page and row prints its new digest | (b) Skip one folder in the re-key: that folder's replay misses at exit 5 |
| `audit_writes_the_model_it_tuned_on`, new in `tests/audit_model.rs` | The five `audit --write` rows of the edge table, each pinning the file's bytes and standard error | (c) Write the model when lines disagree: the two-model row fails. (d) Write the requested alias in place of `meta.model`: the alias row fails. (e) Write the model with no bar: the no-bar row fails |
| `a_cache_that_mixes_versions_stops_the_record`, new in `tests/backend/annotate` | The last edge row: a `--cache` folder recorded from a loopback answering `fake-1`, one question edited, then a loopback answering `fake-2`. Exit 4, the whole new sentence, and no row | (f) Skip the version check for cached chunks: the row prints with exit 0. (g) Restore the old message: the sentence differs |

The existing two-version tests in `tests/backend/annotate.rs` and `annotate/splitting.rs` update to the new sentence.

The four questions:

- **What behavior does it protect?** The pinned default on every surface, a tuned file that records its model, and a two-version stop that names the cache.
- **What credible regression fails it?** A default that slips back to the alias, a digest left stale by the re-key, an audit write that records the alias or a mixed model, and a cache that mixes versions without a stop.
- **Why does no existing test catch it?** No test pins the default by its literal name, nothing tests audit's model, and the two-version tests use live chunks only.
- **Does it need a test-only hook?** No. The loopback, the cache folder and the question file are the real boundaries.

## Budgets

Nonblank lines, measured with `grep -c .`.

- `core/adapters/systemone.rs`: 1 changed line.
- `cli/failure.rs` and `public/error.rs`: at most 6 net together.
- `cli/audit/write.rs` and the rest of `cli/audit`: at most 45 net.
- `tests/audit_model.rs`: at most 120, new. The new annotate test: at most 70.
- `sdlc/scripts/rekey-model`: at most 90, new.
- Pages under `specification/`, `demos/`, `CHANGELOG.md`: at most 30 net, beside the replaced model names and digests.
- Renamed recordings change only the model value and their file names.
- `sdlc/ratchet.json` moves to the measured total, at most 70 above main. The commit says what grew.
- No dependency. The `surfaces` rung runs.

## Stop rules

1. Stop before crossing any budget by more than a tenth, or before adding a dependency.
2. Stop if a re-keyed entry's reply names a model other than `jev-1.13.0`. Report the folder.
3. Stop if a probe's committed rows change.
4. Stop if any change needs a file under `site/`. File it for the marketing lead.
5. Stop if the build needs a live call. None is authorized, and the `demos/15` entries already show the backend accepts the pinned name. Never run `sdlc/scripts/live`.
6. Stop if any plant stays green.
7. Stop if another ticket is building at the same time. This ticket re-keys files that every ticket's tests read.

## Build order

It builds after tickets 0150 and 0158 land and before ticket 0146 builds. It builds alone. The planner recommends it before "S1 live run 1", so that the paid recordings 0146 needs are made under the pinned name. Every ticket written after it records under `jev-1.13.0`.

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

## What Ian can overturn

- Decision 1: the pin is `jev-1.13.0`.
- Decision 2: the fixtures are re-keyed rather than re-recorded.
- Decision 3: the probes keep the alias.
- Decision 4: `audit --write` records the model.
- The build order: before 0146 and before S1 live run 1.

## Closes

Ruling 8 and the pin half of ruling 2 in `sdlc/issues/2026-09-22-what-the-vendors-founder-said-about-where-the-model-goes.md`, which then has no open item. Question 1 of the 0.1 backlog. The message half of `sdlc/issues/2026-09-26-a-mixed-model-cache-fails-every-annotate-record.md`. Finding 2.1 of `sdlc/issues/2026-09-26-architect-review-05-answer-contract.md` for single files, and the default half of item 1 in `sdlc/issues/2026-09-26-architect-review-08-cache.md`.

## Evidence

- Starts from: Ian's approval of 2026-09-26. Rulings 2 and 8 of the 2026-09-22 vendor issue. Local experiment 273, report 03 finding 2-5, report 05 findings 2.1 and 2.2, report 08 item 1 and report 09 item 8, as filed in the issues above. The recording scan at `origin/main` `ebd28382`. Experiments 212 and 259 as ADR 0010's amendment records them. `systemone.rs:35`, `cli/failure.rs:344-351`, `public/error.rs:219` and `audit.md`, "Writing the bar".
- Keeps: The resolution order of `--model`, the question file, the configuration file and the default. `jev-latest` as a name a user may type. Every answer, exit code and row except the model name and request digests. The two-version refusal. The probes' measured rows.
- Changes: The default model. Re-keyed recordings and digests in queue-owned folders. `audit --write` records the model beside a written bar in a single file. The two-version message. Pages that name the default.
- Proof: Updated literal pins, the replay rungs over every re-keyed folder, two new outside-in tests with seven plants, and the `install`, `lint`, `test`, `spec` and `surfaces` rungs.
- Defers: The mixed-model cache fix, a run-wide model check, a cache freshness rule and refresh mode, a set's model member, and the site's re-key.
