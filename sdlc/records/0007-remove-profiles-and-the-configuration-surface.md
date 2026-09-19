# Record 0007: Remove profiles and the configuration surface

- Ticket: `sdlc/tickets/0007-remove-profiles-and-the-configuration-surface.md`
- Branch: `ticket/0007-remove-profiles`
- Written: 2026-09-19

## What was removed

Version one has no configuration file, no profile, and no `config` command, as the configuration section of ADR 0010 rules. The backend is `THINKTHEN_API_KEY`, `THINKTHEN_BASE_URL`, and `--model`.

The pages went first.

- `specification/config.md` is deleted. `specification/roadmap.md` gains an entry under "Held by ADR 0010" saying what the file held, why it left, what would bring it back, and that the git history keeps the page and demo 10.
- `specification/backends.md` loses its profile section and its ad-hoc rules. It keeps one rule for the key: the key goes to the address the user named, because naming the address is the user's own act. It gains a rule for `--url` and a short section for `--model`.
- `specification/result.md` drops `meta.profile` and `meta.adapter` from the table and from all four examples.
- `specification/channels.md` drops the removed options from both option lists, prints the four-field plan, and drops `config` from the exit-code line.
- `decide.md`, `choose.md`, `score.md`, `filter.md`, `rank.md`, `annotate.md`, and `find.md` carry one backend-options row naming `--url` and `--model`. `records.md`, `threshold.md`, and `specification/README.md` drop the `config` rows and the dead link.
- `demos/10-another-backend/` is deleted. `demos/README.md` drops its index row, marks demo 01 green, and says 10 and 11 are empty numbers. `demos/FINDINGS.md` drops the findings that belonged to demo 10. Demos 03, 05, 08, 09, and 12 are swept.

Then the code.

- `crates/thinkthen-core/src/adapter.rs` is deleted whole, and the `Adapter` enum and `UnknownAdapterError` with it. `systemone::NAME` is the one name the wire shape answers to.
- `backend.rs` loses the `Profile` row, the `PROFILES` table, `BackendValues`, `resolve_backend`, and five of the seven `BackendError` variants. What is left is `Backend::resolve(url, base, model)`, `DEFAULT_MODEL`, `KEY_VAR`, and two errors: `Blank` and `NotAnAddress`.
- `text.rs` loses `ProfileName` and `KeyVar` and their two `BlankTextError` variants.
- `result.rs`: `Meta::new` takes four values. `plan_document.rs`: the document holds four fields.
- `recording.rs`: `Exchange::new` takes the URL and the request bytes. The digest hashes `systemone::NAME` first, exactly the bytes the adapter name hashed before, so every pinned digest holds.
- `args.rs`: three options are gone. `--model` carries `default_value = thinkthen_core::DEFAULT_MODEL`, so clap fills the default in and the core no longer needs a branch for an absent model.
- `decide.rs` loses two `match backend.adapter()` blocks. `edge::key()` reads `KEY_VAR` and takes no argument. `http::Exchange::key` is a `&Key`. The `Option` around it is gone, because a request that goes out always carries a key.

## Each acceptance bullet and what proves it

| Bullet | Proof |
| --- | --- |
| Each removed option exits 2, with a test | `crates/thinkthen/tests/decide_edge.rs::every_option_the_configuration_surface_carried_is_a_usage_error` over `--profile`, `--adapter`, `--key-env`, `--config`, the old three-flag ad-hoc form, and `--profile` beside `--dry-run`. It asserts exit 2, empty standard output, and clap's `unexpected argument`. `spec/decide.md` runs the same four options through the shell. Red first: the test failed with `left: Some(4), right: Some(2)` on `["--profile", "jev"]` before the option was removed |
| A test pins the fields of `meta` and of the plan document | `crates/thinkthen/tests/backend/recordings.rs::meta_holds_the_url_the_model_the_usage_and_the_replayed_flag` and `crates/thinkthen/tests/decide_edge.rs::the_plan_holds_four_fields_and_names_the_key_variable_without_reading_it`. Both compare the whole printed object, so a field added or reordered fails. Both were written and watched fail before the code changed. The core pins the same two shapes in `result.rs` and `plan_document.rs` |
| The pinned recording digest holds | `crates/thinkthen-core/src/recording.rs::the_digest_of_the_fixture_request_is_the_name_the_entry_keeps`, unchanged, still `bd370a64…4a78.json`. `sdlc/scripts/test` exit 0 |
| `sdlc/scripts/spec` prints `demos: 1 green` with the key unset | `env -u THINKTHEN_API_KEY -u THINKTHEN_BASE_URL sdlc/scripts/spec`, exit 0, `demos: 1 green, 12 red`. Demo 01's recording was never touched, and its page needed no change, because it asserts on no `meta` field |
| A search of the repository for the swept words finds them only in ADRs, records, tickets, the roadmap entry, and tests of removed options | See the section below |
| The ratchet falls, equals the measured total, and the commit says what was deleted | 4520 before, 4168 after the code commit. The leftovers commit raised it to 4192 and defends the raise, and the review fixes raised it by one line to 4193. `sdlc/scripts/lint` exit 0 |
| The whole ladder is green | `install`, `lint`, `test`, `spec` all exit 0. 105 tests, 22 spec examples, one green demo |
| A second agent reviews the public surface change | An independent agent reviewed it and said merge with fixes. It checked the export list before and after, the purity bans, `unwrap`, `expect`, and `panic` outside tests, every path a secret or the evidence could reach a message or a `Debug` line, the digest bytes, the lint tables, and the ticket's own Scope. Its findings are below |

