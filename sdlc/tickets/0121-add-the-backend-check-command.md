---
flow: build
priority: 121
opens: crates/thinkthen/src/core crates/thinkthen/src/cli crates/thinkthen/tests/backend conformance/backend/src conformance/README.md specification spec sdlc/ratchet.json sdlc/records sdlc/tickets
---

# 0121: Add the backend check command

Status: landed on main at `cad9237a` (`sdlc/records/0121-build-backend-check.md`). Owner: Claude.

Review route: a fresh read-only Claude session reviews this design and the final diff. Codex does not review this ticket unless Ian routes it.

## Outcome and authority

A user names an address and sets a key. `thinkthen check` then answers one question: does this backend work with ThinkThen? It checks the connection, the key, the one endpoint, and every request and reply field ThinkThen uses. It reports each finding as critical or warning. It exits 0 only when nothing is critical. A script or a list of backends can then trust the exit code.

The ask is the issue `sdlc/issues/2026-09-24-a-backend-compliance-check.md`. Ian ruled on 2026-09-24 that the check goes into 0.1. The issue lets Ian overturn the command name and the split between critical and warning.

## Design

`thinkthen check` sends four fixed requests to `BASE/systemone`, one after another. Each request is built by the production question grammar and written by the production `systemone` encoder. The engine sends it with the production transport, retry rule, and key rule. The production decoder reads each reply. The check adds no second encoder, decoder, or transport. A critical finding therefore means the same thing a real run would meet, and it carries the sentence a real run would print.

The pure core gains `core/check.rs`. It holds the four probe plans and one grader. The grader takes a probe and its decoded reply and returns findings. It touches no socket or environment. The command gains `cli/check.rs`. It resolves the address, the model, and the timeout. It builds one engine with no cache, no recording, and no replay. It builds each probe's one chunk with `Engine::split` and sends that chunk with `Engine::ask_chunks`. It maps an engine error to a row, prints the report, and returns the exit code. Both facade methods survive on the 0086 branch. `--dry-run` prints the body of the same chunk from the same `Engine::split` call. The bytes it prints are the bytes a live check sends.

The check stays in the command. The public Rust API that ticket 0086 builds gains nothing. Its inventory is frozen by 0084, and a library caller can run the command.

## Decisions

Each is the agent's decision. Ian can overturn any of them.

1. **The name is `thinkthen check`.** `status` does not fit. ADR 0034 rules that status sends nothing, and it lists offline status among the rulings that stand. `check` is one flat word beside `status` and `audit`. `thinkthen backend check` was weighed and dropped, since one subcommand does not earn a group. `probe` was dropped because `probes/` already names live measurements.
2. **The check sends only to an address the user named.** The address comes from `--url`, then `THINKTHEN_BASE_URL`, then the configuration file's `url`, as every command resolves it. The built-in default address is refused. The check would otherwise spend requests at the hosted service when the user named nothing. The refusal exits 2 before key access and sends nothing. A user who wants the hosted service names it with `--url`.
3. **The key rule is unchanged.** The key comes only from `THINKTHEN_API_KEY`. An unset or blank key exits 4 with the sentence every command prints, before any request. The check never prints, logs, hashes, or records it. A keyless local server still needs some key set, as it does for every other command today.
4. **Four requests, one per wire shape.** ThinkThen posts to one endpoint. The issue's three endpoints are three question shapes on it, plus the several-question request that `tag` and `annotate` send. Each probe carries every field form its shape can carry. The table below fixes them.
5. **The check stops after a failure that no request body causes.** A transport failure, or status 401, 402, 403, or 404, stops the check. Every later request would meet the same failure and spend a request. Any other failure belongs to the probe that met it, and the check goes on to the next probe. Status 400 and 422, a retried status after the last retry, and a refused reply all go on.
6. **Critical means a ThinkThen function fails.** Warning means a reply leaves out a field ThinkThen reads but can do without. The table below lists every finding. The check does not compare the model a reply names with the model sent. The hosted service answers a `jev-latest` request as `jev-1.13.0`. An alias is normal and proves nothing. Every result already records the model that answered in `meta.model`.
7. **One output form.** The report is fixed text lines on standard output. A `--json` form waits until a consumer asks for one.
8. **The same retry and timeout rules as every command.** `--timeout` is accepted. `--max-retries` keeps its default of 2 and is not accepted. Four probes send at most twelve attempts.
9. **The check counts in the usage totals.** Its attempts and reported tokens are real spend. They count through the engine's counters like any live request.
10. **No cache, recording, replay, or profile.** A cached or replayed check proves nothing about the backend. `--cache`, `--no-cache`, `--record`, `--replay`, and `--profile` are unknown options and exit 2.
11. **`--dry-run` prints the four request bodies.** It reads no key and sends nothing. A server author can then see the exact bytes. The bodies come from the same `Engine::split` call the live path sends.

