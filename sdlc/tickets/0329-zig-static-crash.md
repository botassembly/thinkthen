# 0329: Link the static Zig consumer with LLD

Status: COMPLETE.

Opened as: 2026-10-11. 2026-09-30. A fresh code review accepted it after one round of fixes. Owner: Claude. Plan: `sdlc/planning/cleanup-2026-09-30.md`, step 1.

## Outcome

A Zig consumer that links `libthinkthen.a` in Debug runs every matrix case, including `decide` with `deadline_ms = 0`. The installed static consumers in `libraries/zig/check.sh` pass again.

## Evidence

- Starts from: the Zig check failed in static mode after 0304 slice 2 (`1fbbe08e0`); it passed at `5ad9f0998`. Shared mode passed. A debug C library and gdb gave the fault: SIGSEGV at `std::sys::pal::unix::time::Timespec::sub_timespec+156`, under `Instant::duration_since`, `thinkthen::engine::Cancel::remaining`, `Cancel::stop`, `Recorder::gate`, `Engine::judge` and `public::Engine::details_with`. The faulting instruction is `xorps -0xb778fb(%rip),%xmm0`, an SSE load that needs a 16-byte aligned operand. Its operand sat at `0x10ee0f8`, an 8-byte address. Zig 0.15.2's own ELF linker merged every `.rodata.cst4/8/16/32` input into one `.rodata.cst` output with entry size 4 and did not keep each constant's 16-byte alignment. Slice 2 only moved the layout. The engine code is correct. The main static library holds no SQLite symbols, so the SQLite guess does not apply.
- Keeps: the shared link path, the header checks, the system libraries, and every consumer build flag.
- Changes: `linkNative` in `libraries/zig/build.zig` sets `use_llvm` and `use_lld` for the static mode. LLD keeps section alignment. The Zig ratchet rises from 1,247 to 1,251 for these four lines.
- Proof: the static matrix passes under the loopback backend. `libraries/zig/check.sh` passes: shared matrix, settings, bulk, and all four installed consumers (`shared-0`, `shared-1`, `static-0`, `static-1`) with 42 exact request bodies each.
- Defers: an upstream Zig report, tracked in `sdlc/issues/2026-09-30-zig-0-15-2-linker-drops-constant-alignment.md`. Remove the flags once Zig's own linker keeps merged constant alignment. The release run (`portable_batch.py`) links shared only, so `Tests/installed.py` alone guards static mode.

## What the build taught us

- Zig's default Debug path links Rust objects with its own ELF linker. A layout change anywhere in Rust can move a 16-byte constant onto an 8-byte address. The static consumer check caught it; the shared mode cannot, because the system linker builds the shared library.
