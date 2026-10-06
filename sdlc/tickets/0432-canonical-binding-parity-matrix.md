# 0432: Enforce shared behavior and generate the current parity table

Status: in progress. Phase A declares shared cases and executes the existing baseline; family adoption and the mandatory full parity run remain open.

Milestone: 0.2

Owner: builder.
Ticket review: ACCEPT, 2026-10-06; fresh read-only review. Conflicting image deferrals corrected where found.

## Outcome

The existing build ladder fails when a required public variant lacks a named function, admitted input, stable result field, error or canonical behavior. Every binding executes the complete shared suite with no skipped cases.

## Evidence

- Starts from: Current main c64b71859; 0405 audit is historical and predates 0377/0420. PM message `2026-10-06-pm-0-2-is-not-done-every-sdk-consistent-and-the-sdk-ready-for-the-proxy.md`, asks 2, 3 and 5.
- Keeps: Preserve existing bare calls and generic JSON compatibility doors, backend selection from 0377, six error kinds, cancellation, secrecy, count-only usage, and zero-send strict replay. Reuse the Rust engine, C boundary and native file reader; add no host cache, scheduler or second parser.
- Changes: Extend existing conformance/cases.json, settings.json, types/corpus.json and file fixtures; declare coverage independently of host implementations. Generate one current matrix from actual named public consumers and compiled/runtime field checks. Include all languages, separate JVM/Flutter/TS consumers, CLI, SQL and dataframe variants. Compatibility JSON cannot pass typed cells.
- Proof: Plant a missing function, field and skipped case and confirm the existing gate fails with the affected cell. Unknown/duplicate case IDs, missing required runner/toolchain and exit 77 fail the complete parity run. Correct obsolete annotate packing expectations to ADR 0111 without losing partial-member behavior. Exercise cancellation at each real host boundary. Preserve defect-error coverage through a narrow existing test-only native failure bridge; no public crash input or general fault framework.
- Defers: Proxy service/screens, unrelated features and Windows Node/C#/JVM packaging remain outside this outcome.

## Dependencies and ownership

Agree case/schema changes before parallel host work; integrate each semantic/family ticket. Focused local selectors may remain, but the required full parity run has no selector or skip. Generated matrix reports typed versus compatibility capability and visible reviewed language rulings.

## Design notes

The historical 0405 table is not current proof. Platform availability determines which job executes a consumer; a missing required platform is blocked, not a passing cell. No receipts, fingerprints, provenance chains or checker self-audits.

## Expanded matrix

Require multi-image decide/choose/score, whole-file image inputs and stable full-result IDs on each public surface. Execute text-only and dropped-image-route refusals as declared cases, never skips. Use the same shared multi-image fixture and typed consumers; SQL adoption belongs to 0452. Rulings remain visible cells. Fixture configuration is explicit; consumers must not depend on ambient user configuration.

## Phase A ownership and adoption

Lane claude-0, branch `ticket/0432-shared-parity-cases`, base origin/main
`bcafbc1a7`. Own shared declarations, fixtures and existing runner/matrix
integration only. Native engine/images/C and host-family implementation remain
with their owners. `conformance/cases.json` declares the public rows and required
cases; `conformance/README.md` defines the family adoption interface.
`surfaces --parity-baseline` executes CLI/Rust/C for phase A and reports
all other rows missing/not checked. `surfaces --parity` remains the complete
explicit run during transition and fails missing cells;
routine gates validate declarations without claiming complete parity.

Phase A can land independently. Closing 0432 requires all families and
semantic owners to adopt the cases, supply real named/compiler/runtime
assertions, replace existing skips with boundary executions, and make the
passing strict full run mandatory. No publication or release approval changes.
