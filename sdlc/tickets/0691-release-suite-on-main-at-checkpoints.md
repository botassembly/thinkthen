# 0691: Run the release suite on main at each checkpoint

Status: OPEN.

Milestone: 0.2.1

Phase: 1

Depends on: 0615, 0618, 0619

Reviews: revision 5b6430e, accept

Reviews: revision 63cc2ab, accept

Reviews: revision 5ad7d11, accept

Reviews: revision efa8d45, accept

Reviews: revision 8c9d297, accept

## Outcome

- Changes: `AGENTS.md`, `conformance/test_candidate_campaign.py`, `sdlc/planning/release-process.md`, `sdlc/scripts/README.md`, `sdlc/scripts/test-full-cases`.
- One command runs the release suite (`test-full-cases`, `package`, `test-stress`) on a named main revision, writes one log, and needs no separate runner, verifier or receipt format.
- At each checkpoint (the end of phase 1, the core-freeze commit 0688, and each batch of phase 3 landings the coordinator names) the coordinator runs that command through pm job with `--memory 16` on the M5 natively, in the arm64 container on the M5, and in the x86_64 container on Yellow.
- A failure on a checkpoint run is a ticket against the lane that caused it, fixed on main before the next checkpoint.
- At the first checkpoint after 0613, 0620 and 0622 land, the coordinator dispatches one nonpublishing hosted rehearsal on the same revision to prove those workflow tickets. No other hosted run serves as an everyday proof.
- The repo instructions describe this command in place of the current "before asking anyone to bless a release" line.

### Proof

- The phase 1 checkpoint revision passes the command on all three targets, each with one log the PM can cite by revision.

## Amendment, 2026-10-19 (core cut)

0619 now closes on the core image only.

- The PM's 2026-10-20 scope ruling makes the phase 1 checkpoint and the core-freeze checkpoint (0688) run the existing full Rust and C smoke selection (`smoke libraries/rust libraries/c`) and `package` natively on M5, plus the configured gate and existing `package` check in both containers.
- The whole native `test-full-cases` and `test-stress` campaign and the full container campaigns start at the phase 3 checkpoint where every language's providers and assets are in the manifest, and run at each later checkpoint. R and PostgreSQL provider work stays with its own wave tickets. Missing consumers never count as passing.
- Add no core-only selector to `test-full-cases` or `test-stress`, and never weaken a missing-tool failure.

## Evidence

Rule 6 of the 0.2.1 ruling says candidates are main revisions and the release suite runs on main at each checkpoint, so no release-only check meets a candidate first. In 0.2, release-only checks ran for the first time at each candidate, and each candidate found one new setup fault. Nothing in the 0.2.1 set runs the release suite before 0687. The retired campaign-owner verifier needs no replacement runner: the suite already has entrypoints.

## Progress

- 2026-10-18 started
- 2026-10-20 next: Fresh fixing review accepted 63cc2ab41 with current-main integration and corrected checkpoint instructions; PM ruled that existing container package and configured C checks plus lint and test satisfy the amendment, named Rust and C smokes run natively on M5, and no linux-check extension is added, while the unchanged checkpoint regression runs in the pending integrated closing gate.
