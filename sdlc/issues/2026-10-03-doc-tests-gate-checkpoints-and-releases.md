# The doc tests gate each checkpoint and each release

Status: open. Filed 2026-10-03 from the docs message "The doc tests gate the code release" of 2026-10-01. Owner: the queue owner. Ticket 0398 slice C owns the exact-commit rehearsal guard only; this issue retains checkpoint doc-test enforcement and strict release proof checks in items 2 and 3.
Milestone: 0.2

Ian ruled on 2026-10-01 that every documentation example is a test in this repository, run before any release of the site or of the code. The examples already replay: `npm run smoke-bindings` and `npm run smoke-sql` in `site/` run every language and SQL sample with no key and no network. Neither `sdlc/scripts/` nor `.github/workflows/release.yml` runs them.

The work:

1. One command runs every doc test and fails on a missing toolchain. The site owns it. The queue owner owns `site/` since Ian's ruling of 2026-10-02.
2. The checkpoint sweep of `release-process.md` section 2 runs that command before a checkpoint publishes, and the checkpoint message names the result.
3. A release fails when `site/examples/bindings-proof.json` does not match the tagged commit's engine and binding tree. A strict mode of `site/scripts/check-binding-proofs.mjs` does that check with Node alone, so the release workflow can run it.
4. `pages.yml` refuses a stale `bindings-proof.json`.

A doc sample that fails at a checkpoint is fixed on its page, or filed as a code bug.
