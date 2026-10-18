# 0668: Generate every C# native declaration and drop layout probes

Status: OPEN.

Milestone: 0.2.1

Phase: 3

Depends on: 0688

## Outcome

- C# calls the single run-call entry point. Every native import is generated into one `LibraryImport` class from the C header.
- The hand-written `Native` and `NativeSession` declaration classes are deleted.
- Any answer or probability shaping in hand code is deleted in favor of the packet's values.
- The session loop in `OwnedSession.cs:24-111` and its private closed, reader and producer error text are deleted. Async calls run the call on one worker thread and cancel through the handle.
- Its files leave the 0637 answer-rule check's list of exceptions.
- The V1 layout probes and the internal-access test hooks are deleted.
- Sources follow standard .NET formatting, enforced by `dotnet format --verify-no-changes`.

### Proof

- On the M5 through pm job, the isolated NuGet consumer (`isolated_consumer.py`, `tests/source/Installed.cs`) installs the packed package.
- It runs all ten named calls, cancellation and failures against a recorded backend.
- A lint finds no `DllImport` or `LibraryImport` outside the generated declaration file.

## Evidence

- **Hand-written declarations.** The C# pilot writes 18 native declarations by hand, in `libraries/csharp/src/ThinkThen.cs:8-16` and `NativeSession.cs:6-16`. Only 6 live in the generated `NativeBridge.g.cs`. That is three declaration classes for one header, which breaks the guide's rule to generate declarations (`libraries/BINDING-AUTHOR.md:13`).
- **Old declaration style.** It uses `DllImport`, not the source-generated `LibraryImport`.
- **Layout probes.** `tests/package_check.py:224-235` reflects into internal structs (`IgnoresAccessChecksTo`) to probe "V1" layouts left over from the retired layout readers.
- **Style.** The source is packed one statement after another, with one-space indentation (`ThinkThen.cs`, `package_check.py`), which is not normal .NET style.
