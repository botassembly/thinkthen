---
flow: build
priority: 124
opens: crates/thinkthen/src/core/recording.rs crates/thinkthen/src/engine crates/thinkthen/src/cli/cache.rs crates/thinkthen/src/cli/args/command.rs crates/thinkthen/src/cli/failure crates/thinkthen/src/cli/status.rs crates/thinkthen/src/public/error.rs crates/thinkthen/tests/backend/default_cache.rs crates/thinkthen/tests/backend/cache_identity.rs crates/thinkthen/tests/status.rs specification site/src/pages/reference.astro sdlc/ratchet.json sdlc/records sdlc/issues sdlc/tickets
---

# 0124: Fix the answer cache's three faults

Status: landed 2026-09-25. Code review accepted it with one gap, now deferred below. Record: `sdlc/records/0124-build-answer-cache-fixes.md`. Owner: Claude.

Review route: a fresh read-only Claude session reviews this design and the final diff. Codex does not review this ticket unless Ian routes it.

Round 1 review at `57b860c9` returned 1 high, 3 medium, and 3 low findings. This version answers all seven. The prune refusal now fires only on the alias, so an upgrade prune still works. The mismatch tests move into `cache_identity.rs`. The other changes are the ADR 0036 citation, the `core/recording.rs` budget, the files shared with 0126, a generic refusal phrase with `--replay` listed, and a privacy line in the cache help.

## Outcome and authority

Three cache faults stop hurting a user.

1. `thinkthen cache prune DIR --answered-by-other-than jev-latest` deletes every entry today. After this ticket, a MODEL that the requests asked for and no reply named deletes nothing and says why.
2. The default cache refuses a second backend address with a sentence about "the recording folder". After this ticket the refusal names the default cache, the address this run asked for, and the ways around it.
3. `status` prints `cache_target_bytes` beside the cache size. That reads as a limit the tool keeps. After this ticket the words say the number is the size `cache prune` trims to.

The ask is `sdlc/issues/closed/2026-09-25-the-answer-cache-three-fixes.md`, item 2 of section A in `sdlc/planning/issue-backlog-2026-09-25.md`. Pruning by the alias loses user data, so it comes first. Ian ruled on 2026-09-25 that no setting may do nothing, and that `cache_bytes` leaves the library, ticket 0084, and every binding. A later ticket carries that out. This ticket keeps the command's `cache_bytes` configuration field, because it sets prune's default target and so does something.

## Evidence

- Starts from: experiment 259 (`~/workspace/experiments/259-talk-claims`, script `03-cache-record-replay.sh`) reproduced all three faults offline at e70bddab with no key. On a 140-entry Beatles Bench recording, every request asks for `jev-latest` and every response names `jev-1.13.0`. `--answered-by-other-than jev-latest` and `--answered-by-other-than no-such-model` each removed all 140 entries at exit 0. A first run with no key bound the default cache to `http://127.0.0.1:9/v1`, and the next plain run stopped at exit 5 with the recording-folder sentence. A cache at five times its target stayed untouched through a run. The issue quotes each transcript. Tickets 0062 and 0065 and ADRs 0017 and 0033 rule the behavior this ticket keeps. ADR 0036 sets the precedent for renaming an output field inside an unreleased schema.
- Keeps: prune stays the only removal surface, and it never runs on its own. The selectors keep their union, their meaning, and their order. A MODEL that names a newer version still removes every older answer. Prune still validates every entry before it deletes one. A folder still binds on its first write-capable use, before the key is read, and a mismatch still exits 5 before any request. The marker keeps its schema, its digest, and its size bound. No marker byte reaches a diagnostic. The entry format and the configuration file stay as they are. The library's error message stays as it is.
- Changes: prune refuses, at exit 2 and before it deletes anything, a MODEL that some entry's request asked for and no entry's reply named. The backend mismatch refusal names the address this run resolved, and the default cache gets its own sentence with `--no-cache` and `THINKTHEN_CACHE`. `status` renames its two target lines and JSON fields to say "prune target". The cache help says the cache grows until the user prunes it and holds the judged text until then. The prune help, the specification, and the site reference match.
- Proof: one new outside-in table test for prune in `tests/backend/default_cache.rs`, one new outside-in table test for the mismatch in `tests/backend/cache_identity.rs` that absorbs the old `--cache` mismatch test, and updated exact-output tests in `cache_identity.rs`, `tests/status.rs`, and `engine/facade_tests/contract_tests.rs`. Each has a planted fault that turns it red. The Acceptance section lists them.
- Defers: a prune `--dry-run` and a guard for a MODEL typo; a `status` line that shows whether the cache is bound to the current address; binding a folder only when an entry is written; a per-address default cache; renaming the configuration field `cache_bytes`; the library's own mismatch wording; removing `EngineBuilder::cache_bytes`; the titles of ADR 0033, ADR 0017's "cap", and ticket 0062. The Deferred gaps section says why for each.

