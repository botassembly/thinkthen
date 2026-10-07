# 0432: Enforce shared behavior and generate the current parity table

Status: in progress. Slice A establishes the shared cases and adoption interface. Family implementation and the mandatory full parity run remain open.

Milestone: 0.2

Owner: builder.
Ticket review: ACCEPT, 2026-10-06; fresh read-only review. Conflicting image deferrals corrected where found.

## Outcome

The existing build ladder fails when a required public variant lacks a named function, admitted input, stable result field, error or canonical behavior. Every binding executes the complete shared suite with no skipped cases.

## Evidence

- Starts from: Current main c64b71859; 0405 audit is historical and predates 0377/0420. PM message `2026-10-06-pm-0-2-is-not-done-every-sdk-consistent-and-the-sdk-ready-for-the-proxy.md`, asks 2, 3 and 5.
- Keeps: Preserve existing bare calls and generic JSON compatibility doors, backend selection from 0377, six error kinds, cancellation, secrecy, count-only usage, and zero-send strict replay. Reuse the Rust engine, C boundary and native file reader; add no host cache, scheduler or second parser.
- Changes: Extend existing conformance/cases.json, settings.json, types/corpus.json and file fixtures; declare coverage independently of host implementations. Generate one current matrix from actual named public consumers and compiled/runtime field checks. Include all languages, separate JVM/Flutter/TS consumers, CLI, SQL and dataframe variants. Compatibility JSON cannot pass typed cells. Current foreign-family ownership: `libraries/go/**` `libraries/cpp/**` `libraries/csharp/**` `libraries/jvm/**` `libraries/swift/**` `libraries/zig/**` `libraries/objective-c/**` `libraries/php/**` `libraries/dart/**` `libraries/ada/**` `libraries/cobol/**` `libraries/c/tests/c/parity_output.c` `libraries/c/ratchet.c.json`.
- Proof: Plant a missing function, field and skipped case and confirm the existing gate fails with the affected cell. Unknown/duplicate case IDs, missing required runner/toolchain and exit 77 fail the complete parity run. Correct obsolete annotate packing expectations to ADR 0111 without losing partial-member behavior. Exercise cancellation at each real host boundary. Preserve Defect mapping, panic containment and secrecy at their existing private native/C safety boundary; no caller-injectable public fault input or new failure bridge.
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

## Shared integration in lane 2

Branch `ticket/0432-parity-integration`, read-only main baseline `bf373969c`.
Own declarations, shared fixtures, existing runner/table and test-rung integration.
The independent 0456 fixture targets include named/path collisions and refusal,
authored metadata, actual selected items, separate per-item/shared context,
finite/staged-stream admission, annotate members/documents, image ancillary
items and cache/replay validation. All retain 0407/0414 as required dependencies.
MCP uses the saved 0455 public consumer inventory and is a required pending
row; installed execution remains open. No host/native/C implementation is
changed and no completion or landing record is claimed by this integration.

Concrete native/MCP coordination dependency: the main closed surface list and
generated result schema do not yet admit `mcp`. 0455/native owners must supply
that token/schema change with `thinkthen mcp` and `libraries/mcp/check.sh`, then
move the pending row to the settled inventory/register its surface. 0456's
native loaders, declarations, actual-item admission and resolved typed metadata
must land with generated question/result schemas before families can execute
these targets. Existing image scenarios still need their owner's wire/response
and missing media fixtures. The runner refuses incomplete execution rather
than substituting generic JSON or schema/fixture decoding. Main routine gates
remain transitional; full parity is required at adoption completion.

Slice B code review: ACCEPT, 2026-10-06; fresh read-only whole-slice review of b13645e08. Shared integration can land independently; final required consumer execution remains open.

## Public input boundary ruling

Astra's read-only boundary review distinguishes a supported SDK input from an internal invariant or standalone JSON envelope. The two references to `25-defect-fault` remain private native/C safety cases. Forty-five corpus examples with no public call case, including the four native physical-source result envelopes, remain schema/legacy-door cases at the existing types self-test and decoder gates. Every distinct public input and refusal stays required, including described choice, all six safe error mappings where reachable, cancellation and every image admission scenario. The C executor uses explicit native question-role admission, ten named complete calls and typed getters. It does not parse fabricated owned-result envelopes or substitute the generic JSON call door. Public parity remains open until those actual required cells pass.

Coordinator ruling after Astra review: the incremental declaration fixture follows ordinary native packing rather than caller-authored stage markers. Five records at batch two complete the first two rows, then a missing-body fourth item rejects the entire second stage with Usage at position four and one prior request. The valid third sibling is not dispatched; the fifth is not admitted. The finite-all-before-send case keeps zero requests. C adopts the native fallible reader and batch API through 0426; no callback scheduler or staging control is introduced.

## Foreign-family rank-member follow-up

Families 0427–0430 qualified all fourteen source variants and actual extracted release variants against the 248 required cases. Existing public constructors remain valid. Ordered members retain actual author/details metadata; Ada exposes the settled final-row index getter through its public native and controlled APIs. The shared C fixture projects actual final/member facts and original indices. Its exact fixture path and measured fixture ratchet are the only additional C claims; native C product/header ownership stays separate.

The retained whole-family review and fresh narrow High review accepted the resolved changes. The [single family follow-up record and checklist](../records/0432-shared-parity-cases.md#foreign-family-rank-member-follow-up) preserves source/archive qualification and bounded rebuilt-Ada checks. Full tests, lint and executable documentation gate the final family commit. Overall 0432 remains in progress until the complete 29-consumer table passes; 0459 retains mandatory actual C ABI agreement.
