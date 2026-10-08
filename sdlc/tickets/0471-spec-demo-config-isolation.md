# 0471: Isolate spec and demo configuration

Status: OPEN. The historical fixture failure remains possible because Linux spec/demo children inherit the caller’s configuration home.

Milestone: 0.2

Owner: builder.
Severity: medium fixture correctness.

## Outcome

Executable specification and demo checks use owned empty ThinkThen configuration, independent of a caller’s conflicting or malformed settings. They preserve explicit toolchain/cache paths and isolated count-only usage.

## Evidence

- Starts from: 0462 bug sweep; issue 2026-10-04-local-test-fixtures-read-ambient-configuration.md records the failed ambient demo. sdlc/scripts/spec calls usage_home but leaves Linux HOME/configuration unchanged before demos. scratch.sh isolates state, not Linux configuration.
- Keeps: Runtime configuration policy, existing assertions, offline replay, named backend tests, usage isolation, cache mutation locks and owned cleanup. Read no real credential or configuration file to reproduce this.
- Changes: Set an owned empty configuration home for the spec/demo child environment through the existing scratch helpers, after environment composition. Keep Cargo/Rustup and required build caches explicit; avoid changing other commands’ config behavior.
- Proof: Fresh ticket and code reviews; run the affected real fixtures against owned conflicting and malformed configuration plants, retaining exact output/request assertions. Existing spec/demo checks, policy and applicable full tests/lint. No new runner or receipt framework.
- Defers: No runtime configuration redesign, global HOME mutation, paid calls or hosted workflow.
