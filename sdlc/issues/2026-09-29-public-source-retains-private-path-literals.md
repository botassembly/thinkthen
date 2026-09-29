# Public source retains private path literals

Status: Open. Confirmed by the integrated lint checkpoint for ticket 0275 on 2026-09-29.

The configured private-name guard refuses 49 tracked locations. Most are machine-specific home paths in package privacy checks and historical evidence references; three are private repository references in issue records. The guard reports only file and line. The external name list and its contents must stay outside this repository.

## Outcome and proof

Remove the private literals from tracked source and records without weakening the scanner or package privacy checks. Replace machine-specific package rejection strings with appropriate generic home-path checks, preserving existing negative fixtures and installed-consumer receipts. Use portable workspace-relative evidence references in records and a generic owner description for the private repository. Preserve commit identities, measured results and original issue criteria.

Use the saved location list, targeted privacy/guard fixtures and the existing configured private-name scan as the smallest proof. Do not rebuild every language or start a package campaign. This source-hygiene correction changes no SQL/DataFrame behavior or validation. The held DuckDB child-environment finding remains separate, and passing this scan alone does not establish full lint green.
