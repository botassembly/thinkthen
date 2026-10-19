# 0706: Question, recognize and relate files carry no version

Status: WITHDRAWN.

Opened as: 2026-10-19, superseded by 0719.

Milestone: 0.2.1

Phase: 2

Depends on: 0704

Superseded by: 0719

## Outcome

- Changes: none. Every part of this ticket now belongs to another ticket, listed below, and this ticket only waits for the PM to withdraw it.
- 0704 removes `version` from question sets, recognize files and relate files.
- 0719 removes `wording_version` from question files, SQL declarations, the public Rust API, the C interface, results and wrappers, and removes `thinkthen.request/1` and `RequestVersion`.
- 0720 removes the configuration file `schema` member.
- 0721 merges the backend profile into one shape without `schema`.

### Proof

- The proofs live in 0704, 0719, 0720 and 0721. This ticket closes by withdrawal, superseded by those four.

## Evidence

Ruling: [no version numbers before 1.0](../decisions/2026-10-19-no-version-numbers-before-1-0.md). The 2026-10-19 preparation split this ticket into 0719, 0720 and 0721 and kept the file `version` members here. The coordinator's second amendment below then moved those file members into 0704. Nothing remains that one of those four tickets does not own.

## Amendment, 2026-10-19

Ian ruled that `wording_version` does not belong in the SDK: the SDK asks the question directly, and tracking wordings of a question belongs to the proxy, where question optimization will live. This ticket also deletes `wording_version` from question files, the public Rust API (`with_wording_version`), the C interface (`thinkthen_complete_wording_version_v1`), results, generated wrappers, the specification and fixtures. A question file that still carries it is refused with the same one-sentence rule as `version`.

## Amendment, 2026-10-19

The question-set, recognize-file and relate-file `version` removal moved to 0704, which finishes it once. 0706 keeps the configuration `schema`, the merged backend profile, `thinkthen.request/1` and `wording_version`.
