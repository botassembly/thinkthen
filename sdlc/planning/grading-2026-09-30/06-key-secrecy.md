# Area 6: Key secrecy across every surface, error, debug line and recording

Commit `58014776c`. Reviewer: Sonnet 5.5, fresh, read only. Rubric: `README.md` in this folder.

## Scope

The API key reaches one authorization header at the address the user named and no other byte the tool writes, prints or records, through every command, failure, `Debug` line, panic and recording.

All paths below are under `crates/thinkthen/src/` unless they start with `crates/`, `specification/`, `sdlc/` or `databases/`.

| Kind | Paths (nonblank lines) |
| --- | --- |
| Code, guard | `cli/edge/key.rs` (67), `public/panic.rs` (42), `engine/http/observation.rs` (91), `Key` in `engine/http.rs:25-55`, `Secret` in `public/settings.rs:53-60`, `Withheld` in `core/text.rs:293-299`, about 260 lines in all |
| Code, constrained | `cli/failure.rs` (480) and `cli/failure/` (683), `engine/store/` (733), `core/recording/convert.rs` (197), 71 hand-written `Debug` impls and 109 `Withheld` uses in 42 files |
| Tests | About 34 named tests. `crates/thinkthen/tests/backend/secrecy.rs` (19, which drive 518 cases through the compiled binary) with `secrecy/routes.rs` and `secrecy/damage.rs`, `secrecy_find.rs` (1), `secrecy_recognize.rs` (4), `secrecy_relate.rs` (1), `profile/secrecy.rs` (1), `question_file/secrecy.rs` (2), `tests/library/key_address.rs` (4), plus key checks in `tests/backend/check.rs`, `resend.rs`, `refused.rs` and the `Debug` tests in `tests/library/public_members.rs:335`, `:359` |
| Contract | `specification/backends.md` "The key" and "Named backends"; `specification/recording.md`; `CLAUDE.md` "Boundaries"; ADRs 0010 (with its 2026-09-27 amendment), 0024, 0098, 0114, 0115, ADR 0111 section 3; tickets 0306, 0310, 0321, 0147; `sdlc/scripts/policy.py` (recording check) |

## Complexity: 3 of 5

| Driver | Score | Measured fact |
| --- | ---: | --- |
| Code size | 3 | About 260 lines of dedicated guard code, constraining about 1,160 lines of failure wording and about 930 of store and recording code, so about 2,350 in all |
| States and concurrency | 2 | Sequential. One `OnceLock` key snapshot (`cli/edge/key.rs:22`), one process panic hook installed once with a thread-local depth counter (`engine/workers.rs:10-32`) |
| Rules and refusals | 3 | About 18. Absent or blank key exits 4 and names the variable, no key in a plan, result, recording, log or error, control characters refused, key in the address refused, loopback sends no header, named backends read only their own variables in order, a built-in's key never goes to another built-in's host (ADR 0115), a shared-writable file may not hold `backends`, a recording holds no header, `key_env` matches `[A-Z_][A-Z0-9_]*`, PostgreSQL's `thinkthen.api_key` is ignored and refused, a panic payload is forgotten, `Debug` withholds text |
| Surfaces touched | 5 | 22 of 22. Each surface reads the key or carries an error that must not hold it |
| Settings | 3 | Six rows: Key, Backend key, Named backends, Address, Recording, Answer cache |
| Contract weight | 4 | Four spec pages (`backends.md`, `recording.md`, `settings.md`, `channels.md`) and six ADRs (0010, 0024, 0098, 0114, 0115, 0111 section 3) |
| Churn and debt | 4 | 27 commits on the key-specific paths since 2026-09-23, 107 on the wider paths that include `cli/failure*` and `engine/store`. New rules in the window: named-backend keys (`44ca79f9e`), control characters (`6d24900b3`), the address guard (`657201a06`), the loopback rule (`4ef942302`), and the shared panic guard (`33e32a807`, `6e36b14ad`). One open issue: `sdlc/issues/2026-09-30-reference-page-exit-codes-and-key-rule-drift.md` |

Mean 3.4, rounded to 3.

