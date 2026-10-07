# 0459: Check copied bindings against the C header

The existing C export checker now derives layouts, constants and function signatures from the actual header and target compiler. C#, JVM, PHP, Dart/Flutter, Ada and COBOL gates compare their own compiled or runtime declarations against those facts. The public C ABI is unchanged. Direct-header and native-source bindings retain their existing routes.

Retained Linux results establish agreement for 127 represented layouts in each copied family, with each family's represented constants and imported functions. Existing plants reject changed layouts, constants and signatures. The corrected JVM check also rejects equal-size and nested by-value carrier changes; its focused checks and policy passed at 3d11a7986. Installed Java, Kotlin, Scala, PHP, Dart, Flutter, Ada and COBOL consumers passed their existing public-call checks. C# layout and signature checks passed; its portable fixture now asserts the single authored request independently. The earlier C# oracle failure is not counted as an installed pass.

The coordinator consumed the accepted whole-change review and the accepted JVM correction review. Current main f464b9b5e is integrated. The coordinator ran the final checks on e5a0d1731 in an owned memory-limited scope after the task sandbox refused access to the user bus. Full tests passed: 1738 workspace tests, 336 library-only tests, 23 consumer tests, doctests, retained shell and ledger checks, and all 19 binding smokes. Full lint and specification checks passed, including all 24 green demos. Only ticket and plan metadata changed after those checks.

Windows/macOS execution and the complete installed 248-case campaign remain under 0432 and platform qualification. These Linux checks establish no other platform's layout. Existing secrecy, cancellation, memory ownership and request-count checks remain.

## What the build taught us

Matching sizes alone cannot establish a by-value structure's calling layout. Compare its recursive structure as well. A packed request's actual send count and a conservative plan count describe different behavior.
