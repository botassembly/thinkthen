---
flow: build
priority: 98
opens: crates/thinkthen/src crates/thinkthen/tests sdlc/scripts sdlc/ratchet.json
---

# 0098: Build the binding members

Status: design accepted 2026-09-24 after re-review. Owner: Claude.

Split out of 0086 on 2026-09-24 so 0086 fits its budget (`sdlc/records/2026-09-24-spine-review-engine.md`, finding F7). It lands after 0086 and before 0093.

## Outcome and authority

Build the public members that ticket 0095 defines and 0086 leaves out: `QuestionKind`, `Question::kind`, `QuestionSet::members`, `LabelBuilder` with `choose_labels` and `tag_labels`, `Recognize` and `Relate` `from_json` and `load`, `Row::probability`, `ErrorKind::name`, and the four JSON methods. Each delegates to the existing owner: the core question, recognize, and relate parsers, the 0085 facade, and the command's one result serializer. The proposed ADR 0017 amendment on the 0084 branch records these as supporting forms. Ian can overturn any member.

## Design

- The command's result serializer moves out of `cli` into one private shared module. `Details::to_json`, `AnnotatedRecord::value_json`, `Recognized::to_json`, and `Edge::to_json` call it, and so does the command.
- `from_json` and `load` call the 0080 and 0081 parsers of the `recognize @FILE` form and the version-one relate file. A non-default `fields` pointer is `Usage`.
- `LabelBuilder` reuses the typed choose and tag builders' checks and yields an unbound `Question`.
- No new parser, serializer, request path, or dependency.

## Acceptance

- Each JSON method equals the command's `--details` or bare bytes on the shared cases (G8).
- A `choose_labels` question equals `Question::from_json` of the same file. `details` over it returns the typed call's value with no added send on a counted listener (G5).
- `Relate::from_json` with `"either":true` on two kinds equals the `both_ways` builder, digest included. `Recognize` and `Relate` report a broken rule as `Usage` from `from_json` and as `Local` from `load`.
- `QuestionSet::members` follows set order, and a banded member reads `Decide`. `Row::probability` equals the yes probability `details` returns for the same record. `ErrorKind::name` returns the six conformance words.
- The public inventory check now covers 0084 and all of 0095. A planted extra export fails it.
- Planted-bug proof: the record plants a serializer that drops `meta.requests_sent` and a label builder that skips the order check, and shows the tests turning red.
- Focused tests, policy, exact ratchet, formatting, Clippy, and `git diff --check` pass. The coordinator runs `install`, `lint`, `test`, and `spec` in order. No non-loopback socket and no paid call.

## Scope

At most ten production files and 500 nonblank production lines, net of the serializer lines that leave `cli`; at most 700 test lines. No dependency.

## Dependencies

After 0086. Before 0093, 0094, and every surface ticket.

## Routing

Builder: Claude (Opus subagent). Reviewer: a fresh Claude session for design and for code.

## Review

- Design review: accepted in `sdlc/records/2026-09-24-rereview-engine.md`.
- Code review: pending.