## Grades

| Dimension | Grade | Evidence |
| --- | :---: | --- |
| Quality | B | The code matches `specification/backends.md` "The key" rule for rule. A control character in the key is refused before any send (`crates/thinkthen/src/engine/http.rs:37-43`, `:259`). The key is refused when the posting URL holds it, by exact bytes (`core/backend.rs:187-190`, `cli/edge/key.rs:38-40`, `public/settings.rs:382`). A missing key names the variable and never a value (`cli/failure.rs:264-267`). A loopback address takes an empty key and sends no header (`engine/facade.rs:284-289`, `engine/http.rs:400-403`). The wire is one place: `Bearer` is built once (`engine/http.rs:402`). No redirect is followed and a plain-HTTP client uses no proxy (`engine/http.rs:155`, `:166-168`). A recording has no header field at all, which is stronger than filtering: `rg` finds no `header`, `authorization` or `bearer` in `engine/store/` or `core/recording/`, and `sdlc/scripts/policy.py:2105-2160` also plants and refuses them in committed recordings. A response header that could carry the key is read only as an allowlisted request ID, dropped if it holds the key, the URL or any body bytes (`engine/http/observation.rs:38-53`). PostgreSQL ignores `thinkthen.api_key` and refuses an interactive set (`databases/postgresql/src/ffi.rs:157-172`, `call/settings.rs:198-202`). Drift is on the site: the reference page says an absent or empty key is always exit 4, but localhost needs none (`sdlc/issues/2026-09-30-reference-page-exit-codes-and-key-rule-drift.md`, item 3) |
| Reliability | B | The proof is broad and counted. One sweep drives six verbs through 17 routes, 518 cases, and asserts the key is on no byte of standard output, standard error or any file written, and the evidence is in no diagnostic (`crates/thinkthen/tests/backend/secrecy.rs:155-178`, `:395-425`). A companion case pins where the key does go, so a sweep that sent no key cannot pass (`:448`). Routes include a status body that quotes the evidence, an unreadable reply and a hostile recording entry (`tests/backend/secrecy/routes.rs:15-60`). `Debug` lines of builders and relate entities are tested (`tests/library/public_members.rs:335`, `:359`). Three things keep it at B. The area's rules are new: the control-character rule landed on 2026-09-30 (`6d24900b3`) and the panic guard was moved twice in two days (`33e32a807`, `6e36b14ad`). The panic guard states limits that leave a hole in a host that replaces the hook, and a first call from an already-panicking thread aborts the process (`public/panic.rs:15-23`). The sweep's `VERBS` holds six verbs and `RECORD_VERBS` two (`tests/backend/secrecy.rs:41-96`), so `find` and `annotate` depend on their own files (`secrecy_find.rs`, `tests/backend/annotate*.rs`) |
| Maintainability | B | One owner for each concern: the snapshot (`cli/edge/key.rs`), the wire (`engine/http.rs`), the panic guard (`public/panic.rs`), the observation filter (`engine/http/observation.rs`). `Key`, `Secret` and the snapshot's raw `String` are three holders of one secret, each with its own `Debug` or none (`engine/http.rs:26`, `public/settings.rs:54`, `cli/edge/key.rs:16`). No static check covers `Debug`: the repo has 323 `derive(Debug)` and 71 hand impls in non-test code, and `sdlc/scripts/policy.py` and `sdlc/scripts/lint` hold no rule for a derived `Debug` on a type that holds text, so the guard is review plus tests. `tests/backend/secrecy.rs` holds 464 of 500 lines and carries a file-wide `#![allow(clippy::indexing_slicing)]` (`:3-6`). `engine/http.rs` holds 496 of 500 lines and owns `Key` |

## Strengths

