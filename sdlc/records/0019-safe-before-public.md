# 0019: Safe before public

Branch `ticket/0019-safe-before-public`. Built 2026-09-19.

## What landed

The findings of the security and command-line review of 2026-09-19 are closed. Six things changed.

**Plain `http://` reaches loopback and nothing else.** `address` in `crates/thinkthen-core/src/backend.rs` reads the host out of the authority and takes `localhost`, `127.0.0.1`, and `[::1]` alone under `http://`. `BackendError::KeyInClear` carries the sentence, and no option overrides it. `https://` is untouched. The rule is a pure function, it resolves no name, and it reads text alone, so `localhost.`, `127.1`, `0.0.0.0`, `[::ffff:127.0.0.1]`, and `localhost.example.com` are all refused.

**A record over 16 MiB is refused.** `Reading::record` measures the record after the line that ended it is dropped, and before the bytes are read as text, so a huge record that is also not text is too large first. It is exit 2 for that record, before any request. `Chunks` in `crates/thinkthen/src/edge.rs` reads no more than two bytes past the limit, so a stream with no line feed in it never reaches this process's memory whole.

**A person at a terminal is told what the command waits for.** With no `--input` and a terminal on standard input, one line goes to standard error. It never appears in a pipe, in a redirection, or under `--input`.

**The build checks its dependencies.** `deny.toml` allows exactly the seven licenses the tree carries and fails on a license nothing in the tree offers. Rung 1 runs `cargo deny --offline check advisories bans licenses`, rung 0 fetches the advisory database and refuses to pass without the checker.

**The release profile is stated and held.** `[profile.release]` sets `overflow-checks = true` and `panic = "abort"`, and `policy.py` holds it to an accepted copy the way it holds the lint tables.

**The workflow pins what it runs.** Both actions carry a commit SHA with the tag in a comment, and the downloaded `jq` is checked against a recorded SHA-256.

## The secrecy sweep

The coordinator asked for one shared helper that proves no secret reaches any output, over every command and every failure path. `secrecy::nothing_leaked` is that helper. It reads standard output, standard error, and every file a run wrote. The key may reach no byte of any of them, and no file may hold the word `authorization` or `bearer` either. The evidence may reach no byte of a diagnostic.

- `crates/thinkthen/tests/backend/secrecy.rs` drives every verb down sixteen backend paths, on one document and over records, in the bare view and under `--details`. Each row pins its exit code and the number of requests the listener saw.
- `crates/thinkthen/tests/backend/refusals.rs` drives thirty rows of usage error and local failure over every verb that has them. Each row pins the part of the message that names it and no other refusal.
- `crates/thinkthen/src/failure.rs` holds the two the integration sweep cannot reach: every `Debug` line that could hold either marker, and every diagnostic `report` writes.

A command enters both sweeps by adding one row to `VERBS` in `secrecy.rs`.

Three narrower tests went: `address::the_key_comes_from_thinkthen_api_key_and_reaches_nothing_but_the_header`, `exchange::the_key_is_sent_as_a_bearer_token_and_never_printed`, and `http::tests::an_exchange_shows_neither_the_key_nor_the_evidence_it_carries`. The sweep proves what each proved, and `secrecy::the_key_reaches_the_authorization_header_and_nothing_else` pins where the key does go, so the sweep cannot pass on a run that sent no key at all.

## Red then green

- **The address rule.** `error[E0599]: no variant or associated item named KeyInClear found for enum BackendError`, then one existing case failed with `a base names an address: KeyInClear`, because it used `http://host/v1` as a taken spelling.
- **The record limit.** `error[E0425]: cannot find value MAX_RECORD_BYTES in module super`.
- **The read bound.** The first chunk read 67,108,864 bytes where the case asked for 16,777,218.
- **The terminal line.** `error[E0425]: cannot find function waiting_on_terminal in module super`.
- **The refusal table.** Pinning the message caught two rows at once: `a-base-that-is-no-address-decide` was reaching clap's `the argument '--url <URL>' cannot be used multiple times` rather than the refusal it named, because the driver added the listener's base beside the base the row typed. Both sweeps now add the listener's base only when the row names none. Without the message assertion, twelve rows would have passed on the wrong error.

