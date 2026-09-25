---
flow: build
priority: 124
opens: crates/thinkthen/src/engine crates/thinkthen/src/cli/cache.rs crates/thinkthen/src/cli/args/command.rs crates/thinkthen/src/cli/failure crates/thinkthen/src/cli/status.rs crates/thinkthen/src/public/error.rs crates/thinkthen/tests/backend/default_cache.rs crates/thinkthen/tests/backend/cache_identity.rs crates/thinkthen/tests/status.rs specification site/src/pages/reference.astro sdlc/ratchet.json sdlc/records sdlc/issues sdlc/tickets
---

# 0124: Fix the answer cache's three faults

Status: ready. Design review pending. Owner: Claude.

Review route: a fresh read-only Claude session reviews this design and the final diff. Codex does not review this ticket unless Ian routes it.

## Outcome and authority

Three cache faults stop hurting a user.

1. `thinkthen cache prune DIR --answered-by-other-than jev-latest` deletes every entry today. After this ticket it deletes nothing and says why.
2. The default cache refuses a second backend address with a sentence about "the recording folder". After this ticket the refusal names the default cache, the address this run asked for, and the ways around it.
3. `status` prints `cache_target_bytes` beside the cache size. That reads as a limit the tool keeps. After this ticket the words say the number is the size `cache prune` trims to.

The ask is `sdlc/issues/2026-09-25-the-answer-cache-three-fixes.md`, item 2 of section A in `sdlc/planning/issue-backlog-2026-09-25.md`. Pruning by the alias loses user data, so it comes first. Ian ruled on 2026-09-25 that no setting may do nothing, and that `cache_bytes` leaves the library, ticket 0084, and every binding. A later ticket carries that out. This ticket keeps the command's `cache_bytes` configuration field, because it sets prune's default target and so does something.

## Evidence

- Starts from: experiment 259 (`~/workspace/experiments/259-talk-claims`, script `03-cache-record-replay.sh`) reproduced all three faults offline at e70bddab with no key. On a 140-entry Beatles Bench recording, every request asks for `jev-latest` and every response names `jev-1.13.0`. `--answered-by-other-than jev-latest` and `--answered-by-other-than no-such-model` each removed all 140 entries at exit 0. A first run with no key bound the default cache to `http://127.0.0.1:9/v1`, and the next plain run stopped at exit 5 with the recording-folder sentence. A cache at five times its target stayed untouched through a run. The issue quotes each transcript. Tickets 0062 and 0065 and ADRs 0017 and 0033 rule the behavior this ticket keeps.
- Keeps: prune stays the only removal surface, and it never runs on its own. The selectors keep their union and their order. Prune still validates every entry before it deletes one. A folder still binds on its first write-capable use, before the key is read, and a mismatch still exits 5 before any request. The marker keeps its schema, its digest, and its size bound. No marker byte reaches a diagnostic. The configuration file keeps its schema and its `cache_bytes` field. The library's error message stays as it is.
- Changes: prune refuses a model no entry answered, at exit 2, before it deletes anything. The backend mismatch refusal names the address this run resolved, and the default cache gets its own sentence with `--no-cache` and `THINKTHEN_CACHE`. `status` renames its two target lines and JSON fields to say "prune target". The prune help, the specification, and the site reference say the cache grows until the user runs `cache prune`.
- Proof: two new outside-in tests in `tests/backend/default_cache.rs`, each an edge-case table, and the updated exact-output tests in `tests/backend/cache_identity.rs` and `tests/status.rs`. Each has a planted fault that turns it red. The Acceptance section lists them.
- Defers: a `status` line that shows whether the cache is bound to the current address; binding a folder only when an entry is written; a per-address default cache; a prune `--dry-run`; renaming the configuration field `cache_bytes`; the library's own mismatch wording; removing `EngineBuilder::cache_bytes`; the titles of ADR 0033, ADR 0017's "cap", and ticket 0062. The Deferred gaps section says why for each.

## Design

### 1. Prune refuses a model no entry answered

`engine/cache_prune.rs` already scans and validates every entry before it selects one. Each scanned entry carries the model its response named. After the scan and before any selection, `run_at` adds one check. When `--answered-by-other-than MODEL` is given, the folder holds at least one entry, and no entry's response names MODEL byte for byte, prune returns the engine's existing `Error::Usage` with a fixed sentence. The command exits 2 and prints:

