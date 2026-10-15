# 0543: Pass one 0.2 candidate on Linux, macOS and Windows

Status: OPEN.

Milestone: 0.2

Depends on: 0542, 0549, 0550, 0551, 0552, 0553, 0554, 0555, 0556, 0558, 0560, 0561, 0562, 0563, 0564, 0565, 0568, 0569, 0573, 0574, 0576, 0577, 0578, 0579, 0580, 0581, 0582

Reviews: revision fe9bf662f0234e8177eecd2a0f0adea96b36e5d0, accept

Reviews: revision d0428c12fe3a6e13b0295c92a0d8069c1419522e, accept

Reviews: revision 175753e6db48df1d548e952101689be89e986b21, accept

Reviews: revision b17f9693d10918b04638aaa0b71578f0ca51bbad, accept

Reviews: revision 1d1686487, accept

Reviews: revision fe3906cfd, accept

Reviews: revision d1368f621, accept

Reviews: revision 2d7ca433a, reject

Reviews: revision 412ba61a1, accept

Reviews: revision 7326dcf5b09ac6811c0afbc0062f8639e4c6c077, accept

Reviews: revision f23c075965dc12a966847fee80c13bfab3b29385, accept

Reviews: revision 597ef8ae60b452689aa5f3813d6aea3967a5fe7b, accept

Reviews: revision 6712593, accept

Reviews: revision d20e618, accept

Reviews: revision ca927f9, accept

Reviews: revision f8005a6, accept

Reviews: revision 2c83b76, accept

Reviews: revision f718e5d, accept
Reviews: revision f6f3d75, accept

Reviews: revision 7c9fc07, accept

Reviews: revision 7856080, accept

Reviews: revision 4b43792, accept

Reviews: revision 3ca1b3e, accept

Reviews: revision 04e4f99, accept

Reviews: revision fe88e98, accept

Reviews: revision a209ecd, accept

Reviews: revision 55686da, accept

Reviews: revision 4536876, accept

Reviews: revision 95288ad, accept

Reviews: revision caaf55b, accept

Reviews: revision 8698f5f, accept

Reviews: revision b8d2d85, accept

Reviews: revision 8565eaa, accept

Reviews: revision 4815c6c, accept

Reviews: revision fdb26d3, accept

Reviews: revision dcbf4b3, accept

Reviews: revision 2a7c1f7, accept

Reviews: revision 02f341b, accept

Reviews: revision a9c44fd, accept

Reviews: revision 2e69ab7, accept

## Outcome

One reviewed commit on `ticket/0543-final`, tagged `rc/0.2.0-rc.13`, passes the local release suite, the coverage comparison and the nonpublishing hosted runs on Linux, macOS and Windows, and publishes nothing.

## Evidence

- Starts from: `ticket/0543-final`, which collects every 0.2 lane. [The 2026-10-10 ruling](../decisions/2026-10-10-drive-0-2-to-done.md) lets the coordinator cut candidates and run nonpublishing workflows. This ticket takes release preparation and the candidate stage from 0425.
- 0575 depends on this ticket for landing and deploy only, so it is not listed above. Its merge of `docs/0.2-refresh` into `ticket/0543-final` is still a freeze gate, because `site/scripts/check-binding-samples.mjs` checks the site samples.

## Decisions

- PM decision, 2026-10-13: drive this ticket and every ticket it depends on by [maximum parallelism](../decisions/2026-10-13-drive-0-2-with-maximum-parallelism.md) and [the standing build rulings](../decisions/2026-10-13-standing-build-rulings.md). Ian ruled that all library quality work ships in 0.2.
- PM decision, 2026-10-13: every `check.sh` shares one heavy-build lock with four permits. A checkpoint or candidate routine run takes all four, records `/proc/pressure/io` before and after, and reruns a timeout under high disk pressure once under the same hold before anyone treats it as a failure. The load cap applies only on the Beelink.
- PM decision, 2026-10-13: use only standard hosted runners, which cost nothing for this public repo. A lane dispatches only its own language's check by a manual run. Apple checks run on GitHub macOS, never on the M5 in their place.
- PM decision, 2026-10-14: the Rust ratchet is not a gate on a lane or the candidate, and step 4 extends this to every line-count ratchet for 0.2 growth. At freeze it equals the measured total, and the record names the 0.2 features that raised it above main. The consolidated final stood 1,567 lines above main after 0564 found no safe cut that kept every distinct check.
- PM decision, 2026-10-13: 0560, 0561, 0563 and 0565 close on the passing candidate SHA with 0562, 0564 and the other wave tickets.
- PM decision, 2026-10-14: hosted Windows and macOS runs gate every frozen commit the coordinator gates. The coordinator starts them in parallel with the local gates, not only after tagging.
- PM decision, 2026-10-14: 0576's formal grading runs on the candidate as part of the area review, and the coordinator owns it.

