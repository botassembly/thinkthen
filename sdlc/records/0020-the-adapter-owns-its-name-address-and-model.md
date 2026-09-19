# 0020: The adapter owns its name, its address, and its model

Branch `ticket/0020-adapter-owns-its-defaults`. Built 2026-09-19.

## What moved

ADR 0010's clarification of 2026-09-19 rules that other backends will come. Four values that name one vendor sat outside the vendor's adapter, so a second adapter would have had to edit files that know nothing about it. All four now live in the adapter's module and reach their callers as values.

`crates/thinkthen-core/src/systemone.rs` and `systemone/` became `crates/thinkthen-core/src/adapters/systemone.rs` and `adapters/systemone/`. A new `crates/thinkthen-core/src/adapters.rs` declares the adapters this version compiles and names the one this build uses, as `built_in`. Nothing else changed inside the encoder or the decoder.

`crates/thinkthen-core/src/adapters/systemone.rs` now owns four constants.

| Value | Constant | Where it was |
| --- | --- | --- |
| The adapter's name | `NAME` | already there, `pub(crate)` |
| The default address | `DEFAULT_BASE` | `backend.rs`, private |
| The default model | `DEFAULT_MODEL` | `backend.rs`, public |
| The endpoint path | `ENDPOINT_PATH` | nowhere. `backend.rs` appended `NAME` |

Every caller asks `built_in` for the value it needs.

- `backend.rs` takes the default base and the endpoint path, and its own doc lines no longer spell the vendor's address.
- `recording.rs` takes the name for the digest, for the entry's `adapter` field, and for the replay check.
- `plan_document.rs` and `crates/thinkthen/src/judge.rs` take the encoder and the decoder. `crates/thinkthen/src/failure.rs` takes `DecodeError`.
- `question_file/resolve.rs` takes the default model, and `lib.rs` re-exports it at the crate root as it did before.
- `crates/thinkthen/tests/backend/harness/mod.rs` builds the loopback URL it expects from `ENDPOINT_PATH` rather than from a literal.

`SEAM_ALLOWED` in `sdlc/scripts/policy.py` is empty, and `adapter_paths` computes what the check exempts from the `pub mod` lines of `adapters.rs`. `specification/backends.md` says in one sentence that an adapter owns its name, its default address, its default model, and its endpoint path.

## Nothing changed by a byte

`crates/thinkthen/tests/backend/wire.rs` is the proof the ticket asked for. It sends one request per question type to a loopback listener, counts the requests, pins the request line, and compares the body with the bytes written out by hand rather than encoded again, so a change in the encoder cannot move the test with it. It was written first and watched pass on the code before the move. Flipping one byte of the `score` body failed it on the byte, which is how its assertion was proved.

The pinned `decide` digest in `recording.rs::tests::PINNED` is unchanged and still passes. Every committed recording replays: the `spec` rung reports `demos: 15 green, 7 red`, the same count record 0019 measured.

## Red then green

- **The pinning test.** `left: [... 104, 105, 103, 104 ...] right: [... 72, 73, 71, 72 ...]` on a deliberately altered level name, then green on the real bytes.
- **The move.** `error[E0432]: unresolved import crate::systemone` in `recording.rs` and in `question_file/resolve/tests.rs`, and `couldn't read .../specification/fixtures/systemone/decide-urgent.request.json` from each `include_str!` in the adapter's own tests, which sat one directory shallower than the files now are.
- **The help check.** `the long help does not name the default model jev-latest`, raised by changing the literal in `args.rs` to a name the constant does not carry.
- **The seam check over its new ground.** A line holding every vendor word was planted in `backend.rs` and the check refused each one by name: `seam: crates/thinkthen-core/src/backend.rs:6 names 'noul' outside the adapter`, and the same for `criteria`, `systemone`, `typesafe`, and, after the review added it, `jev`. `rust-standards.md` asks that every ban be planted and refused rather than trusted, and moving what the check exempts is a change to the ban.

## The choices made where the pages were silent

Each of these is written into the code that states it, and Ian can overturn any of them.

