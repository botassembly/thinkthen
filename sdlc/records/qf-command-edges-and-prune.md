# Quick Fix qf-command-edges-and-prune: refuse an unknown prune model, bound the timeout, gate the test variables, and check model names

Status: built and checked, waiting for a fresh code review and landing. Owner: Claude. Base: `origin/main` at `18f0381e`, with main merged in at `3c409cbc`. Lane: `thinkthen-lane-4`. Branch: `qf/command-edges-and-prune`.

## Evidence it started from

Local experiment 284, the issue register, reviewed its findings against main at `48fd034f`. Each one below was checked again on `18f0381e`, by reading the code or by running the debug binary with no real key against a dead loopback address in a scratch folder.

- Local experiment 284, file 98. `cache prune --answered-by-other-than MODEL` removed every entry whose reply named another model. The alias guard refused only a name some request asked for. A typo such as `no-such-model` matched no reply and emptied the folder at exit 0. `engine/cache_prune.rs` still held that rule, and `specification/recording.md` stated it. It still held.
- Local experiment 284, file 48. `echo some-evidence | thinkthen decide "t?" --timeout 9223372036854775807 --url http://127.0.0.1:9/v1 --no-cache` panicked with `overflow when adding duration to instant` and exit 101. It still held.
- Local experiment 284, file 49. `Environment::read` in `cli/edge.rs` read `THINKTHEN_TEST_RETRY_WAIT_MS` and `THINKTHEN_TEST_SIGINT_ACK` in every build. It still held.
- Local experiment 284, file 90. `--model ' jev '` and `--model $'jev\nx'` both reached the dry-run request body unchanged. It still held.
- Local experiment 284, file 120. `config.rs` reads the configuration file with no mode or owner check, and no page says so. It still held.
- Local experiment 284, file 106. The address rule checks the authority and the query and keeps the path as typed. `specification/backends.md` did not say that a path token lands in every recording. It still held.
- Local experiment 284, file 89. With `THINKTHEN_API_KEY` unset, a live run against a loopback address stopped with ``the environment variable `THINKTHEN_API_KEY` is unset or blank, so no key is sent``. Only `specification/check.md` said a keyless local server needs some key. It still held.

No finding was dropped.

## Retained behavior

- The alias refusal keeps its sentence, and it still wins when some request asked for MODEL. The upgrade prune still works once the new version has answered one entry. An empty folder still prunes to zero at exit 0.
- `--timeout 0` still exits 2 before key, input, or connection. The default stays 30.
- Every test suite still shortens retry waits and orders SIGINT tests, because the suites spawn the debug binary.
- A blank model keeps its sentences on every command.
- The key rule, the address rules, and the configuration reader do not change.

## Change

1. File 98, `engine/cache_prune.rs`. When the folder holds an entry and no reply names MODEL, prune exits 2 before it deletes anything with ``--answered-by-other-than names a model no reply in the folder names, so prune removed nothing; name the version a result's meta.model shows, or delete the folder to remove every entry``. The alias sentence still wins when some request asked for MODEL. I chose this refusal over a dry run because it is one predicate and makes the typo deletion impossible without changing what prune prints or needing a new flag. The cost is the upgrade prune run before the new version has answered anything. That case now refuses, and the sentence says to delete the folder, since every entry would go anyway. Record 0124 kept that case as a whole-folder removal, so this reverses one line of it. Ian can overturn the choice. A dry run stays a later option for ticket 0124's deferred gaps. `specification/recording.md` and the prune row in `specification/settings.md` say the new rule.
2. File 48, `cli/mod.rs`. `--timeout` takes a whole number from 1 to 86400 on every command, `check` included. Anything else exits 2 with `--timeout takes a whole number of seconds from 1 to 86400`, before key, input, or connection. One day is far above any real attempt and far below the clock arithmetic's limit. The help text, `specification/backends.md`, `specification/check.md`, and the settings row say the bound. `--max-retries` shares no panic, because the retry wait doubles with `saturating_mul` and the retry count stays a `u32`.
3. File 49, `cli/edge.rs`. A new `test_only` reader returns `None` unless the build has debug assertions. Both `THINKTHEN_TEST_RETRY_WAIT_MS` and `THINKTHEN_TEST_SIGINT_ACK` go through it, so a release binary ignores both. `cfg(test)` would not work, because integration tests spawn the binary, where `cfg(test)` is false. `sdlc/planning/rust-standards.md` states the reach beside the variables.
4. File 90, `core/text.rs`. `ModelName` leaves the shared text macro and gets its own `new`. It drops white space around the name, as the address rule drops it around a base, and it refuses a control character inside with `a model name holds no control character`. The rule covers `--model`, the configuration file's `model`, a question file's `model`, and the library builders. The model a reply or a saved result names goes through a new `ModelName::reported`, which keeps its bytes and refuses only a blank name, because a backend reports that name and the mixed-model diagnostic already withholds an unsafe one. The first test rung caught this: `annotate::hostile_model_names_never_reach_the_diagnostic` read `the response names no model` when a reply named `jev-1` plus ESC. Its fourth case asked with a control character in `--model`, which now exits 2 before any request, so that case now asks for `jev-latest` and keeps the hostile reply. `recognize` and `relate` build their own specification, so one `edge::model_flag` helper replaces their two copies and says `--model holds no control character`. A caller that padded a model name now sends the trimmed name, so its cache and recording keys change once. `specification/backends.md` and the model row in `specification/settings.md` say the rule.
5. Files 106, 120, and 89 are page sentences, as each file recommends. `specification/backends.md` says the path is kept as typed and written into every recording, so a secret belongs in `THINKTHEN_API_KEY`. It also says a command reads the key before it sends and a keyless local server still needs some value in the variable. `specification/settings.md` says the tool reads the configuration file whatever its mode and owner, and says to keep it writable by its owner alone on a shared machine.

