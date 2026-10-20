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
