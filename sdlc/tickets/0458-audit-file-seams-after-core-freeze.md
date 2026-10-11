# 0458: Repair only flagged file seams after the core freezes

Status: COMPLETE.

Opened as: 2026-10-11. The four flagged file families now follow semantic seams. Declarations, conversion behavior and substantive tests are unchanged; fresh review, full lint, actual package checks and integrated full tests passed.

Milestone: 0.2

Owner: builder.

## Outcome

One fresh read-only reviewer inventories hand-written source files over 400 nonblank lines and numbered or mechanical splits. The builder repairs only the files that reviewer flags as lacking a real seam. Public behavior stays unchanged.

## Evidence

- Starts from: Ian's approved ask 2 in shared mailroom message `2026-10-07-pm-file-size-rule-file-cleanup-after-core-freeze-binding-layout-check-and-cache.md`, including the Dart ABI/view splits and files held just below 500 lines.
- Keeps: All public APIs, engine semantics, storage and error behavior. Existing tests establish retained behavior. The 0457 warning and hard limit permit a coherent larger file with an explained size.
- Changes: After the coordinator names the reviewed core-freeze commit, review every qualifying file and judge its actual seam. Merge or re-split only flagged files. Record the bounded inventory and decisions in this ticket's single record; create no audit framework or new ongoing duty.
- Proof: The fresh review names concrete flagged seams. Run the existing affected tests and required landing checks after the repairs. Add no tests that merely assert filenames, arrangement or source counts.
- Defers: Redesign, new features, broad style cleanup and unflagged files. This is the first ticket eligible to move to 0.2.1 if Ian changes delivery priorities; it does not weaken the required layout or cache checks.

## Dependencies and ownership

Wait for 0457 and the reviewed core-freeze commit. Finish required native SDK, SQL, dataframe and MCP changes first. Do not reopen settled API decisions during this cleanup.

Reviews: accept

Reviews: accept

Landed: qualified source 6853677336bf79e5188ebf2538995420899fedeb.
