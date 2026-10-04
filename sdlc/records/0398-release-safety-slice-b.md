# 0398 slice B: Published install checks

Date: 2026-10-04
Status: offline slice B implementation received fresh ACCEPT at fc4d842b0e627a9e4f96088f4bcff32e7a9e6246; hosted runner proof remains pending
Ticket: `sdlc/tickets/0398-release-safety.md`
Starting revision: `58aee391b351297568aa68ca4ac8f0fdb19fe414`

## Review and implementation

The fresh slice B ticket review accepted amendment `58aee391b351297568aa68ca4ac8f0fdb19fe414`. Its prior findings required historical Homebrew selection, distinct older and superseded R-universe indexes, and qualified SQLite release-asset identity. The implementation retains those boundaries.

The independent workflow retains eighteen channels over forty-seven Unix runner combinations. Each channel installs its requested public package or release archive, checks installed metadata or version, and asks the recorded question with strict replay. The consumer gets a new scratch home and no provider key or user configuration. The result check requires Boolean true and integer zero requests. Scratch cleanup belongs to the current check alone.

Homebrew reads the full public tap history and installs the selected historical formula through an owned local tap with automatic updates disabled. R-universe checks the resolute binary index before installation. An older or absent index reports the retry sentence. A newer version names the requested and listed versions and reports that the registry no longer offers the requested version. Source-only availability and archives without Linux R 4.6 binary metadata refuse without installing Rust or compiling source. SQLite checks the exact named archive and checksum, loads its exact extracted extension, and labels its proof `release-asset identity`.

## Focused offline proof

The independent edge table passes seventeen test methods. It covers all eighteen channels' answer and version contract, wrong versions, placeholders, false, null, missing and nonzero requests, invalid version rejection before installer imports, delayed indexes, superseded R versions, source-only R refusal, current and historical Homebrew selection, absent historical formulas, archive checksum and path escape refusals, exact SQLite load routing, clean environments and all forty-seven workflow cells. The fixture installers run no external package command.

The real workflow validator, ticket check, catalog check and child-environment check pass. C, Node, Ruby and PHP consumer syntax checks pass. The Rust consumer compiles against the local public API through offline Cargo in a lane lock and a user systemd scope with 6 GiB high and 8 GiB maximum memory. The same local Rust consumer replays the repository's recorded first-run question and returns `{"value":true,"requests_sent":0}` with a clean scratch home and offline Cargo. These checks do not establish public registry or hosted runner acceptance.

## Verified tool sources

The existing pins supply Rust 1.95.0, Python 3.13, uv 0.9.17, Node 22.22.3, Ruby 3.4.11, Go 1.27.1, Dart 3.13.4 and DuckDB 1.5.5. Read-only official Git refs verified the setup-dotnet v5 and setup-java v5 commit pins. .NET 8, Java 21 and Maven 3 retain the ticket's major version bounds.

