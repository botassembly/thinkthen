# 0619: Run every Linux check in one pinned container per architecture

Status: OPEN.

Milestone: 0.2.1

Phase: 1

Depends on: 0615

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

## Amendment, 2026-10-18

All Linux checks use the digest-pinned Ubuntu 24.04 bases, tracked recipe and shared tool manifest for each architecture, including hosted Linux jobs. Receipts record each constructed image ID. The existing manylinux_2_28 step stays only to build distribution artifacts at the old glibc floor; installed-artifact checks run in Ubuntu. The later PM answer 4 supersedes the initial Yellow reservation: Yellow books and caps the `linux.yellow_memory_gib` value with one heavy job at a time; M5 and hosted retain `linux.memory_gib`. The M5 container waits until its Docker runtime is running again; ThinkThen changes no host configuration.