1. **The adapter moved into an `adapters/` folder rather than being renamed.** Rust needs the module declared somewhere, and a declaration spells the module's name. A single `adapter.rs` would have hidden the vendor's word at the cost of saying which vendor it is, and a second adapter would then have forced the folder anyway. `adapters.rs` is the one file that names which adapters exist, and the seam check counts it as adapter ground.
2. **`built_in` is a module alias, not a trait.** `pub use crate::adapters::systemone as built_in` is the whole seam. Every caller names the alias, so the choice of adapter lives on that one line. ADR 0017 holds the trait until a second adapter exists, and the ticket excluded it.
3. **The endpoint path is its own constant, though it spells the name.** `backend.rs` used to append `NAME`, which tied the address to the recording key by accident. Another adapter's endpoint need not be its name, so the two are separate values that happen to read alike here.
4. **`DEFAULT_MODEL` keeps its crate-root re-export.** It is the tool's default model as well as the adapter's, the binary's help documents it, and another builder is working in this crate. The constant has one definition and one alias, which is a public surface rather than a second home.
5. **`NAME` stays `pub(crate)`.** The binary never needs it. The address and the entry are both written inside the core.
6. **The tests that pin an exact rendered line keep their literal.** The ticket asked that tests take the vendor's URL and model names from one fixture the adapter owns. The tests that *construct* a value now do: the loopback harness builds its URL from `ENDPOINT_PATH`, and the cases that need the default model read `DEFAULT_MODEL`. A case that pins an exact address or an exact rendered line keeps its literal, because the literal is the contract that case exists to hold still. `backend.rs::tests::BUILT_IN` is the clearest of them: composed from `DEFAULT_BASE` and `ENDPOINT_PATH`, the same two values the code composes it from, it would assert nothing and would follow any change in either one. Ticket scope item 3 is therefore met in part and by design.
7. **The help's copy of the default model went, and `jev` joined the vendor words.** `args.rs` stated `[default: jev-latest]` in a doc comment. A doc comment cannot read a constant, so clap is handed `help` and `long_help` as expressions built from `DEFAULT_MODEL` instead. The help text is unchanged byte for byte, proved by comparing `decide -h`, `decide --help`, `choose --help`, and `score --help` before and after. `VENDOR_WORDS` then gained `jev`, so the rule is mechanical rather than remembered. The check in `decide_edge.rs` stays beside it, because the seam check reads the source and the case reads the help the binary actually prints.
8. **`DEFAULT_BASE` stays `pub`.** The reviewer offered `pub(crate)`, since only `backend.rs` reads it. Its other reader is `crates/thinkthen/tests/backend/recordings.rs`, in the sibling crate, which builds the address it takes a digest over. A value the adapter hands out is the point of this ticket, so the wider visibility is the design rather than an oversight.
9. **The adapters folder is not itself the permission.** `check_seam` reads the `pub mod` lines of `adapters.rs` and exempts only `adapters.rs` and the modules it declares. A file dropped beside the adapters is read like any other source, so the exemption cannot be taken by moving a file into the folder.

## The gates

| Rung | Exit |
| --- | --- |
| `install` | 0 |
| `lint` | 0 |
| `test` | 0 |
| `spec` | 0 |

271 tests. `demos: 15 green, 7 red`.

The ceiling went from 13116 to 13308. The rise is five things: 99 lines for `wire.rs`, the page that pins the request bytes and had no equivalent before; 41 lines for the adapters module, the four documented constants, and the imports that reach them, against the two constants `backend.rs` gave up; 8 lines for the help check; and, after the review, 24 lines for the fourth pinned body and 20 for the help expressions and the tighter seam exemption. No duplication was found to delete in exchange: each value had one home already, and this ticket gives it the right one.

## The review

A second agent with fresh context read `AGENTS.md`, the ticket, ADR 0017, ADR 0010's clarification, and the whole diff against `origin/main`, and ran `lint`, `test`, and `spec` itself. Its verdict on the first pass was **not ready**, with one must-change and six observations.

