# The library drops a question file's calibration profile

Status: Open pending 0203 review and landing. Filed 2026-09-26 by the queue owner from local experiment 273, report 04, finding I1. Ticket 0148's settings sweep found the same gap and promised this file. It is filed now so the gap is findable before 0148 lands. Blocks 0.1 under goal 3: a library must match the command. 0148 says the fix waits for files ticket 0146 holds.

## What happens

A question file may carry a calibration `profile`, such as `{"decide": "...", "profile": "jev-bench"}`. The command puts that name into `question_sha256` and warns when a tuned threshold runs on another backend. The libraries and SQL surfaces accept the key and drop it.

The report ran one question file on three surfaces. The command gave `question_sha256` `6a0e5df25f5e...`. Python and DuckDB gave `968a10461272...`. Without `profile`, all three gave `968a1046...`.

- Audit rows or dashboards joined on `question_sha256` split one question into two across surfaces.
- The calibration warning never fires in a library or database.

## Checked on main

Verified by reading the code: `crates/thinkthen/src/public/results.rs:211` sets `tuned_for: None`, and line 228 computes the digest with `question_sha256_with_profile(question, threshold, None)`. The digests come from the report.

## What would fix it

Carry the calibration name through the public question type into the digest and the warning. Add a shared conformance case with `profile` set, so every surface must give the command's digest.

Ticket 0203 now has a frozen source candidate with one independently pinned question digest, a set digest, selected command, public, library and SQL details checks, and zero-send refusal checks. Its fresh code review is pending. Linux x86-64 DuckDB uses the staged C++ extension; the three retained C API platform packages still await their separate 0201 migration, so this issue remains open for that platform scope after the candidate lands.

## Done when

A question file with `profile` gives the same `question_sha256` and the same warning on the command, every library and every SQL surface, and a conformance case pins it.

When 0148 lands, its lander links this file and does not add a second copy.

## Resolution

The shared public question carries its saved calibration name to the canonical digest and structured mismatch warning. Library entry points preserve it or refuse unsupported combinations before sending. A fixed shared fixture and selected command, Rust, Polars, C, Python, TypeScript, Ruby, R, SQLite, PostgreSQL and Linux x86-64 DuckDB checks passed. See [the reviewed build record](../../records/0203-build.md). The three retained DuckDB C API package migrations and their missing SQL settings remain in the separate 0201, 0149 and 0157 work; this closure does not claim those platform builds or installed artifacts.
