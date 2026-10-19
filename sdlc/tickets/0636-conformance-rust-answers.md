# 0636: Check settled answers and run-call behavior once in shared conformance

Status: OPEN.

Milestone: 0.2.1

Phase: 2

Depends on: 0612, 0638

## Outcome

- `cases.json` holds the expected settled answer for each case: value, probability, pick and resolution. The full packet oracle is 0642's.
- The session and run-call behavior matrix (cancellation, deadline, failing producer, closed engine, error facts) is proven once against the installed C library and once against the Rust library. Language runners check only their idiom: values converted, errors raised as the host's error type, cancellation reaching the native call, and resources freed.
- The CLI's case exemptions in `conformance/parity.py` (`SDK_ONLY_CLI_CASES`, `QUESTION_FILE_CLI_CASES`) are reviewed. Any exemption with no current reason is deleted.

### Proof

- On the M5 through pm job, the routine surfaces run (`sdlc/scripts/surfaces`) passes for the CLI, the Rust library and the C library against the new expectations.
- The behavior matrix passes once for the C library and once for the Rust library.

## Evidence

The conformance suite is strong. `conformance/cases.json` lists 29 consumers with none pending, and the loopback backend is shared. But each language runner checks answers that the language computed itself, and the session rules are proven again in every host (`libraries/BINDING-AUTHOR.md`: "each migration proves its language's idiom without duplicating the native session test matrix"). Language runners consume the shared files unevenly: `settings.json` is read by about 10 surfaces (git grep under `libraries/`). Once Rust settles each answer, the expected value, probability, pick and resolution can be fixed once in the shared cases.

## Progress

- 2026-10-19 started