The [official PGDG noble package index](https://apt.postgresql.org/pub/repos/apt/dists/noble-pgdg/main/binary-amd64/Packages.gz) supplied PostgreSQL 16.15 server and client package names and SHA-256 values. The runtime unpacks those packages into the check's own tree and starts a private socket-only cluster. The [official SQLite download page](https://www.sqlite.org/download.html) supplied the Linux x64 3.53.4 tool archive and SHA3-256. The ticket permits SQLite 3.50.4 or newer; the checked runtime pins 3.53.4. Preparation read metadata only and did not download or install these runtime packages. R uses the previously verified R 4.6.1 amd64 image digest recorded in the accepted design.

## What the build taught us

The CLI refuses a cache option beside explicit replay, so the consumer supplies replay alone. Python facts use a read-only mapping, Ruby facts use symbol keys, and Rust facts use accessor methods. The JVM archive has no implementation-version manifest; its actual resolved Maven POM supplies the installed package version. R installs the exact downloaded binary archive through R CMD INSTALL after checking its binary metadata. It never asks install.packages to select a source package.

## Pending work and authority

Shared edits waited for Windows slice A. The coordinator cleared integration after `115bb95b20111f3f73688157584ff2f4736171ff` landed. The branch rebased cleanly onto that main commit. The publish dispatch, dedicated workflow guard plants and gate registration are now implemented. The eighteen-job release graph and the five Windows/Unix build and smoke targets remain unchanged.

No workflow dispatch, public package installation, image pull, provider call, credential read or publication ran. Hosted runner proof and the eventual real install-check dispatch remain pending Ian's approval. No full checkpoint was repeated. The coordinator recorded the main baseline's complete pass separately.

## First fresh code review corrections

The fresh read-only review at `c98e483a5efd55f5345b6660121a66aebfd5e745` found three substantive failures. The initial archive filter refused the legitimate C soname symlink and blocked six consumers. The Homebrew URL named the wrong repository. The selected scratch formula had no Git commit, so a Homebrew clone could not receive it. Mocks passed without exercising either extraction of the shipped C layout or a real clone.

The extractor now accepts internal file symlinks and direct regular-file hard links while refusing missing targets, cycles, absolute or escaping targets, directory aliases, links in member parents and hard links to symlinks. The offline table retains the shipped C layout and hostile link cases. Extraction also passes against the unchanged checkpoint archive `builds/thinkthen/checkpoint/surfaces/2026-10-04-1/thinkthen-c-0.2.0-x86_64-unknown-linux-gnu.tar.gz`, SHA-256 `bde6bb8f590063a09bcf0ddf0d4c6da7a450f595ed026d7c45363947b428ab7f`. Its actual `lib/libthinkthen.so.0 -> libthinkthen.so` link resolves inside owned extraction scratch. No checkpoint file was changed.

Homebrew now clones `https://github.com/botassembly/homebrew-thinkthen.git`, the publisher and ticket's authoritative repository. The offline routing test pins that exact URL. The installer commits the selected formula into the scratch repository with an explicit local identity and signing disabled. A real local Git clone verifies current and historical formula bytes and a clean cloned tree. No tap is published and no Homebrew installation runs in the proof. The expanded table passed thirteen methods, and the child-environment and diff checks pass. At that review fix, shared release integration still waited for the Windows handoff.

## Homebrew ownership and cleanup correction

The next fresh review found that plain untap refuses an installed formula and could replace a successful consumer result with a cleanup failure. The installer now refuses any preexisting check tap or installed formula named thinkthen before changing Homebrew. After its install attempt, it lists full formula names and uninstalls only `installcheck/selected/thinkthen` if that exact formula exists. It then untaps the tap it created. It removes no other formula, tap or dependency and uses no force option.

A failed install that leaves the exact formula behind still gets that cleanup. A failure before formula creation skips uninstall. Replay, installed-version and install failures remain the primary failure if cleanup also fails; a separate diagnostic reports the cleanup failure. Cleanup failure after a successful replay refuses success with `Homebrew cleanup failed: <failure>`.

The offline fixture now models Homebrew's refusal to untap an installed formula. Sixteen test methods pass. The new cases cover successful ordered uninstall and untap, preserved unrelated state, preexisting formula and tap refusals, partial install failure, replay failure, wrong installed version, combined primary and cleanup failures, and cleanup failure after success. Existing real local formula-clone and shipped C archive-layout checks still pass. The child-environment, ticket, Python compilation and diff checks pass. No Homebrew or registry installation ran. At that review fix, shared release integration still waited for Windows slice A.

## Shared integration and final offline evidence

The independent parts received fresh ACCEPT at `8d9fce024f593cc520acb8302ba9375fc12563e9` before rebasing. The integrated candidate adds only `actions: write` and the final dispatch step to the landed Windows release workflow. A parsed comparison against `115bb95b20111f3f73688157584ff2f4736171ff` proves every prior release field remains unchanged after those two additions are removed. Both build and smoke retain five targets, including `windows-2025`; the existing four Unix binding families remain guarded.

The dispatch follows the exact successful publish operation, uses the resolved release name and version through environment variables, and runs once as the last unconditional step. Only publish may hold `actions: write` or dispatch the install-check workflow. The new workflow guard requires its dispatch version, exact read-only permissions, no environment or secret, its forty-seven Unix cells, pinned R image and final unconditional replay steps. It refuses ignored failures, skipped checks and altered shells. The consumer and workflow suites run through `workflows --self-test` in lint. The script catalog, workflow contract table, release process and qualified R issue now describe that integration.

Focused checks pass: thirty-eight planted workflow cases; seventeen consumer test methods, including six real fake-command process outcomes with exact exit codes and receipts; aggregate `workflows --self-test` 95/95, retaining all ninety-three prior cases; the real workflow validator; tickets; catalog; child-environment boundaries; Python compilation and diff checks. The C consumer also compiles against the actual unchanged checkpoint C archive, validates its version 0.2.0 and replays the actual checksummed first-run sample with Boolean true and integer zero requests. That proof uses owned scratch and no network.

Required lint passes in user scope `thinkthen-0398b-lint` with 12 GiB maximum memory, 1 GiB maximum swap, the lane's heavy lock and two Cargo jobs. Its first attempt stopped because the owned offline Cargo wrapper also blocked lint's fresh local `file://` source-refusal fixture. The wrapper correction permits only that verified sibling file dependency with no dependencies; every other Cargo command stays offline. The rerun passes policy, packaging and workflow checks, deny, formatting, Clippy, docs and the 569-item public API inventory with four refusal plants. The ignored output is `target/0398b-lint/output.log`. No full test, specification or surface sweep ran.

Fresh full slice B code review remains pending. The ticket remains in progress for slice C and live proof. No workflow dispatch, public registry installation, provider call, publication or approval operation ran. Hosted runner proof, native Windows release proof and the release rehearsal still need Ian's approval.

## PostgreSQL startup and cleanup correction

Fresh full slice B review of `dda236c215370574dce6c421200e15cb4ac752d1` found that a startup timeout after spawning PostgreSQL bypassed shutdown. It also found that a cleanup failure could mask the original query failure. The startup attempt now belongs inside the cleanup guard. Version and replay checks finish inside that guard so their failures remain primary. A separate diagnostic reports cleanup failure; cleanup failure after a successful query refuses success.

Shutdown validates the exact private data and socket paths, their ownership and permissions, the private PID file, its cluster and socket fields, the live process's owned executable and its data/socket arguments. A Linux process handle binds the fast-shutdown signal to that verified postmaster and avoids numeric PID reuse. It never stops a process by name. An exited process needs no signal. If ownership or shutdown cannot be verified, the command retains only its own scratch directory and reports its path instead of deleting a possibly live cluster.

Seven offline methods use real local child processes to cover startup spawn followed by timeout, successful query and shutdown, combined startup/query/version and cleanup failures, cleanup failure after success, mismatched paths/runtime/arguments or missing PID, exited PID handling and scratch retention. The fixture stops only the child it created before removing its files. The original seventeen consumer methods and thirty-eight workflow plants still pass. Workflow registration retains the prior ninety-five cases and adds this suite as the ninety-sixth aggregate case.

An actual offline native PostgreSQL 16.15 proof loads the unchanged checksummed checkpoint extension, reports thinkthen 0.2.0, replays true with zero requests and verifies postmaster shutdown. It uses the already cached Ubuntu runtime and local PostgreSQL client in owned scratch under `thinkthen-0398b-pg-proof`, with 12 GiB maximum memory, 1 GiB swap and the lane lock. This proves local lifecycle behavior, not the pinned PGDG download or hosted runner. An initial local proof lacked the cached client; its query failed and the corrected shutdown still stopped the actual private cluster. No external install or download ran.

The native receipt reads `actual offline PostgreSQL 16.15 lifecycle: thinkthen 0.2.0, true, requests_sent 0; verified fast shutdown`. The proof asserts that the owned `postmaster.pid` is gone after shutdown. Deliberate retention cases leave their marker and cluster files intact until the fixture verifies and stops its own child, then explicitly clean only their newly made directories. No retained scratch or private postmaster remains from these checks.

The corrected candidate's required lint exits zero in `thinkthen-0398b-pg-lint`, with the same 12 GiB memory, 1 GiB swap, lane lock and two-job offline Cargo wrapper. Its receipt is `target/0398b-lint/pg-fix-output.log`. It passes the 96/96 aggregate, real workflow guard, policy, tickets, child-environment boundaries, registry checks, deny, formatting, Clippy, docs and the 569-item inventory with four refusal plants. The first corrected-candidate attempt stopped at an existing Swift fixture because the explicitly clean PATH omitted the local Swift binary; restoring the known tool paths fixed the harness setup. No product workaround or external tool installation was needed. The scope is inactive and no tool process remains. Full suite and hosted proof remain under the coordinator's checkpoint and Ian's authority.

## PostgreSQL fixture readiness correction

The fresh High review of `a1bdc6d499e48fbe1a08f1cddde0c9c4131c1cd1` found a fixture race and no production defect. The fixture declared its PID ready immediately after spawning, before Linux always exposed the child's arguments. Production correctly refused the incomplete identity. The fixture child now writes an explicit readiness message from its C main function. A five-second bounded pipe wait must receive that exact message before the fixture writes its ready PID or injects a startup timeout. The fixture closes its pipe and waits for its own child during cleanup. Production ownership validation is unchanged.

All seven cleanup methods pass. A separate clean-environment run repeats both successful query/shutdown and spawn-then-startup-timeout one hundred times each: 200/200 executions pass. The repeat runs in `thinkthen-0398b-pg-handshake` with 12 GiB maximum memory, 1 GiB maximum swap and the lane lock. It compiles a local synthetic child once, uses no network and leaves no child or retained scratch. Required lint for the fixture correction exits zero in `thinkthen-0398b-pg-handshake-lint` under the same limits and safe offline two-job wrapper. The receipt is `target/0398b-lint/pg-handshake-output.log`. It passes aggregate workflows 96/96, all prior consumer and ownership cases, policy, tickets, child boundaries, registry checks, deny, formatting, Clippy, docs and the 569-item inventory with four refusal plants. The scope is inactive and no tool process remains at handoff. Hosted proof and slice C remain pending.
