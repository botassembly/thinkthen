# Issue priorities, 2026-09-30

Status: current. This page orders the 25 open issues in `sdlc/issues/` after the triage of `728ecd9db`. It replaces `issue-backlog-2026-09-25.md` as the order of that folder. It re-triages nothing: every issue stays open with its own text. The 13 issues that named no owner gained one `Priority:` line; the 12 that already named one are unchanged. Ian can overturn any rank or owner.

## How the ranks were set

Ian's direction for this phase: no public release before 0.1, and 0.1 waits for every surface and binding (ruling 10). The phase goals: thin, light and fast tests; one batching and caching path; Rust owns every type; remove hacks; then burn down issues. `cleanup-2026-09-30.md` "Order of work" sets their order: the one path, surfaces onto it, hacks, tests, lighter, types, then the issue list with release blockers first.

- An issue that the running 0304 slice 3a closes ranks first, because its fix is already built.
- A 0.1 blocker with a known fix ranks next, then the release itself, then phase-goal work.
- Marketing's pages, later features and deferred tuning work follow.
- "Blocks 0.1" means the release cannot ship on every channel without it.

## Ranked table

| Rank | Issue | Problem | Blocks 0.1 | Owner | Depends on | Next step | Size |
| ---: | --- | --- | --- | --- | --- | --- | --- |
| 1 | `2026-09-30-typescript-details-request-digest-differs-from-the-command.md` | The TypeScript details test is red on main: the library still sends the request bytes from before 0304 slice 2 | yes | ticket 0304 slice 3a | nothing | Land 3a; its branch passes `shapes.test.mjs` | small |
| 2 | `2026-09-30-public-batch-holds-one-send-under-a-throttle.md` | An ignored test expects throttle-many sends at batch 1 | no | ticket 0304 slice 3a | nothing | Land 3a. The cause is found below: the test's premise is wrong | small |
| 3 | `2026-09-30-postgresql-extension-does-not-build-on-macos.md` | The PostgreSQL extension uses Linux-only `openat2`, `O_PATH` and `/proc/self/fd`, so it neither builds nor reads confined files on macOS | yes | a new ticket (draft below) | M5 or a macOS runner for proof | File the ticket now | medium |
| 4 | `2026-09-25-release-and-install-for-0-1.md` | 0.1 itself: the four-runner rehearsal, the version bump, publishing, the tap, R-universe, the history reset | yes | ticket 0128 phases 3b and 4; Ian dispatches | ranks 1 and 3; 0304 slice 3 in full; rank 10 before the site goes public; rank 11's sentence joins Phase 4's one README commit. Rank 5 runs inside this rehearsal and release | Ian dispatches `rehearse` once 0304 slice 3 and rank 3 land | large |
| 5 | `2026-09-26-language-packages-need-a-release.md` | Eleven C-door packages are built and checked on one Linux host and published nowhere | yes | ticket 0128 with tickets 0268 to 0273; Ian's registry accounts | part of rank 4's rehearsal and release; Ian's one-time NuGet, Packagist, Maven Central and pub.dev setup | Rebuild at the release commit inside rank 4 | large |
| 6 | `2026-09-30-split-tests-between-the-gate-and-release-qa.md` | The gate still holds tests the release QA suite should own, and it lacks one replay smoke per binding | no | ticket 0335 slices 2 and 3 | 0304 slices 3a to 3d; the 0305 cleanup | Start slice 2 when 3d lands | medium |
| 7 | `2026-09-25-public-library-api-gaps.md` | Items 1, 2, 3 and 9 make the SQL hosts copy engine code: per-engine counters, a private relate-rule parser, no public `Error` constructor, a hand match on `LoadedQuestion` | no | ticket 0304 slice 3b for items 1 and 9; a follow-up ticket for items 2 and 3; items 6 and 7 after 0.1 | 0304 slice 3b | After 3b lands, recheck items 1 and 9 and write one ticket for what remains | medium |
| 8 | `2026-09-30-objective-c-headers-collide-on-macos.md` | `release-pack` puts the C header `thinkthen.h` beside `ThinkThen.h` in the Objective-C package | no | a new ticket (draft below) | nothing | File the ticket now | small |
| 9 | `2026-09-26-every-surface-should-give-back-run-facts.md` | Six parts remain: full detail on every host, attempt times past ticket 0302, caller prices past ticket 0300, SQL per-call facts, the `meta.usage` name, the docs | no | coordinator for item 5; tickets 0300 and 0302 for items 2 and 3; 0314 slice 4 for item 1; marketing for item 6 | 0314 slice 4, which waits for 0304 slice 3, for item 1 | The coordinator records the `meta.usage` decision now; the rest waits | large |
| 10 | `2026-09-30-reference-page-exit-codes-and-key-rule-drift.md` | The site's reference page omits exit codes 7, 130 and 143, reserves 7 wrongly and misstates the local-server key rule; three site links point at moved issues | yes | marketing | nothing | Marketing edits `site/src/pages/reference.astro` and the three links | small |
| 11 | `2026-09-29-readme-key-backend-and-overhead-lines.md` | The README states no overhead number | yes | marketing's overhead benchmark, then the queue owner | the benchmark run | Marketing runs the benchmark; the queue owner writes the sentence | small |
| 12 | `2026-09-20-new-user-stumble-register.md` | Row 18 (the key wait) and row 9 (the 255 limit) stay open | yes, both rows: the register fixes every row before the public push | ticket 0128 Phase 4 for row 18; rank 19's page 11 for row 9 | rank 4 | Pull page 11 of rank 19 ahead of 0.1; it is one short how-to | small |
| 13 | `2026-09-25-site-samples-and-pages-after-the-surfaces-land.md` | Library and database samples on the site run under no check; no status-word check; no pandas page; no stray-code-tag check | no | marketing | 0304 slice 3 for the samples' output | Marketing's call | medium |
| 14 | `2026-09-25-recognize-and-relate-scale-and-shape.md` | Items 3 to 6: the both-ways edge shape, re-billing a grown set, blocking before pairing, the 255 refusal on every host | no | a future ticket after 0304 slice 4; Ian for item 3 | 0304 slice 4 | Ian decides item 3 before 0.1 (see "For Ian") | large |
| 15 | `2026-09-26-count-secure-connections-at-sixteen-jobs.md` | Nobody has counted the secure connections a `--jobs 16` run opens | no | queue owner, as a local experiment | nothing | Run the loopback experiment below | small |
| 16 | `2026-09-30-zig-0-15-2-linker-drops-constant-alignment.md` | Zig 0.15.2's linker loses constant alignment; static mode links with LLD | no | upstream (Zig) | a Zig release that fixes it | Remove the two lines when Zig fixes it | small |
| 17 | `2026-09-29-docs-page-naming-supported-providers.md` | No site page lists the supported providers | no | marketing | ticket 0334 for the key names | Marketing writes the page after 0334 lands | small |
| 18 | `2026-09-29-docs-page-for-the-liquid-d1-backend.md` | No site page walks a user from Liquid's console to a first answer | no | marketing | rank 17; ticket 0334 | Marketing writes it after rank 17 | small |
| 19 | `2026-09-25-docs-how-tos-and-spec-claims-owed.md` | Thirteen pages owed; the issue lets them follow 0.1 | no, except page 11, which closes rank 12's row 9 | a future docs ticket; marketing for page 23 | page 18 needs a measured profile; page 17 needs its own ticket | Nothing before 0.1 | large |
| 20 | `2026-09-23-annotate-options-from-a-file-or-a-record.md` | An annotate question set cannot take `choose` options from a file or a record | no | a future ticket after 0.1 | 0304 slice 4 (relate's planner shares the options source) | Nothing now | medium |
| 21 | `2026-09-26-relation-pairs-span-every-mention-and-the-whole-text.md` | `recognize --relation` pairs names across the whole text | no | Ian rules on the default, then a future ticket | Ian's ruling on ADR 0056; 0304 slice 4 | Nothing now | medium |
| 22 | `2026-09-27-a-run-cannot-be-repeated-on-purpose.md` | No `--repeat N` to measure rerun noise | no | a future ticket after 0.1 | nothing | Deferred by the 2026-09-28 tuning review | medium |
| 23 | `2026-09-27-nothing-lists-the-uncertain-hard-or-flip-flopping-cases.md` | No command ranks cases for a person to label | no | a future ticket after 0.1 | nothing | Deferred by the 2026-09-28 tuning review | medium |
| 24 | `2026-09-27-no-run-level-cost-beside-the-score.md` | `audit` prints no cost beside its score | no | a future ticket after 0.1 | rank 9's facts shape | Deferred by the 2026-09-28 tuning review | small |
| 25 | `2026-09-24-rank-by-graded-relevance-for-search-reranking.md` | Asks 2 and 3: per-level weights and a reciprocal rank fusion transform | no | none until a user asks | nothing | Nothing (see "Serves no phase goal") | small |

Blockers in the top ten: ranks 1, 3, 4, 5 and 10. Ranks 11 and 12 also block 0.1. Rank 11 waits on marketing's benchmark; rank 12 waits on the release run and page 11 of rank 19.

The release issue's item 7 lists the Objective-C collision as a macOS release defect. This page ranks it as no blocker, for the reason given under rank 8. The issue text stays as filed; the ticket that fixes it should correct item 7 when it lands.

## Investigations

Each was done by reading code and records only: no build, no network, no paid call, no M5.

### Rank 2: the public batch keeps one send at batch 1 by design

The cause is the interactive rule, not the scheduler. `crates/thinkthen/src/public/batch/planned.rs` sets `interactive` when the setting is `Records(1)`. Its `take` then defers every scheduler ask while a fed record is unanswered (`Event::Ask if self.interactive && self.fed > self.completed`). Commit `503bd9890` of 2026-09-28 added this rule under Ian's approved ADR 0053 amendment. `specification/records.md` and `libraries/rust/README.md` say an interactive caller picks `BatchSetting::Records(1)` and gets each row before the library pulls the next input. That rule allows only one send in flight. The issue's test uses batch 1, so it tests the one setting where the promise cannot hold.

This also explains the dates: record 0305 saw the 5 s waits on 2026-09-29, one day after the rule landed. The 3a branch (`origin/ticket/0304-s3a-engine-ask-all`) reached the same cause independently. It keeps the rule in `public/pull.rs`, runs the test at batch 2 so it fills throttle 2, and moves the issue to `closed/` as a wrong premise. No experiment is needed.

### Rank 3: the PostgreSQL extension on macOS

The cause has two parts, and the issue names only the first.

1. Compile errors. `databases/postgresql/src/ffi.rs` calls `openat2` through `libc::syscall(libc::SYS_openat2, ...)` with `libc::open_how` and `RESOLVE_BENEATH | RESOLVE_NO_MAGICLINKS`, and opens with `O_PATH` twice. None of these exists for Apple targets in libc 0.2.189. The issue also names `AT_FDCWD`, but libc defines it for Apple (`src/unix/bsd/apple/mod.rs` line 2458), so that one name is not a cause. Nothing in the extension carries a `cfg(target_os)` gate.
2. A runtime refusal the compile errors hide. `ffi::path_of` reads `/proc/self/fd/N`, and macOS has no `/proc`. `files.rs::read_within` refuses a confined read when `path_of` returns `None`. A build that only gated `openat2` would still refuse every `@file` read under `thinkthen.file_directory`.

The fix has a clear shape, and libc 0.2.189 already carries its constants. On macOS, open the base with `O_RDONLY | O_DIRECTORY`. Open the file with `openat` and `O_NOFOLLOW_ANY` (0x20000000, macOS 11 and later), which refuses a symlink at any step. Read the descriptor's path with `fcntl(fd, F_GETPATH, buf)`. `files.rs::beneath` already refuses any `..` by spelling, and the one-link and inside-the-base checks still run on the descriptor afterwards. Linux keeps `openat2`. The release plan targets macOS 15 (`MACOSX_DEPLOYMENT_TARGET=15.0` in `sdlc/scripts/release-workflow`), so `O_NOFOLLOW_ANY` is always present.

Proof needs macOS: `cargo pgrx package` and the extension's check on the M5, or the macOS jobs of Ian's `rehearse` dispatch. The ticket and the rehearsal must not wait on each other. Recommended route: one light, one-time M5 check before landing, which the workspace README allows for ThinkThen; the rehearsal then confirms it. If the M5 is not free, land after Linux review and let the rehearsal be the proof of record. This host has no Apple Rust target installed, and pgrx needs macOS PostgreSQL headers, so a Linux cross-check cannot prove it. Experiment 218's M5 run also noted that Homebrew held PostgreSQL 16.14 while `databases/postgresql/runtime-darwin.env` pins 16.15; the macOS proof should check that pin too.

Without the fix the macOS release fails. `release.yml` packs the default parts on both macOS runners, and `release-pack`'s default list includes `postgresql`.

### Rank 8: the Objective-C header collision

The cause is one line. `sdlc/scripts/release-pack` copies `libraries/c/include/thinkthen.h` into the package's `Sources/`, beside `ThinkThen.h` (the `objective-c)` case). Three other places expect that copy: `libraries/objective-c/check.sh` copies it in the checkout (line 54) and compares the packaged copy with the C archive's header (line 36); `sdlc/scripts/release-go-cpp-pair` lists `./Sources/thinkthen.h` among the archive's members and compares it with the C header; and `release-pack`'s archived-source check maps `Sources/thinkthen.h` to the C header.

The issue overstates its reach. It says the package targets macOS. The package README and experiment 218's M5 run say it targets the GNU Objective-C runtime on Linux; Apple clang gave 20 errors on it. `release-workflow`'s `ada_objc_cobol_names` refuses any Objective-C archive on a macOS target. So no release runner meets the collision. Only a Mac user who unpacks the Linux archive on a default volume does. That makes it a small defect, not a 0.1 blocker.

The fix needs no header copy at all. The README already tells a consumer to compile with `-I native/include` from the separate C archive, which supplies `thinkthen.h`. `release-pack` stops copying the header and drops its archived-source mapping for the Objective-C copy. `release-go-cpp-pair` stops expecting and comparing it. `check.sh` copies it into a build folder outside `Sources/` and passes that folder with `-I`. A small check in `release-pack` refuses two members of one archive whose names differ only in case. The same change should fix the README's link to `sdlc/issues/2026-09-27-one-type-contract-for-every-surface.md`, which now lives in `closed/`. Linux proves all of it: pack the archive and list its members. No M5 is needed.

### Rank 15: counting secure connections at sixteen jobs

The issue assumes the count needs the hosted service and a paid run. A loopback count can answer the main question with no network. `THINKTHEN_CA_BUNDLE` (ticket 0211, `specification/backends.md`) replaces the trust roots with a local PEM file, and `crates/thinkthen/tests/ca_bundle.rs` already runs a genuine localhost TLS server with an `openssl`-made certificate authority.

The one experiment: start a keep-alive TLS server on `localhost` that counts accepted connections and answers each System One request after a short hold. Run `filter --jobs 16 --batch 1 --no-cache` over 306 lines with the certificate authority in `THINKTHEN_CA_BUNDLE`, and count the accepts. The client keeps up to 32 idle connections (`Width::MOST` in `engine/mod.rs`, used by `engine/http.rs`), so it should open at most 16. A count near 16 settles ticket 0142's deferred gap for the client. Only the hosted server's own keep-alive policy then remains, and the issue's CONNECT-proxy run can measure that later under `sdlc/scripts/live`. Paid calls are already authorized by ruling 13, under a token cap. Neither run needs the M5.

### Top-ten issues with a known cause and next step

Ranks 1, 4, 5, 6, 9 and 10 already name their cause and next step in their own files. Rank 7's items 1 and 9 wait for 0304 slice 3b, which touches their code. Items 2 and 3 need no investigation: the issue names the copied code and the public function that would replace it.

## Machines

Only rank 3 needs macOS. Its proof needs the M5 or the macOS runners of the release rehearsal. The workspace README lets ThinkThen run only light, one-time checks on the M5, and this investigation did not use it. Ranks 2, 8 and 15 are proved on this Linux host.

## Ready as tickets now

Lane 2 is editing the engine, the public API, the bindings and ticket 0304 (`origin/ticket/0304-s3a-engine-ask-all` touches 212 files). Lane 1 is preparing slices 3b to 3d, which move the SQL hosts. These three do not touch the files the 3a branch changes:

1. **The PostgreSQL extension builds on macOS.** Outcome: `cargo pgrx package` and the PostgreSQL check pass on macOS, and a confined `@file` read works there, through `openat` with `O_NOFOLLOW_ANY` and `F_GETPATH` behind a macOS gate, while Linux keeps `openat2`. Touches `databases/postgresql/src/ffi.rs` and `files.rs` only; 3a changes neither. Slice 3b will edit other PostgreSQL files. Keep this change inside the file-open functions and rebase over 3b if 3b lands first. Proof: one light M5 check, as the rank 3 investigation recommends.
2. **The Objective-C package holds no case-only name pair.** Outcome: the Objective-C archive ships without `thinkthen.h`, the consumer takes the header from the C archive as its README says, and `release-pack` refuses any archive with two names that differ only in case. Touches `sdlc/scripts/release-pack`, `sdlc/scripts/release-go-cpp-pair`, `libraries/objective-c/check.sh` and `libraries/objective-c/README.md`, and corrects item 7 of the release issue; 3a touches only `libraries/objective-c/checks/`. Proof on Linux: pack the archive, run the Ada, Objective-C and COBOL pair check, and run the Objective-C check.
3. **The meta.usage name is settled.** Outcome: one record states whether per-record token shares keep the name `meta.usage`, and `remaining-batches-2026-09-28.md` stops listing it as open. This is a coordinator record, not a build ticket. Recommendation: keep `meta.usage`, as `work-plan-2026-09-27.md` already decided, because renaming a field the schema generator now pins costs every binding a change and ruling 8 asks for less pedantry. The result spec already says the share is even.

The count of secure connections (rank 15) is ready as a local experiment outside the repository, with no ticket. It becomes a ticket only if the loopback count exceeds 16.

Nothing else is ready. Ranks 1 and 2 close with 3a. Rank 7's items 2 and 3 change the public API, which 3a is changing, and PostgreSQL's `call.rs`, which 3b will change; they wait for 3b. Rank 14's item 6 and rank 20 touch the conformance cases and the relate planner, which slices 3 and 4 rewrite.

## Serves no phase goal

Nothing here is closed; each stays open as filed.

- Rank 25, graded relevance asks 2 and 3. The issue itself says to add per-level weights only if a user needs another scale, and no user has asked. The fusion transform needs no model. It serves neither a phase goal nor a named ideal-state gap.
- Ranks 22, 23 and 24, the three tuning issues. They serve the ideal state's tuning loop, where ThinkThen supplies rows for Optimizer. They serve no phase goal and stay deferred past 0.1 as the 2026-09-28 tuning review ruled.
- Rank 16, the Zig linker issue. It records an upstream bug and the condition for removing its workaround. It needs no work from this phase.

## For Ian

1. Rank 14, item 3: whether a both-ways relate edge gets an unordered `pair` shape. A shape change on every surface is cheap before 0.1 and a breaking change after it. You asked on 2026-09-22 that relate's source and target be more obvious. Options: (a) change the shape before 0.1, about one medium ticket after 0304 slice 4; (b) keep the one-way shape and document `--either` edges. Recommendation: (a), because a breaking change after 0.1 costs every consumer.
2. Rank 4's rehearsal: dispatch `rehearse` only after 0304 slice 3 and the PostgreSQL macOS ticket land. An earlier dispatch fails on the PostgreSQL part. The queue owner proves that ticket with one light M5 check first, so the two do not wait on each other.
3. Rank 21: whether relation pairs default to one sentence or neighbouring sentences, which changes ADR 0056's whole-text rule. The same argument applies: a default that changes answers is cheaper to set before 0.1. Options: (a) default to neighbouring sentences, which kept 359 of 359 relations in experiment 275 at about half the questions; (b) keep the whole text and add the limit as an opt-in. Recommendation: (b) for 0.1, because natural text with a relation three sentences apart was never measured, and an opt-in changes no current answer.
4. Release issue item 3: whether Homebrew on Linux is a promised channel. Options: (a) promise it and run `brew install` on Linux in the post-publish check; (b) call Homebrew a Mac option, as the site does, and keep the curl script for Linux. Recommendation: (b), because the script already covers Linux and (a) adds a check with no Linux runner behind it.
