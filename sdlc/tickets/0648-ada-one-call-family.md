# 0648: Keep one Ada call family on run-call and packet answers

Status: OPEN.

Milestone: 0.2.1

Phase: 3

Depends on: 0688

Depends on: 0649, 0727, 0728

## Outcome

- Changes: `build-tools/results/templates/ada*.py`, `build-tools/results/registry/ada.json`, `build-tools/results/legacy/ada.txt`, `libraries/ada/src/`, `libraries/ada/checks/`, `libraries/ada/check.sh`, `libraries/ada/examples/`, `libraries/ada/README.md`, `libraries/ada/README-PUBLIC.md`, `conformance/library-surface/ada.json`, `site/examples/functions/*/ada.adb`, `site/src/data/catalog/ada.mjs`.
- One family remains: the named functions in `Thinkthen.Inputs.Calls`. Each returns an owned result and takes an optional producer for fed input and a cancellation handle. The procedure set in `Thinkthen.Sessions.Calls` and its duplicate in `Thinkthen.Inputs.Calls` are deleted.
- Each call runs `thinkthen_run_call` once. The session loop in `Thinkthen.Native_Sessions` and its private closed, reader and producer error text are deleted. A task that needs to cancel uses the handle.
- Value and probability come from the packet's answer fields. The projection code in `Thinkthen.Answers` and its templates are deleted.
- One Ada packet dumper following the 0642 contract replaces `checks/full_case.adb`, `checks/native_session.adb` and their Python drivers, which are deleted.
- The Ada templates leave this language's exception file `build-tools/results/legacy/ada.txt`, and the Ada entry in `conformance/library-surface/ada.json` names only the remaining family.
- The README and the site's Ada text describe only the remaining family. A site sample whose call changes replays with the change.

### Proof

- On the M5 through `pm job run --machine m5 0648 SHA --memory 8 -- sh libraries/ada/check.sh`, an outside GNAT project imports the unpacked `thinkthen.gpr`, runs `examples/session_demo.adb`, and the Ada dumper matches the 0642 oracle for every shared case.
- `python3 build-tools/results/answer_rules.py` and `python3 conformance/library-surface.py --library ada` pass. Before the change, removing the Ada entries from `LEGACY_FILES` fails the first check on the Ada answer templates.

## Evidence

- After 0727 lands, this language's answer-rule exceptions live in `build-tools/results/legacy/ada.txt` and its surface entry in `conformance/library-surface/ada.json`. This ticket edits only those two files, so it does not collide with other languages' call tickets.
- Every Ada source except `thinkthen.ads` and `thinkthen.adb` is generated; the `ada` group in `build-tools/results/registry.py` lists them, from the `build-tools/results/templates/ada*.py` templates.
- Ada exposes two named-call families: procedures on a session owner in `Thinkthen.Sessions.Calls` (`thinkthen-sessions-calls.ads`) and in `Thinkthen.Inputs.Calls`, plus blocking functions in `Thinkthen.Inputs.Calls` (`thinkthen-inputs-calls.ads`). That conflicts with one public API (`libraries/BINDING-AUTHOR.md`).
- `thinkthen-native_sessions.adb` runs the session loop over `thinkthen_session_new_named_with_surface`, `thinkthen_session_next_packet` and `thinkthen_session_wait`, imported in `thinkthen-session_c.ads`.
- `thinkthen-answers.adb` (about 7,100 generated lines) projects answers and probabilities. `answer_rules.py` lists `ada_answers.py`, `ada_call_results.py` and `ada_inputs.py` as legacy exceptions.
- The fixture consumers use the deleted family: `checks/full_case.adb` reads `Thinkthen.Native_Sessions` and the session result JSON, and `checks/native_session.adb` drives `Next_Packet` through `Thinkthen.Sessions.Calls`. So the dumper lands with this ticket.
- `conformance/library-surface.json` lists `Next_Packet`, `Session_Owner`, `Push_Status` and `Read_Status` for Ada.
- The site samples `site/examples/functions/*/ada.adb` call the blocking functions in `Thinkthen.Inputs.Calls`.
- 0649 adds darwin-arm64, so this ticket proves on the M5.