## Proof

Planted bugs ran once on the fixed tree at `4888c1b7`, and the files were restored before any commit.

| Plant | Test | Result |
| --- | --- | --- |
| Prune's old guard, which refused only the alias | `default_cache::prune_refuses_the_alias_and_an_unknown_model_and_keeps_the_upgrade` | failed at row 2, `jev-1.14.0` before the upgrade answered, with exit 0 in place of 2 |
| The old `timeout == 0` check | `timeout::a_timeout_outside_one_to_a_day_precedes_input_key_and_connection_in_both_argument_homes` | failed at `--timeout 86401`, which reached the absent input and exited 5 |
| The control-character check turned off | `refusals::no_refusal_on_any_command_writes_the_key_quotes_the_evidence_or_sends_anything` | failed at `a-model-with-a-line-break-inside-decide`, which sent a request |

The four questions of `2026-09-24-tests-earn-their-place.md`:

- The prune table turns three rows that removed everything into refusals. It adds a typo beside `--older-than` and an upgrade row that removes only the older answer. It protects the rule that no typo deletes anything. Losing the new predicate fails it. The old table pinned the typo deletion as correct. It spawns the binary and needs no hook.
- The timeout test became one table of 0, 86401, `9223372036854775807`, and `18446744073709551615` over both argument homes, with a `--dry-run` at 86400 that must pass. It protects the bound and the promise that nothing is sent. Dropping the bound fails it. The old test covered zero alone. It needs no hook.
- The refusal table gains `a model with a line break inside`, which runs over every command and checks that nothing is sent or leaked. It found that `recognize` and `relate` mapped every model error to the blank sentence. `a_model_name_drops_surrounding_space_and_refuses_a_control_character` in `core/text.rs` is an edge table of seven inputs. It pins the trim and the control characters the command table does not reach, NUL, DEL, and NEL. Neither needs a hook.
- File 49 has no gate test, because every suite runs the debug build. A hand run proved it instead. A loopback server answered every request with 503, and `THINKTHEN_TEST_RETRY_WAIT_MS=1` was set with a dummy key. The debug binary made three attempts in 0.05 seconds, and the release binary made three attempts in 3.02 seconds, which is the one-second and two-second waits. Both exited 4.

## Size

`sdlc/ratchet.json` rises from 72224 to 72345, 121 lines. `ModelName`'s own constructor takes about 35 lines, because the shared macro would also trim `Url`. The model table and the new refusal row take about 25. The prune table's rows and second sentence take about 25. `ModelName::reported` takes 18. The timeout table, the `test_only` reader, and the `model_flag` helper take the rest. The helper removed two copies of the `--model` mapping, and the timeout table replaced a longer two-case list.

## Checks

With `THINKTHEN_API_KEY` unset. `sdlc/scripts/live` did not run, and no paid call was made.

The rungs ran one at a time at `d4a595f1`, which holds every code change and main merged in at `3c409cbc`. The later commit adds only this record.

- `lint` with the private-names list: exit 0, `ratchet: crates + conformance 72345/72345`. The expected `Killed` line appeared.
- `test`: exit 0, 974 passed, 0 failed, 13 ignored across 38 result lines, `live-test: all cases passed`. An earlier run at `4888c1b7` failed on the hostile reply case that change 4 describes.
- `spec`: exit 0, `demos: 21 green, 0 red`.
- `surfaces`: exit 0, 19 `surfaces: pass` lines, `release smoke` among them. Conformance read `53 passed, 0 failed, 1 not run, of 54`, and the one not run is `25-defect-fault`, which no outside boundary reaches.

## Deferred

- File 98's dry run. Ticket 0124's deferred gaps own it. The refusal makes it less urgent.
- File 120's warning. It rides the folder-writers warning ticket from `sdlc/issues/2026-09-26-folder-writers-decide-the-answers.md`, as the file recommends.
- File 89's loopback exemption. It changes the key rule on the command and the libraries and needs the queue owner's ruling, so only the page sentence landed.
- File 106's audit command. It stays a backlog note until a user asks.
- File 49 has no gate test of the release build. A release smoke with a 503 server would cost more than the proof returns.
- `site/src/pages/reference.astro` still says only that prune refuses the alias. That remains true, and marketing owns the site.
