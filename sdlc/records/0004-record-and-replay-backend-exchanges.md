# Record 0004: Record and replay backend exchanges

- Ticket: `sdlc/tickets/0004-record-and-replay-backend-exchanges.md`
- Branch: `ticket/0004-record-replay`
- Landed: 2026-09-19

## What was built

`--record DIR` and `--replay DIR` work on `decide if` as `specification/recording.md` describes, and the `spec` rung now runs every green demo.

The core gained one module.

- `recording.rs`: `Exchange`, `Digest`, `Entry`, and `EntryError`. An exchange is an adapter, a URL, and the request bytes. Its digest is the SHA-256 of the adapter name, a newline, the URL, a newline, and the request bytes, in lowercase hex written from a nibble table rather than by a crate. An entry is the file one exchange is recorded in. Neither type shows a body in its `Debug` line, because a request body carries the evidence.
- `adapter.rs`: `Adapter::as_str`, which the digest reads. It is crate-private, because nothing outside calls it.
- `result.rs`: `meta.replayed`, always present, last in `meta`.

The binary gained one module.

- `recorder.rs`: the folder as files. It creates the folder when absent, writes each entry to a temporary name inside the folder and then renames it, and reads an entry back by digest. Every `std::fs` call in this ticket is in this file.

`decide.rs` now encodes once, names the exchange, tries the recording, and only then reads a key. `failure.rs` gained five variants and the fixed phrase table.

## The two carry-over fixes

**A profile named only by the environment.** `resolve_backend` now knows which source each value came from. The table it follows:

| `--backend` or `THINKTHEN_BACKEND` | URL source | Outcome |
| --- | --- | --- |
| Flag | Flag | Usage error |
| Flag | Environment | Usage error |
| Flag | Neither | The named profile |
| Environment | Flag | The ad-hoc backend, and the name is dropped |
| Environment | Environment | Usage error |
| Environment | Neither | The named profile |
| Neither | Flag or environment | The ad-hoc backend |
| Neither | Neither | The built-in profile |

A flag beats an environment variable, so an environment name yields to a URL given by flags. When both come from the environment neither beats the other, so it stays the usage error it was. A URL from flags that lacks its adapter or its model is still an incomplete ad-hoc backend, and the yielded name does not rescue it.

**The fixed phrases.** The six statuses `specification/backends.md` names now print their phrase after the code, as `the backend answered with status 402: the account has no credit`. Any other status prints the code alone. The phrases are written in `failure.rs`, so the rule against printing a response body still holds.

The first documentation lines of `edge.rs` and `failure.rs` no longer join two jobs with "and".

## Red then green

Each test below was watched failing for the reason given.

- `a_name_beside_a_url_is_read_from_the_source_each_one_came_from`: failed at `an environment name yields to an ad-hoc backend given by flags: NameWithUrl`, before the resolver knew which source offered a value.
- `a_common_failure_status_carries_the_phrase_the_specification_fixes`: failed with `left: "thinkthen: the backend answered with status 401\n"` against the phrase the table owes.
- `the_digest_of_the_fixture_request_is_the_name_the_entry_keeps`: the digest was computed outside this program first. Dropping the newline between the adapter and the URL failed it with `left: "4ce638ea…json"` against `right: "bd370a64…json"`.
- `the_two_options_over_one_folder_are_a_cache_that_calls_once`: letting a miss fail even when `--record` names the same folder failed it with `left: Some(5), right: Some(0)` on run 0.
- `a_failed_exchange_is_never_recorded`: recording before the adapter reads the body failed it with `left: Some(70), right: Some(4)`, because the unread body is not a JSON the entry can hold.
- `a_replay_under_the_built_in_profile_reads_no_key_and_opens_no_connection`: reading the key before the recording failed it with `left: Some(4), right: Some(0)`, because `TYPESAFE_API_KEY` is unset in that test.

Three more were watched failing the same way. Taking a named `--replay` folder on trust let the missing-folder page run and print `running 01-no-recording/README.md`. Running every page whatever its status says turned the twelve red demos loose. Dropping the URL comparison in `Entry::replayed` let a file recorded for another host answer.

## What an entry file looks like

`crates/thinkthen/tests/fixtures/demos/01-replay-gate/recording/668ef335….json`, written by the binary and verified against `sha256sum`:

```json
{
  "schema": "thinkthen.recording/1",
  "adapter": "systemone",
  "url": "http://127.0.0.1:8721/v1/systemone",
  "request": {"state":"The blender arrived with a cracked jug. Please put the money back on my card.\n","model":"local-1","questions":{"q1":{"type":"noul","instructions":"the customer asks for money back"}}},
  "response": {"model":"local-1","answers":{"q1":{"type":"noul","noul":0.94}},"usage":{"input_tokens":41,"output_tokens":6}}
}
```

The five fields each sit on their own line. The two bodies do not: `serde_json` writes a `RawValue` through the pretty printer verbatim, so an embedded body keeps the bytes it arrived as. Re-indenting one would mean parsing it into dynamic JSON, which the core bans, or writing a JSON reformatter, which is more code than the prettiness is worth. The bytes are stable, the request compares by bytes on replay, and a diff still shows which field changed. `specification/recording.md` said the file is "spread over lines with sorted, stable formatting". That line is now a plain description of what is written.

## The `--plan` decision

`--plan` with `--record` or `--replay` is a usage error, exit 2, with the message `--plan sends nothing, so it takes neither --record nor --replay`.

The alternative was to accept both options and ignore them. ADR 0006 already carries the finding that options doing nothing in the chosen mode should be usage errors, and demo 09 already says in its own words that a plan needs no recording. A silently ignored `--replay` would let a user believe a recording was consulted when nothing was read. The cost is that a script cannot pass the same options to a plan run and a judging run. That cost is small, because a plan run already drops `--min-prob` and `--status` from its meaning. Ian or the steering agent can overturn this.

## The demo runner

`sdlc/scripts/demos` runs every `demos/NN-name/README.md` whose status line reads exactly `Status: green` and skips every red one. It takes another root as its one argument, which is how its tests hand it fixture pages. `sdlc/scripts/spec` runs `mustmatch test spec` and then calls it.

`mustmatch` already runs a block from the folder of the page it came from, so the runner adds no directory change. That was proved by running the fixture page from the repository root and from `/tmp`, both of which passed, and by a perturbation that no test could tell apart. The redundant subshell was deleted rather than kept.

Three fixture roots under `crates/thinkthen/tests/fixtures/` prove the runner, and `crates/thinkthen/tests/demo_runner.rs` drives all four cases.

- `demos/01-replay-gate/` is green and passes. Its recording entry was written by the binary against a local listener, and its digest was recomputed by hand with `sha256sum` outside the program.
- `demos-wrong/01-wrong-assertion/` is the same page with one expected exit code changed. The run exits 1 and says `1 failed`.
- `demos-missing/01-no-recording/` is green and names a `recording/` folder it does not hold. The run exits 1, says `names no folder recording/`, and runs no block.
- The repository's own `demos/` root reports `0 green, 12 red` and runs nothing.

## What changed in demo 01

Nothing in its commands. All three blocks were run unchanged, with `TYPESAFE_API_KEY` removed from the environment, against a hand-built recording folder for the built-in profile, and all three passed. The flags, the exit codes, and `--replay recording/` are exactly what the built tool answers to.

Two pieces of prose changed. The page said `--replay` was not in the specification, and it is now, so the bullet states what the specification answered. The closing note now says the page is red because no recording exists, and it names `record.sh`.

`demos/01-refund-gate/record.sh` runs the two judged exchanges once with `--record recording/`. Two exchanges answer all three blocks, because the request bytes carry the evidence and the condition and nothing else, so `--min-prob` and `--status` change no digest. The script refuses to start when the key variable holds nothing, and it names the variable and never a value. No gate calls it.

## The ladder

| Rung | Script | Exit |
| --- | --- | --- |
| 0 | `sdlc/scripts/install` | 0 |
| 1 | `sdlc/scripts/lint` | 0 |
| 2 | `sdlc/scripts/test` | 0 |
| 3 | `sdlc/scripts/spec` | 0 |

Eighty-six tests, one documentation test, seventeen spec examples, and the demo runner over twelve red demos pass. The largest file is `crates/thinkthen/tests/record_and_replay.rs` at 316 non-blank lines, against a ceiling of 500.

The `test` rung now needs `mustmatch` on `PATH`, because `demo_runner.rs` runs the real runner. Rung 3 already needed it. A run without it fails loudly rather than skipping.

## The ratchet

