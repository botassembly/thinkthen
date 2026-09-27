# 0171: tuned batch warning build

Status: candidate on `ticket/0171-a-tuned-threshold-warns-at-another-batch`; fresh code review and landing pending. Base: `f7080025`, which includes landed 0170. Ticket: [0171](../tickets/0171-a-tuned-threshold-warns-at-another-batch.md). ADR: [0085](../planning/adr/0085-a-tuned-threshold-names-its-batch-setting.md).

## Result

A threshold-bearing file remembers its batch setting for warning purposes. A different resolved record-run setting prints one warning at the successful output boundary, and detailed rows carry `meta.batch_warning`. The profile and batch warnings have independent print-once state and a fixed order. A single document, a structured JSON question, an untuned file, and a first failed batch do not invent a batch warning. `audit` and `diff` read nested batch settings from raw result lines. `audit --write` writes one batched setting, removes a stale setting after batch-one results, or keeps a setting when results are mixed. Removal preserves the rest of the file bytes. The question digest and cache key remain unchanged.

## Distinct contracts and proof

| Boundary | Focused proof |
| --- | --- |
| Resolved file/typed/default setting, one warning, and detailed metadata | `backend::batching::warning::a_tuned_file_warns_at_another_batch` table; listener request counts distinguish batch 1 from max |
| First successful output, independent profile and batch lines, empty filter, single document, JSON question | `backend::batching::warning::batch_warning_uses_the_successful_output_boundary` |
| First failed batch emits no calibration warning | `backend::batching::warning::a_first_failed_batch_has_no_calibration_warning` |
| Nested saved metadata, single/mixed settings, exact write/remove bytes and eligible file kind | `audit_write::audit_reads_and_writes_the_batch_setting`; unchanged existing audit-write rows |
| Two-sided comparison and unchanged standard output | `diff::diff_warns_at_different_batch_settings`; unchanged existing diff rows |
| Byte-preserving top-level removal in each position and CRLF | `core::measure::splice_tests::removing_a_batch_key_keeps_the_other_file_bytes` |

The new tests observe command output, saved file bytes, and loopback requests; they do not reproduce the implementation's warning formatter as their oracle. The existing profile, audit-write, diff, and batching tests remain relevant. No paid call or broad stress run was used. The ticket's proposed deliberate-break list describes plausible red mutations; this build did not claim a separate red run for each one.

## Measured growth and duplication review

All figures are nonblank Rust lines against `f7080025`. The original estimate was 162 product, 170 tests, 332 total. The candidate is 235 product, 310 tests, 545 total, a proposed measured amendment for fresh review. Source changes: typed batch value and result metadata +81; record-run setting and warning state +65; audit/diff and byte-preserving splice +89. Test changes: command warning cases +188; audit-write +71; diff +23; splice table +28. No file exceeds 500 nonblank lines; `cli/asking.rs` reaches 484 and the existing near-cap batch/args files remain unchanged.

Main advanced independently while this branch was being built: `30d340d1` lands 0207 and raises the root Rust total from 78922 to 78988. The 545-line figure is this ticket's delta against `f7080025`; integration must add the independent 66 lines and set the combined ratchet to the measured source total. That later ratchet merge must not be mistaken for a change in this ticket's scope.

The new `BatchSetting::in_results` is shared by audit and diff instead of copying nested metadata traversal and the absent-as-one rule. `Mismatch` reuses the existing successful-output warning boundary while keeping profile and batch print flags independent. The splice removal uses the existing scanner and has one position table. The command test combines setting-tier rows in one table, keeps the empty-filter boundary in one case, and isolates first-failure behavior in a short case that meets the function-length lint. The earlier order-changing annotate fixture draft was deleted. No product rule or distinct accepted boundary was removed to meet the original estimate. The addition changes no dependency.

## Validation

The final focused pass: `cargo test -p thinkthen --test backend batching` (21), `--test audit_write` (6), `--test diff` (14), and the library splice removal test (1) all passed. `cargo clippy -p thinkthen --all-targets --all-features -- -D warnings` and formatting passed. `sdlc/scripts/tickets` had zero evidence failures; `sdlc/scripts/pages` had 21 green and one coming page. The executable settings check had 43 rows and zero failures; `mustmatch test spec/audit.md` passed eight checks. The Rust ratchet is exactly 79467 and no Rust file exceeds 500 nonblank lines. `git diff --check` is clean.

I also started `sdlc/scripts/surfaces` as a ticket-only check. That was broader than this change warranted and held the shared build lock while library ports rebuilt. I stopped it at the coordinator's request. Its Rust library, C, Python, TypeScript, and R sections had passed before the stop; the whole command exited 130 and is not claimed as a passing gate. The related-ticket integration checkpoint can use the focused proof above. Fresh code review decides the measured amendment before landing.

## What the build taught us

The preflight identified raw nested metadata parsing and tight file headroom, but missed the library initializer of `Run` and the removal capability needed by `audit --write`. A complete mutation inventory needs every constructor of a shared carrier and each edit operation, as well as the command call path. A JSON parse/serialize test shortcut can reorder saved answer members and weaken an exact-byte proof; keep order-sensitive fixtures literal. No comparative timing evidence shows whether preparation shortened this build.
