# 0124 build: fix the answer cache's three faults

Builder: Claude (Opus subagent), 2026-09-25, on `ticket/0124-answer-cache-fixes` from the accepted ticket at `7aae607d`, with `origin/main` at `c8ca9a65` merged before the final rung run. Ian can overturn every decision the ticket lists.

## Outcome

- `thinkthen cache prune DIR --answered-by-other-than jev-latest` on a folder whose requests asked for `jev-latest` and whose replies name `jev-1.13.0` now exits 2 and deletes nothing. `--answered-by-other-than jev-1.14.0`, the upgrade prune, still removes every older answer.
- A backend mismatch names the endpoint this run resolved. The platform default cache gets its own sentence with `--no-cache` and `THINKTHEN_CACHE`. Folders the user named keep "restore its backend settings or choose another folder".
- `status` prints `cache_prune_target_bytes` and `cache_prune_target_source`, and `status --json` names `cache.prune_target_bytes` and `cache.prune_target_source`.
- `cache --help` says the cache never trims itself, grows until `cache prune`, and holds the judged text until then. The prune help, `specification/recording.md`, the specification README and roadmap, and the site reference match.

## Design as built, where it differs from the ticket

1. **Tuple variants.** The engine's `Error::RecordingBackendMismatch(String, bool)` and the command's `Failure::RecordingBackendMismatch(String, bool)` carry the URL and the default-cache flag. The ticket said "two fields". Named fields cost three extra lines per enum after formatting and put the engine group over its budget. The variant carries a doc comment that names both fields. `Entry(String, String)` sets the precedent.
2. **The recorder fills the fields.** `identity::check` keeps its signature, so its fourteen in-module tests stay as they were. It returns the variant with an empty URL, and a comment says the recorder names it. The recorder's folder gate maps the variant to this run's URL and its `private_default` flag. A first draft added a `check_for` wrapper in `identity.rs`. It measured 21 lines and was dropped for the mapping in `recorder.rs`.
3. **The request model reads through a typed struct.** `Entry::inspected` parses the request into `StoredRequest { model: Option<String> }`. A request whose `model` is missing or not a string yields none, and nothing new is refused. A draft read `serde_json::Value`. The lint rung refused it: the core bans dynamic JSON. The typed struct replaced it.
4. **Status keeps its Rust field names.** `#[serde(rename)]` gives the JSON names. The human writer prints the new words. This keeps both writer lines under the line width.

## Plants

Each plant went into the worktree, ran the named test under the heavy lock, and was restored from a copy and touched. The first round's shared scratch script was overwritten by another session mid-run, which left one plant in `recorder.rs`. The builder removed it by hand, confirmed no plant text remained in the diff, and reran all six plants from a private folder. Every result below is from that second round, on the final code before the typed-struct fix. The typed-struct fix touches only `core/recording.rs`, and the prune tests pass after it.

| Plant | File | Test | Result |
| --- | --- | --- | --- |
| Delete the alias check | `engine/cache_prune.rs` | `prune_refuses_the_alias_and_keeps_the_upgrade` | RED: row 0 exits 0, not 2 |
| Refuse any MODEL no reply names, the round 1 rule | `engine/cache_prune.rs` | same | RED: row 2, the upgrade prune, exits 2, not 0 |
| Print the recording-folder sentence for every folder | `engine/recorder.rs` | `a_mismatch_names_the_folder_and_the_address` | RED: the default row reads the recording-folder sentence |
| Drop the refusal, so the run goes on to the key read | `engine/recorder.rs` | same | RED: exit 4, not 5 |
| Name the base without `/systemone` | `engine/recorder.rs` | `cache_identity` tests | RED: the table and `concurrent_first_users_at_different_addresses_choose_one_backend` |
| Print the old status name | `cli/status.rs` | `tests/status.rs` | RED: `absent_state_has_one_exact_closed_json_shape_and_changes_nothing` |

The ticket named two plants that became these. "Move the identity check after the key read" is the fourth row. Dropping the refusal is the plantable form of the same regression, and it shows the same red at exit 4. "Build the sentence from the winning run's URL" is the fifth row. The command cannot plant that, since each run knows only its own URL. A wrong URL in the sentence is the regression it guards, and the fifth row shows it red.

## Budgets

Nonblank lines, net against main.

| Budget | Limit | Measured |
| --- | --- | --- |
| `core/recording.rs` | 12 | 12 |
| `engine/cache_prune.rs` | 18 | 15 |
| `engine/error.rs`, `recorder.rs`, `recorder/identity.rs` | 15 | 11 (2, 8, 1) |
| `engine/facade_tests/contract_tests.rs` | 2 changed | 1 changed |
| `cli/failure/recording.rs`, `failure.rs`, `failure/convert.rs` | 20 | 12 (8, 1, 3) |
| `cli/status.rs` | 4 | 2 |
| `cli/args/command.rs` | 10 | 6 |
| `public/error.rs` | 2 changed | 1 changed |
| `tests/backend/default_cache.rs` | 110 | 89 |
| `tests/backend/cache_identity.rs` | 110 net | 62 |
| `tests/status.rs` | 5 | 0 |
| Specification and site | 25 | 1 |

The typed-struct fix first put `core/recording.rs` one line over its budget. The builder folded a two-line doc comment into one before any commit, so no budget was crossed in a commit.

The ratchet rises from 61,768 to the measured total. That is under the ticket's cap of 290 above main. The growth is the prune check, the mismatch plumbing, and two table tests. Before the raise, the builder looked for duplication in the prune tests of `default_cache.rs` and the mismatch tests of `cache_identity.rs`. The old `--cache` mismatch test folded into the new table and left. `prune_model_selection_and_scan_before_delete_hold` stays. Its second half proves scan-before-delete on a folder its first half built, and the new table does not cover that.

## Rungs

`origin/main` at `c8ca9a65` merged cleanly before the final run. The merge brought ticket 0122's pandas work and touched no file this ticket changes.

- `install`: exit 0.
- `lint`: the first run exited 101 on the core's dynamic-JSON ban, and Clippy then found two test-helper faults. Both are fixed. The rerun result is in the hand-back.
- `test`: exit 0.
- `spec`: exit 0.
- `surfaces`: not run. No surface and no public API shape changed. `public/error.rs` changes one match pattern and keeps its message.
