# 0634: Rewrite the binding author guide as the current contract

Status: OPEN.

Milestone: 0.2.1

Phase: 2

Depends on: 0635, 0637, 0638, 0642, 0692

## Outcome

- Changes: `libraries/BINDING-AUTHOR.md`, `libraries/files.md`.
- The guide states the current contract in plain words, without ticket numbers. It covers which entry point a binding calls (the C function `thinkthen_run_call`, or `Engine::call` for direct Rust bridges), how async hosts run it on one worker and cancel through the `thinkthen_call_cancel` handle, how errors and their facts arrive through `thinkthen_call_error`, the build identity check at load, the packet dumper contract from 0642, which generated declarations the binding uses, and what the host may own: naming, value conversion, async scheduling, cancellation idiom, cleanup and packaging.
- Rules now owned by Rust (the session wait protocol, settlement, answer projection, input validation) and the old session, error-slot and projection paths that 0690 deletes are removed from the guide.
- The guide describes the load-time check as a build identity check that the wrapper and native library come from the same build, not as a format version, per the 2026-10-19 no-version-numbers ruling.
- `libraries/files.md` keeps only what is common to every language, and drops the COBOL-specific text.

### Proof

- On the M5, `grep -nE '\b0[0-9]{3}\b' libraries/BINDING-AUTHOR.md` finds no ticket number; it finds 19 lines before the change. `grep -n -i 'cobol' libraries/files.md` finds nothing.
- Every relative link in both pages resolves, checked once with a one-line script over their Markdown links; the script is not committed.
- Any code the guide shows is copied from `libraries/c/examples/session.c` or `libraries/rust/examples/`, which the C and Rust surface checks already compile and run, so the guide needs no replay of its own.

## Evidence

- `libraries/BINDING-AUTHOR.md` (466 lines) is a target, not a contract. It "defines the 0.2 target". It defines behavior by pointing at tickets (0503, 0505, 0511, 0513, 0515, 0516, 0517, 0528, 0529) and at rulings that changed four times in two weeks: ADR 0112, ADR 0125, ADR 0129, and `sdlc/decisions/2026-10-09-thin-first-class-bindings.md` with its 2026-10-13 amendment. It describes three ways into the core: the JSON session, fixed layouts for C, Zig, Ada and COBOL, and direct Rust bridges. It spends long paragraphs on wait, arm and settlement rules that each host must re-implement.
- Current main already has the new path: `thinkthen_run_call`, `thinkthen_call_result_json`, `thinkthen_call_error_*` and `thinkthen_call_cancel_*` in `libraries/c/include/thinkthen.h` (0638), the canonical C input path (0635), declarations-only generation (0637), `thinkthen_version` with its interface number (0692), and `Engine::call` (0629) declared in `specification/rust-api-0.2.md`.
- `libraries/files.md` repeats per-language file rules for Python and C and ends with the COBOL scalar limit.
- Readers of the guide: `specification/README.md`, `specification/settings.md`, `libraries/r/NOTES.md` and several decisions link to it; the file keeps its name.