It confirmed the two questions the ticket turns on. No behavior changed: every one of the four strings survives the move, the address rule and the recording digest swap a constant for the same constant, the encoder and decoder differ only in `use` paths and `include_str!` depth, and the pinned plan line holds the default base, the endpoint path, and the default model together. It also named the strongest evidence, which is the commit split: `wire.rs` landed in `4c3fddf` against the code before the move, and the move commit does not touch it. And it called the move genuine rather than grep-hiding, because rewriting the one alias line really does change the default address, the default model, the endpoint path, and the recording key.

**1. The vendor's model name survived in the help.** `args.rs` held `[default: jev-latest]` as a literal, and the sentence this branch added to `backends.md` says nothing outside the adapter's module names any of the four. The reviewer proved the check was blind to it by setting `VENDOR_WORDS = ("jev",)` and getting exactly one failure. **Fixed**, as choice 7 above describes. Planting the stem again now fails: `seam: crates/thinkthen/src/args.rs:165 names 'jev' outside the adapter`.

**2. `DEFAULT_BASE` could be `pub(crate)`.** **Declined**, and choice 8 says why. Finding 5 asks the sibling crate's test to read it, and the two cannot both hold.

**3. The adapters folder exempted anything inside it.** `relative.startswith(ADAPTER)` was a bare directory prefix, so moving `backend.rs` there would have silenced the check. **Fixed.** `adapter_paths` reads the `pub mod` lines of `adapters.rs` and exempts only the modules it declares, and it fails the rung when `adapters.rs` declares none. A file planted in the folder under no declaration is now refused: `seam: crates/thinkthen-core/src/adapters/smuggled.rs:1 names 'noul' outside the adapter`.

**4. A production file named `tests.rs` is exempt, and the scan stops at the first test module.** **Stands.** Both are older than this ticket and neither is reachable today: no production module in either crate is named `tests.rs`, and the scanner's break was verified against all five files that hold a vendor word below one. The lever is to read a file whole and to exempt a test module by its path in the module tree rather than by its file name.

**5. A test built the vendor's URL as a literal.** `recordings.rs` constructs an address to take a digest over rather than pinning a rendered line, so by this record's own rule it should read the adapter's constants. **Fixed.** It now builds from `DEFAULT_BASE` and `ENDPOINT_PATH`, and its model from `DEFAULT_MODEL`.

**6. The pinned bytes were all plain ASCII.** ADR 0017 calls the JSON escaping the hardest rule to hold still, and nothing pinned it. **Fixed.** A fourth case sends evidence carrying a quote, a line feed, a tab, a control byte, an accented letter, and a dash outside ASCII, and pins the body escape for escape. The three original bodies were left alone, because they are the ones watched against pre-move code. The fourth was pinned after the move, and it is honest to say so: it guards every change from here on rather than this one. What makes it safe is that the encoder's diff against `4c3fddf` is five `use` lines and the ten `include_str!` paths that sit one directory deeper, and no serialization line at all.

**7. The record was uncommitted and the ticket said `in progress`.** The record is committed with this pass. The ticket stays at `in progress` on purpose: the coordinator lands it, which is the instruction this builder works under.

The reviewer also raised a caveat worth keeping. A second adapter's module must export the same nine items by the same names, and nothing but the compiler says so. `thinkthen-core` has no trait to state that contract, and ADR 0017 holds it until a second adapter exists. The tests are the other deferred cost: every case pinning `https://api.typesafe.ai/v1/systemone` or `POST /v1/systemone` belongs to this adapter and would need a home when a second one arrives.

## What in the ticket proved wrong

Nothing in the ticket's outcome proved wrong. Two of its lines read differently once the code was in hand.

- **"The seam check passes with no allowed site outside the adapter's module, the default address aside if the binary must still name it."** The binary never names the default address, so no allowance was needed at all. `SEAM_ALLOWED` is empty rather than nearly empty.
- **"The tests take the vendor's URL and model names from one fixture that the adapter's module owns."** There is no one fixture. The adapter's four constants are the fixture for every test that builds a value, and a test that pins an exact address or an exact rendered line keeps its literal on purpose. Choice 6 above says why.

The ticket stays at `in progress`.
