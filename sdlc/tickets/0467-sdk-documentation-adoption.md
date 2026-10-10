# 0467: Describe the final 0.2 surfaces across the shared documentation

Status: OPEN. SDK module comments and install READMEs still describe complete execution as pending.

Milestone: 0.2

Depends on: 0494
Depends on: 0495
Depends on: 0496
Depends on: 0497
Depends on: 0498
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
Depends on: 0499
Depends on: 0512
Depends on: 0515
Depends on: 0530

Owner: builder.

Reviews: revision e71fa0b01, accept

Reviews: revision 4d59e7c6a, accept

Reviews: revision a087f6dc3, accept

Reviews: revision dead3de26, accept

Reviews: revision 2882bbf527363ef96ab9d65e4a9646168ea87226, accept

Reviews: revision ae72e2568b5c99e6011dc49bc15fa4f7dc37f467, accept

Reviews: revision 1181338a883bbc71082c315438e56d4fdf92c222, accept

Reviews: revision 219008c3d076a8c5f747c096c7b879b3a78c2f48, accept

## Outcome

After the language and database migrations land, the shared documentation describes the final 0.2 experience on every surface. A reader of the root README, the site or the specification finds the one recommended API per language, its install route and its real platform limits. The migrations that own the four files the 0462 review found stale (0518, 0523, 0505 and 0496) fix those statements. One upgrade guide maps each removed 0.1 call to its replacement.

## Evidence

- Starts from: the 0462 SDK review at 7ea661c1e. The Objective-C README line 55, Swift README line 26, Zig README line 28 and the Python `thinkthen/_complete.py` module comment describe complete execution as pending although the APIs exist. The [2026-10-09 binding decision](../decisions/2026-10-09-thin-first-class-bindings.md) then changed every surface, and the [0521 assessment](../records/0521-surface-contract-assessment.md) asked for one consolidated pass.
- Keeps: Actual package versions, installed-evidence limits, admitted image functions and platform restrictions. Released install instructions stay distinct from the 0.2 target until publication; public install stays 0.1.2 until then.
- Changes: Each migration ticket owns its own package README and old-to-new mapping. This ticket owns the cross-surface pass.
  - Claim `README.md`, `site/**`, `specification/**` and one new upgrade guide that collects each migration's old-to-new mapping.
  - Explain the stable JVM runtime floor and Apple-only Objective-C support from their owning tickets, 0504 and 0518.
  - Reuse `../../libraries/BINDING-AUTHOR.md` and generated API documentation. Restate no engine limit and keep no second surface inventory.
  - Add a short section to `AGENTS.md` that tells an outside agent how to report: open a GitHub issue with one of the repository's labels, ask questions on the discussion board, and link a fork in the issue in place of opening a pull request. Change the pull request section of `CONTRIBUTING.md` to match. Keep `AGENTS.md` under the 5,000-character lint cap. This is Ian's 2026-10-03 ask.
  - Close the transcript how-to help gaps. The `--record` help in `crates/thinkthen/src/cli/args.rs` names `thinkthen cache convert` for site recordings. The site says how to pick a `filter --threshold` from a few labeled cases. The site and `specification/result.md` say that a batched row carries an even share of `requests_sent`, so one row can show 0 beside a live request, and `--facts` gives the run total.
- Proof: A fresh review compares the text with the implementation and the retained installed cases. Run the focused documentation link, ticket and privacy checks. Replay an example only if it changes.
- Defers: New SDK behavior, package claims without evidence, hosted workflows and publication. None needs a ticket; publication waits for Ian's release permission.

## Progress

- 2026-10-10 landed 198636def; next: Current CLI examples, help flags, kind rules and recipe references are landed; the complete local site build passes. Finish documentation adoption for the remaining installed surfaces.
- 2026-10-10 landed b91134af5; next: The development upgrade guide maps confirmed typed replacements and accurately states JVM and Objective-C gaps. Finish owning migrations before final documentation adoption and installed qualification.
- 2026-10-10 landed f4f5ac238; next: External contributor reporting now uses labelled issues and fork links, with a question issue fallback while Discussions is disabled. Fresh review and focused links pass. Finish the cross-surface documentation pass against current typed APIs; installed and platform qualification remain held.
- 2026-10-10 landed de97eaeda; next: Cross-surface guidance now matches implemented typed APIs, live persistence, request shares and Foundation limitations. Fresh review and focused documentation checks pass. Final installed and platform qualification remain held.
