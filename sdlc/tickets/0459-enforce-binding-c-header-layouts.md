# 0459: Fail binding checks when copied C declarations drift

Status: in progress. Shared compiler-derived ABI foundation is implemented; copied-family gate adoption remains.

Milestone: 0.2

Owner: lane 2 resumes the shared foundation and copied-family gate adoption. Current exact COBOL claims: `libraries/cobol/checks/exports.py` `libraries/cobol/check.sh` `libraries/cobol/ratchet.py.json`. The shared private carrier fixture claims are `libraries/ada/checks/typed_boundary.c` `libraries/ada/ratchet.c.json`; no public C ABI is changed. Current exact Dart/Flutter claims: `libraries/dart/checks/exports.py` `libraries/dart/check.sh` `libraries/dart/lib/src/door.dart` `libraries/dart/CHANGELOG.md`. Current exact Ada claims: `libraries/ada/checks/exports.py` `libraries/ada/check.sh` `libraries/ada/ratchet.py.json`. Current exact JVM claims: `libraries/jvm/door/thinkthen/NativeCalls.java` `libraries/jvm/door/thinkthen/Door.java` `libraries/jvm/tests/package_check.py` `libraries/jvm/tests/InstalledJava.java` `libraries/jvm/tests/consumer-run.py` `libraries/jvm/check.sh` `libraries/jvm/ratchet.java.json` `libraries/jvm/ratchet.py.json`. Current exact C# claims: `libraries/csharp/tests/source/Installed.cs` `libraries/csharp/tests/isolated_consumer.py` `libraries/csharp/tests/package_check.py` `libraries/csharp/check.sh` `libraries/csharp/ratchet.py.json`. Current exact PHP claims: `libraries/php/fixtures/abi.py` `libraries/php/check.sh` `libraries/php/ratchet.py.json`. Shared foundation paths: `sdlc/scripts/check-c-exports.py` `libraries/c/check.sh` `sdlc/scripts/lint`; add exact host paths as each existing gate is adopted. Native header and product APIs stay unchanged.

## Outcome

Every binding that copies the C ABI has declarations that match the public C header. Generation or an existing-gate check fails a mismatch in struct fields, layout, enum values or exposed function signatures. PHP's missing `thinkthen_result_row` prototype is corrected in the pushed family follow-up `06f295ace`; this ticket prevents future drift.

## Evidence

- Starts from: Ian's approved ask 3 in shared mailroom message `2026-10-07-pm-file-size-rule-file-cleanup-after-core-freeze-binding-layout-check-and-cache.md`. The reviewed SDK candidate carries the additive rank-member details getter; PHP's hand-copied declarations omitted `thinkthen_result_row`; `06f295ace` corrects it and exercises the real FFI refusal path.
- Keeps: The public C ABI, actual platform-dependent alignment and widths, existing bare/complete calls and memory ownership. Bindings that directly compile against the header or native Rust source retain that route.
- Changes: Exact owned paths: `sdlc/scripts/check-c-exports.py` `libraries/c/check.sh` `sdlc/scripts/lint` `libraries/php/fixtures/abi.py` `libraries/php/check.sh` `libraries/php/ratchet.py.json` `libraries/csharp/tests/source/Installed.cs` `libraries/csharp/tests/isolated_consumer.py` `libraries/csharp/tests/package_check.py` `libraries/csharp/check.sh` `libraries/csharp/ratchet.py.json` `libraries/jvm/door/thinkthen/NativeCalls.java` `libraries/jvm/door/thinkthen/Door.java` `libraries/jvm/tests/package_check.py` `libraries/jvm/tests/InstalledJava.java` `libraries/jvm/tests/consumer-run.py` `libraries/jvm/check.sh` `libraries/jvm/ratchet.java.json` `libraries/jvm/ratchet.py.json` `libraries/dart/checks/exports.py` `libraries/dart/check.sh` `libraries/dart/lib/src/door.dart` `libraries/dart/CHANGELOG.md` `libraries/flutter/check.sh` `libraries/ada/checks/exports.py` `libraries/ada/check.sh` `libraries/ada/ratchet.py.json` `libraries/ada/checks/typed_boundary.c` `libraries/ada/ratchet.c.json` `libraries/cobol/checks/exports.py` `libraries/cobol/check.sh` `libraries/cobol/ratchet.py.json`. Inventory the actual copied declarations. Generate them from the header where practical or compare them at their real compiled/runtime boundary through the existing binding gates. Cover layout and function declarations, not just matching names. Keep platform facts derived from the actual C compiler and each language's real declarations; do not assume one platform's offsets everywhere. Batch identical work under existing family owners.
- Proof: Plant a mismatched field/layout, enum and function declaration and make the owning existing gate fail. Run actual C-backed binding checks with required toolchains; exit 77 cannot count as success. Existing installed consumers and secrecy/cancellation cases remain required. No separate per-language write-ups or receipt system.
- Defers: ABI redesign, new functions and unrelated package work. This correctness check is required in 0.2 and is independent of the optional timing of 0458.

## Dependencies and ownership

Use the settled public C header from 0426/0431 and coordinate copied files with 0427–0430. The PHP prototype correction is already pushed in the active family follow-up; the full drift check remains this ticket's outcome.

Reviews: accept

## Shared gate adoption

The existing `check-c-exports.py HEADER LIBRARY` route retains export checking. `--describe HEADER` prints live JSON with records (size, alignment and every field’s type, offset and width), constants, function return/argument types and widths, and the C calling convention. `--self-test HEADER` refuses field order, constant, argument, return, pointer-depth, by-value and missing-prototype plants. C source, installed and smoke checks use their actual header; lint runs the same plants. No baseline is stored.

Copied family gates import `header_abi` and `compare_abi` from that same script. Each gate independently enumerates its represented records, constants and imported functions from its actual host declarations, selects those public declarations from the live header facts, and compares a complete host description for that selection. Missing fields or imports fail as well as changed values. The host supplies actual size/alignment/offset/field-width and calling signatures; it must not copy the C probe’s numbers into its own declaration output. Existing public consumers retain execution, secrecy and cancellation proof.

| Copied family | Actual declarations | Gate adoption |
| --- | --- | --- |
| C# | `NativeAbi*.cs`, `NativeMethods.cs`, `NativeBatchMethods.cs` | Actual managed layout/field offsets and imported method return/argument/by-value signatures |
| JVM Java/Kotlin/Scala | `NativeLayouts*.java`, `NativeCalls.java` | Actual FFM layout, field offsets and function descriptors; all three consumers retain their existing gates |
| PHP | `src/native/abi.h` | Compiler comparison of the copied C declarations and actual PHP FFI layout; generate its declaration-only copy where practical |
| Dart/Flutter | `lib/src/native/abi*.dart`, `functions.dart` | Actual FFI layouts and typed signatures through the shared source/installed consumer |
| Ada | `thinkthen-native*.ads` | Existing `typed_boundary.c` and `carrier_bounds.adb` compare represented declarations |
| COBOL | native copybooks and `src/tt_native.h` | Existing C boundary and `carrier_bounds.cob` compare actual copied storage/declarations |

Rust, Python, JS/TS, Ruby and R compile native source; Go, C++, Swift, Zig and Objective-C compile the installed C header. They retain that route. Clang is an explicit declaration-discovery prerequisite for this foundation; the target C compiler supplies all layout facts. The emitted portable C11 probe has a separate MSVC compilation path. Windows must measure with its own compiler/runtime. MSVC’s C11 support, toolchain environment and host checks remain unqualified until native execution. Missing tools fail qualification and never count as an exit-77 success. No Linux offsets establish Windows or macOS proof.
