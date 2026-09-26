---
flow: build
priority: 126
opens: crates/thinkthen/src/cli crates/thinkthen/src/core/check.rs crates/thinkthen/src/engine/mod.rs crates/thinkthen/src/engine/width_tests.rs crates/thinkthen/src/public/error.rs crates/thinkthen/tests specification spec README.md site/src/pages/backends.astro databases/duckdb/README.md databases/postgresql/README.md databases/sqlite/README.md sdlc/planning/adr/0010-one-wire-shape-two-variables-and-a-smaller-version-one.md sdlc/planning/interface-audit.md sdlc/planning/ten-use-cases.md sdlc/issues sdlc/ratchet.json sdlc/records sdlc/tickets
---

# 0126: Fix command wording, help, and wrong doc claims

Status: landed 2026-09-25 (`sdlc/records/0126-build-wording-help-and-doc-claims.md`). Code review accepted after doc fixes. Owner: Claude.

Review route: a fresh read-only Claude session reviews this design and the final diff. The `check` report and the `recognize --dry-run` output change shape, so a second fresh Claude reviewer checks the code as well. Codex does not review this ticket unless Ian routes it.

## Outcome and authority

Every sentence a user reads should match what the tool does before 0.1.0. This ticket fixes the help, refusals, printed reports, and doc lines that say something else today.

It settles these items:

- `sdlc/issues/2026-09-25-command-wording-and-help-fixes-before-0-1.md`, items 2, 5, 6, 7, and 9, and the `recognize` half of item 8.
- `sdlc/issues/2026-09-25-docs-how-tos-and-spec-claims-owed.md`, section "Spec or doc claims that are wrong", items 1, 2, and 3.
- `sdlc/issues/2026-09-25-status-sees-only-command-spend-and-the-sql-total-has-three-leaks.md`, the docs half. That is option 3 of the issue, the one its recommendation takes now.

The authority is backlog `sdlc/planning/issue-backlog-2026-09-25.md`, rows A7 and A8 and the docs half of A5, and the 0.1 queue in `sdlc/planning/one-line-plan-2026-09-25.md`. Ian ruled on item 6 on 2026-09-25: `check` shows the model the user asked for, the model each reply names, the provider, the URL, and all its outputs, and it prints "unspecified" when no model name is given. Ian ruled on item 2 on 2026-09-24: the ten functions and `help` come first, and the admin commands come last.

Other in-flight tickets own the rest. This ticket touches none of their items:

| Item | Owner | Why it waits |
| --- | --- | --- |
| Wording 1: a huge deadline prints hundreds of digits | 0122 pandas | `libraries/python/tests/test_inputs.py:84` pins the long form, and 0122 owns `libraries/python` |
| Wording 3: `relate --either @FILE` names the wrong cause | 0123 relate | 0123 owns relate |
| Wording 4: relate help lists `--jobs` | 0123 relate | 0123 owns relate's option set |
| Wording 8, the `relate` sentence | 0123 relate | 0123 owns relate's help |
| Docs 4: `find --details` unit ids | 0125 audit, diff, and find | 0125 owns find's words |
| Status spend, the code half | the ticket after pandas 0122 | The plan puts library spend after 0122 |

## Design

Each change reuses a path that exists. No change reaches the adapter, the decoder, the transport, the retry rule, or the public Rust API.

### Help order (item 2)

The six admin subcommands get a clap `display_order` above clap's own `help` subcommand. Clap gives `help` 999 and numbers the rest from 0 in enum order. The admin commands take 1000 to 1005, in the order the issue asks: `audit`, `diff`, `status`, `check`, `cache`, `transform`. The ten functions keep their enum order, since the ruling lets the team keep another order when they sit together. The builder may set the order on the six variants or with `mut_subcommand` on the root command. The enum does not move, so parsing does not change.

`tests/version.rs` already pins the whole root order in its `ORDER` array. This ticket edits that array to the new order and adds no test. `tests/audit_refusals.rs:289` and `tests/diff.rs:90` each pin a slice of the root help that includes its neighbors, and the new order breaks both. Each assert is rewritten to find its own row, the name and its whole description, anywhere in the root help. Each still pins its description and no longer pins order. These two lines are the only lines this ticket changes in 0125's files. Whichever of 0125 and 0126 lands second merges them.

