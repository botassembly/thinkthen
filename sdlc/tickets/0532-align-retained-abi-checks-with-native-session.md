# 0532 — Check retained and native binding declarations against the C header

Status: OPEN.

Milestone: 0.2

## Outcome

Binding checks compare the declarations a caller actually uses with the canonical C header. Frozen compatibility declarations retain their checks, and generated native session declarations receive the matching checks. Adding native carriers does not make an unchanged compatibility declaration appear to promise the whole expanded header.

## Evidence

- Starts from: b95df1a7d. The routine installed replay smoke fails PHP, Dart, JVM, Ada and COBOL at their existing ABI comparison. Each reports newly added complete carriers or session imports absent from an older declaration inventory. Confirm whether each is a real missing import or an incorrect scope before changing it.
- Keeps: Target-compiler layout measurements, exact represented field and function comparisons, enum checks, installed artifacts, and planted wrong-layout or missing-import refusals. No unconditional omission of native declarations.
- Changes: The affected existing ABI checks and their declaration inventory sources. Read generated native and retained compatibility declarations from their actual package sources. Fix missing declarations through the owning generator when required. Keep one shared mechanism where the existing checker can express it; add no receipt or verification framework. Claim only these checks and narrowly required declaration sources.
- Proof: Reproduce the five routine smoke failures. Run each repaired installed replay smoke and the existing negative ABI cases. Verify generator freshness for any changed generated declaration. No full parity, load, large-input or release run.
- Defers: Publication and wider platform qualification remain under their existing tickets and Ian's release hold.

## Order

Inspect all five failures, distinguish incomplete declarations from stale comparison expectations, repair the shared cause, then test each affected installed package once. This is a binding-family repair and receives one fresh review.
