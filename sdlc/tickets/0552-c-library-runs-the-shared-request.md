# 0552: Run every C library entry point through the shared request

Status: OPEN.

Milestone: 0.2

Reviews: revision f23c075965dc12a966847fee80c13bfab3b29385, accept

Reviews: revision 842ef95437345568f4fb3b344d8ba947612693fe, accept

Reviews: revision 0333414aa3ac33dc1d617f24344a05dad91b1186, accept

## Outcome

Every C export that asks a question, frozen 0.1 exports included, admits and executes through `Engine::execute_request`. The C library keeps one helper for reading caller arrays. No shipped Rust type or module is named for the old "door" or "legacy" API.

## Evidence

- Starts from: the [2026-10-11 adversarial review](../records/2026-10-11-0-2-adversarial-review.md), finding A2.
  - `thinkthen_call` uses `call/legacy.rs` to admit a request, but then executes it through the per-function engine methods (`libraries/c/src/call.rs:251-309`).
  - The typed exports call the engine directly (`libraries/c/src/door.rs:88-227`).
  - `texts` (`ffi.rs:110`) and `records` (`ffi/typed_facts/ffi.rs:34`) duplicate each other.
  - In shipped code, "door" or "legacy" appears on 103 non-test lines of `crates/thinkthen/src`. Examples are the public `DoorReply` (`public/results/call.rs:235`), `public/results/legacy.rs` and the `Door` type.
- Keeps: the frozen 0.1 C ABI symbols and their behavior, including `{"usage":true}` (`call.rs:93`, pinned by `tests/door/golden.rs:47`).
- Changes: the C library's routes and names.
  - Execute admitted requests through `execute_request` and convert the outcome to the reply.
  - Route the typed exports the same way.
  - Delete `records` in favor of `texts`.
  - Rename the remaining door and legacy types and modules after what they do, and remove "door" from the doc text that generates `thinkthen.h`.
- Proof: these checks.
  - The C installed checks and the 0.1 compatibility vectors pass.
  - A search finds no engine per-function call in `libraries/c/src`.
  - `policy.py` fails on "door" in shipped source, including the doc text in the generated `thinkthen.h`.
- Defers: nothing.

## Progress

- 2026-10-11 started
- 2026-10-11 next: Combined constructor review found frozen C input-kind selection still duplicated, so the developer is replacing it with neutral records and shared named inference before final acceptance.
- 2026-10-11 next: The final C input-kind correction passed fresh review and all focused checks; whole-ticket checks and landing await 0549 and the accepted 0551 core slice.
