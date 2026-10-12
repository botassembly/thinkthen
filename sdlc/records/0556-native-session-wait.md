# Native session wait

The native session owns a timed wait for result availability, input capacity and settlement. Waiting observes readiness without consuming a result. Cancellation wakes waiters; settlement waits for the terminal state. A timeout is a successful readiness result. The condition variable uses the same locked predicates as the queue, so a notification cannot race past a waiter.

The bindings replace repeated readiness polls with the native wait while following their host scheduling rules. Python releases the interpreter lock and uses a worker for asynchronous calls. Ruby releases its lock and preserves Fiber scheduling. R uses bounded waits and checks interrupts. JavaScript waits on a worker thread. Dart uses an isolate, Swift and Foundation use dispatch workers, and managed runtimes use workers for asynchronous calls. Go retains the calling thread when reading thread-local errors. Direct synchronous callers use bounded waits.

Linux installed checks exercised C, C++, PHP, Go, Dart, C#, Java, Kotlin, Scala, Swift, Zig, COBOL and Ada. Ruby, R, JavaScript and Python retained their installed checks on the unchanged binding paths. C, C++, Zig and COBOL each passed 40 parity cases plus two session regressions. The Dart check exercised progress on the calling isolate; managed checks exercised worker scheduling. These checks do not establish macOS Foundation execution.

The Mac prerequisite experiment checked the genuine XCFramework assembly path at `a3ba06ec994f5b2697c48c44fb01194dba2e0504`. The selected developer directory contains only Command Line Tools; `xcrun --find xcodebuild` exits 72, and searches found no full Xcode installation. Rosetta and an official arm64 Rust 1.95 toolchain are available, but downloading the Intel Rust target would not supply the required assembler. No downloads, builds, machine configuration changes or substitute frameworks were started. Foundation execution remains unproved by this experiment.

The subsequent experiment invoked both current `check.sh` entry points on the Mac at the same source revision after rereading their READMEs. Swift exited 1 because no final installed Apple package was supplied. Objective-C compiled and executed its standalone generated-result consumer, printing `FOUNDATION_RESULTS_PASS`, then exited 77 because the matching assembled package was missing. Thus result-carrier execution passed, but these commands did not execute installed sessions or cancellation. The checks used isolated environment controls. All processes and evidence transfers exited; the original checkout stayed unchanged.

The shared Engine repair and complete official routine are recorded in `sdlc/records/0554-engine-request-repair.md`. The routine at `ea591efb433b62981f03ac372f022764ce7337cf` passed test and lint. Fresh review accepted that code, the retained recognize regressions, and the explicit C++ fixture ceiling correction.

## What the build taught us

A successful synchronous wait does not prove that a binding preserves its host scheduler. Installed checks must exercise work continuing on the calling scheduler while the native session waits.

Cancellation and settlement require distinct readiness predicates. Treating a cancelled session as permanently ready for settlement would recreate a busy loop before owned work finishes.

The first focused runs did not include the CLI backend recognize cases. The complete routine found missing zero-send facts, closed-output dispatch and observer pacing errors. Both recognize tests remained unchanged through the repair. The retained failed receipts show why narrower session checks could not establish the entire shared execution contract.

Fixture setup and package contents are part of installed checks. Missing C output parents, missing C++ includes and missing generated COBOL constants produced real failures; the corrected checks used rebuilt packages and retained the original logs. Ada compilation exceeded a focused wrapper timeout once, so the unchanged check was rerun with its isolated build output and awaited to completion.