### The engine throttle sentence (item 5)

`WidthActive`'s `Display` in `engine/mod.rs` says throttle: `throttle N is already active for this process; use throttle N or drop the throttle argument`. `public/error.rs` then prints `active.to_string()` in place of its own copy. One sentence lives in one place. The command and every surface print it.

Two tests pin the sentence: `tests/public_env.rs` at the public layer, and `cli/schedule/width_tests.rs` at the command. `engine/width_tests.rs` stops pinning the text and asserts the `WidthActive` value it gets, so one contract is not pinned at three layers.

### What `check` prints (item 6)

The report gains a header and one line per decoded reply. A full pass at `/arm/full/v1` with no model given prints:

```text
url http://127.0.0.1:PORT/arm/full/v1/systemone
provider systemone
model asked unspecified
model sent jev-latest
reply noul {"model":"jev-latest","answers":[ANSWER],"usage":{"input_tokens":1,"output_tokens":1}}
reply choice {...}
reply score {...}
reply mixed {...}
ok connection
...
critical 0, warning 0
```

- `provider` names the wire interface the tool speaks through. The build prints `core::adapters::systemone::NAME`, never the crate's `core::NAME`, which is `thinkthen`. Today that is always `systemone`. It does not name who runs the server. The tool knows no fact about that.
- `model asked` prints what `--model` or the configuration file's `model` named. With neither, it prints `unspecified`.
- `model sent` prints the model the requests carry. That is the asked model, or `jev-latest` when none was asked. The wire rule still needs a model in every request. Ian did not rule on option 2, an optional model.
- `reply PROBE JSON` prints one line for each probe whose reply decoded, in probe order. JSON is one compact object: `model` is the model that reply names, `answers` lists each logical question's decoded answer in plan order, and `usage` is the reply's token counts or `null`. An answer prints as the result object prints `answer`. A failed logical question prints as the result object prints its failure. A probe that met an error status, a transport failure, or a refused reply prints no reply line. Its finding row says why.
- The check still does not compare the asked and answered names, and a mismatch is no finding. An alias such as `jev-latest` answering as a version is normal.
- The reply JSON goes through the core's one JSON writer, so a control character in a server's model name prints escaped.
- `--dry-run` prints `url`, `provider`, `model asked`, and `model sent`, then its four `request` lines. It sends nothing, so it prints no reply line.

Reply lines print the decoded reply. They never print raw reply bytes. A finding still names a question only by its wire name.

`core/check.rs` keeps each decoded reply beside its row. `cli/check.rs` prints the header and the reply lines. The `check` help gains one sentence: the report names the model asked for, the model sent, and the model each reply names.

No loopback arm changes, and `conformance/backend/src/arms.rs` is not touched. Every arm echoes the request's model, so the test that tells the sent model from the answered one uses `Listener::serving` with four canned replies, one per probe, each naming `other-1`. The limits and keeping tests already use this listener.

`specification/check.md` changes line by line:

| Line | Today | After |
| --- | --- | --- |
| 5 | Four fixed requests, then the eight rows | Adds one sentence: the report opens with the address, the provider, and the models, and prints each decoded reply |
| 16 | `--model` resolves from the option, the file, then `jev-latest` | Adds: `model asked` prints the option or the file's value, or `unspecified`. `model sent` prints the resolved value |
| 60 | A finding never quotes a reply. The check does not compare models | Keeps both sentences. Adds: a reply line prints the decoded reply as JSON, never raw bytes, and names the model that reply names |
| 64 to 78 | The full pass shows `url`, `model`, and the rows | The full pass shows `url`, `provider systemone`, `model asked unspecified`, `model sent jev-latest`, four exact `reply` lines, then the rows |
| new, after 78 | none | One paragraph: `provider` names the wire interface the tool speaks, `systemone`, and not who runs the server |
| new, after 78 | none | The reply-line table from this ticket's edge cases |
| 90 | `--dry-run` prints `url` and `model`, then four `request` lines | `--dry-run` prints `url`, `provider`, `model asked`, and `model sent`, then four `request` lines, and no reply line |

### The recognize dry run (item 7, option 1)

`recognize --dry-run` keeps its counts and adds the requests, as `relate --dry-run` prints them. The line becomes:

```text
{"schema":"thinkthen.recognize-plan/1","url":"...","model":"jev-latest","key_env":"THINKTHEN_API_KEY","words":4,"detection_questions":4,"kind_questions":4,"request_count":1,"requests":[{"digest":"...","bytes":N,"body_utf8":"..."}]}
```

- `schema` names the plan, as `thinkthen.relate-plan/1` names relate's.
- `url`, `model`, and `key_env` come first after `schema`, as in relate's plan. They cost at most 9 nonblank lines, because the backend is already in hand. If they cost more, the build drops them and records them as a gap. `schema` stays either way.

- `tokens` becomes `words`, because it counts words and not model tokens. A word is what `tokenize` in `core/recognize.rs` returns. The text splits at white space. Each trailing `.`, `!`, `?`, `,`, `:`, or `;` of a piece then counts as its own word. `Ada met Acme.` is four words: `Ada`, `met`, `Acme`, and `.`. `specification/recognize.md` states this definition beside the field.
- `requests` becomes `request_count`, and `requests` now holds the list. These are relate's names for the same two things.
- Each request carries its recording `digest`, UTF-8 `bytes`, and exact `body_utf8`. They come from the same `facade::split` call the count already uses.
- `from`, `relation_pairs_upper_bound`, and `relation_requests_upper_bound` keep their meaning. `requests` prints last.
- The request entry is a small struct of three fields. Relate has the same struct in `cli/relate/dry_run.rs`. This ticket copies it into `cli/recognize/dry_run.rs` and does not touch relate code, because 0123 owns it. A shared helper is a gap to close after 0123 lands.
- An empty text prints `"request_count":0,"requests":[]`.

`specification/recognize.md`, `spec/recognize.md`, and `specification/channels.md` change in the same commit. `channels.md` gains one sentence beside its `relate` sentence: `recognize --dry-run` reports its counts and every exact split request for the first record.

### Cost and connection sentences (items 8 and 9)

- The `recognize` help gains: `Each record makes paid requests: a detection question for every word, a kind question for every word when two or more kinds are given, and relation questions when rules are given. --dry-run prints the exact requests for the first record.` "Word" has the meaning the dry-run section defines, so a trailing `.` gets its own detection question. The spec states the kind rule at `specification/recognize.md:7`.
- The `--jobs` help and `specification/records.md` gain: `A run opens up to one connection for each request in flight, so --jobs N opens up to N connections.`

### Repeat answers are not the same (docs item 1)

- ADR 0010 gains a dated amendment. It keeps the 2026-09-19 measurement as history. It cites experiment 212: 63 of 100 repeated requests moved, with a mean of 0.02 among those that moved and at most 0.08. The fifty borderline messages, 0.33 to 0.67, moved by up to 0.08. The fifty others moved by at most 0.03. Four answers flipped at 0.5, all between 0.43 and 0.51. It carries 212's limit: one question, one set, one hundred messages, one day, and half the sample picked as borderline, so 63 of 100 overstates an ordinary file. It cites experiment 259: 178 of 681 repeated digests held different answers, all from `jev-1.13.0`, with gaps up to 0.09. It withdraws the sentence "The same request returns the same number."
- `specification/recording.md:78` is replaced with the measured play from those two experiments: a mean move of 0.02, at most 0.03 away from the middle, and up to about 0.08 to 0.09 near it, on one model version. It names both experiments and 212's sampling limit.
- The conflict message in `cli/failure/recording.rs` becomes: ``the backend answered the request in entry `NAME` differently from the saved response; record into a fresh folder, or use --cache DIR to answer from the saved entries``. It still exits 5.
- The `--record` help gains: `A folder that already holds an answer stops at exit 5 when the backend answers that request differently.`
- `specification/threshold.md` gains the 0.09 figure and one sentence: a band narrower than about 0.1 on each side of a cut does not keep a flip out.

### The interface audit (docs item 2)

`sdlc/planning/interface-audit.md` gains a dated line at the top. It names rows 31, 32, 38, 66 and line 87 as history, with the ticket that landed each change, and says the rest stand. The builder checks each named row against the binary before writing the line.

### The 300 ms claim (docs item 3)