## Command line

```text
thinkthen check [--url BASE] [--model NAME] [--timeout SECONDS] [--dry-run]
```

The check reads no standard input and no evidence from the user. `--model` resolves as it does for every command: the option, then the configuration file's `model`, then `jev-latest`. The address rules of `specification/backends.md` apply unchanged, through `Backend::resolve`. They cover the clear-text loopback rule, the proxy rule, and the safe refusals.

The refusal for no named address reads exactly:

```text
thinkthen: check needs an address you name: give --url, set THINKTHEN_BASE_URL, or set url in the configuration file
```

That is the whole standard error line. The `thinkthen: ` prefix comes from the one failure reporter, so the message itself begins at `check`.

## The four probes

Every probe carries the same short made-up evidence about a parcel delivery. It holds no user data. The evidence text is `The parcel arrived on Tuesday and the box was intact.` The model is the resolved model.

| Probe | Serves | Logical questions, in the question-file grammar | Field forms it covers |
| --- | --- | --- | --- |
| `noul` | `decide`, `filter`, `rank`, and the yes/no questions of `recognize` and `relate` | `decide` `Did the parcel arrive undamaged?` with `true` `The text says the box or its contents were intact.` and `false` set to `null` | Text `state`. String `instructions`. `criteria.true` as a string. `criteria.false` as a present `null` |
| `choice` | `choose`, `find`, and the choice questions of `recognize` and `relate` | `choose` `Which day did the parcel arrive?` over options `Monday` with no description, `Tuesday` described as `The second working day of the week.`, and `Wednesday` described as `{"what":"The middle of the week","examples":["midweek"]}` | `criteria` as a map with a `null`, a string, and an object description |
| `score` | `score` | `score` `How well was the parcel packed?` over the level map `fair` to `null`, `good` to `The box was dented but the contents were fine.`, and `excellent` to `{"what":"The box was intact","not_for":"a dented box"}` | `criteria` as an array holding a `null`, a string, and an object. The `mixed` probe's level list sends bare level names |
| `mixed` | `tag`, `annotate`, and the many-question requests of `recognize` and `relate` | One question set over the object state `{"note":"The parcel arrived on Tuesday.","box":"intact"}`: `decide` `Did it arrive on time?`, `choose` `Which day?` over `Monday` and `Tuesday`, `score` `How was the packing?` over the level list `poor` and `good`, and `tag` `Which labels fit?` over `on_time` described as `{"what":"It arrived when promised"}` and `damaged` described as `The box or its contents were harmed.` | Object `state`. Five wire questions of three types in one request. A `noul` with no `criteria`. Bare level names in a score `criteria` array. The structured tag form: array `instructions` and a `criteria.true` object |

The `mixed` probe sends five wire questions. The one second backend tried so far caps a request at sixteen. A threshold never travels on the wire. No probe checks one. The reply fields every probe checks are `model`, `answers`, each answer's `type`, `noul`, `probabilities`, and `confidence`, and `usage`.

`specification/check.md` prints the four exact request bodies at the default model. The code review checks them against this table.

## Findings

The report has eight rows, always in this order: `connection`, `key`, `endpoint`, `noul`, `choice`, `score`, `mixed`, `usage`.

