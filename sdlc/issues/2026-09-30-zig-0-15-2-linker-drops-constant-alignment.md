# Zig 0.15.2's linker drops constant alignment

Status: open. A dependency bug noted for Ian. Found by ticket 0329. Owner: upstream (Zig); then a Quick Fix removes the workaround.

Kind: debt

Pay when: a Zig release fixes its ELF linker's constant alignment.

Debt: 002

Severity: low

Keeping it ties static Zig builds to LLVM and LLD, and a later Zig may drop or change those switches.

## The problem

Zig 0.15.2's own ELF linker merges every `.rodata.cst4/8/16/32` input into one `.rodata.cst` output section with entry size 4. It loses each constant's 16-byte alignment. A static Debug consumer of `libthinkthen.a` then put a Rust constant at an 8-byte address, and an aligned SSE load in `Instant::duration_since` faulted.

## Our workaround

`linkNative` in `libraries/zig/build.zig` sets `use_llvm` and `use_lld` for static mode. LLD keeps section alignment. Shared mode is unaffected, because the system linker builds the shared library.

## Closing

When a Zig release fixes the linker, remove the two lines `exe.use_llvm = true;` and `exe.use_lld = true;` (and their comment) from `libraries/zig/build.zig`, rerun `libraries/zig/check.sh` in both modes, and close this issue.