The builder searches `probes/`, `sdlc/records/`, and the workspace experiments for the run that measured one call over 300 ms. If a record holds it, `README.md:31` and `sdlc/planning/ten-use-cases.md:14` cite it. If none does, both drop the number and keep the sentence about a process start. Workspace experiment folders are outside this repository, so a citation names the experiment number, as `records.md` already does for experiment 206.

### Status counts only command spend (status issue, docs half)

- `specification/recording.md`, the `status` paragraph, and `README.md:27` say plainly: `thinkthen status` counts only what the command sends. A library, SQL extension, or data frame keeps its counts in memory for its own process, and `status` never sees them.
- `site/src/pages/backends.astro:49` says the same in one sentence.
- Each database README says `status` never sees its spend, beside its request total. It then names the three ways calls pass the total: DuckDB `thinkthen_warm` sits outside it, PostgreSQL counts it per backend process so a pool of N connections can spend N times it, and calls running at the same time can each spend what remains. DuckDB and SQLite already name some of these, and this ticket adds only the missing ones. These are README sentences only. `databases/` code belongs to 0129.

## Decisions

Each is the agent's decision. Ian can overturn any of them.

1. **`provider` names the wire interface.** The tool speaks one interface and knows nothing else about who runs a server. Printing the host would repeat the URL.
2. **The check prints what it asked, what it sent, and what came back.** `model sent` stays because `unspecified` alone would hide the `jev-latest` the requests carry.
3. **"All its outputs" means every decoded reply.** Each reply line holds the answering model, every answer, and the usage. A user who brings a backend can then see what it said, not only whether it parsed. The reply is printed decoded, so no raw server bytes reach the terminal.
4. **The recognize dry run takes relate's names.** `request_count` and `requests` mean the same thing in both plans. This renames one field a script may read. The tool is before 0.1.0, and no surface or test outside the command reads it.
5. **Admin commands follow the issue's order.** `audit`, `diff`, `status`, `check`, `cache`, `transform`, after `help`. The ten functions keep their current order.
6. **The throttle sentence lives in the engine.** The public layer prints the engine's sentence, so the two cannot drift again.
7. **The conflict message names the common cause.** Experiment 259 shows the backend's own variation causes it without any model change. The stop itself stays, because the spec decides it.
8. **Status spend stays a doc fix now.** The issue recommends option 3 now and option 1 as a ticket after the surfaces. The plan places option 1 after pandas 0122.

## Edge cases

`check` header lines:

| `--model` | Configuration `model` | `model asked` | `model sent` |
| --- | --- | --- | --- |
| absent | absent | `unspecified` | `jev-latest` |
| absent | `local-1` | `local-1` | `local-1` |
| `local-2` | `local-1` | `local-2` | `local-2` |
| blank | any | exit 2, as today | nothing sent |

`check` reply lines:

| Probe outcome | Reply line | Finding row |
| --- | --- | --- |
| Decoded, every answer good | yes | `ok` or a warning |
| Decoded, one logical question failed | yes, with that answer as a failure | `critical`, as today |
| Decoder refused the reply | no | `critical`, as today |
| Error status or transport failure | no | `critical`, as today |
| Not reached after a stop | no | `unchecked` |
| Reply names another model than sent | yes, with the reply's model | no finding |

`recognize --dry-run`:

| Input | `words` | `request_count` | `requests` |
| --- | --- | --- | --- |
| `Ada met Acme.` | 4 | 1 | one entry |
| `Ada met Acme.` with one kind | 4, with `kind_questions` 0 | 1 | one entry |
| Empty text | 0 | 0 | `[]` |
| A text the profile splits | N | more than 1 | one entry per split request, in send order |
| Record mode | the first record's counts | as the first record | the first record's requests |

## Proof

Every test runs the built command against the loopback backend or with no backend at all, under a fake key or none. The four questions of `CLAUDE.md` are answered in the columns: what it protects, the credible regression that fails it (the plant), and why no existing test catches it. The fourth answer, the test-only hook it needs, is "none" for every row. Each row drives the real binary or the real public layer.