## Scope

- Changes: `.github/workflows/**`, `sdlc/planning/release-process.md`, `sdlc/scripts/surfaces*`, `sdlc/scripts/release-workflow`, `site/scripts/check-binding-samples.mjs` and `sdlc/records/0543*`. Fixes found by the candidate go to the lane that owns the file.
- Checklist:
  1. Done. Every lane merged into `ticket/0543-final` with an accepting review, including `docs/0.2-refresh` from 0575. Ian approved the public texts from 0575, 0577 and 0578 on 2026-10-14.
  2. Done. `release.yml` rehearse mode qualifies installed Swift and Objective-C Foundation on `macos-15` and `macos-15-intel`, builds the R source package from the published crate, and runs `check-binding-samples.mjs` on the resolved commit. `install-check.yml` has `windows-2025` rows for every channel that ships Windows.
  3. The frozen SHA runs routine `test` and `lint`, the Polars nextest and Clippy, complete C and installed C, a build of every library, and hosted Windows and macOS, and the failures are collected. This step is done when the 0543 record shows those results for the frozen SHA.
  4. Fresh reviewers review the frozen SHA by area: core, generators, each language group and docs. This step is done when the freeze recount lands. Line-count ratchets are not candidate gates for 0.2 growth. The freeze record is written once, at the candidate commit after the last fix. The root 44-line overage and the SQLite Python 3-line overage need no cuts.
  5. Tag `rc/0.2.0-rc.13`.
  6. Run in parallel: on the Beelink, `sdlc/scripts/test-full-cases --run`, `sdlc/scripts/package` and `sdlc/scripts/test-stress --run`, where exit 77 or a pending platform row fails and only Swift and Objective-C may skip; on the Beelink, `sdlc/scripts/coverage` compared by counts with the 0560 baseline net of deletions; on GitHub, `release.yml` in rehearse mode and `windows.yml`, covering Linux x86 and ARM, `macos-15`, `macos-15-intel` and Windows. The Windows run carries the platform proofs for 0383, 0384, 0385 and 0455 and the 0474 and 0480 cache checks. The candidate also carries these duties from closed or closing tickets, each recorded in the 0543 record:
     - 0560: one table that compares each coverage collection's reached count with its lower baseline count minus the deleted-line allowance, which is 59,962 minus the candidate's mapped line count.
     - 0561: the `nm -D --defined-only` export list for `libthinkthen.so` with one caller per export.
     - 0565: `git grep -n -w -i -E 'legacy|compat|compatibility|frozen' crates libraries databases`, with a one-line justification for each identifier it finds.
     - 0568: `sdlc/scripts/library-size --store`, showing no host library above three times the median except the named Python exception.
     - 0576: the `tools/list` measurement, which must stay under 25 KB, and the formal fresh grading of every library, database and the MCP server. The grading runs as part of the area review, and the coordinator owns it.
  7. Record each run's ID, exit and counts in the 0543 record, with no `target/` or `builds/` paths.
  8. Fix forward on `ticket/0543-final`, Windows included. A fix confined to one language reruns that language's tier and its hosted job. A core or generator fix reruns step 6. File a separate ticket only for a fix that needs its own design.
  9. Close this ticket and the wave tickets on the passing SHA and hand the SHA to the PM, who brings 0425 to Ian. Main takes that SHA after Ian's go.
- Keeps: no registry publication, no approval of the `release` environment, no paid call, and no secret read in any rehearsal. The exact-commit rehearsal guard from 0398.

## Proof

The 0543 record names the candidate SHA once with each run's ID, exit and counts, and every run in step 6 passes on that SHA.

## Defers

Release QA, publication and after-publication checks stay with 0425, 0393 and 0398. Text changes Ian asks for after the freeze rerun the README check, `sdlc/scripts/package`, the site build and the hosted rehearsal under 0575, not the full campaign. Pinning and pushing the DuckDB descriptor follows Ian's go under 0577.

## Progress

