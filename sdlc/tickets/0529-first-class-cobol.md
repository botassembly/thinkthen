# 0529: Generate COBOL access from the complete C views

Status: COMPLETE.

Milestone: 0.2

Depends on: 0505
Depends on: 0517

Reviews: revision a087f6dc3, accept

Reviews: revision a6cb8f0877aa288a450715b79102570b8b8273af, accept

Reviews: revision d7eff7196b9cad6c6e0cc8db994e3d13954081de, accept

Landed: d3c8890

## Outcome

COBOL callers use one named callable interface with generated copybooks. Results arrive as flat fixed-width records with indicator bytes for presence, and failures as typed status records with retained facts. Rust owns every rule, so the COBOL label grammar goes. Hand-mirrored copybooks and old public names are gone.

## Evidence

- Starts from: The [2026-10-09 decision](../decisions/2026-10-09-thin-first-class-bindings.md) and the [0521 assessment](../records/0521-surface-contract-assessment.md). The seven `libraries/cobol/copybooks/tt-*.cpy` carrier copybooks are hand-mirrored. `libraries/cobol/src/tt_shape.c` restates the label grammar. These copybooks cannot be generated completely until 0505 supplies the complete C session views.
- Keeps: Every frozen 0.1 C symbol, documented COBOL representation limits and explicit overflow refusals, all ten functions, failure facts, cache and replay behavior and installed package checks.
- Changes: Meet the caller acceptance and the COBOL section of `../../libraries/BINDING-AUTHOR.md`. This ticket owns:
  - generated copybooks from the complete C header produced by 0505, as flat fixed-width records with indicator bytes;
  - typed status records with facts, owned cancellation and explicit session and buffer release;
  - generated COBOL request records, which the binding may serialize to JSON internally for the session, so callers never build JSON;
  - deletion of the label grammar and hand-mirrored copybooks once installed typed cases pass;
  - the package slice carrying the native library under 0517's design;
  - `libraries/cobol/README.md`, with a short old-to-new call mapping, and removal of old public names after parity.
  One public API is one coherent family of named typed calls. Claim `libraries/cobol/**` narrowed and named per slice.
- Proof: An installed COBOL consumer reads every known field and failure facts through shared cases, including a value that exceeds a COBOL representation limit and receives the explicit refusal. Record handwritten code removed and added, counting generator changes, in the landing record.
- Defers: Final distribution assembly belongs to 0530.

## Progress

- 2026-10-10 landed 7e6992ad8; next: Typed COBOL native sessions and generated request graph are landed with explicit scalar bounds. Final installed archive parity and platform qualification remain.