## The swept-word search

```sh
grep -rIn -E -- "profiles|--profile|--adapter|--key-env|--config|THINKTHEN_PROFILE|THINKTHEN_CONFIG|config path|config show|config check" .
```

Run over the whole worktree with `target/` and `.git/` excluded, it finds the words in these places and nowhere else.

- ADRs 0004, 0007, and 0010, which are the decisions and are not edited.
- Records 0003, 0005, and 0006, which are history.
- Tickets 0003, 0005, 0006, and 0007.
- `specification/roadmap.md`, the entry this ticket wrote.
- `crates/thinkthen/tests/decide_edge.rs` and `spec/decide.md`, the tests of the removed options.
- `sdlc/planning/plan.md` and `sdlc/planning/flat-verbs-review.md`. The ticket forbids editing the plan, and the review document is the dated record of the survey that proposed the surface. Neither is a page a user reads, and both describe a decision already overturned in `roadmap.md`. They are left alone on purpose.

No page under `specification/`, no page under `demos/`, no README, no source file under `crates/`, and no script under `sdlc/scripts/` carries any of the words.

The independent review pointed out that this search misses the bare words `profile` and `adapter`, so it was widened and run again.

```sh
grep -rInE -- "profile|adapter|config" specification/ demos/ spec/ README.md crates/ sdlc/scripts/
```

The bare word `profile` survives in three places, and each one is right. `sdlc/scripts/policy.py` reads the `profile` key of `rust-toolchain.toml`, which is a Rust toolchain setting and not a backend. `specification/roadmap.md` holds the entry this ticket wrote. `crates/thinkthen/tests/decide_edge.rs` and `spec/decide.md` prove the option is refused. Two pages that the narrow search had missed were fixed: `specification/records.md` called its rate-limit table "the built-in profile's published limits" and now names the hosted address, and `specification/annotate.md` said the saved question file holds no profile and now says it holds no backend.

The bare word `adapter` survives in the places version one still has one. The adapter is the pair of pure translations that `backends.md` specifies, and a recording entry names the shape it recorded. Nothing selects an adapter and no option names one. `demos/FINDINGS.md` carried a settled finding saying that a URL needs an adapter and a model beside it, which is the rule this ticket deleted, so the bullet now records the address rule that replaced it.

## Choices made where the ticket was silent

Ian can overturn each of these cheaply.

- **`--url` takes no companion, and the key rule does not change when it is given.** `--url` is the command-line spelling of `THINKTHEN_BASE_URL`. It outranks the variable, it names a base, and the request is posted to `BASE/systemone`. The key still comes from `THINKTHEN_API_KEY`. This is the simplest rule and it is the one ADR 0010 already states for the key. `backends.md` says it under "The address" and under "The key". The alternative was to send no key when `--url` is given, which would have made a local server the only thing `--url` could reach and would have contradicted the ADR.
- **The plan document drops `adapter` as well as `profile`, and keeps `key_env`.** The ticket names only `profile`. `adapter` printed a constant once the enum was gone, and `meta` drops it for the same reason ADR 0010 gives. `key_env` stays because it names a real thing, the variable a run would read, and `channels.md` and demo 09 both rest on it as the proof that a key stays home. The plan is `url`, `model`, `key_env`, `request`.
- **`key_env` is never `null` now.** One key variable exists, so the field is always `THINKTHEN_API_KEY`. `channels.md` and demo 09 dropped the sentence that said it could be `null`.
- **The `KeyVar` and `ProfileName` types are deleted.** A constant needs no validation, so `rust-standards.md` asks for no type here. `KEY_VAR` and `DEFAULT_MODEL` live in the core, beside the address rule that uses them, and the binary reads the environment.
- **The recording entry keeps its `adapter` field.** `thinkthen.recording/1` is a file format and the committed recording of demo 01 holds that field. Dropping it would have changed the format and the entry written for the same exchange. The value is `systemone::NAME`, and a replayed entry that names anything else is refused. `backends.md` says why the entry still names the shape.
- **The three test files that share the loopback harness became one test binary, `tests/backend/`.** That is how the harness suppression was deleted. Moving it to each item would have kept it.
- **`systemone::NAME` is `pub(crate)`.** The binary never needs it, because the address and the recording entry are both written inside the core. The old `Adapter::as_str` was `pub(crate)` for the same reason.
- **`BackendValues` is gone.** It carried five flag values. Two arguments and the base do not earn a builder.