- 2026-10-11 started
- 2026-10-11 landed b118ee482; next: Finish code tickets and final review before running the candidate suites and nonpublishing platform workflows.
- 2026-10-11 landed fa49e95b9; next: The checkpoint fixture now proves documentation replay precedes staging and replay failure prevents staging; finish code tickets before candidate qualification.
- 2026-10-11 next: Qualify the reviewed candidate with local full cases, package and stress checks, then run the nonpublishing release rehearsal and Windows workflow.
- 2026-10-11 next: Correct the package graph rule for the shared CSV reader, then qualify a new candidate; rc.7 stopped before full cases or hosted dispatch.
- 2026-10-11 landed 09b4ab1ee7e61fcdf90498385cb71984db527c3e; next: The package graph retains shared CSV parsing and rejects CLI dependencies; qualify the next candidate locally before hosted rehearsal.
- 2026-10-11 next: Make the generated request schema part of the Rust source package, then qualify a new candidate; rc.8 passed library-only tests but failed default package verification.
- 2026-10-11 landed 483faa4e2; next: Qualify a new candidate with the reviewed portable MCP schema; rc.8 retained the failing source-package evidence.
- 2026-10-11 landed 4be5631e0; next: Qualify the corrected full checkpoint on a new candidate, reusing unchanged native artifacts and rebuilding packages that record the candidate commit.
- 2026-10-11 next: Repair the common smoke isolation and native package helper plus the affected language smoke paths, then qualify their reviewed batch on a new candidate.
- 2026-10-11 landed 933be5367; next: Finish and review the common native installer and native smoke repairs, then qualify their batch on a new candidate.
- 2026-10-11 landed 2c139537a; next: Finish the reviewed native smoke batch and qualify one candidate through full installed parity, stress and hosted platform checks.
- 2026-10-11 landed 1e6e4b562; next: Qualify the reviewed candidate through full installed parity, stress and hosted platform checks.
- 2026-10-11 next: Stopped rc.11 and retained its logs; land 0549 first, then 0550 through 0554 before a fresh search review and new candidate.
- 2026-10-11 next: Retain stopped rc.11 logs, land 0549 through 0556, search every reviewed change, then qualify a fresh candidate with background runs.
- 2026-10-11 next: Read amendments 22b005e1b and consolidate current main, 0549, 0555 and the product stack into one revision; runner-only ad4b49b4a exited 1 on a missing retained TypeScript archive, three developers now own consolidation and runner fixes, decode and direct ABI proof, and Engine regressions, with compiler checks released and no source-only acceptance permitted.
- 2026-10-11 next: Combined ba41be5a7 passes unchanged installed C 42/42, Polars 46/46 and owning Rust checks; whole routine test and lint now run on that frozen revision before any acceptance, while exhaustive ratchets confirm JVM overages and a new C fixture overage, with a separate safe fixture repair lane and Mac package prerequisites unresolved.
- 2026-10-11 next: Official test on frozen ba41be5a7 exited 0 with 2002 workspace, 21 consumer and 44 Python cases; full C passed 25 unit and 82 integration cases plus installed 42/42 and Polars 46/46, but lint exited 1 on child-environment violations that the runner lane is repairing, so no acceptance or candidate qualification is recorded.
- 2026-10-11 next: Prepared current-main combination e52ac9593 is pushed and all developers and jobs exited; routine test passed on ba41be5a7 and runner plus Ada generation repairs pass their owning checks, but final acceptance waits for unchanged C, JVM and Ada source-limit rulings, genuine matching M5 Apple packages and final same-revision gates followed by fresh review and the strict candidate campaign.
- 2026-10-12 next: Third amendment is applied and cold C nextest timeouts reproduce on unchanged main before either C caller launches; frozen 243d59647 passed routine tests, Polars and installed C, but complete C exposed a repaired fixture include dependency and lint exposed stale public API declarations, so the API provenance repair precedes a new frozen revision and its required gates before acceptance.
- 2026-10-12 next: Frozen 6712593f6 passes routine test and lint, Polars nextest and both Clippy configurations, complete C and installed C, and both fresh reviewers accept every assigned change and limit; rc.12 is pushed and its background local full-case, package and stress sequence precedes nonpublishing macOS and Windows qualification.
- 2026-10-12 next: Rc.12 at 6712593f6 exited 1 with 24 local consumer rows passing, C selected-object-false-unicode refusing item_schema, and Dart plus Flutter preparation failing on the missing Flutter Isolate import; all children exited, package and stress were unrun, and two fresh developers own the retained-assertion repairs before new exact-revision gates and qualification.
- 2026-10-12 next: The fourth amendment moves the next full campaign to the final candidate; exact-source gates passed on 48b22e71f, fresh review found Go request duplication and JVM/Dart immediate metadata loss, and targeted repairs passed before integration and new same-revision gates.
- 2026-10-12 landed aa595d6950d0f7243e980d27f417570b65239a49; next: The tested and freshly reviewed corrective batch is pushed and its nine tickets are closed; follow the fourth amendment with 0560 baseline, then 0561, 0565, 0562 and 0563, with 0564 after baseline and final candidate qualification after those changes.
- 2026-10-12 next: Follow the fifth amendment: finish the 0560 baseline, combine 0561 and 0565 alongside Rust-only 0564, then 0562 and language groups for 0563 with library 0564, using wave checkpoints and one final candidate on ticket/0543-final.

