# Each patch release costs two hand passes on the site proofs and install text

Status: open. Filed 2026-10-04 by the queue owner from the 0.1 releases. Owner: the queue owner.
Milestone: 0.2
Kind: debt
Debt: 035
Severity: medium
Pay when: before the next release that changes a version.

Keeping it costs a builder hour or more for every release, and a missed pass leaves the site naming the old version.

Each 0.1.x release needed:

1. A re-proof of `site/examples/bindings-proof.json` on `release/0.1` after the version bump: `9463cef05` for 0.1.1 and `abac3ce61` for 0.1.2. The bump changes no behavior, but it changes the hashed trees, so every page goes stale.
2. A ticket on main to move the install text to the new version, then another re-proof and a Pages deploy: tickets 0392 and 0396, each with 22 to 34 pages replayed.

Small source edits stale pages the same way. One four-line R change staled 18 R samples.

To evaluate:

1. The proof hash leaves out version-only lines, or the version files, so a bump stales nothing.
2. A script does release-process section 5 step 5: it moves the install text to a named version and checks that nothing else changed.
3. Whether the install text can read the latest release at build time, so main never names a version.

Issue `2026-10-03-doc-tests-gate-checkpoints-and-releases.md` owns gating releases on the proofs. This issue owns the cost of keeping them fresh.
