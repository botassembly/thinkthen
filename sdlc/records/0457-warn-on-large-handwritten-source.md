# 0457: Warn before the source limit and preserve real file seams

Date: 2026-10-07

The existing policy warns at 500–999 nonblank lines and fails at 1,000 across hand-written Rust and binding code, including tests. Generated, vendor and build-output exclusions remain explicit; measured totals stay in the existing ratchets. The standing instructions now match this permanent rule.

Extended C schema prose moved to its existing public design reference. Ownership, lengths, borrowed pointers, cancellation and failure contracts remain adjacent to their declarations; all noncomment declaration tokens are unchanged. Restoring the release tool's recognized version marker leaves the header at 986 lines.

Swift question-definition carriers and their conversions moved verbatim to NativeQuestionViews.swift. Observations and result ownership remain together. The files have 971 and 140 nonblank lines; the aggregate grows by one import to 3312. Both actual package copy lists include the new file.

Fresh whole review accepted c76333c98. Policy boundaries, exclusions, actual Swift public/native compilation, helper behavior, archive secrecy and actual gitless package inventories passed. Full lint passed on the same product source. Final full tests and specification checks passed on 2febe18b4, whose only changes after the lint-qualified product are this record and ticket/team metadata. All workspace, library-only and external-consumer tests, doctests, shell checks, nineteen binding smokes and specification checks passed. Other large-file warnings remain because they are cohesive existing modules, and they no longer force artificial splits.

## What the build taught us

A warning should prompt judgment about a real seam. Comment cleanup must preserve version markers consumed by actual packaging tools.