| Test | Protects | Plant that turns it red | Why no existing test catches it |
| --- | --- | --- | --- |
| `tests/version.rs`: its existing `ORDER` array changes to the ten functions, then `help`, then `audit`, `diff`, `status`, `check`, `cache`, `transform`. No new test | Ian's help order | Drop `display_order` from `status`. It lists first again | The array pins today's order. `audit_refusals.rs` and `diff.rs` stop pinning order and keep their description pins |
| `tests/backend/check.rs`: every existing report test pins its new header and reply lines | The four new line kinds on every path | Print no reply line for a partial reply. The missing-answer test loses its `reply mixed` line | The existing tests pin today's two-line header |
| `tests/backend/check.rs`, new: `--model local-1` against `Listener::serving` with four canned replies naming `other-1` prints `model asked local-1`, `model sent local-1`, and `"model":"other-1"` in each reply line | The answered model comes from the reply | Print the sent model in the reply line. It reads `local-1` | Every current arm echoes the sent model, so no test can tell the two apart |
| `tests/backend/check.rs`, new: with a configuration file naming `local-1` and no `--model`, `model asked local-1`. With neither, `model asked unspecified` and `model sent jev-latest` | `unspecified` means nothing was named | Print the resolved model as asked. The no-model case reads `jev-latest` | No test runs `check` with a configuration file |
| `spec/check.md`: the dry run's first four lines and its line count | The dry-run header | Leave `provider` out of the dry run. The line count reads 7 | It pins today's two header lines |
| `tests/backend/recognize.rs`: a dry run and a `--details` live run of the same text at the generic arm. The dry run's `requests[].digest` list equals the live result's `meta.requests`. `words` and `request_count` are pinned | The dry run shows the bytes a live run sends | Build the dry-run plan without the kind questions. The digests differ | Today's dry run prints no request, so nothing ties it to a live run |
| `spec/recognize.md`: the `Ada met Acme.` dry run with its new fields | The page a user copies | Keep the key named `tokens`. The page fails | It pins the old names |
| `cli/schedule/width_tests.rs` pins the throttle sentence at the command. `tests/public_env.rs` keeps its pin at the public layer. `engine/width_tests.rs` asserts the value and drops its text pin | One throttle sentence at every layer | Put `width` back in the engine `Display`. The command pin fails. Give `public/error.rs` its own copy with `width`. The public pin fails | The command pin holds the old word today. No new test |
| `tests/backend/recording_conflicts.rs`: pin the whole new conflict sentence in place of `contains("already records a different response")` | The message names the fresh-folder and `--cache` routes | Drop the `--cache` clause. The pin fails | Today's check is a substring |
| Help pins in `tests/decide_edge.rs`: the `--jobs` connection sentence, the `--record` conflict sentence, the recognize cost sentence, and the new `check` help sentence, each as one whole sentence | The four new help sentences | Remove any sentence. Its pin fails | None of the four exists yet |

The doc-only changes have no test. `sdlc/scripts/spec` runs the two `spec/` pages above. The reviewer checks each doc line against its source: experiment 212's and 259's figures, the audit rows against the binary, and each database README against the issue's three leaks.

The secrecy helper in `tests/backend/check.rs` already asserts the key reaches no stream or file on every run. It covers the new lines unchanged.

## Budgets

In nonblank lines added, net of removals.

- `crates/thinkthen/src/cli/check.rs` and `core/check.rs` together: at most 50.
- `crates/thinkthen/src/cli/recognize/dry_run.rs`: at most 30.
- `cli/args/command.rs` and `cli/args.rs`: at most 20.
- `engine/mod.rs`, `public/error.rs`, and `cli/failure/recording.rs`: at most 5.
- Tests under `crates/thinkthen/tests` and the two unit test files: at most 170.
- `spec/` pages: at most 15. `specification/` pages: at most 40. Other docs, ADR, and README lines: at most 40.
- No dependency. No public API change.
- The ratchet rises to the measured total in the code commit, at most 280. That commit names what grew and where the builder looked first for duplication to delete. The public error's throttle copy goes. Relate's request struct in `cli/relate/dry_run.rs` stays duplicated until after 0123, as the recognize dry-run section says.

## Stop rules

Stop and report before any of these:

- Crossing a budget above.
- A change to the adapter, decoder, transport, facade, or public Rust API.
- A line in `libraries/python`, `databases/*/src`, `conformance/backend/src/arms.rs`, or relate, cache, audit, diff, or find code. The two root-help assertions in `tests/audit_refusals.rs` and `tests/diff.rs` are the only exception.
- A check reply line that needs raw reply bytes, or a reply field the decoder does not keep.
- A recognize dry-run digest list that differs from the live run's. That means the dry run and the live run split differently. Report it and do not paper over it.
- A doc number with no source. Cut the number instead.
- Any live or paid call. `sdlc/scripts/live` never runs for this ticket.

## Scope and exclusions

Excluded: the items the ownership table names. A request-entry helper shared with relate, until 0123 lands. The docs issue's "Pages owed for 0.1" and later sections. The status code half. An optional model in the request. A strict check mode or a `--json` check report. Escaping C1 control characters in a model name, which the JSON writer leaves as is here and in every result.

## Dependencies and order

Build from `origin/main`. It needs nothing from 0122 to 0125 or 0127 to 0129. For every overlap below, whichever ticket lands second merges:

- 0125: the root-help asserts at `tests/audit_refusals.rs:289` and `tests/diff.rs:90`, as the help-order section says.
- 0123: the `--jobs` help at `cli/args.rs:152`. This ticket adds the connection sentence there, and 0123 hides `--jobs` from relate's help.
- 0123: the recognize cost sentence in `cli/args/command.rs` sits next to the relate help, which 0123 owns and whose own cost sentence 0123 writes.
- 0123: `cli/check.rs` and `public/error.rs`, where 0123 changes the 400 reason.
- 0124: the `--record` help in `cli/args.rs` sits next to the `--cache` and `--no-cache` help that 0124's cache wording may change.
- 0124: `cli/failure/recording.rs`, where 0124 changes the mismatch arm beside the conflict arm, and `specification/recording.md`.
- 0127: `databases/sqlite/README.md` and `tests/version.rs`.
- 0128: `README.md`.
- 0129: `databases/sqlite/README.md` and `databases/duckdb/README.md`.

## Issues this closes

Landing closes no issue whole. The lander edits each issue in the landing commit:

- The wording issue keeps only items 1, 3, 4, and the relate half of item 8, and names their owners.
- The docs issue loses items 1 to 3 of "Spec or doc claims that are wrong" and keeps item 4 for 0125.
- The status issue gains a line that the docs half landed in 0126, and keeps option 1 open.

## What Ian can overturn

Each decision above. The reading of "provider" and of "all its outputs" matter most. Cutting either is a small change to `cli/check.rs` and the spec pages. The field rename in the recognize dry run is the one change a script could notice.

## Complexity

Contract 2; state and timing 0; reach 2; proof 1; cost of error 1; total 6. Level 1. The risk is a doc or help line that states a new wrong thing. Each line cites its source, and the reviewer checks each one.

## Evidence

- Starts from: The three issues above, checked against main at `a95474be` and `44de5c8b`. Ian's rulings of 2026-09-24 and 2026-09-25 in the wording issue and the backlog. Experiment 212's `RESULTS.md`: 63 of 100 repeated requests differed by up to 0.08, four flipped at 0.5, all from `jev-1.13.0`. Experiment 259's `challenge/CHALLENGE.md`: 178 of 681 repeated digests held different answers, gaps up to 0.09. Experiment 218, wave 1, area 5: 30 to 35 open descriptors at `--jobs 32` and 7 at `--jobs 4`. Ticket 0121 and `specification/check.md` for the check. `cli/relate/dry_run.rs` for the request list.
- Keeps: Every exit code. What the engine answers. The check's probes, rows, findings, and stop rules. No comparison of asked and answered models. The recording conflict stop. The enum order and parsing of commands. Every test pin except the ones named above.
- Changes: The help order. The engine's throttle word. The check header and its reply lines. The recognize dry-run schema, fields, and request list. Four help sentences and one refusal. The spec, ADR, audit, README, and site lines named above.
- Proof: The ten rows under "Proof", each with a planted fault that turns it red. `spec/check.md` and `spec/recognize.md` run under the spec rung. The gate ladder runs with the real key unset.
- Defers: Wording items 1, 3, 4, and the relate half of 8 to 0122 and 0123. Docs item 4 to 0125. Status spend in code to the ticket after 0122. The docs pages owed. An optional model in every request, which needs Ian's ruling. C1 control characters in a model name. One request-entry helper shared by the recognize and relate dry runs, after 0123 lands.