| Row | Level | When | Sentence after `LEVEL ROW: ` |
| --- | --- | --- | --- |
| `connection` | critical | The first probe meets a transport failure | The transport sentence every command prints, such as `the backend refused the connection; check that it is running and that --url is correct` |
| `key` | critical | The first probe meets status 401, 402, or 403 | The status sentence every command prints, such as `the backend answered with status 401: the key was refused` |
| `endpoint` | critical | The first probe meets status 404 | `the backend answered with status 404: nothing answers at this address` |
| probe | critical | Any other error status after the allowed attempts | The status sentence every command prints |
| probe | critical | The decoder refuses the whole reply | `the reply was refused: ` and the decoder's sentence, as every command prints it |
| probe | critical | The reply fails one logical question and keeps another | ``the answer to questions `qA` to `qB` failed as `CAUSE` ``, once per failed logical question. `qA` to `qB` is the wire range that logical question covers, and a one-wire question reads ``the answer to question `qA` failed as `CAUSE` ``. The decoder folds a tag's labels into one failure. The range is all the check can know. CAUSE is the failure cause the result JSON already names |
| probe | critical | A later probe meets a transport failure or status 401 to 404 | The same sentence as the first-probe rows. The check then stops |
| probe | warning | A `choice` or `score` answer carries no `confidence` | ``the answer to question `qN` carries no confidence`` |
| `usage` | warning | Any decoded reply carries no `usage` | `a reply carries no token counts, so results and usage totals leave them out` |

A row with no finding prints `ok ROW`. A row the check did not reach prints `unchecked ROW`. The first probe decides `connection`, `key`, and `endpoint`. A reply of any status marks `connection` ok. Any status but 401, 402, 403, and 404 also marks `key` and `endpoint` ok. After a stop, every row not yet decided prints `unchecked`. So a first-probe stop marks `noul` unchecked, and a 404 leaves `key` unchecked. `usage` is ok when every decoded reply carried it. It is unchecked when no reply was decoded.

The first three to five lines after `model` for each first-probe stop:

| Stop | Lines |
| --- | --- |
| Transport failure | `critical connection: SENTENCE`, then `unchecked` for `key`, `endpoint`, and every later row |
| 401, 402, 403 | `ok connection`, `critical key: SENTENCE`, then `unchecked` for `endpoint` and every later row |
| 404 | `ok connection`, `unchecked key`, `critical endpoint: SENTENCE`, then `unchecked` for every later row |

A finding never quotes a reply. It names a question by its wire name and nothing the backend sent back.

## Output and exit codes

A full pass prints exactly these lines, with the resolved address and model:

```text
url http://127.0.0.1:PORT/arm/full/v1/systemone
model jev-latest
ok connection
ok key
ok endpoint
ok noul
ok choice
ok score
ok mixed
ok usage
critical 0, warning 0
```

A finding prints `critical ROW: SENTENCE` or `warning ROW: SENTENCE` in place of `ok ROW`. A row with several findings prints one line each, in wire order. The last line counts the finding lines.

| Exit | When |
| --- | --- |
| 0 | The report holds no critical line. Warnings may stand |
| 2 | A usage error: no named address, an address the rules refuse, a blank model, a zero timeout, or an unknown option. Nothing is sent |
| 4 | The report holds a critical line, or the key is unset or blank. An unset key prints nothing on standard output. `specification/channels.md` gains one clause for the first meaning |
| 5 | Standard output could not be written |
| 70 | A defect |

The check prints the report once, after the last probe. On SIGINT the check prints no report. It follows the interrupt rule of `specification/channels.md` through the same engine cancel every command uses. No new test covers it.

`--dry-run` prints the `url` and `model` lines and then one `request PROBE BODY` line per probe, with the exact body. It exits 0.

## The loopback arms it adds

`conformance/backend` gains two arms. None changes an existing arm.

| Base path | Answer |
| --- | --- |
| `/arm/full/v1` | The generic answer, plus `"confidence":0.9` on every choice and score answer and `"usage":{"input_tokens":1,"output_tokens":1}` |
| `/arm/status/CODE/v1` | Status CODE with body `status arm`, for CODE 401, 402, 403, or 404. Any other value gets the drift status and `the status arm takes 401, 402, 403, or 404` |

