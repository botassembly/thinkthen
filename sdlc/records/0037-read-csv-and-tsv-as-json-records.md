# 0037: Read CSV and TSV as JSON records

Ticket 0037 adds explicit `--csv` and `--tsv` input to every record-capable command. A required header names an ordered JSON object, every cell stays a string, and every result remains JSONL. How-to 03 now demonstrates CSV input through `filter` and reuses its existing recordings.

## What landed

The binary uses `csv-core` incrementally at the input edge. It accepts quoted delimiters, doubled quotes, quoted line feeds, LF and CRLF, Unicode, a leading byte-order mark, and empty cells. It preserves header spelling and order, refuses invalid headers and field counts without quoting input, and bounds each encoded header or logical row at 16 MiB while reading. An oversized tail cannot become another record.

Parsed rows enter the existing typed record and scheduling paths. `--field` addresses the parsed object. `--details` carries it under `input`. `annotate` appends answers to it. `filter` and `rank` emit compact JSON objects. Document, line, and JSONL behavior stays unchanged. `choose --options` remains JSONL-only, and table input refuses `choose --raw` because table results stay JSONL.

## Review and proof

The first end-to-end test failed because `--csv` did not exist. Design review rejected the first draft until it chose a feasible bounded parser, retained exit 5 for invalid UTF-8, defined header comparisons and record numbering, and required concurrent partial-failure proof.

Code review rejected two implementation rounds. The first found that `choose --raw` broke JSONL-only output, boundary failures lacked compiled-command proof, and two pointer paragraphs were stale. The second found that choose help still promised raw output under every framing. The remediation refuses raw table output before parsing or a request, pins safe exit codes and diagnostics through the compiled binary, covers exact and over-limit LF, CRLF, EOF, BOM, multiline, and escaped-quote cases, and makes help and specifications agree. The same independent reviewer accepted the final diff.

The table command matrix covers all seven commands. A jobs-four test reverses reply completion around a bad later row and proves ordered prefix output, stable stopped-record numbering, and no request for the bad row or its tail. Other tests cover dry run, broken pipes, secrecy, header-only input, request order at one job, details, replay compatibility, and unchanged existing framings. A one-time fixture initialization also removes a pre-existing test race exposed by the larger concurrent suite.

## Validation

All four repository rungs pass on the reviewed diff. Lint checks 93 resolved packages and the exact 21,084-line ratchet. The Rust suites pass 387 tests with no failure or ignored test, followed by the live-fixture suite. The specification rung passes 26 shell cases, 2 replay cases, all 16 replay checks, and 18 green how-tos. `git diff --check` passes. No paid call or external network call ran.

The coordinator has not committed or pushed this work yet.
