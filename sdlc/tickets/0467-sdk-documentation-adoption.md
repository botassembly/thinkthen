# 0467: Describe the landed complete SDK APIs

Status: OPEN. SDK module comments and install READMEs still describe complete execution as pending.

Milestone: 0.2

Owner: builder.

Reviews: revision e71fa0b01, accept

Reviews: revision 4d59e7c6a, accept

## Outcome

Objective-C, Swift, Zig and Python documentation consistently describes the implemented complete API while preserving genuine platform and publication limitations.

## Evidence

- Starts from: 0462 SDK review at7ea661c1e; Objective-C README line55, Swift README line26, Zig README line28 and Python thinkthen/_complete.py module comment retain pre-adoption claims contradicted by implemented APIs and later examples.
- Keeps: Actual package versions, installed-evidence limits, admitted image functions and platform restrictions. Public install stays0.1.2 until publication.
- Changes: Replace stale pre-adoption statements with current supported APIs and remaining restrictions. Update prose only, without changing examples or product outputs.
  Claim `README.md`, `libraries/**/README.md`, `databases/**/README.md`, `specification/**` and `site/**`.
- Proof: Fresh ticket/whole-change review against implementation and retained installed cases; focused documentation links, tickets and privacy checks. Replaying examples is necessary only if examples change.
- Defers: No new SDK behavior, package claim unsupported by evidence, hosted workflow or publication.

## Surface assessment amendment

Describe the final 0.2 experience across every migrated surface, extending the earlier four-language wording. Each existing README teaches the one recommended API, native inputs/results, failure and absence behavior, cancellation/cleanup, and the actual package installation requirements. Carry forward 0515's old-to-new call mapping and explain the stable JVM runtime floor and Apple-only Objective-C support from their owning contracts. Keep released instructions distinct from the 0.2 target until publication. Reuse the author guide and generated API documentation; do not restate engine limits or maintain another surface inventory. Name actual documentation paths per slice.
