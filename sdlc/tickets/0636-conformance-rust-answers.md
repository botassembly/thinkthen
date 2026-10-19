# 0636: Check settled answers and run-call behavior once in shared conformance

Status: OPEN.

Milestone: 0.2.1

Phase: 2

Depends on: 0612, 0638

Reviews: revision 788fda3, accept

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
- 2026-10-19 next: PM accepts closed-engine proof as a session safely outliving its original engine with owned result and error lifetimes; shared conformance adds no freed-pointer test or close API, and disposed-wrapper errors remain each language phase3 idiom checks.
- 2026-10-19 next: PM accepts the existing installed CLI, Rust and C entrypoints with routine case selection from one fresh production artifact set as this ticket proof, without claiming the all-host surfaces command; the routine C source-path Darwin suffix defect belongs to0641.
- 2026-10-19 next: Retained production job1792441871889-70440 at a8527005c passed installed C including static and blocking checks and Rust routine plus call, session and settled behavior, while full CLI passed242 of243 and its Linux-proc signal-consumption fixture cannot run honestly on Darwin, so the exact artifacts and all passes remain retained while PM rules on unchanged Linux-cell proof.
- 2026-10-19 next: Approved platform-split proof is complete at a8527005c: retained Mac job1792441871889-70440 passed installed C, Rust and242 CLI cells, and Linux job1792443373174-7176 passed the unchanged held-call cancellation cell for SIGINT and SIGTERM, so fresh whole-ticket review now checks the combined evidence before closing.
- 2026-10-19 next: Fresh whole-ticket source and installed-evidence review found no issues at a8527005c and independently verified retained archive hashes and the approved Mac/Linux proof split, while closing lint and test including policy remain required before acceptance and landing.
- 2026-10-19 next: Fresh source and installed-evidence review found no issues, but closing M5 job1792444614735-51795 at45e7de8ea failed in the hosted-workflow Apple swift exit probe at its unchanged five-second timeout, so the failed receipt is retained and shared fixture diagnosis precedes any new gate.
- 2026-10-19 next: Combined closing job1792445449323-47359 stopped at the children scanner before tests because0703 hid env in keyword expansion, and fresh0703 fix94a6a34a8 now passes scanner and Darwin proof while the PM-priority0705 fixture-reliability repair is reviewed before the next combined closing gate.
