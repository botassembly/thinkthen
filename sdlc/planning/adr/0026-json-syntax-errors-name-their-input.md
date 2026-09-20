# ADR 0026: JSON syntax errors name their input

- Status: Decided by the agent on 2026-09-20. Ian can overturn any line
- Date: 2026-09-20

ADR 0013 makes `@FILE` a JSON question file. ADR 0007 assigns local-file failures to exit 5 and input errors to exit 2. ADR 0008 item 5, accepted by ADR 0010, keeps a failed JSONL record at exit 2 before its request. The shared JSON parser currently calls every syntax failure a record and discards serde's safe line and column, so its messages do not follow those owners.

Serde's full error text can quote an input token. Question files, whole input, and records may contain private evidence. A line and column tell the user where to look without carrying any input byte into a diagnostic.

## Decision

- The shared JSON parser carries syntax classification, line, and column only. It carries no serde message and no input text or bytes.
- A question-file syntax error is exit 5 and says exactly: `the question file is not valid JSON: the JSON at line LINE column COLUMN is not one`.
- A whole-input syntax error under `--field` is exit 2 and says exactly: `the input is not valid JSON: the JSON at line LINE column COLUMN is not one`. Standard input and `--input FILE` use the same sentence.
- A JSONL syntax error remains exit 2 and keeps the exact sentence `the record is not valid JSON`, with no location. The stopped-record line remains unchanged.
- Duplicate-name, non-finite-number, invalid-UTF-8, pointer, framing, and request behavior stay unchanged.

## Consequences

The question-file and reading layers supply the noun because they know which input they own. The parser supplies only a safe place. JSONL stays terse because its existing record wording and stopped-record position already identify the failed input. Exact tests use a real non-dry-run path, a key, and a counted loopback listener to prove every syntax refusal opens no connection and sends no request.
