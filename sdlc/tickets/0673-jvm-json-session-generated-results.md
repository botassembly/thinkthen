# 0673: Move JVM onto run-call and generated results

Status: OPEN.

Milestone: 0.2.1

Phase: 3

Depends on: 0688

## Outcome

- Changes: `libraries/jvm/session/`, `libraries/jvm/tests/`, `libraries/jvm/examples/`, `libraries/jvm/README.md`, `build-tools/results/templates/jvm.py`, `build-tools/results/templates/jvm_host.py`, `build-tools/results/templates/jvm_native.py`, `build-tools/results/templates/jvm_usage.py`, `build-tools/results/answer_rules.py`, `conformance/library-surface.json`.
- Java, Kotlin and Scala call `thinkthen_run_call` and read the final answer, probability, pick and resolution that Rust puts in the packet.
- Input is the canonical JSON request (`sdlc/decisions/2026-10-19-c-input-json.md`) and results are the owned call result JSON (`sdlc/decisions/2026-10-19-c-call-owned-json.md`), decoded into generated types.
- The JVM session loop in `OwnedSession.java`, `NativeWait.java` and `SessionWorkers.java` and its private closed, reader and producer error text are deleted. Calls run on one worker thread attached to the JVM and cancel through the cancel handle.
- `NativeResults.java`, `NativeUsageLayouts.java` and their generator template `jvm_native.py` are deleted, along with every layout reader and every probability or answer computation in JVM hand code and in `jvm.py` and `jvm_host.py`.
- The hand-written encoder `Json.java` is replaced by generated serialization of the request JSON.
- Old public names are removed, with no compatibility shims. The JVM entry in `conformance/library-surface.json` names the lifecycle owners and members that remain.
- `jvm.py` and `jvm_host.py` leave the `LEGACY_FILES` exceptions in `build-tools/results/answer_rules.py`.

### Proof

- On the M5 through `pm job run --machine m5 0673 SHA --memory 8 -- sh -c 'python3 build-tools/results/answer_rules.py && python3 build-tools/results/generate.py --target jvm --check && sh libraries/jvm/build.sh'`, the generator is current and the three host JARs compile. Before the change, `answer_rules.py` fails once the two JVM exceptions are removed.
- `git grep -n 'ValueLayout\|MemoryLayout' libraries/jvm/session` finds only the declarations that call `thinkthen_run_call` and free its handles; on current main it finds the layout readers.
- The JVM check needs bubblewrap and is declared for `linux-x86_64` only in `sdlc/tools.json`, so the installed-package checks (`session_installed.py`, `usage_installed.py`, `package_check.py`) run all ten named calls for Java, Kotlin and Scala against a recorded backend at the 0619 container checkpoint on Yellow (x86_64), through `sh libraries/jvm/check.sh`. Each packet dump matches the 0642 oracle.

## Evidence

- The JVM binding decodes errors, plans and packets by reading native C memory layouts directly: `Engine.java` calls `NativeResults.failure` and `NativeResults.value`, and `NativeResults.java` walks `thinkthen_complete_session_packet_v1` views. `jvm_usage.outputs` in `build-tools/results/templates/jvm_usage.py` writes `NativeResults.java` (from `jvm_native.py`), `NativeWait.java`, `NativeNamed.java` and `NativeUsageLayouts.java`; `build-tools/results/generate.py` calls it for `--target jvm`.
- Every change to a native layout breaks it. Commits that chase layouts include cef6f6f6c, 7fa174151 and 1b88c2d2e. The JVM side also patched the probability type four times on 10-14 and 10-15 (55686dac0, d5c7838bc, d1369ebf1, 211b2159b) because it shapes answers and probabilities itself.
- `Json.java` is a hand-written JSON encoder used by `Engine.java`, `Inputs.java`, `NativeNamed.java` and `NativeSession.java`.
- The generated `RequestVersion.java` holds the request format version `thinkthen.request/1`. It goes with the core's removal of format versions under `sdlc/decisions/2026-10-19-no-version-numbers-before-1-0.md`, before the freeze; this ticket adds no version check.
