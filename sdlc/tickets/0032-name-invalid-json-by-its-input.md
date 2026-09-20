---
flow: build
priority: 46
opens: crates/thinkthen-core/src crates/thinkthen/tests/question_file crates/thinkthen/tests/backend/streaming.rs specification/question-file.md specification/records.md sdlc/ratchet.json
---

# 0032: Name invalid JSON by its input

Status: ready

## Outcome

A JSON syntax error names the input that owns it. A question file names the question file, one whole document names the input, and a JSON record keeps the record wording. Question-file and whole-document errors give a safe line and column without quoting input bytes.

## Current Facts

Finding 5 in `sdlc/issues/2026-09-19-hands-on-test-pass-one.md` records the wrong noun. On current main, an empty question file, a UTF-8 BOM before an otherwise valid question file, and a trailing comma each exit 5 with `the question file is not JSON this tool reads: the record is not valid JSON`. The same three inputs as a whole document under `--field` exit 2 with `the record is not valid JSON`.

The shared `Json::parse` turns every serde syntax error into unit variant `JsonError::Syntax`. That variant owns the record-specific sentence and discards serde's line and column. `QuestionFileError` adds the question-file prefix after the detail has already lost its owner. `Reading::record` still knows whether it is parsing a whole document or a JSONL record, but both currently use the same `RecordError::Json` path.

The observed serde positions are deterministic: empty input is line 1 column 0, a leading BOM is line 1 column 1, `{"decide":"q",}\n` is line 1 column 15, and `{"a":"x",}\n` is line 1 column 10.

## Scope

- Apply ADR 0026's safe JSON-location and input-owner decision.
- Make the shared JSON parser own syntax classification and the safe serde line and column only. It carries no serde message and no input bytes.
- Make `QuestionFileError` own this exact exit-5 sentence: `the question file is not valid JSON: the JSON at line LINE column COLUMN is not one`.
- Make the reading layer use its framing to give a whole document under `--field` this exact exit-2 sentence: `the input is not valid JSON: the JSON at line LINE column COLUMN is not one`.
- Keep malformed JSONL at exit 2 with the exact existing sentence `the record is not valid JSON`. Duplicate-member, non-finite-number, UTF-8, empty-evidence, pointer, and stopped-run behavior remain unchanged.
- Update `specification/question-file.md` and `specification/records.md` with the owner, location, safe-content, and status rules.

Excluded: accepting a BOM or trailing comma, repairing JSON, printing a serde message or input token, adding locations to JSONL errors, changing non-syntax JSON refusals, changing invalid-UTF-8 wording, and changing framing or request behavior.

## Acceptance

- Exact question-file integration cases cover empty bytes, a leading BOM before valid JSON, and `{"decide":"q",}\n`. Every case runs the real non-dry-run path with a key and a counted loopback listener. It exits 5 with the question-file sentence at 1:0, 1:1, or 1:15, prints no standard output, and opens zero connections and sends zero requests.
- Exact one-document `--field` integration cases cover empty bytes, a leading BOM before valid JSON, and `{"a":"x",}\n`, from standard input and from `--input FILE`. Every case runs the real non-dry-run path with a key and a counted loopback listener. It exits 2 with the input sentence at 1:0, 1:1, or 1:10, prints no standard output, and opens zero connections and sends zero requests.
- Fixed malformed question-file and whole-document fixtures contain a private marker before the syntax error. Neither diagnostic nor debug rendering repeats the marker, the invalid token, or any other input bytes. Unit tests pin that syntax errors retain only line and column.
- Existing JSONL refusal coverage passes unchanged and pins `the record is not valid JSON`, exit 2, the stopped-record line where applicable, no failed-record request, and no evidence echo.
- Existing duplicate-name, non-finite-number, UTF-8, question-file grammar, and shared secrecy coverage remains green. The ratchet equals the measured total, and the whole ladder passes with the key and base address unset.

## Dependencies

ADR 0026, decided with this ticket. ADR 0013 establishes `@FILE` as JSON. ADR 0007 assigns local-file and input exit classes, and ADR 0008 item 5, accepted by ADR 0010, fixes failed-record behavior. Ticket 0031 is the landed sequencing predecessor in the plan and is not a technical dependency.

## Complexity

- Contract score: 2
- State and timing score: 0
- Reach score: 1
- Proof score: 2
- Cost of error score: 1
- Total: 6
- Minimum level floor: none
- Final level: 3
- Reasons: ADR 0026 fixes exact public diagnostics across three input owners and two exit classes; the change crosses the shared JSON parser, question-file errors, record framing, integration tests, and two public pages; proof uses exact byte fixtures, safe locations, both document input homes, real keyed paths, connection and request counts, and no-echo checks; a wrong message misdirects a user but is locally correctable.
- Selected model: `gpt-5.6-sol` with medium reasoning

## Review

- Design review: accepted after ADR 0026, counted keyed listener cases, and the level 3 Sol Medium route were added. The reviewer confirmed the three owners, exact statuses and messages, location-only parser data, secrecy boundary, and unchanged JSONL behavior.
- Code review: pending.