The ceiling was 3010 and is 3964. It rose in four steps, each in the commit that needed it: 3094 for the source-aware resolver, 3145 for the fixed phrases, 3433 for the core recording module, 3896 for the recorder and the record-and-replay tests, and 3964 for the demo runner tests.

## Dependencies

`sha2 0.11.0` with `default-features = false`, in `thinkthen-core` alone. The standard library has no hash. It resolves seven crates, every one of them MIT or Apache-2.0, so no license exception was added and none was needed. `policy.py` accepts the new direct dependency in the same commit.

## What the specification got wrong or left unsaid

- **The entry file's formatting.** `recording.md` promised "sorted, stable formatting". Embedded raw JSON is written verbatim, so only the five outer fields are laid out. The page now describes what is written.
- **What a digest match with different content means.** The page says a miss is exit 5 and says nothing about a file that exists and records another exchange. It is exit 5 too, with its own message, because a digest match with different content means the file was damaged or edited by hand. A file that cannot be parsed and a file naming another schema are the same.
- **`--plan` with the recording options.** No page said anything. The decision above fills it.
- **Where a recording folder's failures land.** `channels.md` gives exit 5 to input that cannot be read and output that cannot be written. A recording folder that cannot be written is neither in so many words. It is exit 5, because the recording is a file on this machine.
- **Whether `--replay` alone still reads a key.** `recording.md` says it reads no key. The order that makes it true is not stated anywhere: the recording is read before the key is. A test holds it.
- **The temporary name.** The page asks for a temporary name inside the folder and a rename. It does not say what makes the name unique. It is a leading dot, the process identifier, and the entry name, so two commands recording at once do not collide.
- **A profile name beside a URL from the environment.** `backends.md` now settles the flag case and the environment-only case. The mixed case of a flag name beside an environment URL is still not written on the page. This ticket makes it a usage error, on the reading that the `--backend` flag beside a URL is refused whichever source offered the URL.
- **`EntryError::Unwritable` cannot fire**, the way `EncodeError` cannot. It stays for the same reason: the alternative is a suppression of the `unwrap` ban.
- **The live recording of demo 01 still waits on vendor credits.** Record 0003's live call ended at status 402 for every attempt, and the organization has no available credits. `record.sh` is the one step that turns demo 01 green once Ian adds them. Nothing else in this ticket depends on it.

## Review

A second agent reviewed the branch on 2026-09-19, against `AGENTS.md`, `specification/{recording,channels,result,backends}.md`, ADR 0005 and 0006, `sdlc/planning/rust-standards.md`, the Rust ideal state, the ticket, and the Review sections of records 0001 to 0003. The verdict is that the work is good enough to land after the six fixes below. Ian or the steering agent can overturn any of them.

### Privacy

Every entry, temporary file, diagnostic, `Debug` line, and plan was read for a key or a header. `Exchange` and `Entry` withhold their bodies in `Debug`, `Key` prints a fixed placeholder, `failure.rs` writes every message itself and quotes no response body, and the core unit test and the integration test both search a written entry for `authorization`, `Bearer`, and the key value. No message carries evidence text. `EntryError::Malformed` carries a `serde_json` message, which names a line and a column and quotes no input.

The file modes were the one real finding. Under the machine's umask of 002 the tool wrote entries at 0664 and created the folder at 0775, and under the common umask of 022 they would be 0644 and 0755. A recording holds the evidence, so any account on the machine could read private input. Owner-only is the smaller safe rule, and the fix is a mode on the open and a mode on the folder creation. The folder the tool creates is now 0700, every entry is 0600, a `DIR` that already exists keeps the mode it has, and on a system that is not Unix the umask alone still decides. `specification/recording.md` says so in one sentence, and an integration test reads both modes back.

### Durability

The temporary name is a leading dot, the process identifier, and the entry name, all inside `DIR`, so a rename is on one filesystem and is atomic. A reader looks up one exact `DIGEST.json` and reads nothing else, so a stray temporary file, an unrelated file, and a folder of anything else are all invisible to replay. The digest input was recomputed outside the program with `sha256sum` and is exactly the adapter name, a newline, the URL, a newline, and the request bytes, with no trailing newline; the pinned literal test holds it, and the fixture entry's name was recomputed the same way and matches. The schema string is compared on every read and an unknown one gets its own message. An entry whose adapter, URL, or request bytes differ from the exchange being replayed is exit 5 with the damaged-or-hand-edited message. The entry layout is byte for byte stable, which the pinned `FILE` constant holds, so a re-record of an identical response writes an identical file.

