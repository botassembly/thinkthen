# 0508: Run the single final after-sprint review of the 0.2 migrations

Status: OPEN.

Milestone: 0.2

Depends on: 0494
Depends on: 0495
Depends on: 0496
Depends on: 0497
Depends on: 0498
Depends on: 0501
Depends on: 0504
Depends on: 0505
Depends on: 0512
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

Reviews: revision 056b72947, accept

Reviews: revision 4d59e7c6a, accept

## Outcome

After the final language and database migrations, and before the full installed-package run, fresh read-only reviewers find the consequential bugs and specification drift in the final 0.2 code. Every confirmed finding gets a fix ticket or an explicit scope ruling. This is the only after-sprint review the rollout requires.

## Evidence

- Starts from: Ian's requested second review and the PM's 2026-10-08 order. The first review is retained in 0462; its acceptance does not cover the later Request, recognition, generated binding and package changes.
- Keeps: All approved 0.2 functions, languages, host rulings and the release hold. Each distinct secrecy, cancellation, lifetime, admission and cache failure check stays.
- Changes: Divide one read-only audit by area: core and engine, CLI, C ownership and generated contracts, language families, SQL and data frames, MCP, packaging, and documentation. Read all 0.2 changes since rc/0.1.0-rc.1, with emphasis on the later migrations and installed public interfaces.
  - Judge both goals of the [2026-10-09 binding decision](../decisions/2026-10-09-thin-first-class-bindings.md) with the caller acceptance in `../../libraries/BINDING-AUTHOR.md`: complete native values and failures, real nonblocking behavior where promised, cleanup, and final installed artifacts.
  - Count duplicated maintained logic, including generator templates. Check dead code, stale help and specification drift.
  - Confirm that the constrained-host views from 0505, 0528 and 0529 expose the same known observations as native results.
  - Reuse applicable family and package evidence. Do not rerun unchanged checks only to make them fresh.
  Claim `sdlc/records/0508*` only. Reviewers change no product code.
- Proof: One concise record gives each confirmed finding with file and line evidence, severity, its ticket or ruling, and the reviewed revision. Required defects are fixed before the final installed-package run. Passing tests alone do not prove absence of bugs.
- Defers: New proof machinery, per-language audit records, broad redesign unrelated to confirmed defects, and release management. Native platform checks owed to an authorized candidate stay explicit release obligations; they do not substitute for local acceptance or permit dispatch. Publishing and hosted qualification still need Ian's permission.
