# 0733: Select the pinned GCC only in Linux checks

Status: OPEN.

Milestone: 0.2.1

## Outcome

- Changes: `sdlc/tools.json`.
- Remove `darwin` from the platforms of the GCC 13.3.0 tool entry. Keep the version, groups, probe and every other tool entry unchanged.
- Native macOS tool resolution does not require this Linux-only compiler. Linux tool resolution still requires the same pinned compiler.

## Keeps

- No host installation, compiler substitution, deadline change, new checker or tool resolver feature.
- Ada and COBOL compiler checks remain in the Linux containers.

## Proof

- Inspect the merged native and Linux selections using the existing resolver to prove the GCC entry is excluded only on Darwin and retained on Linux.
- Resolve the existing native lint group through an admitted M5 job. Preserve setup failures and report any other missing prerequisite rather than silently dropping a required group.
- Run the configured integrated landing gate before closure.

## Evidence

M5 job 1792487935214-21502 failed before tests with `gcc: version differs from tools.json`. The entry selects Darwin and Linux but has no provider, so native lint probes an ambient compiler against the Linux GCC pin. The Ada and COBOL fragments declare only Linux support. The PM approved this exact platform correction in the shared inbox on 2026-10-20 and requested this small ticket. No product result was established by that failed job.

## Progress

- 2026-10-20 started
- 2026-10-20 next: The approved one-entry correction is pushed at8c57b33d6; M5 proof1792488815417-40662 exited2 in shared tool preparation because Ruby4.0.7 psych could not configure, so selection and lint checks never ran, and the developer is diagnosing the retained setup logs without another attempt.
- 2026-10-20 next: Full-log correction: job1792488815417-40662 passed merged Darwin/Linux GCC selection for all four groups after outer setup; only the later check-only lint invocation failed because mise auto-installed Ruby without the normal libyaml configuration, and source diagnosis is checking the intended ordinary resolver invocation before any further attempt.