Forty processes, twenty at a time under `xargs -P`, recorded the same request into one folder against a local listener. One valid entry came out, no file was torn, no temporary file was left, and every process exited 0. Distinct process identifiers give distinct temporary names, and the last rename wins.

A failed rename left its temporary file behind, and that file holds the evidence. The write path now removes the temporary file whenever the write or the rename fails, and it clears a leftover from an earlier crash before it writes. A test drives a rename onto a name a directory already holds and reads the folder back.

### Conformance

Record, replay, and cache were each run and each matches the page. `recorder.replayed` is called before `backend.key_env().map(edge::key)`, so replay reads no key variable at all rather than ignoring its value; moving the key read one line earlier turns the built-in-profile replay test red at exit 4. Replay opens no connection, which the closed-port test and the listener request count hold. Two folders exit 2, only a decoded exchange is recorded, and `meta.replayed` is always present. The `--plan` decision now stands on `channels.md` and `recording.md`, which are the pages a reader consults. `backends.md` now states the two mixed pairings the resolver refuses. The six fixed phrases match the table.

### The rungs and the demos

`sdlc/scripts/demos` was run with zero green demos, with a folder name holding a space, with a missing recording folder, and with a missing root. It exits 0 on zero green, it quotes every path it passes, its status match is the anchored `^Status: green$`, it runs no red page, and `set -eu` fails the rung on a failing green page. The `test` rung's new need for `mustmatch` is acceptable, because a runner that is not exercised is not a gate. The `install` rung now names `cargo`, `python3`, `node`, and `mustmatch` before any build goes looking for one, and `sdlc/scripts/README.md` says so.

`demos/01-refund-gate/record.sh` refuses to start without the key variable, names the variable and never a value, writes only into that demo's `recording/`, and is reachable from no gate, because the runner reads only `README.md`. Its three judged commands are the page's own.

### Tests broken and restored

Four tests were broken at the code and watched failing for the right reason. Dropping the newline between the adapter and the URL failed the pinned digest with `left: "4ce638ea…"`. Reading the key before the recording failed the built-in-profile replay with `left: Some(4), right: Some(0)`. Widening the entry mode to 0666 failed the new privacy test with `left: 436, right: 384`. Dropping the cleanup failed the new leftover test with the temporary file it left, `.2375499.9ba0f37c….json`.

### What the review changed

- Write a recording for its owner alone and leave no partial file.
- Tell a missing entry from an entry that cannot be read. Every read failure was reported as a replay miss, so a folder whose entry could not be read told the user to record what was already recorded.
- Check every tool the later rungs need from the install rung.
- Read a value's source from the values themselves. The resolver threaded a boolean out of one helper through five call sites and discarded it at three.
- State on the settled pages that a plan takes neither recording option.
- Write down the mixed name and url cases the resolver settles.

The ceiling was 3964 before the review and is 4080 after it.

### Left for the steering agent

- The live recording of demo 01 waits on vendor credits, so demo 01 is still red and the `spec` rung still runs zero green demos. The ticket already excuses this.
- `record.sh` runs under `/bin/sh` with `set -eu` rather than `set -euo pipefail`. It holds no pipeline, so `pipefail` would change nothing, and `pipefail` is absent from `dash`. Adopting it means depending on `bash` for a script that is run by hand.
- The demo runner word-splits the folder names it harvests from `--replay`, so a demo page naming a folder with a space would be checked under the wrong name. No demo names one.
- `EntryError::Unwritable` cannot fire, as the record says. Deleting it means an `unwrap` the lint table bans, so it stays on the precedent `EncodeError` set.
- `crates/thinkthen/tests/record_and_replay.rs` is 391 non-blank lines, and five of its tests repeat the same record-into-a-folder call. A helper would shrink it before the file nears the 500-line ceiling.
- `EntryError::Schema` prints the schema string it read out of a file, so a hand-edited entry can put arbitrary text in a diagnostic. It is the user's own file and it holds no key.
- `crates/thinkthen-core/src/backend.rs` is the largest file at 411 non-blank lines, and its first documentation line joins three sources with "and". It predates this ticket.
- `crates/thinkthen/tests/fixtures/demos-wrong/` copies the whole `demos/01-replay-gate/` page to change one expected exit code. Fixture text costs no ratchet lines and does cost a reader.
