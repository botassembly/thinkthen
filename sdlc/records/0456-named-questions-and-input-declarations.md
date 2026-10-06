# 0456: Named questions and declared inputs

Date: 2026-10-06
Status: in progress. Slice A records the accepted design and 0.2 completion plan; native implementation and surface adoption remain open.

The shared design adds optional name, wording_version, item_schema and context_schema while preserving old files and paths. Explicit named loading reuses platform config roots and the native reader. Admission validates the actual selected item and separate context before lookup or sending; streaming keeps prior completed work. Existing 0407/0414 remain required. No proxy catalog or routing enters the SDK.

Fresh High review found one defect: annotate JSON documents were described as strings. The correction preserves typed JSON and member projection; the reviewer confirmed ACCEPT. Ticket, private-name and diff checks passed. Full tests and lint run on the slice A landing commit. An initial full run timed out on the existing image-input signal acknowledgment test; the same test passed in isolation, so the full suite runs again. No cancellation check was weakened. No product code or release qualification is claimed by this slice.

The plan also records full Linux/hosted macOS gates, Windows qualification, docs-only image/MCP use, old-file/cache compatibility and publication-dependent package/0035 checks. Ian retains publication authority. Later implementation updates this same record.

## Native foundation slice B

Explicit native named loading, author metadata, typed object context and bounded declaration admission are included with 0443 slice A. Metadata stays outside semantic identity. Corrected EOF deadline behavior preserves completed prefixes and reports pending input; the original reviewer accepted its prior-failing regression. Complete native role loaders, CLI serialization and host adoption remain open. See the single 0443 record for integrated checks.
