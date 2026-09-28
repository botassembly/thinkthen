---
flow: build
priority: 229
opens: sdlc/tickets/0229-identify-unused-fixture-entries.md sdlc/records/0229-unused-fixture-preflight.md
---

# 0229: Report unused entries in an explicit recording folder

Status: proposed for fresh independent design review. Owner: Codex. The [preflight](../records/0229-unused-fixture-preflight.md) pins main `e182b546` and the current boundaries. No runtime or settled specification edit is authorized by this note. The coordinator owns source claims and register closure.

## Outcome and authority

Give a fixture maintainer a read-only report of valid entries in one named recording folder that a **complete, caller-supplied run digest list** did not use. Register 107 in experiment 284 and architect-review issue 09 item 12 describe the dead-entry problem. The utility is useful after a harness has finished and saved its complete request digest list. It cannot infer that list from mtimes or ordinary saved output, and it never removes an entry. The copied Beatles Bench site example is not its runner; keeping that external bench lean remains with its owner.

## Evidence

- Starts from: register 107, architect-review issue 09, `specification/recording.md:97,107`, `cli/args/command.rs:278-309`, `engine/cache_prune/scan.rs:84-150`, the accepted 0163/0208 cache scans, and current main `e182b546`.
- Keeps: byte-exact request digests and entry names, explicit folder scope, `cache prune DIR` as the only removal command, replay and cache behavior, folder gate, bad-entry protection, and no implicit default cache or paid call.
- Changes: propose `thinkthen cache unused DIR --used DIGESTS` as a read-only lexical report of validated final entries absent from a supplied one-digest-per-line manifest, with a total and no content-bearing output.
- Proof: a compiled no-key replay with one complete detailed result over a two-entry scratch folder, independently pinned used/unused names, exact report, invalid and duplicate manifest edges, bad-entry refusal, and before/after names, bytes and stable metadata.
- Defers: producing a complete digest manifest for arbitrary suites, automatic deletion, external Beatles Bench integration, access journaling, and any claim that filtered or bare saved result output proves every request. The coordinator must not close register 107's bench-lean criterion from this CLI utility alone.

## Proposed behavior

The `--used` file contains lowercase 64-hex request digests, one per line. Empty lines are ignored and duplicates collapse. Reject another nonblank line as usage error before opening `DIR`. Accept an empty valid file as an honest report that the supplied run used nothing; the output must say it reflects the supplied list, not prove that a suite ran. Read an existing explicit directory through its shared folder gate. Reuse the accepted `cache_prune` final-entry scan and identity checks. Sort valid unused names lexically and print `unused N entries from supplied digests`, then one `unused DIGEST.json` line per entry. A bad digest-named object takes the existing generic local entry refusal with empty stdout, so it cannot print a misleading clean report. Ignore unrelated names, temporary files and `.locks`; do not classify them as unused. Do not follow symlinks or read entry bytes into diagnostics. Extra valid manifest digests that are absent from this folder are not automatically an error or evidence of a recording miss; the manifest may span folders. A concurrent cooperating prune waits for or excludes this snapshot through the existing folder gate. Do not promise a transaction against an external writer that ignores it.

For a one-command replay, a maintainer can save `--details` output and extract `meta.requests[]` into DIGESTS, then call `cache unused`. That recipe is complete only if the saved output lists every request of that run. `filter --details` omits dropped rows (`tests/backend/keeping.rs:262-287`), and bare or quiet output does not print digests; a suite with such paths needs a complete manifest from its harness. The command cannot silently promote partial output to a complete usage claim. It neither edits the folder nor supplies a delete option. An operator may review the report before maintaining fixtures with existing file tools; the external bench owner chooses its own retention workflow.

## Implementation and proof boundary

After an exact runtime claim, edit `cli/args/command.rs`, `cli/args.rs`, `cli/mod.rs`, `cli/cache.rs`, `engine/cache_prune.rs` (one private child only if its measured parent needs it), `tests/backend/default_cache.rs` and one small `default_cache/unused.rs` child, `specification/{recording,settings}.md`, the measured ratchet and build/review records. The scanner and folder lock are read-only sources. No engine request/facade/public accounting, site source, provider fixture, marker migration or shared harness API change is proposed. File headroom and exact safeguards are in the preflight.

The outside-in case must prove that a single completed replay's detailed `meta.requests` is a complete list for that **single** command and equals a known fixture digest independent of the unused-report implementation. Plant a second valid entry and pin its expected name; the report must name only it and the right total. Repeat with duplicate and invalid manifest lines, a bad digest-named sibling, and an empty manifest. Record names and bytes plus stable mode/type/mtime before and after; exclude access time. Check absence of marker, lock files, default-cache folder, usage file and listener requests. Keep existing prune/model/size and replay cases unchanged. Run selected focused cases, strict affected Clippy, format, ratchet, pages/tickets/diff; no full suite, stress, provider call or bench campaign.

## Deferred gaps and routing

This ticket may complete the **reporting utility** after the proof and fresh code review. Register 107 remains open until a complete suite manifest route and the bench's lean-folder workflow are demonstrated by their owner; a partial list must not be labeled a full pass. The settled removal rule in ADR 0017 section 5 and `recording.md:107` remains intact because this command does not remove. Ticket 0208 is precedent for a read-only cache report, not blanket authority for new CLI syntax. Fresh design review must assess the additive public form and whether Ian or a new ADR is needed; no ADR number is reserved. The coordinator claims exact source files only after that decision and review. Ian can overturn the proposed syntax, manifest precondition or reporting scope before build.

## What preparation taught us

Saved request digests made a read-only comparison plausible, but the filter output counterexample prevented a false universal normalizer. The chosen proof uses an actual completed replay and a separate expected fixture digest; it does not derive the expected unused set from the same report being tested. Existing scan and folder-gate behavior avoids a second parser and keeps invalid or replaced entries out of the report. Ownership of the external bench runner remains separate from this repository's copied site example. A later builder should record whether these boundaries stayed sufficient, and name any correction rather than broadening the engine preemptively.
