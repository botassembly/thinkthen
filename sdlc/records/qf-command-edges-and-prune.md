# Quick Fix qf-command-edges-and-prune: refuse an unknown prune model, bound the timeout, gate the test variables, check model names, warn on a shared configuration, and let loopback go keyless

Status: landed. A fresh review accepted the branch at `1db6eeb2`. Owner: Claude. Base: `origin/main` at `18f0381e`, with main merged in at `3c409cbc` and again before the review round. Lane: `thinkthen-lane-4`. Branch: `qf/command-edges-and-prune`.

## Evidence it started from

Local experiment 284, the issue register, reviewed its findings against main at `48fd034f`. Each one below was checked again on `18f0381e`, by reading the code or by running the debug binary with no real key against a dead loopback address in a scratch folder.

- Local experiment 284, file 98. `cache prune --answered-by-other-than MODEL` removed every entry whose reply named another model. The alias guard refused only a name some request asked for. A typo such as `no-such-model` matched no reply and emptied the folder at exit 0. `engine/cache_prune.rs` still held that rule, and `specification/recording.md` stated it. It still held.
- Local experiment 284, file 48. `echo some-evidence | thinkthen decide "t?" --timeout 9223372036854775807 --url http://127.0.0.1:9/v1 --no-cache` panicked with `overflow when adding duration to instant` and exit 101. It still held.
- Local experiment 284, file 49. `Environment::read` in `cli/edge.rs` read `THINKTHEN_TEST_RETRY_WAIT_MS` and `THINKTHEN_TEST_SIGINT_ACK` in every build. It still held.
- Local experiment 284, file 90. `--model ' jev '` and `--model $'jev\nx'` both reached the dry-run request body unchanged. It still held.
- Local experiment 284, file 120. `config.rs` reads the configuration file with no mode or owner check, and no page says so. It still held.
- Local experiment 284, file 106. The address rule checks the authority and the query and keeps the path as typed. `specification/backends.md` did not say that a path token lands in every recording. It still held.
- Local experiment 284, file 89. With `THINKTHEN_API_KEY` unset, a live run against a loopback address stopped with ``the environment variable `THINKTHEN_API_KEY` is unset or blank, so no key is sent``. Only `specification/check.md` said a keyless local server needs some key. It still held.

No finding was dropped. Files 98, 48, 49, 90, 120, and 89 are fixed in code. File 106 is a page sentence only and is not fixed. It needs a design for what a recording stores.

## Retained behavior

- The alias refusal keeps its sentence, and it still wins when some request asked for MODEL. The upgrade prune still works once the new version has answered one entry. An empty folder still prunes to zero at exit 0.
- `--timeout 0` still exits 2 before key, input, or connection. The default stays 30.
- Every test suite still shortens retry waits and orders SIGINT tests, because the suites spawn the debug binary.
- A blank model keeps its sentences on every command.
- A set key still goes only to the address the user named. The address rules and the configuration file's fields do not change.

## Change