## Design

### 1. Prune refuses the alias

`engine/cache_prune.rs` already scans and validates every entry before it selects one. `Entry::inspected` in `core/recording.rs` already parses the whole entry, request included, and returns the digest and the reply's model. It now also returns the request's top-level `model` string, or none when the request carries no string there. A missing request model is not a new refusal. The scan keeps both models on each found entry.

After the scan and before any selection, `run_at` adds one check. Prune refuses when `--answered-by-other-than MODEL` is given, at least one entry's request asked for MODEL byte for byte, and no entry's reply named MODEL byte for byte. It returns the engine's existing `Error::Usage` with a fixed sentence. The command exits 2 and prints:

```text
thinkthen: --answered-by-other-than names the model the requests asked for, and no reply names it, so prune removed nothing; name the version a result's meta.model shows, not the alias passed to --model
```

That is the whole standard error line. Standard output stays empty. The sentence repeats no entry field and no value the user typed. The spec forbids a refusal that repeats an entry field, so the sentence cannot list the models it found. It points to `meta.model`, which every result already carries.

The upgrade case keeps working. The alias moves to `jev-1.14.0`, the cache still holds answers from `jev-1.13.0`, and the user prunes with `--answered-by-other-than jev-1.14.0`. No request asked for `jev-1.14.0`, so the check does not fire, and every older answer leaves.

A typo still removes every entry. No request asked for the typo, so it looks like the upgrade case. The help says so, and the Deferred gaps section pairs it with a `--dry-run`.

The check sits in the engine beside the scan. The command cannot see the entries without a second scan. The check holds the exclusive folder gate that prune already takes.

The help for the option changes from "Remove entries answered by any other model" to "Remove entries whose reply names another model. Give the version that answered, as a result's meta.model shows it, not the alias passed to --model. A name no reply carries removes every entry".

### 2. The backend mismatch refusal names the default cache and the address

The engine's `Error::RecordingBackendMismatch` gains two fields: the resolved endpoint URL of this run, and whether the folder is the platform default cache. The recorder's folder gate already knows `private_default` and holds the exchange. `Exchange` gains a one-line `url()` accessor in `core/recording.rs`, since its `url` field is private to the core. The command's `Failure` carries the same two fields through the existing conversion in `cli/failure/convert.rs`. `cli/failure/recording.rs` prints one of two sentences.

The platform default cache, when neither `THINKTHEN_CACHE`, `--cache`, `--record`, nor `--replay` named a folder:

```text
thinkthen: the default cache is bound to a backend address other than `URL`; go back to that address, use --no-cache, or set THINKTHEN_CACHE to another folder
```

Every folder the user named, by `THINKTHEN_CACHE`, `--cache DIR`, `--record DIR`, `--replay DIR`, or `--record` with `--replay`:

```text
thinkthen: the recording folder is bound to a backend address other than `URL`; restore its backend settings or choose another folder
```

