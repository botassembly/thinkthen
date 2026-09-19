# Record 0003: Answer decide if from the shell

- Ticket: `sdlc/tickets/0003-answer-decide-if-from-the-shell.md`
- Branch: `ticket/0003-decide-if`
- Landed: 2026-09-19

## What was built

`thinkthen decide if CONDITION` now runs end to end. `decide` is a subcommand family with one verb.

The core gained the decisions the binary may not make for itself.

- `backend.rs`: `BackendValues`, `Backend`, `BackendError`, and `resolve_backend`, a pure function from the raw flag values and the raw environment values to one resolved backend or one typed error. The built-in `jev` profile is one `const` row of data. No environment variable is read here.
- `text.rs`: `Url` and `KeyVar` join the macro that already declared the other four text values. All six refuse text that is blank.
- `result.rs`: `Meta` gained `url` and a `backend` that is `null` for an ad-hoc backend, as ADR 0006 settled.
- `plan_document.rs`: `PlanDocument`, the six fields `--plan` prints, built from a backend and a plan.
- `render.rs`: `json_line`, which writes any document as the compact line standard output carries. It exists so the binary needs no `serde_json` of its own.
- `systemone.rs`: `encode_raw` writes the request as a `RawValue` and `encode` hands back its bytes, so one serialization serves both the request and the plan.

The binary has six modules, each with one job in its first line.

| Module | Its one job |
| --- | --- |
| `main.rs` | Parse, run, and turn a failure into an exit code |
| `args.rs` | The command line, as the clap types that parse it |
| `edge.rs` | The environment, standard input, and standard output |
| `http.rs` | One HTTP exchange with a backend, retried as the specification says |
| `decide.rs` | The `decide if` flow, from what was asked to what is printed |
| `failure.rs` | Every way the command fails, and the one map to an exit code and a message |

`http.rs` is the only module that names `ureq`. `failure.rs` is the only module that builds a message a user reads. The environment is read once, in `Environment::read`, and handed inward as `BackendValues`.

## Red then green

Each core and edge test below failed before the code that satisfies it existed.

- `nothing_named_resolves_the_built_in_profile`: failed with `the built-in profile resolves: IncompleteAdHoc`, against a `resolve_backend` that had no body.
- `a_blank_value_is_refused_whichever_source_offered_it`: failed with `left: Err(IncompleteAdHoc)`, `right: Err(Blank(BackendName))`.
- `the_plan_prints_six_fields_and_opens_no_connection`: failed with `left: ""` against the whole plan line, because the binary knew only `--version`.
- `evidence_that_is_blank_or_not_utf_eight_stops_the_command`: failed with `left: Some(2)`, `right: Some(5)`, because clap refused the unknown word before any input was read.
- `the_help_for_decide_if_names_the_condition_and_every_option`: failed at `--min-prob is missing from` an empty help.

The exchange tests were written before the binary could answer them but first ran after it could, so each was proved by breaking the code and watching it fail.

- `a_named_key_variable_is_sent_as_a_bearer_token_and_never_printed`: dropping the header gave `left: None`, `right: Some("Bearer sk-secret-value")`.
- `a_retried_status_is_sent_again_and_the_second_answer_is_taken`: taking 429 out of the retried list gave `left: 1`, `right: 2` requests.
- `the_request_carries_the_encoded_plan_the_content_type_and_no_key`: changing the condition the test encodes moved the `instructions` field and failed the byte comparison.

## The dependency and its licenses

`ureq` 3.4.2, `default-features = false`, `features = ["rustls"]`. That is the smallest set that posts HTTPS under a timeout. The tree grew from 52 resolved packages to 85, and every added package is TLS or the HTTP types under it.

Three licenses joined `ALLOWED_LICENSES` in `sdlc/scripts/policy.py`, each with the crates that forced it named in a comment beside it.

| License | Crates that need it |
| --- | --- |
| ISC | `ring` (as `Apache-2.0 AND ISC`), `rustls-webpki`, `untrusted` |
| BSD-3-Clause | `subtle` |
| CDLA-Permissive-2.0 | `webpki-roots` |