1. File 98, `engine/cache_prune.rs`. When the folder holds an entry and no reply names MODEL, prune exits 2 before it deletes anything with ``--answered-by-other-than names a model no reply in the folder names, so prune removed nothing; name the version a result's meta.model shows, or delete the folder to remove every entry``. The alias sentence still wins when some request asked for MODEL. I chose this refusal over a dry run because it is one predicate and makes the typo deletion impossible without changing what prune prints or needing a new flag. The cost is the upgrade prune run before the new version has answered anything. That case now refuses, and the sentence says to delete the folder, since every entry would go anyway. Record 0124 kept that case as a whole-folder removal, so this reverses one line of it. Ian can overturn the choice. A dry run stays a later option for ticket 0124's deferred gaps. `specification/recording.md` and the prune row in `specification/settings.md` say the new rule.
2. File 48, `cli/mod.rs`. `--timeout` takes a whole number from 1 to 86400 on every command, `check` included. Anything else exits 2 with `--timeout takes a whole number of seconds from 1 to 86400`, before key, input, or connection. One day is far above any real attempt and far below the clock arithmetic's limit. The help text, `specification/backends.md`, `specification/check.md`, and the settings row say the bound. `--max-retries` shares no panic, because the retry wait doubles with `saturating_mul` and the retry count stays a `u32`.
3. File 49, `cli/edge.rs`. A new `test_only` reader returns `None` unless the build has debug assertions. Both `THINKTHEN_TEST_RETRY_WAIT_MS` and `THINKTHEN_TEST_SIGINT_ACK` go through it, so a release binary ignores both. `cfg(test)` would not work, because integration tests spawn the binary, where `cfg(test)` is false. `sdlc/planning/rust-standards.md` states the reach beside the variables.
4. File 90, `core/text.rs`. `ModelName` leaves the shared text macro and gets its own `new`. It drops white space around the name, as the address rule drops it around a base, and it refuses a control character, or any white space but a plain space, inside with `a model name holds no control character or white space but a plain space`. The review asked for the white-space half, so U+2028 is refused too. The rule covers `--model`, the configuration file's `model`, a question file's `model`, and the library builders. The model a reply or a saved result names goes through a new `ModelName::reported`, which keeps its bytes and refuses only a blank name, because a backend reports that name and the mixed-model diagnostic already withholds an unsafe one. The first test rung caught this: `annotate::hostile_model_names_never_reach_the_diagnostic` read `the response names no model` when a reply named `jev-1` plus ESC. Its fourth case asked with a control character in `--model`, which now exits 2 before any request, so that case now asks for `jev-latest` and keeps the hostile reply. `recognize` and `relate` build their own specification, so one `edge::model_flag` helper replaces their two copies and says `--model holds no control character or white space but a plain space`. `audit --write` reads the model its results name through `ModelName::new`, splices the trimmed name, and keeps the question's own model with `a result names a model with a control character or white space but a plain space` when the name fails. Before review it used `reported`, so it could write a name every later run refuses. A caller that padded a model name now sends the trimmed name, so its cache and recording keys change once. `specification/backends.md` and the model row in `specification/settings.md` say the rule.
5. File 120, `config.rs`. One shared predicate, `writable_by_another`, is true when others may write a file, the `0o002` bit. The folder-writers warning can reuse it. The review also asked for a warning when another user owns the file, through `nix::unistd::geteuid`. That call needs `nix`'s `user` feature. `sdlc/scripts/policy.py` accepts `nix` with its `signal` feature alone, under ticket 0078, and the lint rung failed on the added feature. Widening that table is a dependency change that needs its own review, so the owner half is left out and deferred. `Config::read` records it, and the command prints `thinkthen: the configuration file is writable by another user; it decides where the key and evidence go` on standard error before it runs. The file is still read. The libraries print nothing. `specification/settings.md` states the rule. The review first asked for the `0o022` mask. With the owner check deferred, that mask warned on every file saved under the usual 002 umask, so the coordinator ruled for `0o002` alone. Ian can overturn that ruling. A file only its group may write gets no warning, so a shared group that can write the file goes unwarned until the owner check lands.
6. File 89, `engine/facade.rs`. The coordinator ruled that a loopback backend needs no key. `Engine::key` wraps the key reader. When it reads no key and `Backend::is_loopback` proves the host is `localhost`, `127.0.0.1`, or `[::1]`, the request goes out with an empty key, and `engine/http.rs` sends no `Authorization` header for an empty key. Every key reader refuses a blank key, so an empty key means only this case. One place covers both key readers, the command's `cli/edge.rs` and the libraries' `public/settings.rs`. Every other address still exits 4 before any connection. `specification/backends.md`, `specification/check.md`, the Key row of `specification/settings.md`, and an amendment to ADR 0010 state it. Five tests that proved the no-key refusal against a loopback listener now name `https://127.0.0.2`, and the exchange copy was deleted as the same contract. The rules cannot prove that host is this machine, and nothing listens there. `core/backend.rs` holds an edge table of the loopback match over `https://` bases. `databases/sqlite/NOTES.md` still says the engine refuses to send with no key. Its fixed test key still works.
7. File 106, a page sentence only. `specification/backends.md` says the path is kept as typed and written into every recording, so a secret belongs in `THINKTHEN_API_KEY`. That is not a fix.
8. Review round. The prune help now says a name no reply in the folder carries is refused, and nothing is removed. The Prune row in `specification/settings.md` says an empty folder accepts any name. `recognize` sent raw reply model names into the mixed-model diagnostic. It now calls `annotate`'s `check_model`, which names both only when `diagnostic_model` finds each safe. `site/src/pages/reference.astro` lines 32 and 267 went to item 8 of `sdlc/issues/2026-09-25-site-samples-and-pages-after-the-surfaces-land.md`.

## Proof

Planted bugs ran once on the fixed tree at `4888c1b7`, and the files were restored before any commit.

| Plant | Test | Result |
| --- | --- | --- |
| Prune's old guard, which refused only the alias | `default_cache::prune_refuses_the_alias_and_an_unknown_model_and_keeps_the_upgrade` | failed at row 2, `jev-1.14.0` before the upgrade answered, with exit 0 in place of 2 |
| The old `timeout == 0` check | `timeout::a_timeout_outside_one_to_a_day_precedes_input_key_and_connection_in_every_argument_home` | failed at `--timeout 86401`, which reached the absent input and exited 5 |
| The control-character check turned off | `refusals::no_refusal_on_any_command_writes_the_key_quotes_the_evidence_or_sends_anything` | failed at `a-model-with-a-line-break-inside-decide`, which sent a request |
| Recognize's old comparison, before review | `secrecy_recognize::a_second_model_a_step_two_reply_names_is_withheld` | failed, printing ``model versions `local-1` and `local-marker-evidence-7b3ac5` `` |
| No key refused at every address, the behavior before review | `address::a_loopback_backend_takes_a_request_with_no_key_and_no_authorization_header` and the moved no-key tests | the first run of the new rule failed seven old tests, each of which saw a request reach loopback |

The four questions of `2026-09-24-tests-earn-their-place.md`:

- The prune table turns three rows that removed everything into refusals. It adds a typo beside `--older-than` and an upgrade row that removes only the older answer. It protects the rule that no typo deletes anything. Losing the new predicate fails it. The old table pinned the typo deletion as correct. It spawns the binary and needs no hook.
- The timeout test became one table of 0, 86401, `9223372036854775807`, and `18446744073709551615` over both argument homes, with a `--dry-run` at 86400 that must pass. It protects the bound and the promise that nothing is sent. Dropping the bound fails it. The old test covered zero alone. It needs no hook.
- The refusal table gains `a model with a line break inside`, which runs over every command and checks that nothing is sent or leaked. It found that `recognize` and `relate` mapped every model error to the blank sentence. `a_model_name_drops_surrounding_space_and_refuses_a_control_character` in `core/text.rs` is an edge table of seven inputs. It pins the trim and the control characters the command table does not reach, NUL, DEL, NEL, and U+2028. Neither needs a hook.
- The timeout table gains `check --timeout 86401`, because `check` keeps its own field. It needs no hook.
- `audit_write::keeps_a_model_a_later_run_would_refuse` gives results naming `jev-1.13.0` plus BEL. It pins both report lines and a question file with the threshold written and no model. `reported` would have spliced the name.
- `secrecy_recognize::a_second_model_a_step_two_reply_names_is_withheld` answers step two with a model holding the evidence marker. It pins the withheld sentence. No test covered recognize's mixed-model path.
- `cache_configuration::a_configuration_another_user_can_write_is_warned_about` is a table of four modes, 0666 and 0602 warning and 0664 and 0600 quiet. The 0664 row pins the ruling that a group bit alone is quiet. The owner half is deferred.
- `address::a_key_that_is_unset_or_empty_is_exit_four_away_from_loopback` pins the exact no-key sentence at `https://127.0.0.2` and zero connections. `address::a_loopback_backend_takes_a_request_with_no_key_and_no_authorization_header` pins one request with no header for an unset, empty, and blank key.
- File 49 has no gate test, because every suite runs the debug build. A hand run proved it instead. A loopback server answered every request with 503, and `THINKTHEN_TEST_RETRY_WAIT_MS=1` was set with a dummy key. The debug binary made three attempts in 0.05 seconds, and the release binary made three attempts in 3.02 seconds, which is the one-second and two-second waits. Both exited 4.

## Size

`sdlc/ratchet.json` stands at 72630, 308 lines over main's 72322. The first round took 121. `ModelName`'s own constructor takes about 35 lines, because the shared macro would also trim `Url`. The model table and the new refusal row take about 25. The prune table's rows and second sentence take about 25. `ModelName::reported` takes 18. The timeout table, the `test_only` reader, and the `model_flag` helper take the rest. The helper removed two copies of the `--model` mapping, and the timeout table replaced a longer two-case list. The review round took 164, one of them the mask ruling's longer doc comment. The last round took 23: the loopback table in `core/backend.rs` and the wrapped audit reason. The loopback rule takes about 25 in `is_loopback`, `Engine::key`, and the header switch, and its address tests replaced the exchange copy. The configuration warning takes about 15 for the predicate and the print and 25 for its table. Recognize's comparison gave way to `check_model`, and its secrecy test takes about 40. Audit's check and test row, the `check` timeout row, and the U+2028 row take the rest.

## Checks

With `THINKTHEN_API_KEY` unset. `sdlc/scripts/live` did not run, and no paid call was made.

The rungs ran one at a time at `896d91a5`, which holds every change of the three rounds, the mask ruling, and main merged in. The later commit adds only this line. An earlier full run at `69cbcde2`, before main moved, gave the same results.

- `lint` with the private-names list: exit 0, `ratchet: crates + conformance 72630/72630`. The expected `Killed` line appeared. An earlier run at `7c42cdba` failed twice. `nix` had gained its `user` feature, and `engine/http.rs` had 509 non-blank lines over its 500 ceiling. Change 5 and `d837dec2` answer both.
- `test`: exit 0, 980 passed, 0 failed, 13 ignored across 38 result lines, `live-test: all cases passed`.
- `spec`: exit 0, `demos: 21 green, 0 red`.
- `surfaces`: exit 0, 19 `surfaces: pass` lines, `release smoke` among them. No conformance line reports a failure.

## Deferred

- File 98's dry run. Ticket 0124's deferred gaps own it. The refusal makes it less urgent.
- File 106. It needs a design for what a recording stores. The page sentence stays and fixes nothing.
- File 120's owner half, and with it a warning for a file its group may write. It needs `nix`'s `user` feature and a change to the dependency policy in `sdlc/scripts/policy.py`, with its own review.
- The folder-writers warning of `sdlc/issues/closed/2026-09-26-folder-writers-decide-the-answers.md` can reuse `writable_by_another`.
- File 49 has no gate test of the release build. A release smoke with a 503 server would cost more than the proof returns.
- `site/src/pages/reference.astro` lines 32 and 267. Marketing owns the site, and item 8 of the site samples issue asks for both.
- `databases/sqlite/NOTES.md` still says the engine refuses to send with no key.
