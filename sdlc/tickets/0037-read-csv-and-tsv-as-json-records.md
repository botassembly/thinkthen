---
flow: build
priority: 50
opens: crates specification demos sdlc/planning sdlc/ratchet.json Cargo.toml Cargo.lock
---

# 0037: Read CSV and TSV as JSON records

Status: landed

## Outcome

Every record-capable command accepts explicit `--csv` and `--tsv` input. Each data row becomes an ordered JSON object whose keys come from the header. Every result remains JSONL. How-to 03 demonstrates CSV input and JSONL output.

## Current facts

The binary currently frames documents, lines, and JSONL before the core interprets a record. `filter` and `rank` preserve input bytes. The code has no maintained CSV parser dependency. ADR 0028 accepts CSV and TSV as input only, with a required header, string cells, empty strings for empty cells, and JSONL output. Existing recordings can serve how-to 03 when `/body` remains its evidence pointer. No paid call belongs in this ticket.

## Contract

- `--csv` and `--tsv` are mutually exclusive with each other, `--lines`, and `--jsonl`. The tool never guesses from a filename. CSV uses comma and TSV uses tab.
- The first logical row is the header. Empty input is exit 2. A header with no data rows is a successful empty dataset and makes no request. One leading UTF-8 byte-order mark is ignored only at the start of the first header cell.
- After removal of the optional byte-order mark, a header name is blank when `char::is_whitespace` covers every character and contains a control when any character satisfies `char::is_control`. Names are compared exactly and case-sensitively for uniqueness after byte-order-mark removal. Their original spelling, whitespace, case, and order otherwise become the JSON object keys.
- The maintained `csv-core` grammar owns double-quote escaping, delimiters, line feeds inside cells, LF and CRLF records, and irregular quote placement. The tool adds no stricter quote validator. Blank physical lines outside quoted fields are ignored. Every data row has exactly the header's field count; a trailing delimiter means an empty final cell.
- Every cell is a UTF-8 JSON string. Empty cells become `""`. The tool does not infer numbers, booleans, null, JSON, or arrays from cell text.
- The parser produces one compact JSON object per row in header order. `--field` is a JSON Pointer into that object. `choose --options` remains available only with JSONL because table cells stay strings.
- Judgment commands print one JSON value per row. `annotate` prints the parsed object with its added fields. `filter` and `rank` print selected or ordered compact JSON objects. Existing document, line, and JSONL output behavior does not change.
- `--details.input` contains the parsed object. `--dry-run` reports `framing` as `csv` or `tsv`, expands only the first data row, and needs neither a key nor a connection.
- A header or logical data row may contain at most 16 MiB of raw encoded bytes before its record terminator. Quoted line feeds and doubled quote bytes count; a leading byte-order mark counts. The record's LF or CRLF terminator does not count. An EOF-terminated record uses the same limit. The implementation uses bounded incremental parsing and never allocates or consumes an unbounded row. An oversized row ends the stream; its unread tail never becomes another row.
- A header error says `the CSV header ...` or `the TSV header ...`. A data-row error says `the CSV record ...` or `the TSV record ...`, and the established stopped-run line counts data rows from one; the header is not data record 1. These messages name the violated rule without echoing a header or cell value. Table-grammar refusals exit 2. Invalid UTF-8 in either header or data remains the settled local failure at exit 5.
- A maintained `csv-core` parser owns quoting and record boundaries at the binary edge. The pure core receives typed records and keeps filesystem and stream access out. No handwritten quote-state parser or second buffering parser enters.

## Acceptance

- Parser tests cover comma and tab separation, quoted delimiters, doubled quotes, quoted line feeds, irregular quote placement under `csv-core`, LF and CRLF, a leading byte-order mark, Unicode, empty and trailing cells, preserved whitespace, blank physical lines, and a header-only file.
- Exact-message tests reject empty input; blank, exact-duplicate, control, invalid UTF-8, and oversized headers; wrong field counts; and invalid UTF-8 and oversized data rows. Limit tests cover exactly 16 MiB and one byte more for headers and for multiline, escaped-quote, EOF-terminated, LF, and CRLF data rows. They prove bounded reader consumption and prove an oversized tail cannot become a record. A local listener proves a bad first data row sends no request.
- A compact command matrix covers `decide`, `choose`, `tag`, `score`, `filter`, `rank`, and `annotate`. It pins field lookup, details input, JSONL output, and input-order behavior with jobs. A slow earlier row, reverse reply completion, `--jobs 4`, and a bad later row prove that only the valid prefix prints in order, the bad row and every following row send no request, and the stopped-run summary counts data rows consistently.
- Dry-run tests pin both framing names and the first parsed object, need no key or connection, and prove that a malformed second logical row is not read.
- Compatibility tests pin unchanged line and JSONL requests and outputs. Every committed recording replays. How-to 03 runs with the key unset and no network.
- Dependency license and repository policy checks pass. The four repository rungs, `git diff --check`, push, and the remote check pass.

## Excluded and following order

Excluded: filename detection, configurable delimiters, headerless tables, type inference, JSON inside cells, CSV or TSV output, a table transform, and per-record tag labels. After this ticket, the plan resumes the small correction pass before `find`.

## Complexity

- Contract: 2
- State and timing: 2
- Reach: 1
- Proof: 2
- Cost of error: 1
- Total: 8
- Minimum level floor: level 3 for concurrency and partial failure
- Final level: 3
- Reasons: the public input grammar includes multiline logical rows and partial failure while the existing scheduler preserves ordered output. Hostile parser cases and unchanged existing framings require exact proof.
- Selected model: `gpt-5.6-sol` with medium reasoning.

## Review

- Design review: accepted after one revision by a separate Sol Medium agent. The first review required one feasible bounded parser design, the settled UTF-8 exit code, mechanical header rules, exact record numbering and concurrent failure proof, boundary tests, and the level 3 floor. The same reviewer confirmed that the revised contract fits ADR 0028, the record rules, the scheduler, and the library boundary.
- Code review: accepted after two remediation passes by a separate Sol Medium agent. The first pass fixed table `--raw`, compiled boundary and diagnostic proof, and stale `records.md` wording. The second aligned choose help and its specification with the table refusal and pinned the promise in a compiled help test. The reviewer then found no remaining contradiction or regression.