```text
thinkthen: --answered-by-other-than matches no entry's answering model, so prune removed nothing; name the version a result's meta.model shows, not an alias such as jev-latest
```

That is the whole standard error line. Standard output stays empty. The sentence repeats no entry field and no value the user typed. The spec forbids a refusal that repeats an entry field, so the sentence cannot list the models it found. It points to `meta.model`, which every result already carries.

The check sits in the engine beside the scan. The command cannot see the entries without a second scan. The check holds the exclusive folder gate that prune already takes.

The help for the option changes from "Remove entries answered by any other model" to "Remove entries whose reply names another model. Give the version that answered, as a result's meta.model shows it, not an alias".

### 2. The backend mismatch refusal names the default cache and the address

The engine's `Error::RecordingBackendMismatch` gains two fields: the resolved endpoint URL of this run, and whether the folder is the platform default cache. The recorder's folder gate already holds both. It knows `private_default`, and the exchange carries the URL. The command's `Failure` carries the same two fields through the existing conversion in `cli/failure/convert.rs`. `cli/failure/recording.rs` prints one of two sentences.

The platform default cache, when neither `THINKTHEN_CACHE`, `--cache`, `--record`, nor `--replay` named a folder:

```text
thinkthen: the default cache is bound to a backend address other than `URL`; go back to that address, use --no-cache, or set THINKTHEN_CACHE to another folder
```

Every folder the user named, by `THINKTHEN_CACHE`, `--cache`, `--record`, or `--record` with `--replay`:

```text
thinkthen: the recording folder is bound to a backend address other than `URL`; restore its backend settings or choose another folder
```

`URL` is the resolved endpoint URL, such as `http://127.0.0.1:9/v1/systemone`. The existing stopped-run line still follows. The exit code stays 5.

The address this run resolved is safe to print. `specification/backends.md` refuses a base with user information, a query, or a fragment, "because the address is printed in a plan and kept in a recording". The bound address is not printed. The marker holds only its digest, and the spec forbids a marker byte in a diagnostic. The sentence names the address this run asked for, which is the one the user may not know came from `THINKTHEN_BASE_URL` or the configuration file.

`public/error.rs` matches the variant with `{ .. }` and keeps its message. The library's words wait for the ticket that removes `cache_bytes` from the library.

### 3. The words say "prune target"

The cache grows until the user runs `thinkthen cache prune DIR`. Ian ruled out automatic trimming. So the fix is the words.

- `status` prints `cache_prune_target_bytes` and `cache_prune_target_source` in place of `cache_target_bytes` and `cache_target_source`. `status --json` renames `cache.target_bytes` and `cache.target_source` to `cache.prune_target_bytes` and `cache.prune_target_source`. The schema stays `thinkthen.status/1`, since nothing has shipped. No repository in the workspace reads the old names.
- `cache --help` says: "Inspect and maintain answer-cache folders without sending a request. The cache never trims itself. It grows until you run cache prune."
- `cache prune --help` says "Remove selected entries, then the oldest entries until the folder fits the size target". `--max-size` says "Trim to this many allocated bytes. Without it, the configuration's cache_bytes applies, or 100000000".
- `specification/recording.md` says the configuration's `cache_bytes` is the target `cache prune` trims to, and that nothing trims the cache on its own. It names the two status fields. Its prune section states the MODEL match and the new refusal. Its binding paragraph states the two sentences.
- `specification/README.md` and `specification/roadmap.md` drop "bounded" before "cache". `site/src/pages/reference.astro` says the value is the size `cache prune` trims to and that the cache grows until then.

The help lines above belong to the cache commands. Ticket 0126 owns help text outside them, and this ticket touches none of it. The `status` help stays with 0126.

## Decisions

Each is the agent's decision. Ian can overturn any of them.

