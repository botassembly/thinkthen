# Current functional test adapters

Source at the failing checkpoint: `64a5c196ae37bef5df95726cc7ab0749f8248fdc`. This branch subsequently merged main `7e1c3033` (separate public fixture test repairs); no runtime source or canonical corpus changed in that merge. The raw failed test and spec logs remain under `target/codex-builds/shared-functional-checkpoint-2026-09-28/continuation/`; this fix does not rewrite them. The changed tests use the current canonical `conformance/settings.json` and `conformance/cases.json`; neither corpus nor runtime was edited.

## Failure, correction, retained proof

- `settings_cases` expected `text` in every step. The current relation row instead names a directed `linked=item:item` rule and 18 entities. The CLI adapter now sends that JSON entity set to `relate`, checks all 306 ordered source/target pairs, rule and numeric probability, and retains the corpus's exact cumulative send counts of 1 and 3. Other setting steps retain their previous status, value, model, send and recording assertions. The CLI's one-process-per-step shape still cannot express the corpus's process-scoped `max_requests` case; the public Rust adapter covers it.
- `sdlc/scripts/test` selected a renamed panic regression. Its exact selector now names `engine_diagnostics_hide_worker_payloads_and_preserve_host_hook`; the existing `--list | grep` fail-if-zero check and parent/child test remain.
- Canonical filter/rank cases 13, 14, 15 and 16 contain one saved request body per record. Explicit batch size one restores that wire identity without changing the default packed API. The backend's exact-body matching, observed request-count delta and original answer/index oracles remain. The public settings adapter now gives its bare builder the loopback key and uses batch size one for the `max_requests=2` case; it checks two answered rows, a third usage refusal and exactly two sends. The earlier one-row failure was a zero-send no-key refusal, not a budget result.
- The convenience parent now expects 12 actual sends: six distinct first-round requests from each engine path. The child independently checks six sends per path, equal usage, cache answers in the second round and equal values/errors. This replaces the obsolete `>=20` singleton-era lower bound with an exact packed/cached oracle.

No original test was deleted or weakened. The retained tests prove answer values, wire identity, requests and error kind; the new relation checks compare every emitted pair with the input order. No test-only hook was added. I searched the existing public batch and corpus adapters for a reusable singleton option path; one small helper serves both canonical filter and rank checks. The source ratchet is 97185 nonblank lines: 32 from the separately landed fixture merge and 219 from this adapter slice, including formatting of the three already touched consumer test files to make their affected format check pass. No new production source or duplicated runner was added. The two existing whole-scenario consumer tests needed local `too_many_lines` expectations for strict Clippy; no lint table or global policy changed.

## Focused verification

Offline with the lane 7 lock, empty Rust compiler wrappers and the retained isolated Cargo home/targets:

- `cargo test --locked --offline -p thinkthen --test settings_cases`: 1/1.
- Exact library selector `--list` finds the panic/host-hook test; executing it: 1/1.
- `THINKTHEN_CONFORMANCE_IDS=.../qf-cases.ids cargo test --locked --offline --manifest-path conformance/consumer/Cargo.toml -p consumer --test public --target-dir target/consumer cases::every_applicable_shared_case_passes_through_the_public_api -- --exact --nocapture`: 4 selected, 4 pass, 50 unselected.
- Exact consumer settings and convenience parents: 1/1 each, including the convenience child.
- Strict affected root and consumer Clippy `-D warnings`, `CARGO_NET_OFFLINE=true python3 sdlc/scripts/policy.py`, root `cargo fmt --all -- --check`, affected-file `rustfmt --check`, `node sdlc/scripts/ratchet.mjs`, and `git diff --check`: pass after the final ratchet update.

Final successful integrated runtime and Clippy log: `target/codex-builds/shared-functional-checkpoint-2026-09-28/continuation/qf-final-integrated.log`. Policy, format and ratchet logs sit beside it. The selected consumer corpus test executes exact requests and answers; Clippy and formatting are compile/static checks, not replacements for that execution.

## What the build taught us

The budget row's old count assertion hid a no-key refusal: count rows and inspect the first result and listener sends before changing a budget expectation. Canonical singleton fixtures need an explicit batch choice when the public default becomes packed; changing expected answers or the backend would have concealed the request mismatch. Check the selected test name through `--list` when a script uses an exact filter.

The later spec replay fixture and demo page failures in the shared checkpoint remain with the public documentation owner; this adapter fix neither reruns the full rungs nor claims those failures resolved.
