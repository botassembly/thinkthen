# 0530: Assemble final distributions from the package inventory

Status: OPEN.

Milestone: 0.2

Depends on: 0494
Depends on: 0495
Depends on: 0496
Depends on: 0497
Depends on: 0498
Depends on: 0499
Depends on: 0501
Depends on: 0504
Depends on: 0505
Depends on: 0516
Depends on: 0518
Depends on: 0519
Depends on: 0522
Depends on: 0523
Depends on: 0524
Depends on: 0525
Depends on: 0526
Depends on: 0527
Depends on: 0528
Depends on: 0529

Reviews: revision a087f6dc3, accept

Reviews: revision 36382970a, accept

Reviews: revision 651edfb82, accept

Reviews: revision fe45b8d3f00f1400e013b96e781590f5b06a0af3, accept

Reviews: revision e0522c2bcd2ce368209a5af1abefdd7aa50e338d, accept

Reviews: revision 80d53652ca0d4bb00a8b7059a4ae924ab72a1877, accept

Reviews: revision 8bf850435a4efee744ad27fcbde3983a16cc5474, accept

Reviews: revision 048f35439df3668ceb034427aa9fc542cf3156df, accept

Reviews: revision 231a3aed02e71ea7ab2d35f443a7271708bc1cdf, accept

## Outcome

The release scripts assemble every final registry-format artifact from 0501's inventory. Each artifact installs and runs with its declared dependencies, without a checkout, warm loader state or a manual library path. An unsupported target refuses clearly.

## Evidence

- Starts from: [the ticket cleanup ruling](../decisions/2026-10-09-rewrite-binding-tickets-in-place.md) and finding 5 of [the surface assessment](../records/0521-surface-contract-assessment.md). `sdlc/scripts/release-registry.py` selects only the existing JVM jars. `sdlc/scripts/release-workflow` permits Objective-C artifacts only on Linux. Artifact collection in `.github/workflows/release.yml` has target-specific inputs.
- Keeps: The existing installed artifact checks and Ian's publication approval.
- Changes: Registry assembly, workflow validation and artifact collection read the native inventory from 0501. Objective-C routing follows the Apple-only ruling in 0518. The `npm-assemble` operation in `sdlc/scripts/release-workflow` packs every inventoried addon, including `win32-x64`. Each host migration adds its own package to the inventory, as 0501 set out. Assembly refuses when a shipped package is missing from the inventory. The npm and JVM package definitions gain their native file names, as [the native package design](../decisions/2026-10-09-native-package-design.md) sets them, and `sdlc/scripts/package-inventory.py` reads the names there and keeps no list of its own. Claim `sdlc/scripts/package-inventory.py`, `libraries/typescript/package.json`, `libraries/jvm/pom.xml`, `sdlc/scripts/release-registry.py`, `sdlc/scripts/release-workflow`, their existing self-tests and artifact collection in `.github/workflows/release.yml`. Name each affected installed-consumer route before editing.
- Remaining installed consumer: `sdlc/scripts/install_check_consumers.py:72` still imports the retired JVM `thinkthen.Door`, and `install_check_channels.py:173–174` compiles and runs it with Java 21 preview flags. Move that Maven consumer to the stable typed client and the supported JDK before candidate install checks. The 0504 closure review confirmed this pre-existing distribution-check gap; it does not hold the local JVM migration open.
- Remaining full-parity consumer: `libraries/go/check.sh` accepts the `full` profile but invokes the same `fixtures/shared_installed.py` routine selector, which filters out error cases and uses the small type corpus. Restore a typed installed Go route through every required shared conformance case before the candidate; keep the small routine selector for migration checks and run full parity only at the candidate.
- Remaining Apple execution: run 0518's Foundation shared-case route against the assembled Apple package, including all ten named calls, typed result access, ARC ownership, failures and held-provider cancellation. Linux generation or SDK compilation does not establish Apple runtime parity. The previous Foundation fixture covered generated carriers and two execution calls; the old GNU runner cannot count as Foundation evidence.
- Proof: Install each final registry-format artifact with its declared dependencies in a clean environment and run a real call with no library path. A missing or wrong native asset and an unsupported target fail in the existing installed checks. A passing development archive does not count.
- Defers: Native platform qualification goes to 0383–0385 at the candidate. The [2026-10-10 closure ruling](../decisions/2026-10-10-drive-0-2-to-done.md) authorizes candidate tags and testing workflows when the preceding work is ready; registry publication still requires Ian's go.

Author: project manager.

## Progress

- 2026-10-10 landed 474d0116e; next: Committed Dart, Flutter and COBOL archive inventories are landed and fake workflow checks pass. Final installed distribution and platform qualification remain held.
- 2026-10-10 started
- 2026-10-10 landed 8dc6be679; next: Managed packages, JVM native classifiers, PHP and Ada archive members and Windows npm addon assembly now follow the current native package contracts. Fresh review and small assembly checks pass. Final installed distribution and platform qualification remain held.
- 2026-10-10 landed 670c88047; next: Current typed registry consumers and the full Go route are reviewed and pass focused installed checks; assemble final artifacts and run complete installed and platform qualification at the candidate.
- 2026-10-10 next: Fix final NuGet assembly to include every native RID and add required Windows installed SDK consumers, then build and qualify the final candidate packages.
- 2026-10-10 landed f415310b07a536a24cb4c90c5f98c8f6a0ac5bf7; next: NuGet now assembles all five native RIDs from captured C archives; finish the Windows installed SDK job and qualify final packages.
- 2026-10-10 landed 8aadd204de37aa071373cfc341a5745e8bc324e7; next: The required Windows SDK candidate job is reviewed and landed; build fresh local packages and run the complete installed release checkpoint.
