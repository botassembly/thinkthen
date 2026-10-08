# 0465: Ship the complete npm module inventory

Status: COMPLETE. The assembled npm package includes the complete modules and declarations; installed import and type checks pass. Public source-smoke families remain required.

Landed: b59ab03c6

Milestone: 0.2

Owner: builder.

Reviews: revision e71fa0b01, accept

## Outcome

A fresh install of the locally assembled npm archive imports and exposes the complete API. Optional source-package smoke has an explicit inventory consistent with Flutter’s private pilot status.

## Evidence

- Starts from: 0462 packaging review at 7ea661c1e; release-workflow npm-assemble copies only the legacy facade and omits complete.js, _complete.js and their declaration files required by index.js. release-smoke --source-packages demands Flutter although release-container does not emit it.
- Keeps: All four native npm addons, checksums, typed APIs, package license and independent consumer assertions. Flutter remains a private pilot; do not silently drop required public consumers.
- Changes: Include complete API runtime/declaration files in npm-assemble and exercise the resulting packed archive outside source resolution. Align the optional smoke inventory with the documented pilot ruling; document whether it consumes an externally supplied Flutter archive or excludes it from this release bundle.
- Proof: Fresh ticket and whole-change review. Existing workflow/assembly checks plus an actual isolated packed consumer and declaration resolution; a missing required runtime module must fail import. Focused local checks and applicable full tests/lint before landing. No GitHub workflow or publication.
- Defers: No registry/account change, candidate, hosted run, new package manifest framework or public Flutter release.
