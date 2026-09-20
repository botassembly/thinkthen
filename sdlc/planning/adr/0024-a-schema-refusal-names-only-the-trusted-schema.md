# ADR 0024: A schema refusal names only the trusted schema

- Status: Decided by the agent on 2026-09-20. Ian can overturn any line
- Date: 2026-09-20

`specification/recording.md` says that refusing an entry with another schema names no schema. The implemented refusal names `thinkthen.recording/1`, the fixed schema this version reads, and never repeats the schema read from the entry. The page therefore forbids a safe and useful part of the existing message.

A schema field comes from an untrusted recording file. It can be unbounded, contain control bytes, or contain private evidence. The program's own fixed schema is trusted text and tells the user which format this version accepts.

## Decision

- A foreign-schema refusal may name only the trusted fixed schema `thinkthen.recording/1`.
- The refusal never repeats, quotes, or otherwise derives text from the schema field in the entry.
- The existing exit 5 and sentence stay unchanged: the entry names a schema this version does not read, and this version reads `thinkthen.recording/1`.

## Consequences

The diagnostic remains actionable without printing untrusted file contents. The recording page and core error comment distinguish the trusted fixed schema from the refused entry's schema. Existing exact hostile-entry unit and integration tests continue to prove the boundary.
