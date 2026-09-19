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

## Review

Second review of record, 2026-09-19, on the branch rebased onto `36010af`. Ian writes no Rust, so this review stands in for him. No request left this machine: every test drove a loopback listener, and the only network access was cargo reading the crate registry it had already cached.

### What was checked

**Secrets and evidence.** Every path a key value can take was read end to end: `edge::key` reads the variable, `decide::ask` hands the value to `http::Exchange`, and `http::send` writes it into one `authorization` header on the resolved URL. Nothing else touches it. `failure::report` is the one place a message is built, and its table carries a status code, a variable name, an error kind, and never a value or a body. The evidence reaches the request body and the plan document and no diagnostic. Two holes were found and closed. The key was a plain `String` inside a struct that derived `Debug`, and the request body, which carries the evidence, was a byte slice in the same derived `Debug`. A key now lives in a type whose `Debug` prints a placeholder, the exchange writes its own `Debug`, and a unit test formats an exchange carrying both and finds neither. ureq strips auth headers on a redirect by default, so a key could never have crossed a host, but the evidence could, which the redirect finding below covers. An ad-hoc backend takes no key variable from any profile: `resolve_backend` builds it with `key_env` from the given values alone, and two core tests hold that.

**The rebased pages.** An environment variable set to the empty string was a usage error and now counts as unset. White space in a variable, and a blank flag value, are still exit 2. `--backend` beside a URL is exit 2. A named key variable holding nothing is exit 4 for a profile and for an ad-hoc backend alike. The timeout covers one attempt, which ureq's global timeout gives from connect to the last byte; the help said the whole exchange and now says one attempt. No wait follows the last attempt. An error message gives the status code and never the body. The plan's `request` is a JSON value. A closed pipe keeps the exit code the command earned. The six plan fields, their order, the exit table, `--status`, the flag-then-variable-then-profile precedence, the ad-hoc rule, the six retried statuses, the doubling wait, the two headers, and the one compact line were each read against the page and each has a test.

**Acceptance.** Every bullet of the ticket has a test. Four of the builder's tests were checked by breaking the code and watching each fail for its own reason, then restoring: dropping the content type gave `left: None, right: Some("application/json")`; taking 429 out of the retried list gave one request against two; swapping the no and unsure exit codes gave `left: Some(3), right: Some(1)`; letting a blank key variable through lost the exit 4. Three of the four fixes below were written red first and their failures are recorded in the commits.

**Dependencies and licenses.** `ureq` with `default-features = false, features = ["rustls"]` is the smallest set that posts HTTPS under a timeout. The default adds `gzip`, which this tool does not need, and dropping it is already done. `native-tls` would put OpenSSL under the binary and cost the single portable file the owner asked for. `platform-verifier` would drop `webpki-roots` and its license, and would make the binary depend on the host carrying a trust store, which costs the same thing. All five crates that force a license were confirmed against `cargo metadata`: `ring`, `rustls-webpki`, and `untrusted` for ISC, `subtle` for BSD-3-Clause, and `webpki-roots` for CDLA-Permissive-2.0. `rustls` itself passes on MIT and needs no exception. The three licenses were allowed globally and are now tied to those crates alone. `serde_json`'s `raw_value` feature is needed: the plan embeds the request as a JSON value rather than a string, and `RawValue` is what writes it once for both the plan and the wire.

**Size.** The canned responses in the exchange tests were four-line struct literals and are now one call each. The refused-reply test became a table. `render.rs` and `plan_document.rs` each hold one job and earn their file. The text newtype macro declares six values in the lines two would take by hand. No defensive check was found that cannot fire.

**Structure.** `http.rs` is the only module naming ureq. `failure::report` is the one map from an error to an exit code and a message. No print macro appears outside a test. The largest file is 371 non-blank lines against a ceiling of 500. Every public item in the core is reachable from a public signature the binary calls, so nothing further can be narrowed; `UnknownAdapterError`, `EmptyPlanError`, `Usage`, `BackendName`, and `Url` have no direct caller but each names a type in a signature the binary uses.

**Robustness.** The response body read was bounded only by whatever ureq defaults to. It is now bounded at one megabyte, named in this crate, with a test that serves two megabytes and watches the command exit 4. A 200 carrying text that is not JSON, a 200 carrying nothing, and a connection closed after an unfilled content length are all exit 4 and none hangs. A slow server is cut off by the per-attempt timeout.

**Gates.** The two lint tables, both `clippy.toml` files, the crate root attributes, and every ladder script are unchanged. `policy.py` changed only in its license handling. No `#[allow]` was added.

### What changed

| Commit | What it does |
| --- | --- |
| `ebe8fcb` | Takes an environment variable set to the empty string as unset |
| `2059302` | Keeps the key and the evidence out of every `Debug` line |
| `a62a2ee` | Names a canned response instead of spelling out its two fields |
| `6fc36cf` | Refuses to follow a redirect so a request stays on one host |
| `f350045` | Bounds the response body one attempt will read |
| `df33532` | Ties the three TLS licenses to the crates that force them |
| `1b482c5` | Deletes the assessment reader nobody calls |
| `6f53f8e` | Says in the help that the timeout covers one attempt |
| `f4d3e37` | Judges a body that is not JSON and a body the backend cut short |
| `7518af3` | Shows the empty variable rule in the executable spec |

### What was left

- A profile name in `THINKTHEN_BACKEND` beside a `--url` flag is exit 2, because the values merge one at a time and a name beside a URL is refused whichever source offered it. A user who exports `THINKTHEN_BACKEND` cannot reach an ad-hoc backend at all. `backends.md` refuses the case only for `--backend`, so either reading is defensible and the page has to settle it. The steering agent decides.
- Standard input has no cap. `edge::evidence` reads to the end into memory, so a stream that never ends fills it. The specification sets no cap today, and inventing one is a settled-page change.
- A response past the byte bound and a body cut short both count as transport failures, so both are retried. Downloading a megabyte three times is the cost. `backends.md` says a transport failure is retried, so the current behavior follows the page.
- `edge.rs` and `failure.rs` each carry the word "and" in their first documentation line, which the module rule refuses. Splitting either would cost more lines than the rule saves, because one owns the process edge and the other owns every failure.
- `Failure::Defect` and `Failure::Render` both reach exit 70 with the same prefix, and nobody distinguishes them. Merging them would drop the serde message that `Render` carries.
- clap's `env` feature would delete the hand-written environment reading, and it was refused. It reads the environment inside the parser rather than once at the edge, and it takes a variable set to the empty string as a value that was given, which is the opposite of the rule this branch just settled.
- Items 1 and 3 through 6 of `sdlc/issues/2026-09-19-review-leftovers-from-the-core-tickets.md` still stand. Item 2 is answered: nothing further in the core can be made private.

### The ratchet

The ceiling was 2876 when the branch rebased and is 3010. It rose for the empty variable test, the key type and the hand-written `Debug` with their test, the redirect test, the response bound and its test, and the two response shapes, and it fell for the collapsed canned responses and the deleted accessor.

### The ladder

| Rung | Script | Exit |
| --- | --- | --- |
| 0 | `sdlc/scripts/install` | 0 |
| 1 | `sdlc/scripts/lint` | 0 |
| 2 | `sdlc/scripts/test` | 0 |
| 3 | `sdlc/scripts/spec` | 0 |

Good enough to land.
