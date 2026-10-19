# 0619: Run every Linux check in one pinned container per architecture

Status: OPEN.

Milestone: 0.2.1

Phase: 1

Depends on: 0615

Reviews: revision f10d64c, accept

Reviews: revision 75ba26c, accept

Reviews: revision 1d0aeaa, accept

Reviews: revision e9914f2, accept

Reviews: revision 8fb6a48, accept

Reviews: revision 57a481f, accept

Reviews: revision cfad2cd, accept

Reviews: revision 21eb8a4, accept

Reviews: revision 3e00269, accept

## Outcome

- Each architecture builds its Linux check environment from the digest-pinned Ubuntu 24.04 base in `linux.images` in `sdlc/tools.json`, the tracked image recipe and the shared tool manifest. That environment holds every declared Linux tool and serves building and testing. Receipts record the actual constructed image ID on each machine; matching base digests and recipes do not claim a shared tools-image digest. The manifest owns `linux.distribution_images` for the separate `manylinux_2_28` distribution build at the old glibc floor; installed-artifact checks run in Ubuntu. No image publication or source-built compiler is authorized.
- One entrypoint runs `lint`, `test`, `test-full-cases`, `package` and `test-stress` in that environment. The manifest owns the CPU cap in `linux.cpus`, the M5 and hosted memory cap in `linux.memory_gib`, and the smaller-host Yellow exception in `linux.yellow_memory_gib`. Containers start with `docker run --rm --label pm.repo=thinkthen --label pm.ticket=NNNN`; pm jobs reserve the matching memory value with `--memory`. Yellow runs one heavy job at a time.
- Linux arm64 runs in the container on the M5 through pm job. Linux x86_64 runs in the container on Yellow through pm job. Everyday Linux proofs use the M5 container. Proofs specific to x86_64 use the Yellow container.
- The hosted Linux jobs build the same tracked recipe and tool manifest on the corresponding digest-pinned Ubuntu base and record their actual constructed image IDs. They install no Linux tools outside that environment. Hosted routing and deletion of hosted install steps are 0620's work.
- The pm.json tickets gate runs `lint` and `test` in the M5 container, plus a native macOS `test` for Apple-only cases.
- The routine nextest time limit is set from timings measured in the capped container, with the reason written beside the setting.
- `machines.json` records Yellow's container role for ThinkThen. The PM requests that change; this ticket does not edit `machines.json`.
- The repo instructions and `AGENTS.md` say that everyday builds and tests run on the M5 through pm job, that Linux runs in the pinned container on the M5 and on Yellow, and that hosted workflows run by hand only for releases. The "Linux CI runs on each push" claim is deleted.

### Proof

- On the M5 through pm job with `--memory` matching `linux.memory_gib`, the tickets gate passes: `lint` and `test` in the arm64 container and the native macOS `test`.
- On the M5 through pm job, the capped arm64 container runs `test-full-cases`, `package` and `test-stress`.
- On Yellow through pm job with `--memory` matching `linux.yellow_memory_gib`, the capped x86_64 container reproduces the failing cases of hosted run 39102240049 on main before 0621's fix, or records that they pass.
- Image preparation verifies the declared Linux tools and records the actual constructed image ID and pinned base for each architecture; receipts identify the tracked recipe and manifest revision. Resource receipts show the manifest CPU and memory caps on each machine.
- A grep finds no `push:` or `pull_request:` trigger in `.github/workflows/`, and no instruction claims one.

## Evidence

The PM answers in `2026-10-18-thinkthen-0619-container-prerequisites-need-a-ruling.md` supply the existing authority: answer 2 approves Ubuntu 24.04 checks with manylinux only for distribution artifacts and forbids source-built compilers; answer 3 accepts digest-pinned bases plus the tracked recipe and manifest with actual per-machine image IDs, without publication or a shared tools-image digest claim; answer 4 approves the Yellow-only memory exception while preserving M5 and hosted limits. Commit `52f852a15` implements that exception in `linux.yellow_memory_gib`; review `3ee1ea3cf` accepts that revision. This reconciliation applies those answers without changing the remaining proof obligations.