The generic arm already sends no `confidence` and no `usage`, so it drives the warning rows. The reset, refuse, and malformed arms drive the rest.

## Acceptance

Every test lives in `crates/thinkthen/tests/backend/check.rs`. Each starts its own in-process backend and runs the built command with `THINKTHEN_API_KEY=sk-check-0121`. The command runs through the existing backend harness in `tests/backend/harness`. It clears the environment and sets a temporary `HOME`. No configuration file speaks, and the usage totals land in that temporary folder. One helper wraps the harness. It asserts that the key's bytes appear in neither standard output nor standard error, on every run in the file. Each test pins the whole standard output, with only the port replaced.

| Test | Proof | Planted bug that turns it red |
| --- | --- | --- |
| Full pass | `/arm/full/v1`: the exact report above, exit 0, backend count 4 | Drop the `mixed` probe. The count reads 3 and the `ok mixed` line is gone |
| Warnings keep exit 0 | `/generic/v1`: `warning` lines for `confidence` on `choice` q1, `score` q1, `mixed` q2 and q3, and one `usage` warning. Exit 0 | Count a warning as critical. The exit reads 4 |
| Broken answers | `/arm/malformed/missing_answer/v1`: `noul`, `choice`, and `score` each print the decoder's missing-answer sentence. `mixed` prints ``the answer to questions `q4` to `q5` failed as `missing_answer` `` beside its `confidence` warnings for `q2` and `q3`. Exit 4, count 4 | Report a partial reply as ok. The `mixed` critical line is gone and the exit reads 0 |
| Body refusals go on | `/arm/refuse/v1`: four probe rows print the 422 sentence, exit 4, count 4 | Stop after the first critical probe. The count reads 1 |
| Stops | A table over `/arm/status/401/v1`, `402`, `403`, `404`, and `/arm/reset/v1`. Each pins the whole report as the stop table above gives it, `unchecked usage` included. Exit 4, count 1 | Go on after a key failure. The count reads 4 |
| Named address only | `THINKTHEN_BASE_URL` names backend A and `--url` names backend B. A counts 0 and B counts 4. With neither set, the exact refusal on standard error, empty standard output, exit 2 | Send to the environment address. A counts 4 |
| No key | The key unset: the key sentence on standard error, empty standard output, exit 4, count 0 | Send before reading the key. The count reads 1 |
| Dry run | The key unset and `--url` naming the backend: exit 0, count 0, and the four body lines match `specification/fixtures/check/requests.jsonl`. The live path sends the same chunk bodies, as the Design section fixes | Read the key in a dry run. The exit reads 4 |

Also:
- `spec/check.md` runs the dry run with a loopback `--url` and the no-address refusal as executable pages. Neither needs a key or a network.
- The existing secrecy sweep in `tests/backend/secrecy.rs` stays as it is. It sweeps evidence through judgment verbs, and the check reads no evidence. The helper above carries the check's secrecy rule instead.
- Each test answers the four questions of `CLAUDE.md`. Each row above names the behavior and the regression. No existing test runs `check`. None needs a test-only hook: the arms are the real boundary, and `THINKTHEN_TEST_RETRY_WAIT_MS` already shortens retry waits.
- The pure grader gets no unit tests of its own. The command tests cover every row, and a second layer would test one contract twice.

## Specification pages

- New `specification/check.md`, Settled by this ticket: the command line, the four probes and their exact bodies, the findings table, the output, and the exit codes. It says plainly what the check cannot see. A backend that accepts a description and ignores it passes.
- New `specification/fixtures/check/requests.jsonl`: the four bodies at the default model, one per line.
- `specification/README.md` gains the row for `check.md` and the command.
- `specification/recording.md` adds `check` to the commands that create no default cache.
- `conformance/README.md` gains the two arm rows.
- `specification/channels.md` gains one clause on exit 4 for a check report with a critical line.
- `spec/check.md`, as above.

## Budgets and the ratchet