- The recording format has no header field, so a key cannot reach a recording by any code path, and the policy script checks committed fixtures anyway (`engine/store/`, `sdlc/scripts/policy.py:2105-2160`).
- The key is read once, late and lazily: only when the first request is about to go, and a zero send limit refuses before the read (`engine/pipeline/send.rs:113-125`, `:185-198`).
- The sweep proves presence as well as absence, with a case that pins the key at the authorization header (`tests/backend/secrecy.rs:448`).
- Every error that touches the wire is reduced to a fixed class before it reaches a message, and `Exchange`, `Client`, `Key` and `ResponseInfo` print `<withheld>` under `Debug` (`engine/http.rs:51-55`, `:105-110`, `:354-366`, `engine/http/observation.rs:14-26`).
- A named backend's key never goes to another built-in's host, and a configuration another user can write may not name backends (`specification/backends.md` "Named backends", `crates/thinkthen/src/config/backends.rs:255-287`).

## Cleanup

1. **Correct the site reference page's key rule.** Where: `site/src/pages/reference.astro` (marketing owns it), tracked in `sdlc/issues/2026-09-30-reference-page-exit-codes-and-key-rule-drift.md`. Why: the published page says an absent or empty key always exits 4 and names neither the loopback rule nor the per-backend variables, so a user of a local server is told to set a pretend key. Size: S. Blocks 0.1: yes.
2. **Add a static check for `derive(Debug)` on text-holding types.** Where: `sdlc/scripts/policy.py` or `sdlc/scripts/lint`, over the 323 derives. Why: `CLAUDE.md` asks for secrecy on every `Debug` line, and today only tests of chosen types enforce it. A check that lists each derived `Debug` with a `String`, `Vec<u8>` or `Json` field and requires a named allowance would catch the next one. Size: M. Blocks 0.1: no.
3. **Hold the key in one type.** Where: `engine/http.rs:25-55`, `public/settings.rs:53-60`, `cli/edge/key.rs:15-19`. Why: three holders, three `Debug` decisions. Moving `Key` to its own module and using it in the other two also frees 50 lines of `engine/http.rs`, which sits at 496 of 500. Size: S. Blocks 0.1: no.
4. **Stop the missing-key sentence from echoing a mistaken `key_env` (unconfirmed).** Where: `cli/failure.rs:264-267`, `config/backends.rs:23`, `:82`. Why: `key_env` accepts `[A-Z_][A-Z0-9_]*`, and an all-capitals token pasted there by mistake would pass and then print in "the environment variable `…` is unset". Real keys with lower case or a hyphen fail the check. I did not run it, and the file is the user's own. Size: S. Blocks 0.1: no.
5. **Remove the file-wide lint allowance and keep the sweep under the cap.** Where: `crates/thinkthen/tests/backend/secrecy.rs:3-6` (464 of 500). Why: a blanket `indexing_slicing` allow hides a real index slip in the file that guards secrecy. Replace with `get` and an assertion message. Size: S. Blocks 0.1: no.
6. **Prove every surface's error text, not only the command.** Where: `libraries/*/` and `databases/*/` checks. Why: the command is swept at 518 cases, and the Rust API at chosen types. The 21 other surfaces are covered by per-language checks that I did not read, and the sweep does not reach them. A single table of "key absent from every error the binding can return" per surface would show any gap. Size: M. Blocks 0.1: no.

## Confidence: medium

What was read: `cli/edge/key.rs`, `public/panic.rs`, `engine/http/observation.rs`, the `Key`, `Exchange` and `Client` parts of `engine/http.rs`, `Secret` and `api_key` in `public/settings.rs`, `Engine::key` and `address_contains_key`, the failure sentences for a missing key, the PostgreSQL ignored-key hook, the recording check in `sdlc/scripts/policy.py`, `specification/backends.md` "The key" and "Named backends", `CLAUDE.md`, and the structure, markers and route list of `tests/backend/secrecy.rs` and `secrecy/routes.rs`.

Not checked: ADRs 0010, 0024, 0098 and 0115 and tickets 0306, 0310 and 0321 were read by title only. `cli/failure.rs` and `cli/failure/` were searched for key and body interpolation, not read line by line. `engine/store/` and `core/recording/convert.rs` were searched for header words, not read, so the claim that they hold no header field is from `rg`. The error text of the 21 language and SQL surfaces was not read, apart from the PostgreSQL key hook and a search of the C door and Python glue. No test was run, no key was set and no key file was opened.
