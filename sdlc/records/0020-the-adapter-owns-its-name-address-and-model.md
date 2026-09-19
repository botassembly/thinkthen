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

`SEAM_ALLOWED` in `sdlc/scripts/policy.py` is empty. `ADAPTER` is now the adapters folder. `specification/backends.md` says in one sentence that an adapter owns its name, its default address, its default model, and its endpoint path.

## Nothing changed by a byte

`crates/thinkthen/tests/backend/wire.rs` is the proof the ticket asked for. It sends one request per question type to a loopback listener, counts the requests, pins the request line, and compares the body with the bytes written out by hand rather than encoded again, so a change in the encoder cannot move the test with it. It was written first and watched pass on the code before the move. Flipping one byte of the `score` body failed it on the byte, which is how its assertion was proved.

The pinned `decide` digest in `recording.rs::tests::PINNED` is unchanged and still passes. Every committed recording replays: the `spec` rung reports `demos: 15 green, 7 red`, the same count record 0019 measured.

## Red then green

- **The pinning test.** `left: [... 104, 105, 103, 104 ...] right: [... 72, 73, 71, 72 ...]` on a deliberately altered level name, then green on the real bytes.
- **The move.** `error[E0432]: unresolved import crate::systemone` in `recording.rs` and in `question_file/resolve/tests.rs`, and `couldn't read .../specification/fixtures/systemone/decide-urgent.request.json` from each `include_str!` in the adapter's own tests, which sat one directory shallower than the files now are.
- **The help check.** `the long help does not name the default model jev-latest`, raised by changing the literal in `args.rs` to a name the constant does not carry.

## The choices made where the pages were silent

Each of these is written into the code that states it, and Ian can overturn any of them.

1. **The adapter moved into an `adapters/` folder rather than being renamed.** Rust needs the module declared somewhere, and a declaration spells the module's name. A single `adapter.rs` would have hidden the vendor's word at the cost of saying which vendor it is, and a second adapter would then have forced the folder anyway. `adapters.rs` is the one file that names which adapters exist, and the seam check counts it as adapter ground.
2. **`built_in` is a module alias, not a trait.** `pub use crate::adapters::systemone as built_in` is the whole seam. Every caller names the alias, so the choice of adapter lives on that one line. ADR 0017 holds the trait until a second adapter exists, and the ticket excluded it.
3. **The endpoint path is its own constant, though it spells the name.** `backend.rs` used to append `NAME`, which tied the address to the recording key by accident. Another adapter's endpoint need not be its name, so the two are separate values that happen to read alike here.
4. **`DEFAULT_MODEL` keeps its crate-root re-export.** It is the tool's default model as well as the adapter's, the binary's help documents it, and another builder is working in this crate. The constant has one definition and one alias, which is a public surface rather than a second home.
5. **`NAME` stays `pub(crate)`.** The binary never needs it. The address and the entry are both written inside the core.
6. **The tests that pin an exact rendered line keep their literal.** The ticket asked that tests take the vendor's URL and model names from one fixture the adapter owns. The tests that *construct* a value now do: the loopback harness builds its URL from `ENDPOINT_PATH`, and the cases that need the default model read `DEFAULT_MODEL`. A case that pins an exact address or an exact rendered line keeps its literal, because the literal is the contract that case exists to hold still. `backend.rs::tests::BUILT_IN` is the clearest of them: composed from `DEFAULT_BASE` and `ENDPOINT_PATH`, the same two values the code composes it from, it would assert nothing and would follow any change in either one. Ticket scope item 3 is therefore met in part and by design.
7. **The help's copy of the default model became a check.** `args.rs` states `[default: jev-latest]` in a doc comment, and clap's derive cannot read a constant into one. The help case in `decide_edge.rs` now builds the sentence it looks for out of `DEFAULT_MODEL`, so a rotted copy fails a rung. The alternative, adding `jev` to `VENDOR_WORDS`, would have forced a change to the help text, which this ticket forbids.

## The gates

| Rung | Exit |
| --- | --- |
| `install` | 0 |
| `lint` | 0 |
| `test` | 0 |
| `spec` | 0 |

271 tests. `demos: 15 green, 7 red`.

The ceiling went from 13116 to 13264. The rise is three things: 99 lines for `wire.rs`, the page that pins the request bytes and had no equivalent before; 41 lines for the adapters module, the four documented constants, and the imports that reach them, against the two constants `backend.rs` gave up; and 8 lines for the help check. No duplication was found to delete in exchange: each value had one home already, and this ticket gives it the right one.

## The review

To be written when the reviewer reports.

## What in the ticket proved wrong

Nothing in the ticket's outcome proved wrong. Two of its lines read differently once the code was in hand.

- **"The seam check passes with no allowed site outside the adapter's module, the default address aside if the binary must still name it."** The binary never names the default address, so no allowance was needed at all. `SEAM_ALLOWED` is empty rather than nearly empty.
- **"The tests take the vendor's URL and model names from one fixture that the adapter's module owns."** Choice 6 above says which tests do and which keep their literal, and why.

The ticket stays at `in progress`.
