# 0381: Windows stage 1: the C library ships as a DLL for Windows x86-64

Status: in progress. Slice A owns claude-1 on `ticket/0381-windows-c-dll`, starting from landed main `6d26206aa8861490cc6540b6bafafc08e3249c3c`. The corrected complete DLL design received fresh High acceptance. Slice B and native runner execution remain pending. Plan: `sdlc/planning/windows.md`, stage 1; 0.2 only.

Milestone: 0.2

## Outcome

- The Windows target builds `thinkthen.dll` and its import library `thinkthen.dll.lib`, and packs them with the header in their own archive.
- The DLL exports exactly the C door's public functions.
- Either a static link against the static library exposes no Rust standard library symbol, as on Linux and macOS, or the DLL ships alone, `DESIGN.md` says so, and an issue for stage 3 holds the static library.
- The door's C tests build and pass with MSVC on `windows-2025`, apart from `fork.c`, which stays Unix.
- `libraries/c/DESIGN.md` states the DLL, import library and static library rules.
- Linux and macOS libraries are unchanged.

## Evidence

- Starts from: ticket 0373 (stage 0). `libraries/c` builds, passes Clippy and passes its Rust tests on Windows. Its C tests build C with a Unix compiler under ASan, so they stay on Unix today. The stage 1 report in `sdlc/planning/windows.md` sizes this ticket at 400 to 900 lines and 2 slices, with low to medium Linux and macOS risk: `build.rs` and the pack script are shared, but each change sits in a Windows arm.
  - `libraries/c/localize.sh` hides Rust's symbols with `nm`, `ld -r` and `objcopy`. MSVC has no twins for them.
  - The door tests use `cc`, ASan, `readelf` and `fork.c`.
  - No experiment preceded this ticket.
- Keeps: the C header, every exported name and every Linux and macOS archive's contents. The `.pc` file's meaning on Unix. The rule that a static link exposes no Rust standard library symbol.
- Changes: slice A, the DLL: `libraries/c/build.rs` makes the import library and a `.def` export list; `release-pack` and `release.yml` build and pack the DLL, import library and header on the Windows target; an export check with `dumpbin`; the MSVC build of the door's C tests. Slice B, the static library: hide the Rust internal symbols in the COFF static library, the `.pc` file's `Libs.private` for Windows, and a static-link smoke. `DESIGN.md` gains the rules in each slice.
- Proof: `dumpbin /exports` lists exactly the header's functions. The MSVC door tests pass on the runner. MSVC's ASan gets its own check, or the builder records why it cannot run. A static-link smoke links a small C program and finds no Rust standard library symbol. The Linux and macOS C door tests and `release-pack` self-tests stay green.
- Defers: the main unknown, how to hide the Rust standard library's symbols in a COFF static library. If slice B finds no clean way, the builder ships the DLL alone, files the static library as an issue for stage 3, and says so in `DESIGN.md`. `fork.c` stays Unix-only. The stage 2 bindings that load this DLL (C++, Go, Ruby, R, PHP, Swift, Dart, Zig) wait for stage 2.

## What the build taught us

Slice A keeps the Windows command ZIP separate and gives C exactly a header, public DLL and import library. Packaging generates the import library for `thinkthen.dll`; Cargo's internal import descriptor cannot be corrected by renaming its file. Header declarations provide the export authority, and native inspection compares every export without prefix filtering. Both archived-release and command collection fixtures must migrate with the six-file verifier; their missing-C refusals occur before any collection output.

The fixture-only CRT definition retains MSVC `/W4 /WX` and the driver's framing. Native thread handles, atomic arrival ordering and binary streams preserve the shared assertions; the Unix fork regression remains. The source door and the downloaded-archive consumer are distinct proof paths. Portable PE/COFF and saved-dumpbin fixtures establish no native ABI, loader, execution or sanitizer result. The separate native ASan test first demands a detected owned negative; the C Rust DLL is outside that instrumentation.

0380 C source landed at `6d26206aa` after source and canonical receipts completed. Its native privacy/interruption gaps and W1/W2 remain pending. Slice A adds no dependency, unsafe allowance, provider or surface setting. Static COFF localization and metadata remain slice B. The parent owns fresh High ABI/platform review and the named full checkpoint. No native dispatch, publication, signing or rehearsal has run for this slice.

Rejected slice A source `fd98fbbaaab7db4de3a892dedd7425251bac1478` historically passed required lint, policy, C Clippy and all 36 focused Linux C door tests (53 shared C cases pass, two retained exclusions). Both six-file collection fixtures and 45 Windows C workflow plants historically passed within the 113-case workflow gate. Historical ceilings were root 114995, C Rust 5223 and C fixtures 1323. [The build record](../records/0381-a-windows-c-build.md) holds growth, source identity and receipt digests. Fresh High review rejected candidate `07c7d66241ab` for unbounded native tools and unsafe typed-facts continuation after failed join. The correction bounds tool lifetimes and owned cleanup and stops the fixture before shared-state access on join failure. Corrected frozen source `3281e7a9a506a1229ae541e09f015b001d617e16` passes policy, C Clippy, all 40 focused Linux C door tests, the retained shared C corpus, all 113 workflow cases and required lint. Independent plants reject both the old join continuation and disabled owned-child cleanup for pinned causes. Exact renewed ceilings are 114995/5487/1329; the build record freezes distinct fix receipts and source fingerprint. Another fresh High ABI/platform review and the parent's subsequent rebase/full checkpoint remain pending. Native Windows compilation, loader/ABI/consumer/sanitizer execution and slice B remain open, so the ticket stays in progress.


Additional fresh High review rejected candidate `ffb20a0c0cd3` / source `3281e7a9a` because Windows Python timeout cleanup can still wait indefinitely on descendants holding output pipes. The accepted prior Rust deadline and typed-facts join refusal fixes remain intact. The release C, downloaded consumer and MSVC setup now share file-backed bounded capture with owned Windows job containment before the suspended root runs; POSIX cleanup targets only its new session. Independent inherited-output and unrelated-child regressions execute on Linux; mocked handle/job refusal oracles establish source control flow only. Three planted omissions refuse with pinned causes. Actual Windows process/job and native C execution remain pending. Exact ratchets stay 114995/5487/1329. New frozen receipt/source details live in the build record; another fresh High review and the parent's integration/full checkpoint remain required.


Final frozen release-capture source `cb12581b5ebf26280f82f64e06b6214036e97985` passes focused policy/C Clippy/40 Linux door tests, the unchanged 53-pass/two-exclusion shared corpus, all 113 workflow cases/45 Windows C plants, both collection fixtures and required lint. The source fingerprint, per-file digests, successful receipts and retained failed attempts are frozen in the build record. All source ratchets remain exact and no Rust/C/public/native-workflow source changed from the prior lifecycle correction. Owned scope cleanup is complete. Another parent-supplied fresh High review, integration and the parent checkpoint remain required; native Windows authorization/execution remains pending.

Slice A source and integration received fresh High acceptance on `7a1b5dc38`; the named Linux checkpoint, actual 350 replay retry, strict zero-stale verification and final site build pass. Final receipt review and A landing remain pending. Native Windows execution and B’s static-library disposition remain pending; these Linux receipts do not complete them.
