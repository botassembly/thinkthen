# The doc tests gate each checkpoint and each release

Status: closed.
Resolution: 0425
Milestone: 0.2

Ian ruled on 2026-10-01 that every documentation example is a test in this repository, run before any release of the site or of the code. The examples already replay: `npm run smoke-bindings` and `npm run smoke-sql` in `site/` run every language and SQL sample with no key and no network. Neither `sdlc/scripts/` nor `.github/workflows/release.yml` runs them.

The work:

1. One command runs every doc test and fails on a missing toolchain. The site owns it. The queue owner owns `site/` since Ian's ruling of 2026-10-02.
2. The checkpoint sweep of `release-process.md` section 2 runs that command before a checkpoint publishes, and the checkpoint message names the result.
3. A release fails when `site/examples/bindings-proof.json` does not match the tagged commit's engine and binding tree. A strict mode of `site/scripts/check-binding-proofs.mjs` does that check with Node alone, so the release workflow can run it.
4. `pages.yml` refuses a stale `bindings-proof.json`.

A doc sample that fails at a checkpoint is fixed on its page, or filed as a code bug.

Items 1 and 4 landed at `8a2db1df0`: `npm run test-docs` runs the doc tests, and Pages checks strict binding proof. Items 2 and 3 remain open with release safety. Ticket 0402 slice A changes no checkpoint or release workflow.

## Reconciliation, 2026-10-08

Items 1 and 4 remain landed at `8a2db1df0`; item 2 checkpoint enforcement and item 3 strict release proof remain open. `68abc60a4` enforces exact-commit rehearsal routing, which does not by itself run every documentation sample or replace those two obligations. The introductory sentence about neither workflow running them is historical; the current test-docs/Pages checks own their stated scope.