`URL` is the resolved endpoint URL, such as `http://127.0.0.1:9/v1/systemone`. The existing stopped-run line still follows. The exit code stays 5.

The address this run resolved is safe to print. `specification/backends.md` refuses a base with user information, a query, or a fragment, "because the address is printed in a plan and kept in a recording". The bound address is not printed. The marker holds only its digest, and the spec forbids a marker byte in a diagnostic. The sentence names the address this run asked for. The user may not know that address came from `THINKTHEN_BASE_URL` or the configuration file.

`public/error.rs` matches the variant with `{ .. }` and keeps its message. `engine/facade_tests/contract_tests.rs` matches it the same way. The library's words wait for the ticket that removes `cache_bytes` from the library.

### 3. The words say "prune target", and the help says what the folder holds

The cache grows until the user runs `thinkthen cache prune DIR`. Ian ruled out automatic trimming. So the fix is the words.

- `status` prints `cache_prune_target_bytes` and `cache_prune_target_source` in place of `cache_target_bytes` and `cache_target_source`. `status --json` renames `cache.target_bytes` and `cache.target_source` to `cache.prune_target_bytes` and `cache.prune_target_source`. The schema stays `thinkthen.status/1`. ADR 0036 set this precedent when it renamed `meta.replayed` to `meta.cached` and kept `thinkthen.result/1` in an unreleased interface. No repository in the workspace reads the old names.
- `cache --help` says: "Inspect and maintain answer-cache folders without sending a request. The cache never trims itself. It grows until you run cache prune, and it holds the judged text until then."
- `cache prune --help` says "Remove selected entries, then the oldest entries until the folder fits the size target". `--max-size` says "Trim to this many allocated bytes. Without it, the configuration's cache_bytes applies, or 100000000".
- `specification/recording.md` says the configuration's `cache_bytes` is the target `cache prune` trims to, and that nothing trims the cache on its own. It names the two status fields. Its prune section states the alias refusal and the typo gap. Its binding paragraph states the two sentences.
- `specification/README.md` and `specification/roadmap.md` drop "bounded" before "cache". `site/src/pages/reference.astro` says the value is the size `cache prune` trims to and that the cache grows until then.

### Files shared with tickets 0123 and 0126

Ticket 0126 owns help text outside the cache commands. It may also edit these files, which this ticket touches:

- `cli/args/command.rs`: this ticket edits only the `Cache` variant doc, `CacheCommand`, and `PruneArguments`.
- `cli/failure/recording.rs`: this ticket edits only the backend mismatch arm.
- `public/error.rs`: this ticket changes one match pattern.
- `specification/recording.md`: this ticket edits the configuration, binding, status, and prune paragraphs.

The `status` help stays with 0126.

Ticket 0123 adds a `TokenLimit` error and edits relate help. It shares these files:

- `engine/error.rs`: 0123 adds a variant to `Error` and its arm in the exhaustive class match. This ticket adds two fields to `RecordingBackendMismatch` in the same enum and its arm.
- `cli/failure.rs`: 0123 adds a `Failure` variant and a message arm. This ticket adds fields to `Failure::RecordingBackendMismatch` and edits its arm in the recording-defect list.
- `cli/failure/convert.rs`: both tickets edit arms of `From<EngineError> for Failure`.
- `public/error.rs`: both tickets edit arms of the engine-error match.
- `cli/args/command.rs`: 0123 edits the `Relate` doc and relate arguments. This ticket edits only the cache docs.

The same rule covers both tickets. Whichever of 0123, 0124, and 0126 lands second merges the other's lines in these files. The lander resolves the overlap and reruns the rungs both tickets touch.

## Decisions

Each is the agent's decision. Ian can overturn any of them.

