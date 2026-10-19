# 0651: Thin COBOL calls onto run-call and packet answers

Status: OPEN.

Milestone: 0.2.1

Phase: 3

Depends on: 0688

Depends on: 0650, 0727

## Outcome

- Changes: `build-tools/results/templates/cobol*.py`, `build-tools/results/registry.py`, `build-tools/results/legacy/cobol.txt`, `libraries/cobol/native-helpers.json`, `libraries/cobol/copybooks/tt-native-generated.cpy`, `libraries/cobol/src/`, `libraries/cobol/checks/`, `libraries/cobol/check.sh`, `libraries/cobol/examples/`, `libraries/cobol/README.md`, `libraries/cobol/README-PUBLIC.md`, `conformance/library-surface/cobol.json`, `conformance/library-surface-test.py`, `conformance/children/usage_installed.py`, `sdlc/scripts/release-pack`, `site/examples/functions/*/cobol.cob`, `site/src/data/catalog.mjs`.
- One family remains, the blocking `TT_DECIDE` family, and it calls `thinkthen_run_call`. Its answer records copy the packet's answer fields instead of projecting them.
- The streaming `TT_SESSION_*` family, including the `TT_SESSION_NEXT` loop, `examples/session.cob` and the private closed, reader and producer error text, is deleted.
- The COBOL binding enforces its own 8192-byte text limit and no longer calls `thinkthen_session_new_typed_with_surface`, so 0690 can delete the shared cap and `THINKTHEN_COBOL_OVERFLOW`.
- The projection code in `cobol_blocking.py` is deleted, and `cobol_results.py` leaves this language's exception file `build-tools/results/legacy/cobol.txt`. Generated copybooks shrink to match.
- One COBOL packet dumper program over the public copybooks, following the 0642 contract, replaces the per-case drivers (`checks/blocking.py`, `checks/session.py`, `checks/session_records.py`, `checks/session_consumer.py`, `checks/native_parity.py`, `checks/owned_reader.py`), which are deleted. Python only feeds cases and compares dumps.
- The README, the COBOL entry in `conformance/library-surface/cobol.json` and the site's COBOL text describe one family. A site sample whose call changes replays with the change.

### Proof

- On the M5 through `pm job run --machine m5 0651 SHA --memory 8 -- sh libraries/cobol/check.sh`, the gate unpacks the archive, compiles `examples/blocking.cob` and the dumper outside the repository, and the dump matches the 0642 oracle for every shared case.
- `python3 build-tools/results/answer_rules.py` and `python3 conformance/library-surface.py --library cobol` pass. Before the change, removing `cobol_results.py` from `LEGACY_FILES` fails the first check.

## Evidence

- After 0727 lands, this language's answer-rule exceptions live in `build-tools/results/legacy/cobol.txt` and its surface entry in `conformance/library-surface/cobol.json`. This ticket edits only those two files, so it does not collide with other languages' call tickets.
- `libraries/cobol/native-helpers.json` (generated, in the `cobol` group of `build-tools/results/registry.py`) lists two call families: blocking `TT_DECIDE` through `TT_RELATE`, and streaming `TT_SESSION_DECIDE` through `TT_SESSION_RELATE`. The templates `build-tools/results/templates/cobol.py`, `cobol_native.py` and `cobol_blocking.py` generate both.
- `cobol_blocking.py` builds answer records with its own per-function projection, another copy of the answer rules.
- `native-helpers.json` names `thinkthen_session_new_typed_with_surface` as its native constructor. That path passes the COBOL identity that makes `libraries/c/src/ffi/native_request/ffi.rs` apply the 8192-byte cap; 0692 kept the cap for this ticket to move into COBOL.
- The checks compile COBOL per case through Python: `checks/blocking.py`, `session.py`, `session_records.py`, `session_consumer.py`, `native_parity.py` and `owned_reader.py`. `session.py`, `session_consumer.py` and `native_parity.py` call the `TT_SESSION_*` family this ticket deletes, so the dumper lands here.
- `conformance/children/usage_installed.py` compiles a COBOL caller that uses `TT_SESSION_TEXT` and `src/tt_session.h`. `conformance/library-surface-test.py` copies `libraries/cobol` and plants changes in `src/tt_session.h`. `conformance/library-surface.json` names `TT_SESSION_` as COBOL's function prefix and `TT_SESSION_NEXT` and `TT_SESSION_TEXT` as representation calls.
- The `cobol` case of `sdlc/scripts/release-pack` copies `examples/*.cob` and `src/tt_session.*`.
- The site samples `site/examples/functions/*/cobol.cob` use the blocking family.
