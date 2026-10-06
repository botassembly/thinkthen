# 0411: Reread stored answers under changed rules on every surface

Status: ready. Planning only; implementation follows accepted ticket review.

Milestone: 0.2

## Outcome

All named SDK, SQL and dataframe routes reread stored answers under each free reading rule with zero additional calls.

## Evidence

- Starts from: Existing ticket and 0405 audit; landed 0377/0401D/0420 are the current baseline, replacing the older audit-only matrix. PM message `2026-10-06-pm-0-2-is-not-done-every-sdk-consistent-and-the-sdk-ready-for-the-proxy.md`, asks 4.
- Keeps: Preserve existing bare calls and generic JSON compatibility doors, backend selection from 0377, six error kinds, cancellation, secrecy, count-only usage, and zero-send strict replay. Reuse the Rust engine, C boundary and native file reader; add no host cache, scheduler or second parser.
- Changes: Extend existing canonical saved cases and public settings adapters; function contracts without a reading cut are explicit, never skipped.
- Proof: Change cuts with identical wire identity; pin output/counts for native details, member rules, entity/relation dependencies, frames and R index conversion.
- Defers: Proxy service/screens, images, unrelated features and Windows Node/C#/JVM packaging remain outside this outcome.

## Dependencies and ownership

0408 owns missing details; 0432 owns common enforcement; no new fingerprint or store-verification framework.