- `crates/thinkthen/src/core/check.rs`: at most 110 nonblank lines.
- `crates/thinkthen/src/cli/check.rs`: at most 140 nonblank lines.
- Argument and dispatch wiring in `cli/args` and `cli/mod.rs`: at most 30 nonblank lines.
- `conformance/backend/src/arms.rs`: at most 30 nonblank lines added.
- `crates/thinkthen/tests/backend/check.rs`: at most 260 nonblank lines.
- `specification/check.md` at most 110 lines. `spec/check.md` at most 40 lines. Other pages at most 20 lines added in total.
- No dependency.
- The ratchet rises to the measured total in the commit that adds the code, at most 570. That commit says what grew and why. It names where the builder looked for duplication first: the failure sentences in `cli/failure.rs`, the address and model resolution in `cli/status.rs`, and the engine build in `cli/asking.rs`.

Stop and re-score before crossing a budget, adding a dependency, changing an existing arm's answer, or touching the public API, the facade, or the adapter.

## Scope and exclusions

Excluded: any change to the adapter, the decoder, the transport, the retry rule, or the address rules. The build crossed the adapter exclusion and the core budget. `sdlc/records/0121-build-backend-check.md` says how and why. Any change to `status`. A `--json` form. A strict mode that fails on warnings. Probing a backend's limits. Any live or paid call. `sdlc/scripts/live` never runs for this ticket.

## Dependencies and order

Build after ticket 0086 lands. 0086 moves `cli/config.rs` to the crate root and reshapes `cli/annotate.rs` and the facade around `split` and `ask_chunks`. This ticket touches the same command wiring and the ratchet. It needs nothing from 0086's public layer. It needs the arms of 0092 and 0117, which are on main.

## Routing

Builder: Claude (Opus subagent). Reviewer: a fresh Claude session for design and for code. A new command widens the public surface and raises the ceiling, so the code review names what it checked.

## Complexity

Contract 2; state and timing 1; reach 2; proof 2; cost of error 2; total 9. Final level: 2. The risk is a check that passes a backend a real run fails on. Reusing the production encoder, engine, and decoder is the guard.

## Evidence

- Starts from: The issue `sdlc/issues/2026-09-24-a-backend-compliance-check.md`. `experiments/220-thinkthen-second-backend/` in the workspace outside this repository, closed in `sdlc/issues/2026-09-21-a-second-backend-tried-through-the-systemone-adapter.md`. There a local open decision model behind a small shim answered the exact request bytes with no change here. It capped a request at sixteen questions and refused oversize text with 422. The decoder ignored its extra reply fields. ADR 0034 keeps `status` offline. `specification/backends.md` fixes the wire shape, the key, the address, retries, and the failure sentences. The loopback backend of tickets 0092 and 0117 supplies the generic, reset, refuse, and malformed arms. The 0086 branch at `09d81bc9` keeps `Engine::split` and `Engine::ask_chunks` and freezes the public inventory.
- Keeps: Offline `status`. The key and address rules. The retry rule and its default. Every failure sentence. The decoder and its distribution tolerance. No cache, recording, or replay for a check. The existing arms' answers.
- Changes: A new `check` command, the first that sends requests with no evidence from the user. Two new loopback arms. A new specification page and fixture.
- Proof: The eight tests in "Acceptance", each with a planted bug that turns it red. `spec/check.md` runs the dry run and the refusal. Every test uses a loopback backend and a fake key. The gate ladder runs with the real key unset.
- Defers: Seeing whether a backend uses a description, a criterion, or an option order. One reply cannot show it, and a model-dependent probe would flake. Probing limits on questions, options, and request bytes, which ADR 0032 profiles state. A `--json` form and a strict mode. Running the check against the open servers the issue names. That is the marketing session's work after this lands, and it needs no change here.

## Review

- Design review: `sdlc/records/0121-design-review.md`. The first pass found ten items. The confirmation at `10427e1d` accepted the design with three builder notes, applied in the next commit. The first fix version dropped the `model` warning and its arm, builds the `score` probe as the grammar allows, names a failed tag by its wire range, pins the stop reports and the refusal line, runs tests in the harness's temporary home, and ties the dry run to the live chunks.