1. **Prune refuses only the alias.** It refuses when MODEL matches a requested model and no reply's model. That is the exact trap the issue found. Refusing any unmatched MODEL would also block the upgrade prune, the case the selector exists for. Option 3 of the issue, matching the requested model too, changes the selector's meaning. Option 2's confirming flag and option 4's `--dry-run` add surface. Option 5's help wording comes along.
2. **A typo still removes every entry, and the help says so.** Nothing tells a typo from a new version name. Any guard for it needs a new flag, such as `--dry-run` or a confirmation. That waits for a user who asks.
3. **The refusal is exit 2.** The folder is fine. The option's value is wrong, so it is a usage error, as a blank MODEL already is.
4. **The match stays exact.** `JEV-LATEST` matches no request, so it acts like a typo. Case folding would guess at what the backend means.
5. **The refusal names this run's address, never the bound one.** Storing the address in the marker would change its schema, drop its size bound, and put marker bytes in a diagnostic. The spec forbids the last.
6. **Two sentences, split on who named the folder.** A folder the user named keeps "restore its backend settings or choose another folder". A folder the user never named gets "the default cache" and the two ways around it. `THINKTHEN_CACHE` counts as named, since the user set it.
7. **The default-cache sentence does not suggest deleting the folder.** Prune is the only removal surface, and deleting the folder loses answers for the old address. `--no-cache` and `THINKTHEN_CACHE` lose nothing.
8. **Binding order stays.** A run with no key still binds the folder. Binding only at the first write would let two first users with different addresses both send, and one would pay for a refused install. Ticket 0065 ruled the order to stop exactly that.
9. **Rename the status words; keep the configuration field.** The status line is output the user reads next to the cache size, and ADR 0036 allows the rename inside `/1`. The configuration field is a file the user wrote, and the tool refuses unknown fields. Renaming it would break every existing file for a word.
10. **Leave landed tickets and ADR titles as history.** The specification is the contract, and it changes here. ADR 0033's title and ADR 0017's "cap" record what was decided then.
11. **The cache help names the privacy cost.** The folder holds the judged text, and nothing removes it until the user prunes. One clause in the cache help says so, beside the sentence that says the cache grows.

## Edge cases

Prune with `--answered-by-other-than MODEL`. Unless a row says otherwise, the folder holds two entries whose requests ask for `jev-latest` and whose replies name `jev-1.13.0`.

| Folder | MODEL | Other options | Result |
| --- | --- | --- | --- |
| The two entries | `jev-latest` | none | Exit 2, the refusal, empty stdout, both entries byte for byte unchanged |
| The two entries | `jev-latest` | `--older-than 1d` | Same as above. The refusal comes before any selection |
| The two entries | `jev-1.14.0` | none | Exit 0, both removed: the upgrade prune |
| The two entries | `no-such-model` | none | Exit 0, both removed. A typo acts like the upgrade prune. Deferred gap |
| The two entries | `JEV-LATEST` | none | Exit 0, both removed. Exact match, so no request asked for it |
| The two entries | `jev-1.13.0` | none | Exit 0, `removed 0 entries and 0 bytes; 2 entries and B bytes remain` |
| One entry whose reply names `jev-latest`, one whose reply names `jev-1.13.0` | `jev-latest` | none | Exit 0, the `jev-1.13.0` entry removed. A reply names MODEL, so no refusal |
| One entry by `jev-1.13.0`, one by `jev-old` | `jev-1.13.0` | none | Exit 0, the `jev-old` entry removed. Today's test keeps this |
| No entries, marker only | `jev-latest` | none | Exit 0, `removed 0 entries and 0 bytes; 0 entries and 0 bytes remain` |
| A malformed digest-named file beside good entries | `jev-1.13.0` | none | Exit 5, the scan refusal, nothing removed. Today's test keeps this. The alias check runs after the scan, because it needs the scanned entries, so the alias meets the same refusal |
| Any | blank | none | Exit 2, the existing blank-model refusal |

Backend mismatch, with the folder first bound by a run against loopback address A and then used by a run against loopback address B:

