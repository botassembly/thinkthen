---
flow: build
priority: 271
opens: sdlc/issues/closed/2026-09-27-ada-consumer-proof-needs-a-supported-package.md sdlc/issues/closed/2026-09-27-objective-c-consumer-proof-needs-a-supported-package.md sdlc/issues/closed/2026-09-27-cobol-consumer-proof-needs-a-supported-package.md
---

# 0271: Prepare Ada, GNU Objective-C and COBOL for the Linux x86 release workflow

Status: in progress. Static code accepted at `72aa3260` after fresh independent High review; actual runner qualification remains open. The design was accepted at `2d83e3ef` after [fresh independent High review](../records/0271-0272-language-workflow-design-review.md). The coordinator approved the bounded design. [Preparation](../records/0271-0272-language-runner-preparation.md) and the [runner input research](../records/0271-0272-runner-toolchain-inputs.md) record the remaining tool boundaries. The [code review record](../records/0271-ada-objc-cobol-workflow-code-review.md) names the accepted scope and retained proof. No native compiler, backend, container, Actions or runner qualification is claimed. Ian’s SQL/DataFrame hold remains in force.

Milestone: 0.1

## Outcome and boundaries

The three source wrappers join the **same** x86 manylinux `release-pack` call and its one fresh C archive after 0270, with the checked Git tar/SHA and full extracted-tree check before C. `release-pack` already has exact copy arms and fixed C manifests. Extend only the landed archived-source part allowlist and copied-member comparison for `ada objective-c cobol`. Their archives must contain the 0265 curated members byte-for-byte from the selected archive. Preserve the legacy plus Go/C++/Swift/Zig/PHP/Dart parts, one C, non-reuse, empty selected C output and checkout clean-tree rules. No outer-host C, Apple Objective-C or new binding code enters this ticket.

Extend the **landed** x86 selected-family gate to require all three exact version/target `.tar.gz` names and adjacent sidecars before smoke and before draft collection. Inspect every Ada, Objective-C and COBOL prefixed entry that the copy path would see, including other suffixes, versions, targets and links. Reject these families on the other three targets. Preserve 0268's exact four nonlinked platform folders and Go ZIP refusal; do not claim a complete legacy-file allowlist. Call existing `release-go-cpp-pair DIR ada-objective-c-cobol` for exact members, sidecars, metadata and C identity. `release-smoke` already invokes that preflight and routes `libraries/{ada,objective-c,cobol}/check.sh` with `THINKTHEN_C_ARTIFACT`; retain its nine historical families. Static fixtures may check wiring while the full smoke is held.

The installed selectors currently demand `node` and Python `jsonschema` before their installed branch, while neither selected installed path runs a ratchet or imports jsonschema. Move those two prerequisite checks to source mode in each of the three `check.sh` files; keep source diagnostics and checks. Keep actual installed prerequisites: GNATMAKE/GCC and `gprbuild` for Ada, GCC with GNU Objective-C runtime and `-lobjc` for Objective-C, GnuCOBOL 4 `cobc` and C compiler for COBOL, plus Python, `cargo` for the checkout-owned conformance backend, `nm`, `ldd`, `tar`, `cmp` and `flock`. Probe resolved executables, versions and linkable GNU runtime before treating an `ubuntu-24.04` selector as qualified. The local 0265 versions are evidence, not runner setup. If exact pinned/authenticated tool sources are unavailable, report that fact before any runner execution; no mutable `latest` or tolerated exit 77.

## Smallest proof and file claim

Under the hold, use the existing archive fixture to accept the mixed legacy-plus-language part list and refuse one changed copied member in **each** newly admitted family before compilation/output. Preserve the separate whole-tree pre-C check. Use the existing selected-family workflow fixture to refuse a missing member, a wrong-suffix/target entry and an unknown copied platform folder before output or consumer start; the mock only records calls. Reuse existing pair validator plants rather than repeating sidecar/C/package inventories. Verify installed/source prerequisite branch placement statically, and register fixtures in the routine workflow checker. Run `sdlc/scripts/pages`, `sdlc/scripts/tickets` and `git diff --check`; do not start backend, container, Actions, SQL/DataFrame or aggregate smoke.

After the hold and explicit runner authorization, a separate checkpoint must run a current SHA's container C and wrappers on actual `ubuntu-24.04`, check the C header/shared/static member hashes and glibc floor, resolve tools, and invoke the three existing installed selectors. Each must retain its five returned values, mixed-answer order witness and exactly three complete literal body/count observations. COBOL retains direct row digest checks; Ada and Objective-C retain the qualified shared-C inference. A final release pin, Actions result, supported checked distribution and other hosts remain open in the three issues. GNU Objective-C does not imply Apple Objective-C.

Prospective executable claim after landed 0270 and fresh design review: `.github/workflows/release.yml`, `sdlc/scripts/{release-container,release-pack,release-archive-self-test.py,release-workflow,workflows}` only where the actual route needs changes; `libraries/{ada,objective-c,cobol}/check.sh` only for prerequisite placement; focused workflow fixtures and this ticket/build record. Coordinate these shared files with the 0269/0270 owners. No source binding, C/engine, package manifest, existing ticket, shared plan, SQL or DataFrame edit. Fresh independent High code review checks provenance and pre-collection gate before landing.

## Evidence

- Starts from: [0265 local package proof](../records/0265-ada-objc-cobol-release-build.md), [0260 host proof](../records/0260-ada-objc-cobol-host-proof.md), accepted 0268 gate and the three open release issues.
- Keeps: Accepted source/installed matrices, one C identity, public callers and literal body/count oracles.
- Changes: Adds workflow source admission, selected-family presence and source-only prerequisite placement.
- Proof: Registered static archive and gate refusals under the hold; later selected installed calls on the actual runner.
- Defers: Actual compiler/container/runner/Actions proof, final pin, distribution and other hosts.

## What the build taught us

The 0265 packer already contained the curated Ada, GNU Objective-C and COBOL copy arms. Admission of those parts from checked archived source and comparison of every staged source member needed only allowlist changes. The existing pair validator already checked the fixed member sets, adjacent sidecars, metadata, C identity and selected SHA; the new gate calls it after refusing unknown family names. The workflow calls the gate before installed consumers and before collection. The selected installed scripts did not use Node or Python jsonschema, so their prerequisites now sit after the installed return while source diagnostics remain.

The first fixture revision expected one Cargo stub call overall. Three new copied-member plants each reached the stubbed C build before changing the staged wrapper member, so that copied-count expectation was wrong. The corrected assertion compares the call count immediately before and after the whole-tree refusal. The fixture now tests both distinct boundaries: changing an extracted member fails before the C stub, and changing a staged Ada, Objective-C or COBOL member fails before its wrapper archive appears. No full source or installed matrix was repeated.

The `ubuntu-24.04` tool probe checks actual selected commands and GNU Objective-C linkage before installed selectors. It does not install or pin them. The accepted runner input record still lacks proof of GnuCOBOL 4 dependency closure and compatibility, and the available gprbuild version differs from the locally tested one without proof of incompatibility. An authorized runner checkpoint must resolve those facts, then run the existing installed selectors and retain sanitized command and nonsecret setting receipts. Static checks cannot qualify the runner or close the three release issues.
