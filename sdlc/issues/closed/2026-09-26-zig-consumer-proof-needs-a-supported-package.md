# A Zig consumer works through C, but no supported package exists

Status: closed 2026-09-30. Merged into `../2026-09-26-language-packages-need-a-release.md`, which keeps this language's open release items and limits.

## Gap

The ideal state makes the C interface the route to additional languages. Experiment 273 now supplies the Zig implementation, package metadata, examples, documentation, tests and local archive rehearsal. Ticket 0249 supplied reviewed integration. Release packaging and final-release verification remain before supported Zig use.

## Final evidence

Beelink experiment local experiment 273's `stage2/` uses Zig 0.15.2 on Linux x86_64 with glibc. Its immutable source is landed main `e0a6150a0325d92b93ac8f4e504ffbfc3792fba9`; the parent verified all 4,761 exported blobs. No ThinkThen implementation, C header or compiler upgrade was needed. The local native archive uses a release build with remapped builder paths, not the first stage's debug library.

The six-file `package/` exports a real Zig module. It wraps typed scalar/bulk decisions, recognize, relate and the generic JSON door for all ten functions. Rust retains judgment, grammar and scheduling. Zig owns copies of results and error metadata. The package rejects interior NUL in C strings and preserves counted evidence bytes. Its documentation covers blocking calls, thread-safe allocators, engine/token lifetime, one-shot cancellation and the known native limitation.

The worker and parent separately completed the final gate. Parent evidence is `logs/gate-20260926T212430Z/`: 47 outer subprocess receipts and nine child receipts per isolated consumer matched expected outcomes. Two shared and two static-C consumers each accepted 48 independently counted requests across example, verb/ownership matrix, allocation failures, simultaneous callers, held deadlines, cancellation and fresh-token recovery. Actual shared loading used the supplied archive. Static mode had no ThinkThen shared dependency; it is not a fully static executable. Each consumer had a fresh cache, an unrelated path containing spaces, a real archive dependency, and a private network/filesystem namespace without Rust, Cargo, repository source or previous build outputs.

The gate verifies hashes, archive members, source/member byte equality, C layout, version macros and native identity evidence. Planted stale archives, bad header versions, private bytes, source-tree fallback and sanitizer misuse failed at their intended checks. Planted timeout/interruption descendants stopped while an unrelated control survived. Two fresh read-only reviews ran. The second found no additional experiment-side blocker after the approved corrections. The parent accepts the bounded handoff, not release readiness.

## Sealed native blocker evidence

This section records the pre-fix failure. Ticket 0166 repaired it; the post-fix verification below records the passing strict proof.

At the sealed stage-two pin, a scalar C call could return success after its cancellation token fired during an accepted, held HTTP request. All four parent consumers reproduced it. Three independent Python ctypes trials also reproduced it without Zig. Held bulk cancellation, held deadlines and fresh-token recovery passed.

[The closed cancellation issue](2026-09-26-cancelled-c-scalar-call-can-return-success.md) records the repair. The stage-two wrapper did not hide the fault. The sealed stage-two gate reported `FINDING` and exited 1. Matching expected subprocess exits does not make this an all-pass contract.

## Handoff and remaining work

Historical stage-two artifacts remain local and unchanged:

- `stage2/package/`: the six source-package files.
- `stage2/fixtures/` and `stage2/gate.sh`: the implemented Linux tests and offline gate.
- `stage2/artifacts/manifest.json`: source pin, compiler versions, archive/header/library hashes, contents, exports and dependencies.
- `stage2/HANDOFF.md`: exact proposed product destinations and path adaptations.
- `stage2/CASE-LEDGER.md`, `FINDINGS.md`, `REVIEW.md`, and `reviews/`: coverage, parent verification, limits and independent reviews.
- `stage2/parent-verification/`: parent source checks, gate launch evidence and unchanged-source hashes.

The historical experiment copy instruction is complete and superseded by `libraries/zig/` and [ticket 0249](../../records/0249-integration-closure.md). Use the integrated source and its product `check.sh` for the next release work; keep the original experiment evidence below. The remaining requirements are a final-pin rebuild, actual Actions execution, checksummed native release assets, direct source installation and other-host proof.

Keep Zig 0.15.2 for this measured target. Start with a thin optional source package and a separately supplied matching native archive. The current shared mode embeds the supplied library path and requires rebuilding if it moves. A relocatable installation policy or automatic download is a separate product choice. The C constructor remains environment-based and has no public throttle setter.

## Limits and earlier evidence

An earlier intermediate static run accepted 33 rather than 34 requests without identifying the missing request. The final gate checks the input multiset; all eight final worker/parent consumers passed their count checks. The original shortfall remains unexplained. Zig allocation checks and C-caller AddressSanitizer checks passed, including a deliberate misuse. Native Rust allocations were not instrumented, and Valgrind was unavailable. The C API exposes no runtime binary ABI identity. Exhaustive diagnostic secrecy, other targets, other Zig versions, performance and live-model quality remain unproved. Synthetic loopback replies do not measure model accuracy. No paid inference ran.

