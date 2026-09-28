---
flow: build
priority: 256
opens: sdlc/issues/2026-09-27-audit-writes-the-tuned-bar-in-place.md sdlc/records/0256-audit-output-preflight.md
---

# 0256: Write an audited candidate beside its source

Status: proposed bounded design for fresh Medium review. The coordinator may approve routine details within the filed outcome after that review. No runtime, test, issue or public documentation file is changed by this design. [Preflight](../records/0256-audit-output-preflight.md) pins main `e920ae28` and the source, policy and test paths.

## Outcome

`thinkthen audit RESULTS KEY --write QUESTIONS --write-to OUTPUT` writes a reusable tuned question file or set at a new named path and leaves `QUESTIONS` byte for byte unchanged. `--write QUESTIONS` alone keeps its current in-place behavior, reports, digest refusal and failure precedence. The new option creates a file even when audit keeps every bar; that file then copies the validated source bytes. Both forms send no request and read no API key. This is the separate-candidate outcome in [the filed issue](../issues/2026-09-27-audit-writes-the-tuned-bar-in-place.md), not a claim that an observed crash damaged an existing file.

## Proposed command and destination rule

Use the additive name `--write-to OUTPUT`, requiring `--write QUESTIONS`. Neither path may be `-`. The old `--write` restrictions on `--threshold` and `--by` still apply. A missing `--write` or `--write-to -` is usage exit 2 before reading inputs. All other grading, key parsing, question resolution, digest checks and byte-preserving threshold/model/batch decisions run exactly once through the existing writer. A digest mismatch retains the current exit 2 and exact diagnostic before any output file is published.

`OUTPUT` must be absent when published. An existing regular file, even empty, a directory, a symlink including a dangling symlink, a hard-link alias, and `QUESTIONS` itself are all refused at exit 5 with `thinkthen: audit: the --write-to output already exists; choose a new path`. Never follow or replace that final entry. An unavailable parent, denied write, exhausted temporary name or unsupported no-replace publication fails at exit 5 with `thinkthen: audit: cannot write the --write-to output`. The source is unchanged on every new-destination outcome. These messages print no path or question text. Old `--write`'s exit 5 `cannot write the question file` stays unchanged.

For the new path only, write all bytes into a create-new regular temporary file in `OUTPUT`'s directory, close it, then atomically publish with `std::fs::hard_link(temp, OUTPUT)`. The final link is the no-replace decision even if another writer creates `OUTPUT` after grading; map `AlreadyExists` to the existing-output diagnostic. This reuses the recorder's standard-library publication pattern locally, without importing engine storage or adding a shared framework. Remove the temporary name after publication or on a pre-publication error. If publication succeeds but that removal fails, return success because `OUTPUT` is complete and append the fixed warning `thinkthen: audit: wrote the --write-to output but could not remove its temporary file`. If removal fails before publication, return exit 5 with `thinkthen: audit: cannot write the --write-to output; could not remove its temporary file`; no destination was published. No report promises power-loss durability or hostile replacement protection for a concurrently modified parent directory. An unsupported hard-link filesystem fails safely rather than falling back to a partial visible write.

Normal success keeps the existing per-bar standard-error report and ordinary audit standard output. The output's bytes must parse as the same single-question or set kind and preserve all source bytes outside the existing splice rules. A no-suggestion run still publishes an exact copy, never a hard link to the source. `--write-to` gives no overwrite or force mode. An operator chooses a fresh path for each candidate.

## Evidence

- **Starts from:** [the open issue](../issues/2026-09-27-audit-writes-the-tuned-bar-in-place.md), experiments 296 and 297 as recorded there, and the [reviewed tuning intake](../records/2026-09-28-tuning-loop-intake-preparation.md). Main `e920ae28` writes back in `cli/audit/write.rs::bars`; the in-place overwrite is deliberate, not observed crash corruption.
- **Keeps:** Current `--write` form, error order and exact messages; 0135's saved-question digest gate; single/set/recognize/relate question resolution; `rank`/`find` no-bar behavior; threshold byte splices, model and batch decisions, result reports, offline/no-key routing and zero sends.
- **Changes:** The paired `--write-to` form publishes a fresh, valid tuned file beside an unchanged source. It refuses every existing final destination and uses a complete temporary file plus atomic no-replace link for the new form only.
- **Proof:** One literal single-question CRLF/escaped-key output and one literal set-member output; source-byte snapshots and parse/reuse; current old-result digest refusal; absent/existing-empty/same-path/symlink/hard-link and parent-failure cases with exact exit, stdout, stderr, zero sends and no residual temporary file in ordinary failures. Retain and run existing in-place assertions. Source-review the final no-replace primitive and narrow policy plants; no provider or stress run.
- **Defers:** Changing in-place `--write` crash semantics, an overwrite/force option, cross-filesystem fallback, hostile parent-directory substitution, fsync or power-loss claims, public-page edits while marketing holds them, and tuning-loop repeat/cost/per-case selection work in the separate open reports.

## Source and review scope

Prospective build claim: `crates/thinkthen/src/cli/audit.rs`, `cli/audit/write.rs`, `cli/measure.rs`, `cli/args/command.rs`, new `crates/thinkthen/tests/audit_output.rs`, `sdlc/scripts/policy.py`, measured `sdlc/ratchet.json`, this ticket and a build record. Reuse `core/measure/splice.rs` unchanged and the current `tests/audit_write.rs`, `audit_model.rs`, `audit_refusals.rs` as retained regression proof. The policy currently permits only fully qualified `std::fs::write` in `audit/write.rs`; extend **only that writer** for exact create-new, hard-link and remove calls, while retaining writer/reader plants against `rename`, alias imports, broader writes, settings/network/process access and all other measure paths. Do not turn off its measure gate.

Current nonblank counts are audit 214, writer 204, measure 279, command help 415, audit_write 374, audit_model 79, audit_refusals 482. Keep the near-cap refusal test unchanged and put the new outside-in table in its own file. Measure the Rust ratchet exactly and explain any growth after checking the current writer, recorder publication and measure test helper for duplication. Use focused audit tests, policy, strict relevant Clippy, format, ratchet, pages and tickets. Public text to coordinate later: `specification/audit.md`, `specification/settings.md` and the command help; specification still describes temporary files as unwanted for the old in-place form, which remains true for that form.

## What the build taught us

Pending implementation and fresh code review. Record corrected assumptions, source growth, exact proof and any retained gaps here before landing.
