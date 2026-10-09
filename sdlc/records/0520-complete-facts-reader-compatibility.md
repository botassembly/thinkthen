# Complete facts reader compatibility

The repair starts from `e94ab9b5c63ba851e308b1f0b3744371f1e1d4a8`. Python, TypeScript, Ruby and R retain request byte counts, nullable estimated token counts, the estimate method and the shared persistence observation. The persistence carrier retains state, observation point and optional advice. Older facts retain their absence semantics. Readers still reject unrelated unknown fields.

The engine serializer in `crates/thinkthen/src/public/results/complete_facts.rs` supplies this contract. Python declares the carrier in `_complete.py`; `complete.pyi` already reexports those declarations. TypeScript adds the carrier and facts fields to `_complete.d.ts`. Ruby and R use their existing descriptor-derived carriers. No engine or C ABI code changes.

The shared old fixture remains intact. Its new observation table covers numeric, zero and unavailable estimates, all persistence states and failure advice. Each language checks typed access and plain serialization. Existing installed native consumers check actual eager and terminal batch facts. Python's existing declaration consumer also checks the new types.

## Evidence

The saved observation cases rejected the old readers. Python reported `ValueError: invalid complete result`; TypeScript and Ruby failed their added case; R reported `invalid complete result`. The repaired pure suites pass: Python 19 tests, TypeScript 7 tests, Ruby 7 tests and the R fixture script. The original snapshots remain in those suites.

Existing `native_fixture.run` cases `01-decide-yes-captured`, `settings-replay-answers-from-the-folder-alone`, `images-decide` and `stream-staged-batch` pass for installed Python, JavaScript, compiled TypeScript, Ruby and R consumers. The decide case includes eager and terminal batch calls. The saved case retains its zero-send assertion. Existing request counters remain authoritative. Python and TypeScript compile against their installed public declarations.

The local wheel, npm archive, native gem and vendored R source archive were installed outside the source packages. Archive readers matched the repaired source files; consumer imports resolved to those installations. Archive SHA-256 values:

- Python wheel: `e6086e5a2c3e31662f6194dcfcdac16546d252fad5d89e042563fc9f1825d958`.
- npm archive: `147afa0fc0776df7970612048edb3dd2c75da5751761c4992b11e6cf80b4933c`.
- Native Ruby gem: `6d5b9328a1ce9977bbd2fd4774b3b27bfd9a64265b9f79a71c30c4a79bb08a87`.
- Vendored R source archive: `ff32a34e1971bfe298275d352d68175ffb622d72bc1a0b64dd5df76326ac0e7e`.

The retained `sdlc/scripts/smoke --one target/debug libraries/r` check passes through the existing fresh-install runner. `CARGO_NET_OFFLINE=true python3 sdlc/scripts/policy.py` passes. Removing duplicate Python descriptor overrides and repeated rejection branches keeps that reader at 999 nonblank lines. The TypeScript declaration growth describes the required engine fields and typed observation.

The checks used an owned systemd unit with 8 GiB memory, 1 GiB swap and two build jobs. Its final process exited successfully and the unit is inactive. Logs and temporary artifacts are under `target/0520-readers/`; installed Python and JavaScript evidence is in `installed-consumers.log`, the remaining consumers and R smoke are in `installed-remaining.log`. Initial environment failures were corrected by selecting stable Python 3.13, the pinned compiler, the existing vendored R builder and Rust 1.95.0.

The coordinator's combined qualification at `986324637d902f165a7459fef53f8cf07dfab984` stopped during lint with exit 1. `target/0520-combined-qualification/lint.log` records the Python source count of 7784 against its stale ceiling of 7754; `result` records `lint 1`. The runner stops at the first failure, so full test and specification checks did not start and remain owed.

The metadata repair starts from `6fa915274bc2704d32090eccc2eb72c08a45c7ef`. The existing `sdlc/scripts/ratchet.mjs` checker measures Python at 7784, R at 3356, Ruby at 3361, and TypeScript JavaScript, test modules and declarations at 825, 1727 and 1578. Their existing ceilings now equal those measurements. The retained facts fields, observation types and existing reader and installed-consumer assertions account for the growth. Inspection covered those additions and the Python duplicate removals already made by the repair. The root and every other binding and database ceiling already match. This metadata repair adds no product logic, tests or checking tools; the coordinator resumes combined qualification.

## Limits and lessons

Combined qualification at `dfc8094b3aa3a50ced84b49dd16f0bf2d39d5a28` passed `sdlc/scripts/lint`, `test` and `spec`. The owned run in lane 2 recorded `lint 0`, `test 0` and `spec 0` under `target/0520-combined-final/` and exited successfully. This includes the installed-package smoke checks. No candidate, release workflow or publication ran.

Ordinary native calls emit numeric token estimates. The current checked arithmetic can return null only for a request byte count beyond practical body limits. Saved snapshots cover that nullable type boundary; no enormous body was allocated, and image calls are not claimed to produce null. Actual calls cover the measurements and persistence observations the engine emits.

This defect came from repeated result contracts drifting behind the engine serializer. Existing raw native checks missed rejection by public eager and terminal readers. Keep those public consumer assertions when generated results replace the handwritten copies under `sdlc/tickets/0513-generated-typed-results-with-presence.md`. This repair does not begin that migration. The coordinator owns whole-repository qualification and landing.
