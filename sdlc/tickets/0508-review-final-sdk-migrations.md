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
Depends on: 0499
Depends on: 0467
Depends on: 0484
Depends on: 0515
Depends on: 0530

Reviews: revision 056b72947, accept

Reviews: revision 4d59e7c6a, accept

Reviews: revision a087f6dc3, accept

Reviews: revision 2b4e0fa245a8f67d40946ba9b125e3a9333922b9, reject

Reviews: revision 1bb6b60bbf2456c755f588864256752be6caab46, accept

Reviews: revision ae711b41405f8ca96eb68ba0e3195f8ecb7d5dfe, accept

## Outcome

After the final language and database migrations, the documentation pass and the bounded cleanup, and before the full installed-package run, fresh read-only reviewers find the consequential bugs and specification drift in the final 0.2 code. Every confirmed finding gets a fix ticket or an explicit scope ruling. This is the only after-sprint review the rollout requires.

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

## Progress

- 2026-10-10 started
- 2026-10-10 landed cc66bc590; next: The canonical after-sprint record now covers all reviewed areas, confirmed caller harms, repairs and evidence limits. Four reviewed core, CLI and SQL fixes still need compilation and focused runtime checks under the lane cap. Keep open until those repairs land; installed and platform qualification remain separate and held.