| How the folder was chosen | Sentence | Exit | Requests to B |
| --- | --- | --- | --- |
| Platform default under a temporary `HOME` | Default cache, naming B's endpoint URL | 5 | 0 |
| `THINKTHEN_CACHE` | Recording folder, naming B's endpoint URL | 5 | 0 |
| `--cache DIR` | Recording folder, naming B's endpoint URL | 5 | 0 |
| `--record DIR` | Recording folder, naming B's endpoint URL | 5 | 0 |
| `--replay DIR`, the folder bound and holding A's entry | Recording folder, naming B's endpoint URL | 5 | 0 |
| Platform default, second run with `--no-cache` and a key | No refusal. The run goes to B | 0 | 1 |

A second run that changes only `--model` meets no refusal, because the digest leaves the model out. The existing `normalized_address_spellings_and_models_share_one_folder` test keeps that.

## Acceptance

Every new test runs the built command through the existing backend harness, with a temporary `HOME`, and pins the whole standard error.

| Test | Proof | Planted fault that turns it red |
| --- | --- | --- |
| `prune_refuses_the_alias_and_keeps_the_upgrade` in `tests/backend/default_cache.rs` | The prune table, rows 1 to 7 and 9. Each refused row pins exit 2, the exact sentence, empty stdout, and every entry's bytes unchanged. Each other row pins exit 0, the count line, and which entries remain. Rows 8, 10, and 11 stay with today's prune tests | Two plants. Delete the check: row 1 exits 0 and both entries are gone. Refuse any MODEL no reply names, the round 1 design: row 3 exits 2 and the upgrade prune removes nothing |
| `a_mismatch_names_the_folder_and_the_address` in `tests/backend/cache_identity.rs` | The mismatch table, all six rows, with the file's `folder`, `decide`, `files`, and `default_cache` helpers. The first run binds and fills through A. Each refused row pins exit 5, empty stdout, the exact sentence with B's port, B's listener count 0, and the folder's files byte for byte unchanged. Row 6 pins exit 0 and B's count 1. This test replaces `a_cache_refuses_another_address_before_key_lookup_or_send`, which becomes row 3 | Two plants. Print the recording-folder sentence for every folder: row 1 reads the wrong sentence. Move the identity check after the key read: the refused runs carry no key, so they read the key sentence at exit 4 |
| `concurrent_first_users_at_different_addresses_choose_one_backend` in `cache_identity.rs`, updated | The losing run's stderr equals the recording-folder sentence with its own listener's endpoint URL. The test builds both expected sentences and checks that exactly one run matches its own | Build the sentence from the winning run's URL. The loser's comparison fails |
| `engine/facade_tests/contract_tests.rs`, updated | The mismatch match becomes `Error::RecordingBackendMismatch { .. }`. Behavior unchanged | None needed. The compiler forces the pattern |
| The existing `tests/status.rs` tests | The whole human output and the JSON object carry the renamed lines and fields | Keep the old name in the human writer. The whole-output comparison fails |

The four questions for each new test:

- `prune_refuses_the_alias_and_keeps_the_upgrade`. Behavior: the alias deletes nothing, and a newer version name still clears older answers. Regression: a refactor drops the check and the alias empties a cache at exit 0, or a stricter check blocks the upgrade prune. No existing test covers it: today's prune test only uses a model a reply names. Test-only hook: none. The fixture entries and the binary are the real boundary.
- `a_mismatch_names_the_folder_and_the_address`. Behavior: the refusal tells the user which cache and which address, and sends nothing. Regression: a message change loses the default-cache sentence or the address, or a refactor moves the check after the key read or the send. No existing test covers the default cache, `THINKTHEN_CACHE`, `--record`, or `--replay` mismatch: today's test covers only `--cache`, and this table absorbs it. Test-only hook: none. The listener counts requests at the real socket.
- The status and contract changes update existing tests and add none.

The engine's in-module prune tests keep testing selection. None is added, since the command test covers the check through the real boundary and a second layer would test one contract twice.

No new help test. Help wording is read by eye in review.

## Budgets

In nonblank lines:

- `core/recording.rs`: at most 12 added. One line is the `Exchange::url()` accessor. The rest return the request's model from `Entry::inspected`.
- `engine/cache_prune.rs`: at most 18 added.
- `engine/error.rs`, `engine/recorder.rs`, and `engine/recorder/identity.rs` together: at most 15 added.
- `engine/facade_tests/contract_tests.rs`: at most 2 changed.
- `cli/failure/recording.rs`, `cli/failure.rs`, and `cli/failure/convert.rs` together: at most 20 added.
- `cli/status.rs`: at most 4 added. The rename changes lines in place.
- `cli/args/command.rs`: at most 10 added, all in the cache help.
- `public/error.rs`: at most 2 changed.
- `tests/backend/default_cache.rs`: at most 110 added.
- `tests/backend/cache_identity.rs`: at most 110 added net, after the old `--cache` mismatch test is removed.
- `tests/status.rs`: at most 5 added.
- Specification and site pages: at most 25 lines added in total.
- No dependency.
- The ratchet rises to the measured total, at most 290 above main. The commit that raises it says what grew. Before raising it, the builder looks for duplication to delete in the prune tests of `default_cache.rs` and the mismatch tests of `cache_identity.rs`, since both plant folders and pin sentences.

## Stop rules

Stop, say so in the build record, and ask the coordinator before any of these:

- crossing a budget above;
- changing the marker schema, the binding order, or the entry format;
- refusing an entry for a missing or non-string request model;
- changing the library's public error text or any binding under `libraries/`;
- touching help text outside the cache commands, `databases/`, the test harness under `tests/backend/harness`, relate, the splitter, audit, or diff;
- adding a dependency, a flag, or a test-only hook;
- any paid call, `sdlc/scripts/live`, or a network test.

## Deferred gaps

- **A prune `--dry-run`, and a guard for a MODEL typo.** A typo removes every entry, as the upgrade prune does, because nothing tells the two apart. The help says so. A `--dry-run` that prints the count line without deleting would let a user check first. Both wait for a user who asks, since each adds a flag.
- **An over-target line in `status`, or a run warning (issue item 2, options 3 and 4).** The rename and the help now say that nothing trims the cache. A warning would add a stderr line to every run, and it waits for a ruling.
- **A status line for the binding.** `status` could print whether the default cache is bound to the current address. It cannot print the bound address, for decision 5's reason. The refusal now tells the user what they need. A later ticket can add the line if a user asks.
- **Bind only at the first write, or one cache per address.** Options 2 and 3 of issue item 1 change ruled behavior. Decision 8 gives the paid-race reason against option 3. Option 2 changes the default cache layout. Either needs Ian.
- **Renaming the configuration field `cache_bytes`.** Decision 9.
- **The library's mismatch wording and `EngineBuilder::cache_bytes`.** Ian's ruling 5 sends `cache_bytes` out of the library in a later ticket. That ticket can also word the library's default-cache refusal.
- **ADR 0033's title, ADR 0017's "cap", ticket 0062's title.** Decision 10.

## What Ian can overturn

- Refusing only the alias, and leaving the typo to the help (decisions 1 and 2), and exit 2 (decision 3).
- The two mismatch sentences and their exact words (decisions 6 and 7).
- Keeping the binding order (decision 8).
- Renaming the status words and keeping the configuration field name (decision 9).
- Leaving the ADR and ticket titles alone (decision 10).
- The privacy clause in the cache help (decision 11).

## Issues it closes

Landing closes `sdlc/issues/closed/2026-09-25-the-answer-cache-three-fixes.md`. The lander writes its closing status line and moves it to `sdlc/issues/closed/` in the landing commit. The deferred gaps above stay recorded in this ticket. The typo gap is a real risk the help now names. It needs a flag, so it waits for a user who asks, and it gets no new issue.

## Routing

Builder: Claude (Opus subagent). Reviewer: a fresh read-only Claude session for design and for code. The change raises the ceiling and changes `status` output, so the code review names what it checked.