## The choices made where the pages were silent

Each of these is written into the page or the code that states it, and Ian can overturn any of them.

1. **The loopback set is three exact spellings, compared without regard to case.** The ticket named `localhost`, `127.0.0.1`, and `[::1]`. Everything else is refused, including spellings that do resolve to loopback. Resolving a name inside the rule would put a resolver in the pure core and would make the answer depend on the machine's DNS. `backends.md` says so.
2. **A refused address keeps its own sentence.** `KeyInClear` is a second variant of `BackendError` beside `NotAnAddress`, not a second table. Ticket 0013's table of refusal sentences is `failure.rs::refused`, which holds the option clashes that carry no payload. An address error already had its sentence in the core enum, and the binary prints it through the one `error.to_string()` arm that was already there.
3. **A port must be digits or absent.** `http://localhost:x/v1` read its host back as `localhost` and passed the rule. A port that is not a number is no address, so it is refused as `NotAnAddress` before the scheme is read. The reviewer's prompt asked for this class of attack and the rule now covers it.
4. **An authority holding a second colon outside brackets is no address.** A bare IPv6 address with no brackets cannot be split into a host and a port without guessing, and this rule guesses nothing.
5. **The limit is measured on the record and not on the line.** A line of exactly 16 MiB ending in `\r\n` is a 16 MiB record and is judged. The two bytes that ended it were never part of it.
6. **The read bound is the limit plus two bytes.** One byte past the limit settles the refusal, and the second allows a record at the limit to arrive with `\r\n` after it.
7. **The terminal line is written before the run starts reading.** `main` writes it, because the line belongs to every verb and `Command::common` gives `main` the shared options whichever verb was named.
8. **The terminal line says "Ctrl-D".** It names the key a person presses, not the byte it sends.
9. **`script` is the pseudo-terminal the check runs on.** The check tries the util-linux spelling and then the BSD one, so Linux and macOS both work, and it fails rather than skipping when neither does. `sdlc/scripts/install` names the tool, so a machine without it fails at rung 0. A hidden test-only variable that pretended standard input was a terminal would have tested the message and not the wiring.
10. **The advisory database is fetched from rung 0.** Rung 1 reads it offline. Rung 0 is the one rung that already reaches the network, and this keeps rung 1 runnable on a laptop with no network. The first push proved the need: `cargo deny --offline` on a runner with no database failed with `failed to get 'FETCH_HEAD' metadata`.
11. **`unused-allowed-license = "deny"`.** A license in `deny.toml` that nothing in the tree offers fails the rung, the way `policy.py` already fails on a license exception whose crate left. The allow list cannot rot into a blanket allowance.
12. **`allow-wildcard-paths = true`.** The one wildcard in the tree is the path dependency between the two crates in this workspace. Every other wildcard fails.
13. **`multiple-versions = "allow"`.** The tree holds two versions of `syn`, `getrandom`, and `windows-sys` through crates it does not choose. Failing on that would fail on a transitive graph nobody here controls, and `policy.py` already holds the direct set.
14. **The release profile is checked by `policy.py`.** A profile stated in a file and checked by nobody is a remembered rule.
15. **The refusal sweep pins a fragment of each message rather than the whole sentence.** The whole sentence is pinned once, in `failure.rs` and in the core's own tables. The fragment is there so a row cannot reach some other refusal at the same exit code and pass.

## What the recordings do

Every committed recording stores an address under `http://127.0.0.1:` or `https://api.typesafe.ai/v1/systemone`. Both are addresses the new rule takes, so no committed recording holds an address a user could no longer name.

Replay never applies the rule to a stored address. `Entry::replayed` compares the stored `url` against the address the run resolved, as text, and the address rule runs only over the base the user gave. A recording written before the rule therefore replays unchanged. The `spec` rung proves it: every demo marked green replays its committed recording, and `probes/replay-check.sh` reproduces every probe's committed rows from its own recording.

## The pinned versions and SHAs