Qualification has moved between machines: the Linux mini PC (load cap only there, `535653f6c`), the M5 (`c53055095`, `cb3473396`, both 2026-10-16) and the hosted runners. Each move brought its own run of fixes, such as the Mac path aliases in `fd8744a30` and the Mac shebang delay in `42cbde2e2`. Linux is now exercised only on hosted runners, so Linux-only failures appear late and cannot be reproduced locally. Release record 0543 (its final entry, 2026-10-18) says the C image groups pass on the M5 in 16.10 seconds at about 4.06 GB peak, and the Linux timeouts remain unexplained.

`sdlc/scripts/release-container:16-18` already pins a `manylinux_2_28` image by digest for each Linux architecture, but only release packing uses it. The M5's default pm job memory is 8 GiB, below the hosted runner's 16 GB.

The repo instructions are also wrong about Linux. `CLAUDE.md:12` and the matching line in `AGENTS.md` say "Linux CI runs on each push". Every workflow in `.github/workflows/` starts only by `workflow_dispatch` or `workflow_call`, which matches the ruling that GitHub Actions run only for public releases after local testing.

## Progress

- 2026-10-18 started
- 2026-10-19 next: Clean extraction dad2b4ac5 has source review with execution proof pending; existing PM answers2-4 in the0619-container-prerequisites mail authorize Ubuntu checks, distribution-only manylinux, recipe and manifest plus actual image IDs, and Yellow13.5GiB while M5 and hosted retain their manifest limits, so reconcile the older ticket wording before container qualification.
- 2026-10-19 next: Before core freeze0688 the container checkpoint must run the deferred DuckDB and pinned PostgreSQL installed routine checks from0643 and0627, per PM mail on0627, in addition to0619 own image and capped-suite proof; native phase1 landings remain independent.
- 2026-10-19 next: Updated-manifest image preparation at f10d64cf6 stopped before probes on Ubuntu snapshot HTTP502 in job1792415676742-53168; one unchanged infrastructure retry is authorized while native landings continue independently.
- 2026-10-19 next: The single unchanged preparation retry job1792415714835-65984 also stopped before probes on snapshot HTTP503; retain both failed receipts, diagnose the immutable certificate-bootstrap input without edits or further retries, and keep native landings independent.
- 2026-10-19 next: Read-only diagnosis confirms both preparation failures are remote Docker ADD of the unchanged SHA256-pinned certificate package before verification; no existing offline input route exists, so hold further preparation until endpoint recovery or a scoped recipe ruling, keep both receipts and old image, and continue native phase1 landings.
- 2026-10-19 next: Reviewed75ba26cfd keeps the exact certificate checksum while using the official archive; M5prepare job1792416297174-36106 passed with imagef090f2aa90834abd29c4c13cba83df9546e63b1b975eccd5845c44941a566756, both declared-tool checks and unchanged caps, and the first bounded Linuxlint proof now runs while native phase1 landings continue.
- 2026-10-19 next: Container routine proof failed at a formula fixture that hid declared Ruby; e18ca03fc restores the shared test environment and declares Linuxobjcopy, but reprepare stopped on a Swift key hash mismatch, diagnostics then retrieved the exact pinned identity bytes, and one unchanged recovery attempt with the remaining Linuxfixture tail is running with all failed receipts retained.
- 2026-10-19 next: Fresh review accepts8fb6a4843 after pidfd-safe libc fallback and owned-home repairs passed PostgreSQL14 and conformance58 plus installer, registry, hosted, children and policy checks, with all failed receipts retained; tools.py changes the derived image tag to eaff8aa4e10dcb70a62f, so prepare and qualify that image after native integration before remaining capped suites and x86 proof.
- 2026-10-19 next: Image preparation job1792422700610-64942 passed with actual image6187179e9fec18f0bf4b2a02d27e9b8209b128caf1d1772c8b5cc51e6da777ed, unchanged pins and declared-tool probes; fresh review accepts integrated57a481fa7 with all seven image inputs identical, while final-image suites, native Mac proof, resource measurements and x86 plus database checkpoint checks remain.
- 2026-10-19 next: Fresh review accepts integratedcfad2cd71 with all seven image inputs unchanged; Yellow x86 preparation refused before starting with exit125 because the required16GiB booking exceeds its15GiB admission limit, and PM has the resource ruling while the accepted629-635 pair closes its gate before final-image suites.
- 2026-10-19 next: PM clarified that Yellow bookings and container caps are both13.5GiB under the prior ruling, while16GiB applies to M5 and hosted jobs; the refused16GiB prepare started nothing and the unchanged Yellow prepare now retries with the authorized13.5GiB booking before final integrated suites.
- 2026-10-19 next: Fresh component review accepts543fdcad9 after reviewer-fixed atomic download publication and unchanged-pin transfer handling passed M5 job1792430297368-81535, including a ten-assertion old-helper counterexample and all tools/workflows/installer checks; whole-ticket approval still requires both rebuilt images and the complete Linux/native, resource, x86 and database proofs, and image preparation follows the active0700/C source checks.
- 2026-10-19 next: M5 image preparation1792432529962-67410 passed with fingerprint9b3c79c71eb231159f3b and image5a66e2411bcc under16GiB/fourCPUs; Yellow preparation1792433682407-172346 failed before final image at GitHub403 while resolving pinned cargo-binstall1.15.5, so the integrated main revision next runs the configured M5 Linux/native gate while the x86 preparation waits for the reported API reset.
- 2026-10-19 next: Fresh component review accepts14a2ea67b after targeted job1792434662170-35407 passed all six Linux packaging fixture groups and native registry42/42 with every refusal assertion preserved; configured gate1792434010284-45859 retains its earlier Swift-fixture failure, and the next free slot reruns the full Linux/native gate before full-cases, package, stress, x86 and database checkpoint proofs.
- 2026-10-19 next: Configured gate1792435252854-29497 on11f09f03e passed Linux lint,2052 core cases,21 consumer cases and the registry repairs before the Python container fixture failed because its isolated PATH omitted sh;0701 now owns the repeated fixture-runtime class and the undeclared installed-C pkg-config dependency, so complete gates wait for that reviewed repair and rebuilt images while unrelated lanes continue.
- 2026-10-19 next: M5 image preparation passed with declared tools at fingerprint144a79e08586978ccaff and the independent Linux fixture batch passed18 of20 checks, while the stale Python bootstrap boundary is fixed in11029bd29 and the remaining coverage linker failure awaits a bounded diagnostic after the active Yellow image preparation.
- 2026-10-19 next: Fresh component review accepted44c23ea3a and its repairs are merged at153cf822f; Yellow job1792449735916-256181 passed the archive fixture and1792449579815-247892 passed all17 coverage cases, completing the two failures from the earlier18-of20 fixture batch while whole container gates and installed proofs remain open.
- 2026-10-19 next: Fresh component review accepted21eb8a401 after M5 job1792452113122-60314 passed explicit Docker selection; merged at86d7eccda, with whole container gates, installed checks and image refresh pending the next data integration.

## Amendment, 2026-10-18

All Linux checks use the digest-pinned Ubuntu 24.04 bases, tracked recipe and shared tool manifest for each architecture, including hosted Linux jobs. Receipts record each constructed image ID. The existing manylinux_2_28 step stays only to build distribution artifacts at the old glibc floor; installed-artifact checks run in Ubuntu. The later PM answer 4 supersedes the initial Yellow reservation: Yellow books and caps the `linux.yellow_memory_gib` value with one heavy job at a time; M5 and hosted retain `linux.memory_gib`. The M5 container waits until its Docker runtime is running again; ThinkThen changes no host configuration.
