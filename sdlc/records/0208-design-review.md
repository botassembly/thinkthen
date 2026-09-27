# 0208 design review handoff

Status: submitted for fresh independent design review at main base `13febe404838a7bd6a6ab2bfcd83fdffd72ffcd2`. No reviewer verdict is recorded here. No runtime source, test, specification or plan file changed in this design-only draft.

Preparation checks: `python3 sdlc/scripts/tickets` reported 0 evidence failures from ticket 0120 on; `git diff --cached --check` passed for the three design files. No build, runtime test, provider call or full gate ran.

Review the proposed [ticket](../tickets/0208-cache-maintenance-preview-and-orphans.md) against the [source preflight](0208-cache-maintenance-preflight.md), local experiment 284 details 47 and 98, landed tickets 0124 and 0163, and `specification/recording.md:65-71,94-96`. In particular:

1. **Safety:** Does holding the current exclusive folder gate before temporary scan/unlink protect a cooperating live writer on each supported host? Does the proposed exact-name, regular-file-only rule preserve a final entry even when a crashed partial is its hard link? The design deliberately does not use PID existence or age and makes no hostile named-folder writer promise.
2. **Preview:** Is additive `--dry-run` sufficient for the now narrower register 98 gap, given the landed unknown-model guard, or does review require default preview plus a separate commit flag? Check exact stdout order, bad-entry stderr suppression on refusal, selected-versus-removed wording when an active digest lock can be skipped, and the absence of writes during preview.
3. **Reporting:** Should committed temporary cleanup name each file or report only count/allocated bytes? Are status's proposed `temporary_entries`/`temporary_bytes` names honest when the shared status gate can overlap a live writer? Should an unsafe object with an exact temporary name be named separately or remain an ignored unknown name?
4. **Scope and proof:** Confirm parent and adapter inventory, 463/500 `cache_prune.rs` headroom, no test-only hook, one held loopback writer plus one content-bearing planted orphan, and separate final-entry/temporary byte totals. Preserve selector union, modification-time/digest ordering, alias and unknown-model refusals, active-digest skip, and existing bad-entry diagnostics. Model freshness, binding, usage wait and folder authority remain separate.

Please record findings and an explicit accept/revise verdict on the frozen design before any implementation claim. Do not infer closure of register 47 or 98 from this note.
