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

Reviews: revision 500bf5301c559d7e7b4e590388e48d45e4622da0, accept

Reviews: revision 8102ec5f95e7cc285ddba18e35cff8628ed002d6, accept

Reviews: revision f1f46107d45d4909b33b7fb367c368a782302d5e, accept

Reviews: revision e04aac100a7a1bd098709e4c0f580a02e71a6c11, accept

Reviews: revision 00e932aebf63e5d3d2bed017928a7e133e37d5b2, accept

Reviews: revision a11c38061716102c79ab279ccef76088eb081388, accept

Reviews: revision 7a5d5e6359c86b989dac48e0c839815f1675806d, reject

Reviews: revision 3239001da64a32af38db83e345ef0de5009ef684, accept

Reviews: revision 6e3201fb054d513e8b08fbc1e786070b120ebf83, accept

Reviews: revision cd07539b64fb200987fba612d851813133f767ee, accept

Reviews: revision fb05f5b9bafc7dfb12d95af2b2a4dc49c5eaa775, accept

Reviews: revision 852f1579e29f125ab4b30e25c7ef3e2a16ed4db5, accept

Reviews: revision b978c7b61496f9c0c39907d7937f13ecd812c720, accept

Reviews: revision 45c570919b1d928aa71486a37d88d52a2e32d622, accept

Reviews: revision d10f7824f2c03cbf233f8c1f30e0cd54914b1223, accept

Reviews: revision 95aef2cdc74b22c1b9cff16bc3d803eec3064acb, accept

Reviews: revision 8221895a34b35d3d5f4dc9331e1be34778e2d10d, accept

Reviews: revision 5160c658b8dfbba1c0e37928d558e8abf71b281b, accept

Reviews: revision 947b447e1107e3fdd1899850d7719cc969826686, accept

Reviews: revision 04a7dbd2b47b9d69f95f08cede666398123adfbe, accept

Reviews: revision 3e818f3f7c692e3e7139948b21ed8c65c1b3ef4d, accept

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
- 2026-10-10 landed d83c883d3; next: All local parts are built; rebuild the Python wheel with the reviewed Polars repair, resume package smoke and run the complete installed release suite.
- 2026-10-10 landed e7ba3aadf0cdec9e5aea744c7faf4084c6a23894; next: Rebuild final packages with the reviewed native admission repair, then finish installed qualification without repeating unchanged checks.
- 2026-10-10 landed 76d0d24952ca2305940d8f587dd0e56c4096195d; next: Final release packages are assembled; reviewed Go and Flutter check repairs pass, PHP setup is retrying, and full installed parity follows.
- 2026-10-10 landed 2c60fd31b3f06d71eacd7b70fd86adfc65fd93f0; next: Routine and release boundaries pass; reviewed fixture and Apple assembly repairs are landed, and final installed parity resumes using unchanged release payloads.
- 2026-10-10 landed b5203e9b1801c74940ddd7058f75aa7d58ee226b; next: The reviewed signal repair passes held-call and internal-error checks; rebuild affected release packages and finish installed parity.
- 2026-10-10 landed 5210a46cb00246f44a024a70d31477da83143051; next: Routine checks and lint pass with reviewed repairs; rebuild the CLI for caption attachments and finish installed parity.
- 2026-10-10 next: Repair raw-question and native-error framing in the installed consumer fixture, then finish the installed table; CLI passes 244 cases and Rust passes 247 with eight fixture failures. All qualification processes stopped and awaited.
- 2026-10-10 next: Repair the authored-question fixture in lane 0, review the change, and resume remaining installed consumers using the retained passing CLI result.
- 2026-10-10 landed 995d4791f; next: The reviewed authored-question fixture passes Rust, R, JavaScript and TypeScript focused checks; resume the remaining installed package table with unchanged artifacts and retain the passing CLI result.
- 2026-10-10 landed fe259d30a; next: Both authored-input fixture repairs are reviewed and landed; combine the passing affected Rust cases with unchanged results and finish the remaining installed package consumers.
- 2026-10-10 landed 724f912d2; next: JVM now honors the full parity profile; installed C passes all 255 cases, Rust ordering repairs are being completed, and independent package families run in lanes 0 and 2.
