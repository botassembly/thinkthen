# 0668: Generate every C# native declaration and drop layout probes

Status: OPEN.

Milestone: 0.2.1

Phase: 3

Depends on: 0669, 0688, 0727

## Outcome

- Changes: `libraries/csharp/src/`, `libraries/csharp/tests/`, `libraries/csharp/examples/`, `libraries/csharp/check.sh`, `libraries/csharp/README.md`, `build-tools/results/generate.py`, `build-tools/results/templates/csharp.py`, `build-tools/results/csharp/`, `build-tools/results/legacy/csharp.txt`, `conformance/library-surface/csharp.json`.
- C# calls `thinkthen_run_call` with the canonical JSON request (`sdlc/decisions/2026-10-19-c-input-json.md`) and reads the owned call result JSON (`sdlc/decisions/2026-10-19-c-call-owned-json.md`). Every native import is generated into one `LibraryImport` class from the C header.
- The hand-written `Native` class in `ThinkThen.cs` and the `NativeSession` declaration class are deleted.
- Any answer or probability shaping in hand code or in the C# template is deleted in favor of the packet's values.
- The session loop in `OwnedSession.cs` (the `OwnedSession` reader and producer and the `NativeWorker` class) and its private closed, reader and producer error text are deleted. Async calls run the call on one worker thread and cancel through the cancel handle.
- `build-tools/results/templates/csharp.py` leaves this language's exception file `build-tools/results/legacy/csharp.txt`.
- The C# entry in `conformance/library-surface/csharp.json` names the lifecycle owners that remain.
- The `V1` layout probes in `tests/package_check.py` and the `IgnoresAccessChecksTo` test hook are deleted.
- The C# conformance runner becomes a packet dumper following the 0642 contract, and the old fixture consumer (`tests/public_types.py` with `tests/source/TypeCase.cs`) is deleted.
- Sources follow standard .NET formatting, enforced in `check.sh` by `dotnet format --verify-no-changes`.

### Proof

- On the M5 through `pm job run --machine m5 0668 SHA --memory 8 -- sh -c 'python3 build-tools/results/answer_rules.py && python3 build-tools/results/generate.py --bridge --check && python3 build-tools/results/generate.py --check'`. Before the change, `answer_rules.py` fails once the C# template's exception is removed.
- On the M5, `dotnet build libraries/csharp/ThinkThen.csproj` and `dotnet format libraries/csharp/ThinkThen.csproj --verify-no-changes` pass; the format check fails on current main.
- `git grep -nE 'DllImport|LibraryImport' libraries/csharp` finds them only in the generated declaration file.
- On the M5 through `pm job run --machine m5 0668 SHA --memory 8 -- sh libraries/csharp/check.sh`, after 0669 declares `darwin-arm64`, the installed NuGet consumer runs all ten named calls, cancellation and failures against a recorded backend, and its packet dump matches the 0642 oracle. Linux runs at the 0619 container checkpoint.

## Evidence

- Order (coordinator, 2026-10-19): the packaging ticket 0669 lands before the call ticket 0668, so 0668 can prove on the M5. The packet dumper belongs to 0668, written once against the run call.
- After 0727 lands, this language's answer-rule exceptions live in `build-tools/results/legacy/csharp.txt` and its surface entry in `conformance/library-surface/csharp.json`. This ticket edits only those two files, so it does not collide with other languages' call tickets.
- **Hand-written declarations.** `libraries/csharp/src/ThinkThen.cs` (class `Native`) and `libraries/csharp/src/NativeSession.cs` declare the engine, error-slot and session functions by hand with `DllImport`. The generated `libraries/csharp/src/NativeBridge.g.cs` comes from `bridge()` in `build-tools/results/generate.py` and also uses `DllImport`. That is three declaration classes for one header, against the guide's rule to generate declarations (`libraries/BINDING-AUTHOR.md`).
- **Session loop.** `OwnedSession.cs` pushes, waits, arms settlement and reads packets through `thinkthen_session_push`, `thinkthen_session_wait`, `thinkthen_session_next_packet` and `thinkthen_session_try_read`. `thinkthen_run_call` (0638) replaces all of it.
- **Layout probes.** `tests/package_check.py` reflects into internal structs through `IgnoresAccessChecksTo` to probe `V1` layouts left over from the retired layout readers.
- **Style.** `ThinkThen.cs` and the C# source embedded in `package_check.py` pack statements together with one-space indentation.
- **No format versions.** The `_v1` names in the generated declarations come from the C header. Renaming them follows `sdlc/decisions/2026-10-19-no-version-numbers-before-1-0.md` in the core, before the freeze; this ticket adds no version check of its own.
