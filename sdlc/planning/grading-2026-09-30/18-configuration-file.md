# Area 18: The configuration file and its safety rules

Commit `58014776c`. Reviewer: Sonnet 5.5, fresh, read only. Rubric: `README.md` in this folder.

## Scope

The tool reads one optional, closed JSON configuration file, refuses any field it does not know, never writes it, warns when another user can write it, and refuses `backends` from such a file.

All paths below are under `crates/thinkthen/src/` unless they start with `crates/`, `specification/` or `sdlc/`.

| Kind | Paths (nonblank lines) |
| --- | --- |
| Code | `config.rs` (478, file read, shape, owner check and platform paths), `config/backends.rs` (287, shared with area 17), `cli/edge.rs:104-130` (the read), `cli/mod.rs:89-95` (the warning), `cli/asking/folders.rs:88-99` and `cli/asking.rs:60` (the folder warning), `engine/roots.rs` (84): 849 together for the first three files |
| Tests | About 24 tests. Unit: `config.rs` (4), `config/backends.rs` (3). Integration: `crates/thinkthen/tests/backend/cache_configuration.rs` (5), `cache_trust.rs` (3), `status.rs` (8), `named_backends/precedence.rs` (1) |
| Contract | `specification/recording.md:44` (the file's fields), `specification/backends.md:61`, `specification/settings.md:31`; ADRs 0033, 0114, 0115; tickets 0242, 0303, 0343 |

`engine/roots.rs` is not folder trust. That file reads the private TLS root bundle (`THINKTHEN_CA_BUNDLE`). Folder trust lives in `cli/asking/folders.rs:88-99`, `engine/store.rs` and the shared-host check in the library.

## Complexity: 3 of 5

| Driver | Score | Measured fact |
| --- | ---: | --- |
| Code size | 2 | 849 nonblank lines across 3 files |
| States and concurrency | 2 | Sequential. Three outcomes: absent, present, present and shared. The file is read and its metadata is taken in two separate calls (`config.rs:93`, `:107`) |
| Rules and refusals | 4 | 25 refusal sentences: 15 in `config.rs` and 10 in `config/backends.rs`. Two warnings (`cli/mod.rs:93`, `cli/asking.rs:62`). Three path rules |
| Surfaces touched | 4 | The command reads it at `cli/edge.rs:106`, and the Rust builder at `public/settings/environment.rs:31`. Bindings and SQL reach it only through `from_env` (unconfirmed per surface) |
| Settings | 4 | Nine rows name a configuration cell: Address, Backend, Named backends, Backend key, Model, Requests a minute, Caller prices, Answer cache, Prune target |
| Contract weight | 3 | Four spec pages (`recording.md`, `backends.md`, `settings.md`, `question-file.md`) and three ADRs |
| Churn and debt | 4 | 18 commits on the config paths in 7 days. The warning rule was rewritten four times in a week: `60868942d`, `c5d9fd99e`, `53ba4edb8`, `beee57712`. No open issue names the file |

Mean 3.3, rounded to 3.

## Grades

| Dimension | Grade | Evidence |
| --- | :---: | --- |
| Quality | B | The code matches ADR 0033 and 0114 on the main paths. The reader uses `deny_unknown_fields` and names the first bad field, never its value (`config.rs:49-50`, `:224-254`). The file is never written (no write call in `config.rs` or `config/backends.rs`). `backends` from a file another user can write is refused with exit 5 and the exact sentence (`config.rs:108-111`, pinned at `config/backends.rs:284-297`). Drift: the contract leaves open what a symlink or a writable parent folder does, and the code checks neither. `fs::metadata` follows a link (`config.rs:107`), and only the file's own owner and other-write bit are read (`:270-272`). A shared file's `url`, `model` and `backend` are still honored with a warning, which `settings.md:31` and ADR 0114 record |
| Reliability | B | Every refusal sentence has an exact-text table with a marker proving no value is echoed (`config.rs:394-440`, `config/backends.rs:116-199`, rate table `:204-235`). The owner rule is a pure function with a table of six mode and owner cases (`config.rs:381-390`). The real-file test uses one mode, 0o646, and one sentence (`config/backends.rs:284-297`). No test runs against a real file owned by another user, a symlink, or a writable parent. The warn rule flipped four times in one week, which caps the grade at B |
| Maintainability | B | One owner for the file's shape and one for entries. `config.rs` holds 478 of its 500 lines and mixes three concerns: the file, the owner rule, and the config, cache and usage path resolvers (`:285-357`). The body is parsed three times (`config.rs:116`, `:197`, `:226`). The field lists in `UNKNOWN` and `EXTRA` are parsed by `sdlc/scripts/settings:47-48`, so a new field cannot skip its settings row. No lint suppression in either file |

## Strengths

- Each refusal is a named constant, closed, and value-free (`config.rs:30-47`, `config/backends.rs:14-28`). The settings script reads those lists and fails a missing row (`sdlc/scripts/settings:29-31`).
- The shared-file rule is tied to the real risk: a file another user can write could name any key variable, so `backends` is refused rather than warned (`config.rs:108-111`).
- A group-write bit alone stays quiet, so the common 002 umask does not warn on every file (`config.rs:256-272`).
- A refused `url` and a refused backend `url` both collapse to one "safe backend base" sentence, so a refusal never prints the address (`config.rs:145-151`, `config/backends.rs:79-81`).
- `Debug` withholds the `url` and counts nothing else of it (`config.rs:71-86`, tested at `:366-374`).

## Cleanup

1. **Test the owner and symlink cases on real files.** Where: `config.rs:107`, `config/backends.rs:284-297`. Why: only a pure function and one 0o646 file are tested. A symlink to another user's file, a file in a writable folder, and a file owned by another user have no test (none found by `rg` in the four integration files). Size: S. Blocks 0.1: no.
2. **Decide whether a symlink or writable parent counts as shared.** Where: `config.rs:93`, `:107`, `specification/settings.md:31`. Why: a user who can replace the file or its folder decides where the key and evidence go, yet no warning fires. Whether this is a live risk is unconfirmed. `lstat` plus a parent check would cover it. Needs a ruling from Ian, because ADR 0033 and 0114 name only the file. Size: S. Blocks 0.1: no.
3. **Decide whether a shared file may still name `url`.** Where: `config.rs:108`, `cli/mod.rs:89-95`. Why: `backends` is refused from a shared file, but `url` is only warned about. Such a `url` sends `THINKTHEN_API_KEY` and the evidence to any `https` host the writer names. The warning and the choice are recorded (ADR 0114), and Ian can overturn them. Refusing `url` too closes the gap at the cost of breaking shared-home setups. Size: S. Blocks 0.1: no, because it is documented and warned. Raise it with Ian before 0.1.
4. **Move path resolution out of `config.rs`.** Where: `config.rs:285-357`. Why: the three resolvers and the owner rule have no link to the file reader, and the file sits 22 lines under the cap. A `config/paths.rs` leaves the next refusal room. Size: S. Blocks 0.1: no.
5. **Parse the body once.** Where: `config.rs:116`, `:197`, `:226`. Why: `parse`, `price_pair` and `shape_fault` each decode the same bytes. Carry one `serde_json::Value` through the three. Size: S. Blocks 0.1: no.
6. **Keep one folder-trust rule in one place.** Where: `cli/asking/folders.rs:88-99`, `engine/store.rs`, `public/settings.rs` (shared-host build). Why: the command warns, the shared host refuses, and each implements its own check over `config::writable_by_another`. The difference is documented (ticket 0318, `recording.md:82`), so this is structure only. Size: M. Blocks 0.1: no.

## Confidence: medium

What was read: all of `config.rs` and `config/backends.rs`, the shared-file read and warning at `cli/edge.rs:104-110` and `cli/mod.rs:75-95`, `engine/roots.rs`, `cli/asking/folders.rs:60-100`, `public/settings/environment.rs:25-100`, ADR 0114's shared-file paragraphs, and the contract text in `recording.md:44`, `backends.md:61` and `settings.md:31`.

Not checked: no test was run. The symlink and writable-parent findings are inferred from `fs::metadata` and the absence of tests. `engine/store.rs` and the `status` code were not read, so the folder-trust claim in item 6 and the status tests' coverage of `status.configuration` are unconfirmed. Which bindings pass through `from_env` was not checked.