All three are permissive and carry no copyleft term. No Rust TLS stack avoids them: `ring` and `untrusted` sit under `rustls`, and a root store has to come from somewhere. Every other package in the tree already passed the old list. Two cases were added to the SPDX table so the `AND` path is held to its answer: `Apache-2.0 AND ISC` passes and `Apache-2.0 AND GPL-3.0` does not.

`serde_json` gained the `raw_value` feature in the core. The plan embeds the adapter's request as JSON rather than as a string, and `RawValue` is pre-serialized text that only `encode_raw` ever writes. No dynamic JSON entered the core, and the `serde_json::Value` bans still hold.

The binary declares `clap`, `thinkthen-core`, and `ureq`. It does not declare `thiserror`, because every message it prints is built in `report`, which is the one function `channels.md` asks for.

## The closed pipe

A write that fails with `BrokenPipe` ends quietly. `edge::write_line` takes that kind as success, prints no diagnostic, and lets the command keep the exit code it earned. Every other write failure is exit 5, as the table says. A reader that stops early, as `head` does, is how Unix pipelines end, and the command had already done its job. A unit test drives a writer that fails with each kind. An integration test would be a race against the pipe buffer, so it was not written.

## The ladder

| Rung | Script | Exit |
| --- | --- | --- |
| 0 | `sdlc/scripts/install` | 0 |
| 1 | `sdlc/scripts/lint` | 0 |
| 2 | `sdlc/scripts/test` | 0 |
| 3 | `sdlc/scripts/spec` | 0 |

Thirty-eight core unit tests, three binary unit tests, twenty-three integration tests, one documentation test, and fourteen spec examples pass. No test opens a socket that is not loopback, and no request left this machine.

## The ratchet

The ceiling was 1283 and is 2876. It moved in five steps, each in the commit that needed it: 1771 for the core resolution and the plan document, 2831 and then 2848 for the binary and its tests, 2877 after formatting, 2867 when the narrowed core items went, and 2876 for the request comparison.

## The public surface

Issue item 2 asked which public core items the binary needs. `SCHEMA`, `Usage::new`, `Probability` with its error, and the two `Answer` accessors are now crate-private, and `Assessment::min_prob` is gone, because its own property test reads the field. The four text values keep their `new` and `as_str` because one macro declares all of them and narrowing one would split the macro. Ticket 0004 still has to answer for `encode`, `Adapter`, `Url`, and `Meta`, which is why they stay.

## What the specification got wrong or left unsaid

- `backends.md` never says what `--backend NAME` beside a `--url` means. A name would do nothing there, and ADR 0006 already says an option that does nothing in the chosen mode is a usage error, so it is exit 2. Ian can overturn this cheaply: the other reading is that the URL simply wins and the name is ignored.
- `backends.md` says an ad-hoc backend takes no key variable from a profile and that the user may name one with `--key-env`. It does not say what happens when that named variable is unset. It is exit 4, on the same rule a profile's variable follows.
- `backends.md` says a blank value is a usage error for all five values. A blank adapter is refused as an unknown adapter instead, because the name is parsed rather than held. Both are exit 2 and the message still says what was wrong.
- `backends.md` says the timeout "covers the whole exchange" and separately that a failure is retried. It does not say whether the timeout covers one attempt or all of them. It covers one attempt, so three attempts under `--timeout 30` may take ninety seconds.
- `backends.md` fixes the wait as doubling from one second and never says whether a wait follows the last attempt. None does.
- `channels.md` says the plan's `request` is "the body the adapter would send" without saying whether it is a JSON value or a string holding JSON. It is a value, so `jq` can read into it.
- `channels.md` gives no exit code for a closed pipe downstream. The decision above covers it.
- `channels.md` says an error body may reach the user and never says whether it is safe to print. It is not: a 422 can quote the evidence back. Only the status code prints.
- `result.md` shows `meta` as backend, url, adapter, model, usage. The order is now fixed by a test, because nothing says it may vary.
- `decide.md` says `--status` needs `--min-prob` and never says what a set but empty environment variable means. An empty `THINKTHEN_MODEL` counts as given and blank, so it is exit 2 rather than absent.
- Nothing in the specification names a way to make tests fast. `THINKTHEN_TEST_RETRY_WAIT_MS` is read at the edge, hidden from the help, and documented here as test-only. It shortens the first retry wait and nothing else.
