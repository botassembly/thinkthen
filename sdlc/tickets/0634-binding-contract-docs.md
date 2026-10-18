# 0634: Rewrite the binding author guide as the current contract

Status: OPEN.

Milestone: 0.2.1

Phase: 2

Depends on: 0635, 0637, 0638, 0642, 0692

## Outcome

- The guide states the current contract in plain words, without ticket numbers. It covers which entry point a binding calls (the run-call C function from 0638, or the Rust run function from 0629 for direct Rust bridges), how async hosts run it on one worker and cancel through the handle, how errors and their facts arrive, the version check at load (0692), the packet dumper contract from 0642, which generated declarations the binding uses, and what the host may own: naming, value conversion, async scheduling, cancellation idiom, cleanup and packaging.
- Rules now owned by Rust (the session wait protocol, settlement, answer projection, input validation) and the old session, error-slot and projection paths that 0690 deletes are removed from the guide.
- `libraries/files.md` keeps only what is common to every language, and drops the COBOL-specific text.

### Proof

- The C library's README example and the guide's examples replay unchanged against the installed C library on the M5 through pm job.
- Every link in both pages resolves.

## Evidence

`libraries/BINDING-AUTHOR.md` (466 lines) is a target, not a contract. It says it "does not certify that each package already meets it". It defines behavior by pointing at tickets (0503, 0505, 0511, 0513, 0515, 0516, 0517, 0528, 0529) and at rulings that changed four times in two weeks: ADR 0112 (2026-09-30), ADR 0125 (2026-10-08), ADR 0129 (2026-10-09), and `sdlc/decisions/2026-10-09-thin-first-class-bindings.md` with its 2026-10-13 amendment. It describes three ways into the core: the JSON session, fixed layouts for C, Zig, Ada and COBOL, and direct Rust bridges. It spends long paragraphs on wait, arm and settlement rules that each host must re-implement. `libraries/files.md` repeats the per-language file rules, including the COBOL limit.
