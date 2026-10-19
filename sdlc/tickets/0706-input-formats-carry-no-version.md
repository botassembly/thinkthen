# 0706 — Input formats carry no version

Status: OPEN.

Milestone: 0.2.1

Depends on: 0704

## Outcome

- Changes: `crates/thinkthen/src/core/question_set/`, `crates/thinkthen/src/core/recognize_file.rs`, `crates/thinkthen/src/core/relate_file.rs`, `crates/thinkthen/src/core/backend_profile.rs`, `crates/thinkthen/src/config.rs`, `crates/thinkthen/src/public/request.rs`, `specification/`, `site/src/`, `conformance/`, `databases/`, `libraries/`, `build-tools/`.
- Question sets, recognize files and relate files have no `version` member. A file that still has one is refused with one clear sentence naming the member to delete. The question-set hash keeps its current constant, so `questions_sha256` values do not change.
- Configuration files have no `schema` member (`thinkthen.config/1`); a present one is refused the same way.
- Backend profiles are one unversioned shape with an optional `image_profile`; `thinkthen.backend-profile/1` and `/2` and their `schema` member are gone.
- `thinkthen.request/1` and `RequestVersion` are gone from the request contract, the public Rust API, the request schema, MCP tool schemas and every generated wrapper. The API inventory and contract lists change in the same commit.
- Docs, site examples, spec fixtures and SQL examples drop these members.

### Proof

- Rust tests: each of the four inputs parses without the member, and each refuses a file that still carries it, with the exact sentence. They fail before the change.
- `sdlc/scripts/spec` and the routine conformance cases pass on the M5; the generated wrappers regenerate cleanly.

## Evidence

Ruling: [no version numbers before 1.0](../decisions/2026-10-19-no-version-numbers-before-1-0.md). Each of these inputs accepts exactly one value today (`profile` accepts 1 and 2, where 2 is 1 plus an optional member), so the marker guards nothing and users must write it. Found in the 2026-10-19 version survey: `core/question_set/parsing.rs`, `core/recognize_file.rs`, `core/relate_file.rs`, `config.rs`, `core/backend_profile.rs`, `public/request.rs`. 0704 made the question-set `version` optional; this ticket removes it.

## Amendment, 2026-10-19

Ian ruled that `wording_version` does not belong in the SDK: the SDK asks the question directly, and tracking wordings of a question belongs to the proxy, where question optimization will live. This ticket also deletes `wording_version` from question files, the public Rust API (`with_wording_version`), the C interface (`thinkthen_complete_wording_version_v1`), results, generated wrappers, the specification and fixtures. A question file that still carries it is refused with the same one-sentence rule as `version`.
