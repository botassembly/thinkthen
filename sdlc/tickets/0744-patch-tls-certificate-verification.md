# 0744 — Patch the shipped TLS certificate dependencies

Status: COMPLETE.

Milestone: 0.2.1

Reviews: revision c5fc56d, accept

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
- Proof command, with SHA replaced by the pushed repair revision: `pm job run --machine m5 0744 SHA --disk 20 --memory 8 -- sdlc/scripts/gate package`; the configured gate routes Linux lint through its declared image and also runs native routine checks; landing uses `pm ticket land 0744 --merge-branch --disk 20 --memory 8`.
- Under the 2026-10-20 freeze ruling, the lander runs `pm job run --machine gator 0744 SHA --disk 20 --memory 7 -- sdlc/scripts/linux-check lint test` on the landing candidate in parallel with its configured gate; both must pass while Gator is available.
- The earlier coordinator-supplied native `sdlc/scripts/job sdlc/scripts/lint` invocation failed in M5 receipt 1792523999973-37872 before advisory checks: generation requires gcc -fdump-ada-spec, while its GCC provider is declared Linux-only; preserve this invocation failure and do not repeat that command on macOS.
- Every proof uses the standard job, linux-check or configured gate entrypoint; preserve failed receipts and classify their reasons as product, setup or invocation.

## Progress

- 2026-10-20 started
- 2026-10-20 next: setup: Gator jobs 1792527683967-1234165 and 1792528070953-1250064 exited 1 before lint/test because the pinned Ubuntu snapshot returned HTTP 503 during image build; no further retry or source workaround. Await coordinator ruling and the M5 standard gate result.
- 2026-10-20 next: product: Gator candidate job 1792529452380-1255765 exited 1 in release-archive-self-test.py after lint and advisory checks; release-pack rejects its derived Dart native-assets.json because it compares the rewritten paired-C checksum manifest with source bytes. This distinct packaging outcome requires coordinator ordering; no unchanged retry or identity-assertion waiver. Await M5 land-check real exit.
- 2026-10-20 next: setup: normal landing gate job 1792532702783-21422 exited 100 after 2164 of 2165 Linux routine tests passed; the unchanged aggregate refusal/secrecy matrix timed out at 5.002 seconds with no assertion diagnostic. Earlier configured job 1792529456380-4483 passed the same product; no unchanged retry, deadline increase or test waiver. Await coordinator ruling for the existing routine-limit outcome.

- 2026-10-20 landing authority: PM explicitly approved integration on green configured M5 receipt 1792529456380-4483 (exit 0), gated tree 8408f056c7d5a8b7effbce06151a6ec21e58a770. Later main edits change only sdlc records. Later M5 receipt 1792532702783-21422 timed out in the existing aggregate refusal fixture; setup repair belongs to 0605. Gator packaging failure remains assigned to 0748; no failed receipt is treated as a pass.