| Thing | Value | Where it came from |
| --- | --- | --- |
| `actions/checkout` | `fbc6f3992d24b796d5a048ff273f7fcc4a7b6c09`, tag v5.1.0 | `git ls-remote --tags https://github.com/actions/checkout.git` on 2026-09-19 |
| `actions/cache` | `caa296126883cff596d87d8935842f9db880ef25`, tag v5.1.0 | `git ls-remote --tags https://github.com/actions/cache.git` on 2026-09-19 |
| `jq` | `jq-1.7.1`, SHA-256 `5942c9b0934e510ee61eb3e30273f1b3fe2590df93933a93d7c58b81d19c8ff5` | `sha256sum` of the downloaded `jq-linux-amd64`, checked against `sha256sum.txt` in the same release. The two agree |
| `cargo-deny` | 0.19.4 | Already installed on the build machine. `sdlc/scripts/install` and the workflow name the same version |
| `mustmatch` | 0.1.0 | Unchanged from before this ticket |

No advisory fires on any crate in the lock file, so nothing had to be bumped.

## The review

A second agent with fresh context read the ticket, the diff against `origin/main`, and "What reviewers keep finding". It was asked to get around the address rule and around the record limit. Its verdict on the first pass was **not ready**, with three findings that had to change and three observations that did not.

**1. A proxy variable defeated the whole address rule.** `ureq` reads `ALL_PROXY`, `HTTPS_PROXY`, `HTTP_PROXY`, and `NO_PROXY` on its own. A run against a loopback base with one of them set sent the key and the evidence to the proxy's host in clear text, which is the exact thing the rule refuses. Fixed. `Backend::is_secure` says whether the resolved address is an `https://` one, and `Client::new` calls `proxy(None)` when it is not. `address::a_proxy_variable_carries_no_plain_http_request` runs a real proxy listener and counts its connections: three before the fix, zero after. The paragraph on `backends.md` said only that a proxy carries the request, which read as an aside while it in fact cancelled the paragraph above it. It now states the rule.

**2. The tail of a record past the read bound was framed as a record of its own.** The reader stops two bytes past the limit. A stream whose line ran longer than that refused the first chunk for its size and then sent what followed the cut to the backend, which is part of a refused record. Fixed. A stream read that found no line feed now ends the stream. `limits::the_tail_of_a_record_past_the_bound_is_never_framed_as_a_record` sends one line of 16 MiB and 12 bytes and counts zero requests on the listener.

**3. Two diagnostics quoted bytes the tool did not write.** `EntryError::Malformed` and `DecodeError::Malformed` each carried a `serde_json` message, and that message quotes the value the reading stopped on. A recording entry is written around the evidence, and a backend can send back whatever was sent to it, so both printed the evidence into a diagnostic. The reviewer found the first. The second is the same defect in the same crate and is fixed with it. Both variants now carry the line and the column. `EntryError::Unwritable` carries nothing. A reply that names no model became its own variant, `DecodeError::NoModel`, because a blank name is not a JSON position. The sweep gained two rows: a recording entry damaged after it was written, and a reply that is JSON no adapter reads. Both quote the evidence marker and both are refused with a message that names a place.

The existing "a malformed answer" row sends `{"model":"","answers":{}}`, which is valid JSON, so it never reached `DecodeError::Malformed`. That is why the sweep had not caught the second one.

Three observations stand with no change, and Ian can overturn any of them.

- **`http://localhost:/v1` is refused as `NotAnAddress`.** An empty port after a colon is a legal URL under RFC 3986 and this rule refuses it. A rule that refuses more than it must is the safe direction here, and the message names the rule rather than the address.
- **A 16 MiB record costs about 104 MB of resident memory.** The bytes are read once, read again as text, and encoded into a request body. The limit is a bound on the damage, not a budget. Nothing in the ticket asked for a streaming encoder, and adding one would be a larger change than this ticket carries.
- **`mustmatch` is pinned by version and not by hash.** It is installed from a source the workflow already trusts. Pinning it by hash is worth a ticket of its own and is not this one.

## The gates

`install`, `lint`, `test`, and `spec` all exit 0. 224 tests. `demos: 13 green, 7 red`. The count is a count of pages marked green, and every one of them passes. It fell from 15 because the merge from `origin/main` brought ticket 0018's reorganization of the how-to list. The ceiling went from 9025 to 10484.

The ticket stays at `in progress`.
