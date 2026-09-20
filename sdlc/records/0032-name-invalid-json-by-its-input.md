# 0032: Name invalid JSON by its input

Branch `ticket/0032-name-json-source`. Built 2026-09-20.

## What landed

JSON syntax errors now name the input owner. A question file exits 5 and names the question file. A whole document under `--field` exits 2 and names the input. Both report serde's line and column. JSONL retains the existing exit-2 sentence `the record is not valid JSON` without a location.

The shared parser retains only syntax classification, line, and column. It retains no serde message or input bytes. The question-file and reading layers choose the public sentence from the input framing they own.

## Red then green

The first core test failed because `JsonError::Syntax` carried no location and `RecordError` had no whole-input syntax variant. The parser now carries only the safe location. The reading layer maps document syntax to `RecordError::InputJson` and keeps JSONL on its existing error path.

Integration cases cover empty input, a leading BOM, and a trailing comma for question files, standard-input documents, and `--input` documents. Each case runs through a keyed non-dry path with a counted local listener and proves zero connections and zero requests. A malformed first JSONL record has the same proof. Marker checks show that diagnostics and debug output repeat no input bytes.

Existing duplicate-name, non-finite-number, UTF-8, question-file grammar, stopped-run, and secrecy suites pass. The new backend cases live in `json_syntax.rs` because `streaming.rs` is at the 500-line ceiling.

## Review

The design reviewer first required a durable ADR and real keyed listener proof. ADR 0026 now fixes the three owner-specific contracts, and every syntax case proves that refusal happens before a connection or request.

The code reviewer found no defect. It accepted the new public `RecordError::InputJson` variant as the smallest local typed boundary. `Reading::record` owns document-versus-JSONL framing, and the crate remains unpublished at version 0.0.1.

## Choices made where the ticket was silent

Ian can overturn this choice.

- **The public record error gains one typed variant.** This keeps input ownership in the reading layer and keeps formatting out of the binary. Avoiding it would require a broader public API change or erase typed state.

## Gates and size

The source ceiling is 17,289 measured Rust lines, up from 17,036. The increase is the owner-specific error paths and the required live-path, location, secrecy, and regression proof. Shared parsing remains one implementation, and the new integration file avoids growing a file past the repository limit.

| Rung | Result |
| --- | --- |
| `sdlc/scripts/install` | exit 0 |
| `sdlc/scripts/lint` | exit 0, ratchet `17289/17289` |
| `sdlc/scripts/test` | exit 0 |
| `sdlc/scripts/spec` | exit 0 |

The coordinator ran the ladder with the key and base address unset. No live call ran.
