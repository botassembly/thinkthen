# 0588: Test a surgical spreadsheet writer

Ticket: 0588
Date: 2026-10-14

Local review: accept. A fresh reviewer checked 31 output archives, 375 preserved ZIP members, 2,482 unchanged existing cells, 1,554 appended cells, all required feature tags, 62 clean SDK results, 31 successful Calc comparisons, macro provenance and delivery hashes. The repaired table-scoping regression reproduces in the retained bytes. The reviewer accepted the stated measurement limits and required the ticket to stay open for Ian's Excel for the web check.

The surgical writer approach passed the bounded generated corpus. This supports keeping the approach eligible for 0.3. It does not establish safety for arbitrary existing workbooks. Excel for the web remains Ian's check. No product code, repository status, branch, release artifact, model call or hosted file changed.

## Identity and scope

Owner: thinkthen/0588. Beelink. ThinkThen base revision: 0e46dc2daf53bba7ad40165e67068948607f7bec. The read-only checkout was /home/ian/workspace/worktrees/thinkthen-03-experiments.

Evidence folder: /home/ian/workspace/experiments/484-spreadsheet-writer. The final corpus is corpus2/, not the earlier originals/, outputs/ or rust-outputs/ attempts. Sources were PRD 08, the ThinkThen ideal state, experiment 229, and the CSV/TSV reader source. Earlier experiment 229 investigated add-ins and did not supply workbook round-trip evidence.

The final Rust prototype uses Rust 1.95.0, zip 0.6.6 and quick-xml 0.39.4, resolved offline from cached crates. Cargo.lock records the complete versions. Open XML SDK 3.3.0 and its dependencies were fetched into tool-cache/nuget. LibreOffice Calc 24.2.7.2, Ubuntu package 4:24.2.7-0ubuntu0.24.04.7, was installed only in disposable Ubuntu containers. Every container had --rm and labels pm.repo=thinkthen and pm.ticket=0588. Offline prototype/SDK checks had no network, one CPU, 768 MiB memory and 128 processes. Disposable package-install/Calc containers had one CPU, 1 GiB memory and 192 processes. No global packages were installed.

## What the Rust writer does

It streams selected worksheet XML through quick-xml, appends a boolean review column and a text kind column, extends the worksheet dimension and table definitions, and copies every untouched ZIP member with raw_copy_file. New strings are inline strings, so the existing shared-string table remains untouched. Existing formulas and saved results remain untouched. Generated answers alternate booleans by row and contain the literal generated; no model supplies them.

## Final corpus and results

All 375 untouched ZIP members passed raw-byte comparison across the final corpus. Thirty generated feature combinations plus one other-sheet-table regression contain combinations of tables, charts, comments, conditional formats, data validation, merged cells, defined names, frozen panes, multiple sheets, inline and shared strings, ordinary and shared formulas with cached results, and generated pivot definitions/cache/records. One .xlsm includes the genuine XlsxWriter example vbaProject.bin. Its BSD-2-Clause license and source URLs are retained. The macro binary was never executed. These are variations on a synthetic template, not thirty independent customer workbooks.

- All 31 Rust output packages passed CRC checks. Every member outside the chosen worksheet and its table parts is identical, including its original local ZIP header and compressed bytes. Existing cells in the selected sheet remain identical under XML parsing, and appended typed values match the generated rule. corpus2/preservation.json identifies every preserved member.
- Microsoft's Open XML SDK 3.3.0 reported zero errors on all 31 final originals and all 31 final outputs. corpus2/sdk-validation-final.jsonl contains every final diagnostic result. This is stronger than merely inheriting the same original errors.
- LibreOffice Calc opened and converted all 31 originals and 31 outputs without a reported loading/conversion error. Every original cell's converted value equals the corresponding output's converted value, including cells on other sheets. New values match the generator. No converted workbook contains a formula-error cell. corpus2/office-results.json and office-originals.log/office-outputs.log hold the evidence. Conversion copies are separate from the byte-preserving deliverables. XLSM-to-XLSX conversion was for validation; the delivered XLSM retains its original macro bytes.
- Streaming verification read the complete million-row Rust output, checked five million cell values against generated expectations, and retained 13,992 KiB peak process RSS. This checks generated values and XML completeness without loading a worksheet tree.

## Size measurements

Each measurement is one local sample with warm caches, one capped CPU and no model. It is not a throughput promise. Row count includes the first row. Large inputs contain three original cells per row and outputs contain five. Plain copy compares the same input archive without parsing, rewriting or adding columns.

| Rows | Input bytes | Plain copy seconds | Rust rewrite seconds | Rust child peak RSS KiB | Stream verification seconds | Verification peak RSS KiB |
| --- | ---: | ---: | ---: | ---: | ---: | ---: |
| 10,000 | 194,345 | 0.000993 | 0.320384 | 13,392 | 0.317057 | 13,784 |
| 100,000 | 1,953,863 | 0.003073 | 2.648463 | 13,432 | 3.183217 | 13,860 |
| 1,000,000 | 19,659,928 | 0.329816 | 25.631964 | 13,556 | 25.074069 | 13,992 |