The first-stage proof remains intact at the experiment root. It tested source `873b04abdeca56bcfc5fcc15b99665b7c32ee116` with a 14-line direct example, an 81-line wrapper, ten independently counted requests and deliberately faulty ordering/message-lifetime copies. Its `FINDINGS.md`, `REVIEW.md` and `HARVEST.md` describe that narrower result. Experiment 205 supplied earlier stand-in lessons; ADR 0037 and ticket 0094 establish the landed C interface.

Go ran as experiment 274 and the JVM as experiment 289, renumbered from a collided 275 claim. Their wrappers account for thread-local C error retrieval; ticket 0249 later integrated both product packages, and their release issues remain open.

## Post-fix verification (2026-09-27)

Ticket 0166 landed the engine fix. A parent-verified rerun at pin `22f36d0006fd34e7390a71d15c0458e493f3e844` (header `7fdca29a...`) rebuilt the native library offline and passed the complete copied gate with exit 0 across two shared and two static-C isolated consumers: held scalar returns cancellation code 5 with untouched outputs, held bulk 5, held deadline 3, fresh-token recovery passes, and a direct-C ctypes proof (`STRICT_C_CANCEL_PASS`) shows code 5, drained accepted request, spent-token refusal, and exact counted states. Evidence: `post-fix/POST-FIX-REPORT.md`, worker gate `post-fix/logs/gate-20260927T140739Z`, parent rerun `post-fix/logs/gate-20260927T142411Z`. The recognize schedule now sends two requests per name on main; consumers must re-derive counts per pin. Integrators carry the strict-pass expectations into the product ticket and rerun on its final release pin.

## Dependencies and installation (Beelink, recorded 2026-09-27)

Host: Ubuntu 24.04.3 LTS, kernel 6.17.0-35-generic, glibc 2.39, x86_64. Zig 0.15.2 at `~/.local/opt/zig-0.15.2` (upstream tarball, pre-existing). Native rebuild needs Rust 1.95.0 with an offline Cargo registry copy. Consumers linking C need clang 18.1.3 (ASan checks) and bwrap for namespace-isolated installs; fixtures are Python 3.12. The package ships as a Zig source archive plus a separate matching native C archive with an explicit absolute path option; no automatic artifact download.

## CI and release direction (Ian, 2026-09-27)

Each port must state its dependencies and how they were installed, and the product must also build and release on GitHub Actions `ubuntu-24.04` runners (x86_64 matches this rehearsal; Zig and the pinned Go/JDK/Kotlin/Scala versions must be fetched or pinned in the workflow), publish native archives through GitHub Releases, and let consumers install directly or through the proper language repository. The queue owner owns the workflow, release, and publishing tickets; nothing is published from the experiments.


## ABI preflight drift note (2026-09-28, from the Dart port)

The C header now exports twenty-one symbols at recent pins: `thinkthen_error_facts_json` joined `thinkthen_engine_new_with` at main `5f069321`. Any consumer ABI preflight this issue recorded with a fixed count (nineteen or twenty at its stage pins) is stale at final pins. J8 must derive the export list from the sealed header at each rebuild instead of pinning a count; the Dart stage-two gate (`exports.py` comparing header declarations against `nm -D`) is the pattern to copy. Evidence: local experiment 300, `stage2/FINDINGS.md`.


## Pre-merge re-pin (2026-09-28, pin 71f25087)

ADAPTED-PASS (packing, envelope, multiset 50->42, retry 3->4 source-proven). Evidence: local experiment's `repin-71f25087-REPORT.md` with the unchanged-gate FAIL preserved as drift record, exact new multisets, and all planted negatives. Contract changes consolidated in `2026-09-28-batching-and-envelope-changed-the-c-door-contract-on-main.md`; merge input and steps in `2026-09-28-language-merge-runbook.md`.

## Landed product source (2026-09-28)

Ticket 0249's landed Zig source used experiment 273 `repin/package/`, which supersedes the earlier post-fix copy instruction above and carries its strict cancellation plus later packing/envelope changes. `libraries/zig/check.sh` currently passes on Ubuntu 24.04.3 x86_64 with Zig 0.15.2 and the current 30-export C header/library: public J1 55 schema / 29 executable cases, named error kinds and copied started-failure facts, settings precedence with invalid-settings zero-send, strict held scalar/bulk cancellation and recovery, and 42 complete normalized request bodies in each of two isolated shared and two isolated static-C source/native consumers. The product keeps the allocation, parser, source privacy, package and host ratchets with a distinct planted private path. The exact build pin and native hash are in `sdlc/records/0249-swift-zig-build.md`. Fresh source review and ticket 0249 integration closure accepted this package. The issue stays open for final release pin, `ubuntu-24.04` Actions build/release, checksummed native distribution and other-platform proof. Static-C linkage is not a fully static executable; local archives are disposable gate outputs.
