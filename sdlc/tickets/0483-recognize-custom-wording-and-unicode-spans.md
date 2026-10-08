# 0483 — recognize-custom-wording-and-unicode-spans

Status: OPEN.

Milestone: 0.2

Reviews: revision b9027d08b, reject

Reviews: revision b9027d08b, accept

Reviews: revision 872b8426330352368f7aca940cb1fa211f4e2f63, accept

## Outcome

Custom recognition descriptions use grammatical entity wording. A leading BOM or the specified zero-width prefixes do not become part of a recognized name, while offsets remain positions in the original Unicode scalars. Internal formatting joiners retain their current behavior.

## Evidence

- Starts from: second-opinion PM message of 2026-10-08, ask 5; questions.rs uses meaning.replace("name", "entity"); pieces.rs can include a leading BOM/format character in its first piece. Existing tables already test some combining marks; reuse those distinct cases.
- Keeps: Default wording and default cache compatibility, original input text, scalar offset conventions, caller instructions/descriptions, thresholds and retained distinct combining-mark behavior. No silent deletion of overlapping entities.
- Changes: First pin the actual public result with owned replay or loopback fixtures. Correct only the built-in custom BILOU descriptions; never rewrite caller wording. Define the exact narrow leading BOM/zero-width set in the specification and exclude it from extracted spans without stripping the original input or shifting later coordinates. Preserve internal joiners and other formatting behavior. Record the intentional custom-mode question/cache identity change and update owned custom replay fixtures together, including the known stale PHP accepted-request body fixture. Its 39 request/state counts already match current main; correct the wording oracle without weakening those counts. Land before new TCGA recordings accumulate, ahead of further recognition controls.
- Proof: Outside-in BOM, zero-width prefix and decomposed combining-mark inputs return exact scalar spans and extracted text. Default request/key vectors stay identical. Corrected custom wording changes its key intentionally; saved reconstruction/replay succeeds for the new fixture with zero unexpected sends.
- Defers: A general tokenizer redesign, accuracy claims and a policy for overlapping spans. Review before changing public reading behavior.