RSS stayed flat over a hundredfold row increase. Rust child RSS is measured by resource.getrusage(RUSAGE_CHILDREN) in a fresh Python parent for each size; inherited pre-exec parent RSS can set a floor, so this is an upper bound on the observed child high-water mark rather than exact allocator usage. The writer parses and rewrites the complete sheet; copy only moves compressed bytes. Exact final-write/copy measurements are scaling/final-{copy,rust}-*.json; independent streamed-value verification is scaling/rust-read-*.json. The last relationship-scoping fix produced byte-identical output on the earlier 30-file corpus. Earlier scaling samples are retained. The final measurements overlapped disposable Calc dependency setup, so disk and CPU contention were not controlled. The earlier Python algorithm probe is retained separately and is not substituted for Rust evidence.

## Limits and remaining proof

The prototype is deliberately fixed to xl/worksheets/sheet1.xml and three existing columns followed by D/E. It does not implement CLI behavior, worksheet-name resolution, arbitrary append positions, filled-cell handling, backups, cancellation, atomic replacement, duplicate header refusal, evidence display formatting or answer resumption. Those product contracts were outside this experiment.

The final writer resolves table relationships from the selected worksheet, normalizes relative/absolute package targets, rejects external table relationships and restricts table edits to those parts. The negative test reproduced unrelated table damage before this fix. The repaired regression changes only the chosen worksheet and preserves the unrelated table byte-for-byte. Both the damaged and repaired files pass SDK validation, which shows why raw preservation checks matter independently. table-scope/ retains the before/after binary, packages, diagnostics and result. The prototype still assumes canonical unprefixed nonempty row elements and fixed table shape. Existing output-column collisions, sparse or self-closing rows, namespace-prefix variants, unusual extensions, signatures, encrypted workbooks, tables with totals, slicers, external links, damaged ZIP inputs and near-format-limit files were not tested. No arbitrary-workbook safety claim follows.

Pivots and their cached records survived unchanged as package parts; this does not prove pivot refresh behavior or visual fidelity. The macro project survived byte-for-byte; macro execution and Excel desktop behavior were not tested. Headless conversion cannot prove absence of an Excel repair prompt or identical chart/layout rendering. The required free Excel for the web opening remains an external check. The report does not claim Windows/Mac desktop Excel acceptance.

## Deliverables for the coordinator

thinkthen-spreadsheet-experiment-outputs.zip contains only the 31 final workbooks, macro source/license information, feature list and output checksums. No build, cache or intermediate workbook is in it. Its .xlsm and .xlsx files are the original Rust outputs, not LibreOffice rewrites.

Suggested first openings in Excel for the web:

1. corpus2/outputs/fixture-00.xlsm: table, chart, macro project, pivot/cache, shared strings, shared formulas, defined name and merged note on the other sheet.
2. corpus2/outputs/fixture-03.xlsx: shared strings, chart and merged cells on the selected sheet.
3. corpus2/outputs/fixture-30-other-sheet-table.xlsx: an independent table and pivot on the other sheet, which exercise the table-scoping regression.

Expect D1 review and E1 kind on Intake, boolean values down D and generated down E. Compare the corresponding corpus2/originals files when checking original layout. Open the full delivered set for the complete manual corpus check. The macro needs no execution for this inspection.

## Attempts and retained evidence

The first Python attempt exposed missing cached formula values from the generator, which used a self-closing value tag; the generated cache patch was corrected. Initial openpyxl files also had SDK style ordering/comment-attribute diagnostics. Final corpus generation normalizes only the originals before writing, and the final originals and outputs are both SDK-clean. Earlier logs remain available.

The host had LibreOffice Writer but no Calc. Read-only host-tool containers therefore could not load even originals. An initial folder-local apt download hit stale-package 404s. Refreshing apt metadata inside a disposable Ubuntu container supplied Calc without touching the host. corpus2/sdk-attempt*.jsonl preserves the generator namespace/pivot-structure failures before correction. The final SDK and Calc logs are unambiguous and separate.

Source licenses: zip 0.6.6 and quick-xml 0.39.4 are MIT; Microsoft Open XML SDK 3.3.0 is MIT; openpyxl is MIT. The authentic macro fixture is BSD-2-Clause, not MIT. Cargo.lock, the downloaded NuGet metadata, apt install logs and corpus2/source-rights/ retain dependency/source details.

All evidence, scripts, package caches, build output and workbooks are retained pending fresh review. No experiment evidence was deleted. Docker removed disposable container layers when they exited. No remaining owned process is required.

## Delivery and retention

The reviewed local finding is ready to record. Keep 0588 open until Ian reports the required Excel for the web opening check; local SDK/Calc checks do not settle an Excel repair prompt. Retain the output ZIP, original comparison files, source license, scripts and raw results for that check. Regenerable compiler output, package caches and scaling copies can be removed after review. No product-readiness claim follows.

After acceptance, the coordinator removed compiler output, SDK package caches, obsolete workbook attempts, converted copies and scaling workbooks. The retained folder is 1.7 MB and holds final originals and outputs, the delivery ZIP, licenses, sources and lockfiles, raw results and logs, and the compact table-scope regression. Those files remain owned by open ticket 0588 for the manual check. Delivery ZIP SHA-256 is `2c6f2cb2297fac51388e8cb73ca1f2fc7d8e8be9ecbd527f091aadf7edb9de07`.