1. **Prune refuses; it does not guess.** Of the issue's options, option 1 catches the alias and a typo, costs one check, and adds no flag. Option 3, matching the requested model too, changes the selector's meaning and needs a spec change. Option 2's confirming flag and option 4's `--dry-run` add surface. Option 5 alone leaves the trap. The help wording of option 5 comes along.
2. **An empty folder is not refused.** With no entries, prune has nothing to lose. A scheduled prune of an empty cache keeps exit 0.
3. **The refusal is exit 2.** The folder is fine. The option's value is wrong, so it is a usage error, as a blank MODEL already is.
4. **The match stays exact.** `JEV-1.13.0` does not match `jev-1.13.0`. A near miss meets the refusal, which is safe.
5. **The refusal names this run's address, never the bound one.** Storing the address in the marker would change its schema, drop its size bound, and put marker bytes in a diagnostic. The spec forbids the last.
6. **Two sentences, split on who named the folder.** A folder the user named keeps "restore its backend settings or choose another folder". A folder the user never named gets "the default cache" and the two ways around it. `THINKTHEN_CACHE` counts as named, since the user set it.
7. **The default-cache sentence does not suggest deleting the folder.** Prune is the only removal surface, and deleting the folder loses answers for the old address. `--no-cache` and `THINKTHEN_CACHE` lose nothing.
8. **Binding order stays.** A run with no key still binds the folder. Binding only at the first write would let two first users with different addresses both send, and one would pay for a refused install. Ticket 0065 ruled the order to stop exactly that.
9. **Rename the status words; keep the configuration field.** The status line is output the user reads next to the cache size. The configuration field is a file the user wrote, and the tool refuses unknown fields. Renaming it would break every existing file for a word.
10. **Leave landed tickets and ADR titles as history.** The specification is the contract, and it changes here. ADR 0033's title and ADR 0017's "cap" record what was decided then.

## Edge cases

Prune with `--answered-by-other-than MODEL`, on the default cache test fixture, where each request asks for `jev-latest` and each response names `jev-1.13.0`:

| Folder | MODEL | Other options | Result |
| --- | --- | --- | --- |
| Two entries, both answered by `jev-1.13.0` | `jev-latest` | none | Exit 2, the refusal, empty stdout, both entries byte for byte unchanged |
| Same | `no-such-model` | none | Same as above |
| Same | `JEV-1.13.0` | none | Same as above |
| Same | `jev-latest` | `--older-than 1d` | Same as above. The refusal comes before any selection |
| One entry by `jev-1.13.0`, one by `jev-old` | `jev-1.13.0` | none | Exit 0, the `jev-old` entry removed. Today's test keeps this |
| No entries, marker only | `jev-latest` | none | Exit 0, `removed 0 entries and 0 bytes; 0 entries and 0 bytes remain` |
| A malformed digest-named file beside good entries | `jev-latest` | none | Exit 5, the scan refusal, nothing removed. The scan still comes first |
| Any | blank | none | Exit 2, the existing blank-model refusal |

Backend mismatch, with the folder first bound by a run against loopback address A and then used by a run against loopback address B:

| How the folder was chosen | Sentence | Exit | Requests to B |
| --- | --- | --- | --- |
| Platform default under `XDG_CACHE_HOME` | Default cache, naming B's endpoint URL | 5 | 0 |
| `THINKTHEN_CACHE` | Recording folder, naming B's endpoint URL | 5 | 0 |
| `--cache DIR` | Recording folder, naming B's endpoint URL | 5 | 0 |
| `--record DIR` | Recording folder, naming B's endpoint URL | 5 | 0 |
| Platform default, second run with `--no-cache` and a key | No refusal. The run goes to B | 0 | 1 |

A second run that changes only `--model` meets no refusal, because the digest leaves the model out. The existing models-sharing-one-folder test in `cache_identity.rs` keeps that.

## Acceptance

Every new test runs the built command through the existing backend harness, with a temporary `HOME` and `XDG_CACHE_HOME`, and pins the whole standard error.

| Test | Proof | Planted fault that turns it red |
| --- | --- | --- |
| `prune_refuses_a_model_no_entry_answered` in `tests/backend/default_cache.rs` | The prune table above, rows 1 to 4 and 6. Each refused row pins exit 2, the exact sentence, empty stdout, and every entry's bytes unchanged. Row 6 pins exit 0 and the count line | Delete the check. Row 1 exits 0 and both entries are gone. Rows 5, 7, and 8 stay with today's prune tests |
| `a_mismatch_names_the_folder_and_the_address` in `tests/backend/default_cache.rs` | The mismatch table above, rows 1 to 5. The first run binds with no key and exits 4. The second pins exit 5, the exact sentence with B's port, and B's listener count 0. Row 5 pins B's count 1 | Print the recording-folder sentence for every folder. Row 1 reads the wrong sentence. A second plant: move the identity check after the key read. The second run has no key, so it reads the key sentence at exit 4 |
| The existing `cache_identity.rs` tests | `MISMATCH` becomes the recording-folder sentence with the listener's endpoint URL. Every test that pins it still passes | Drop the URL from the sentence. Every pinned comparison fails |
| The existing `tests/status.rs` tests | The whole human output and the JSON object carry the renamed lines and fields | Keep the old name in the human writer. The whole-output comparison fails |

