# 0744 — Patch the shipped TLS certificate dependencies

Status: OPEN.

Milestone: 0.2.1

## Outcome

The shipped TLS dependency closure uses patched certificate verification and trust roots for CVE-2026-111943, without changing public APIs, providers, TLS settings or advisory policy.

Changes: `**/Cargo.lock`, `Cargo.lock`. Change a manifest only if the existing compatible constraints cannot resolve the patched versions, after identifying that requirement.

- Update rustls-webpki 0.103.15 to the compatible patched 0.103.16 and webpki-roots 1.0.9 to 1.0.10 through the existing Cargo workflow, after confirming the advisory and compatibility.
- Search every tracked lock for the affected dependencies and repair the whole shipped closure, including library and database packages.
- Keep advisory ignores, checks, assertions and deadlines unchanged; add no tool or workaround.

## Evidence

- Gator checkpoint 0691/1792522536190-1088521 on 52a2e2c48 failed before package with cargo deny advisories exit 1; the normal resolver discarded the child diagnostic.
- Read-only inspection of advisory database commit 5502dedfaf87df1ef64175116bf516640aab2a7c found CVE-2026-111943 entries for rustls-webpki and webpki-roots, whose patched compatible versions are 0.103.16 and 1.0.10 respectively.
- The production ureq rustls path and fifteen tracked locks contain the affected pair; confirm the full mapping before changing it.
- Proof commands, with SHA replaced by the pushed repair revision: `pm job run --machine m5 0744 SHA --disk 20 --memory 8 -- sdlc/scripts/job sdlc/scripts/lint` and `pm job run --machine m5 0744 SHA --disk 20 --memory 8 -- sdlc/scripts/gate package`; the coordinator runs the configured landing gate through `pm ticket land 0744 --merge-branch --disk 20 --memory 8`.
- Every proof uses the standard job, linux-check or configured gate entrypoint; preserve failed receipts and classify their reasons as product, setup or invocation.

## Progress

- 2026-10-20 started
