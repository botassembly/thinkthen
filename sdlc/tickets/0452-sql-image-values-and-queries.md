# 0452: Carry typed image values through SQL

Status: ready. Planning only; implementation follows accepted ticket review.

Milestone: 0.2
Owner: builder.

## Outcome

DuckDB, SQLite and PostgreSQL accept explicit image values and ordered multi-image questions for decide/choose/score with complete results, facts and zero-send replay.

## Evidence

- Starts from: Main 399c6c7c7 and the existing SDK plan. PM message `2026-10-06-pm-vision-ships-in-0-2-and-the-sdk-stays-one-endpoint-while-the-proxy-owns-the.md`, ask 1; experiment 0034 design, recorded spike at 8ec4fbffc0dbf8ceaa0fea5607749ff2f683a2a0 and subsequent OpenRouter controls. 0036 reports are pending.
- Keeps: Existing text behavior, typed SDK parity, six errors, cancellation, secrecy, spend limits and zero-send strict replay. Core remains free of I/O; the one Rust engine and native file reader remain shared.
- Changes: Add reviewed image constructors and judgment overloads using the shared native engine. Proposed thinkthen_image(bytes,mime) plus ordered image-list input: validated DuckDB struct, PostgreSQL composite and SQLite tagged value revalidated after storage. Ordinary BLOB/bytea/text/path values retain current meaning.
- Proof: Public SQL consumers store and reload two-image values, preserve order/duplicates, execute named scalar/image-file routes and compare shared answers/details/facts. Test malformed tags/media, SQL NULL/empty values, implicit-BLOB refusal, text-only function refusals and zero-send limits/replay.
- Defers: Proxy business logic and screens, other modalities and unmeasured function combinations.

## Dependencies and ownership

0447 native input/reader and 0448 admission first. Coordinate 0434 input grammar and 0435 facts in the same SQL folders; no competing parsers. DuckDB/SQLite image readers preserve host file permissions. PostgreSQL uses the 0434 client-reader/keyed-table workaround; it adds no server image-path reader.

## Design notes

Record exact SQL public signatures and null semantics before code. Image-file results preserve file identity and absent text positions. Never resolve an arbitrary SQL path or URL as an image. SQL fact objects come from the same call, never a second judgment.