The four questions for each new test:

- `prune_refuses_a_model_no_entry_answered`. Behavior: a wrong model name deletes nothing. Regression: a refactor of the selector drops the check, and the alias empties a cache again at exit 0. No existing test covers it: today's prune test only uses a model that matches. Test-only hook: none. The fixture entries and the binary are the real boundary.
- `a_mismatch_names_the_folder_and_the_address`. Behavior: the refusal tells the user which cache and which address, and sends nothing. Regression: a message change loses the default-cache sentence or the address, or a refactor moves the check after the send. No existing test covers the default cache or `THINKTHEN_CACHE` mismatch: `cache_identity.rs` covers only named folders. Test-only hook: none. The listener counts requests at the real socket.
- The status change updates existing tests and adds none.

The engine's in-module prune tests keep testing selection. None is added, since the command test covers the check through the real boundary and a second layer would test one contract twice.

No new help test. Help wording is read by eye in review.

## Budgets

In nonblank lines:

- `engine/cache_prune.rs`: at most 15 added.
- `engine/error.rs`, `engine/recorder.rs`, and `engine/recorder/identity.rs` together: at most 15 added.
- `cli/failure/recording.rs`, `cli/failure.rs`, and `cli/failure/convert.rs` together: at most 20 added.
- `cli/status.rs`: at most 4 added. The rename changes lines in place.
- `cli/args/command.rs`: at most 8 added, all in the cache help.
- `public/error.rs`: at most 2 changed.
- `tests/backend/default_cache.rs`: at most 170 added.
- `tests/backend/cache_identity.rs` and `tests/status.rs`: at most 15 added together.
- Specification and site pages: at most 25 lines added in total.
- No dependency.
- The ratchet rises to the measured total, at most 250 above main. The commit that raises it says what grew. Before raising it, the builder looks for duplication to delete in the prune and mismatch tests of `default_cache.rs` and `cache_identity.rs`, since both plant folders and pin sentences.

## Stop rules

Stop, say so in the build record, and ask the coordinator before any of these:

- crossing a budget above;
- changing the marker schema, the binding order, or the entry format;
- changing the library's public error text or any binding under `libraries/`;
- touching help text outside the cache commands, `databases/`, the test harness under `tests/backend/harness`, relate, the splitter, audit, or diff;
- adding a dependency, a flag, or a test-only hook;
- any paid call, `sdlc/scripts/live`, or a network test.

## Deferred gaps

- **A status line for the binding.** `status` could print whether the default cache is bound to the current address. It cannot print the bound address, for decision 5's reason. The refusal now tells the user what they need. A later ticket can add the line if a user asks.
- **Bind only at the first write, or one cache per address.** Options 2 and 3 of issue item 1 change ruled behavior. Decision 8 gives the paid-race reason against option 3. Option 2 changes the default cache layout. Either needs Ian.
- **A prune `--dry-run`.** Useful, but the refusal closes the data-loss path. It waits for a user who asks.
- **Renaming the configuration field `cache_bytes`.** Decision 9.
- **The library's mismatch wording and `EngineBuilder::cache_bytes`.** Ian's ruling 5 sends `cache_bytes` out of the library in a later ticket. That ticket can also word the library's default-cache refusal.
- **ADR 0033's title, ADR 0017's "cap", ticket 0062's title.** Decision 10.

## What Ian can overturn

- The refusal in place of a matching rule for prune (decision 1), and exit 2 (decision 3).
- The two mismatch sentences and their exact words (decisions 6 and 7).
- Keeping the binding order (decision 8).
- Renaming the status words and keeping the configuration field name (decision 9).
- Leaving the ADR and ticket titles alone (decision 10).

## Issues it closes

Landing closes `sdlc/issues/2026-09-25-the-answer-cache-three-fixes.md`. The lander writes its closing status line and moves it to `sdlc/issues/closed/` in the landing commit. The deferred gaps above stay recorded in this ticket. None is a defect a user hits once this lands, so none gets a new issue.

## Routing

Builder: Claude (Opus subagent). Reviewer: a fresh read-only Claude session for design and for code. The change raises the ceiling and changes `status` output, so the code review names what it checked.
