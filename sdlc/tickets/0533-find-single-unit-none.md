# 0533: Admit one source unit when find offers none

Status: OPEN.

Milestone: 0.2

## Outcome

`find --none` accepts one real source unit as two alternatives: that unit and none. Native selection preserves the unit's original record and source position. Every surface inherits the same native admission rule; no dummy item or substitute question is required.

## Evidence

- Starts from: TCGA's [one-candidate report](../../../../agents/inbox/thinkthen/2026-10-10-tcga-demo-allow-a-single-source-candidate-with-find-none.md) reproduces a no-send refusal on main build `1848676089546d8d4a9de3880835abbf79eec128`. Native `Find::validate_count` requires two source units even when the none alternative supplies the second option. `specification/find.md` documents that limitation today.
- Keeps: The existing option bound, input order, selected original/source location, none semantics, empty-input behavior and refusal of one unit without none. No eleventh function or grep alias.
- Changes: Own native find count admission, affected public docs/CLI diagnostic and outside-in CLI plus Rust library cases. Derive every adapter's admission from the native owner. Preserve cache question identity for unchanged requests.
- Proof: One unit with none plans successfully without a send; saved exchanges select the original unit or none with correct position/probabilities through the CLI and Rust API. One unit without none and overflow still refuse before sending. Use small existing fixtures, no live call or load test.
- Defers: This consumer gap does not block TCGA's current experiment. Finish the active migration closures before assigning its code lane; no unrelated classifier or API expansion.