## The five review leftovers

All five are fixed. None waits.

1. **`THINKTHEN_LIVE_LEDGER`.** `sdlc/scripts/live` reads `$REPO/sdlc/live-tokens` and no variable moves it. `crates/thinkthen/tests/live_script.rs` copies the script into a tree of its own and runs that copy, so the test reaches its own ledger through the script's own rule. Every case there is a refusal, which happens before the script builds anything.
2. **The empty folder name in `sdlc/scripts/demos`.** The runner now prints `writes --replay with no folder name after it` and exits 1. `crates/thinkthen/tests/demo_runner.rs::a_green_page_that_names_the_folder_in_backticks_stops_the_run` proves it over a new fixture page. Against the old guard the test fails, because the old runner skipped the check and ran the page.
3. **The suppression at the top of `tests/harness/mod.rs`.** It is deleted. The three pages that share the harness compile as one test binary, so every part of the harness is used and `dead_code` has nothing to report. The fix is mechanical: there is no warning left for anybody to silence.
4. **The wrong numbers in record 0006.** The ratchet line and the two demo counts now read 4520 and thirteen. A correction section at the foot of that record says what was wrong and who fixed it.
5. **The message for `--url ""`.** With `--adapter` gone the ad-hoc message is gone too, and a blank base reaches the blank-address message. `crates/thinkthen/tests/backend/address.rs::a_base_the_option_names_as_white_space_is_refused_as_a_blank_address` asserts the exact line `thinkthen: a URL is text, not white space` for `""`, `" "`, and `"\t"`, and `spec/decide.md` asserts it from the shell.

## The ladder

| Rung | Exit | What it reported |
| --- | --- | --- |
| `sdlc/scripts/install` | 0 | |
| `sdlc/scripts/lint` | 0 | `crates is 4193 non-blank lines, ceiling is 4193` |
| `sdlc/scripts/test` | 0 | 105 tests, one of them a documentation test |
| `sdlc/scripts/spec` | 0 | 22 spec examples, `demos: 1 green, 12 red` |

Rungs 1 to 3 were run with `env -u THINKTHEN_API_KEY -u THINKTHEN_BASE_URL`. No live call was made. This ticket needed none.

## Dependencies

None added, none removed.

## What the review found and what was done

The independent review said merge with fixes. It checked the export list of `thinkthen-core` before and after, traced every `thinkthen_core::` import in the binary and its tests, confirmed the core reaches no file, environment variable, socket, clock, or process, found no `unwrap`, `expect`, or `panic` outside tests, walked every path a key or the evidence could reach a message or a `Debug` line, compared the digest bytes, confirmed the two committed recordings of demo 01 are untouched by the diff, and confirmed the lint tables and both `clippy.toml` files are untouched. It ran the three rungs itself with the two variables unset.

Five findings. Four are fixed here.

1. `specification/records.md` called its rate-limit table "the built-in profile's published limits". It names the hosted address now.
2. `demos/FINDINGS.md` still carried the settled finding that a URL needs an adapter and a model beside it. The bullet records the address rule that replaced it.
3. The swept-word search in this record was too narrow to support the claim it made. The section above now carries the widened search and its result.
4. `systemone::NAME` is `pub(crate)`.
5. `AGENTS.md` says a key is read from the variable its backend profile names and is never sent to a host other than its profile's. That contradicts `backends.md` now. The file is not in this ticket's `opens:` list, so the ticket left it alone and `sdlc/issues/2026-09-19-agents-md-still-describes-profiles.md` records the work.

The review also noted `specification/annotate.md` saying the saved question file holds no profile, and judged the sentence harmless. It was reworded anyway, because the word names nothing now.
