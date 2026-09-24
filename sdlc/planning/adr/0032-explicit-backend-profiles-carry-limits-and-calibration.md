# ADR 0032: Explicit backend profiles carry limits and calibration

- Status: Accepted for ticket 0059
- Date: 2026-09-21

## Context

ADR 0010 removed configuration profiles because they repeated the address, model, adapter, and key settings already available at the command edge. A second server later spoke the same System One wire shape but enforced much smaller limits. The command could only send an oversized request, pay for the refusal, and hide the backend's response body for secrecy. Measurements also showed that a threshold tuned under one backend cannot be assumed safe under another.

## Decision

`--profile FILE` reads one explicit JSON file. The profile gives the backend a safe public name and one or more locally enforceable limits. It selects no address, model, adapter, key, cache, retry, timeout, or width. Existing options and environment variables keep those jobs.

The closed schema is `thinkthen.backend-profile/1`. `name` uses lowercase letters, digits, hyphens, and underscores. `max_evidence_bytes`, `max_request_bytes`, and `max_questions` are independent optional positive integers. At least one is present. Byte limits count UTF-8 evidence bytes after record selection and the exact encoded request body. Question limits count the wire questions after tag expansion. The tool estimates no tokens.

The engine prepares the exact request once and checks the profile before replay, cache locking, key access, or transport. Grouped annotate prepares and checks every group for one record before any group starts. Dry runs perform the same checks before printing a plan.

A single question file or a question set may name one top-level `profile`. That name identifies the backend used to tune its threshold. Nested questions in a set cannot name another. When the saved and selected names both exist and differ, the command prints one fixed warning and detailed output carries `meta.profile_warning` with `calibrated` and `running`. A missing name on either side warns nobody. Calibration identity enters the question or question-set digest. Selecting a run profile changes no request or recording digest.

The ordered output boundary prints the warning when it handles the first successful logical result, before it decides whether `filter` emits that record. A successful run warns even when `filter` rejects every record. A failure on logical record one prints no warning, even if a later parallel worker completed successfully. A dry run prints the warning immediately before its plan.

The option appears in short help. A profile can prevent evidence from reaching a backend and can prevent a paid refusal, so a new user should see it beside `--url`.

## Consequences

This decision supersedes only ADR 0010's removal of the old `--profile` name. It does not restore configuration discovery or backend selection. A user names every profile file. A backend with only a token limit needs a tokenizer or a verified byte ceiling before this schema can enforce it. The repository publishes no invented conversion.

Ian can overturn the file shape, name grammar, units, warning rule, and metadata shape.

## Amendment, 2026-09-24, by ticket 0090

Ian ruled on 2026-09-23 that `meta.profile_warning.calibrated` becomes `tuned_for`. The saved name identifies the profile under which a person tuned the threshold, and it proves no statistical calibration. The metadata value is now exactly `{"tuned_for":NAME,"running":NAME}`. The standard-error warning reads `threshold tuned for profile X is running under profile Y`. The question-file key stays `profile`. No reader accepts the old key, because ThinkThen has not released 0.1.