- 2026-10-14 next: Merged the accepted hosted-runner admission correction at a6bbf9b0b with sixteen admission checks; hosted builds retain four permits without the Beelink load cap, and Swift ARM installed proof passed on GitHub.
- 2026-10-14 next: Merged JVM, Ada, C++ and generated caller repairs into the combined branch; the verified aged cleanup removed 864 SQLite scratch folders and obsolete coverage output freeing 1416732905 bytes, and landers removed their merged lane outputs while language proof continues.
- 2026-10-14 next: Merged the freshly accepted Ruby, Dart and Flutter, and Rust intake repairs with unchanged reviewed source; Go passed its single-language standard hosted check, merged lane outputs were removed, and the remaining language, documentation and tooling lanes continue toward freeze.
- 2026-10-14 next: Merged the freshly accepted website and all 23 public package texts with their installed-example release guard at 15d416c1d; marketing reads the frozen texts while fresh pre-candidate review and shared-tooling repairs continue.
- 2026-10-14 next: Merged the freshly accepted ordered JVM inputs, event-based waits and generated bootstrap metadata at 399fd2a50 with Java, Kotlin, Scala and changed examples passing; the SQLite typed-family migration and Ruby declaration guard repair remain active before freeze.
- 2026-10-14 next: Integrated the 0580–0582 rulings at fba349c8a after the completed af3661d5 checkpoint passed routine tests and every available library build; seven developers now implement the shared result, generator, three SQL families, Rust Polars and Python before example rewrites and freeze.
- 2026-10-14 next: Python one-result API and shared paired-case generation are merged with fresh acceptance; repair reviewed Rust recovery defects and complete SQL and remaining language result migrations before the next checkpoint.
- 2026-10-14 next: Accepted Rust core, TypeScript, Ruby, Polars, C#, PostgreSQL, DuckDB and shared closure checks are merged through b591d6ded; eight developer lanes cover five language examples, Go and its shared probability guard, SQLite integration and hosted candidate preparation, with database examples and Rust caller review next.
- 2026-10-14 next: Ian paused the seven active subagents for logout; Go settlement guards are merged at d9a03bdc0, PHP is accepted but unmerged, local C++ and JVM checks were stopped with logs retained, and Swift smoke plus Objective-C compiler-negative review fixes and remaining language work resume after login.
- 2026-10-14 next: PHP, JVM and Dart/Flutter result work is merged and pushed at d83e2eded; R probability repair, Swift smoke and Foundation compiler checks are running under restored access, Zig/Ada/COBOL implement approved shapes, and the remaining texts and shared inventories are in progress.
- 2026-10-14 next: Accepted C++, R, Apple and registry/archive repairs are merged and pushed; Ada and COBOL finish installed checks, Zig is in fresh review, and Go text and the website integration proceed in parallel.
- 2026-10-14 next: Accepted Zig and COBOL results, Go examples, website and MCP guides, the no-key DuckDB descriptor and registry controls are merged; Ada review and the remaining language texts, catalog and Zig inventory finish before the combined gate.
- 2026-10-14 next: Merged accepted late language texts, source accounting and freeze metadata at 110f84f6d; public fresh-reader and rendered-site checks plus all ten fresh MCP calls pass, while fixed-revision gates collect results and the restored SQLite empty-result assertion dispatch awaits fresh review.
- 2026-10-14 next: Frozen 54dc57da7 gate results are recorded: Linux and ARM Apple checks passed, Windows failures are in five active repair and review lanes, and accepted fixes plus release-note drafts precede a new same-revision gate and candidate.
- 2026-10-14 next: Merged the accepted usage finalization repair at e929ecbca after its concurrent regression and focused checks passed; the Mac cleanup-probe repair now has passing native and hosted Swift proof and is in fresh review before refreeze and exact-revision gates.
