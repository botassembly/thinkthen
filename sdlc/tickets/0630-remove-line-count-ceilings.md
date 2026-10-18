# 0630: Remove every line-count ceiling

Status: OPEN.

Milestone: 0.2.1

Phase: 1

## Outcome

- All 54 tracked `ratchet*.json` files are deleted, `sdlc/ratchet.json` included.
- `sdlc/scripts/ratchet.mjs` and its self-tests are deleted. No gate, lint step, `surfaces` step, release script or workflow calls it.
- `conformance/library-surface-size-test.py` and the size report it checks are deleted.
- The file caps and the format-skip ceiling in `policy.py` are deleted, along with the 0457 source-size warnings. The adapter-word boundary and dependency rules in `policy.py` stay.
- The surface manifest keeps its classification of generated, test and production code only if another check still reads it. Otherwise it is deleted too.
- The repo instructions and `AGENTS.md` drop the ceiling and file-cap sentences and say that code review judges size.

### Proof

- `git grep -n -i -E 'ratchet|ceiling|MAX_FILE_LINES'` over the whole tree finds matches only in history: `sdlc/issues/`, `sdlc/records/`, `sdlc/decisions/`, `sdlc/tickets/` and `sdlc/planning/`.
- On the M5 through pm job, `lint`, `test` and each library gate pass without the ceiling steps.

## Evidence

Rule 4 of the 0.2.1 ruling deletes the line ceilings, and code review judges size. The binding contract already says to "set no arbitrary line target and add no recurring measurement process" (`libraries/BINDING-AUTHOR.md`, "Caller acceptance and code ownership"). Git tracks 54 `ratchet*.json` files: `sdlc/ratchet.json`, the crates and conformance ceiling, plus 53 under `libraries/` and `databases/`. `sdlc/scripts/ratchet.mjs` checks them from `sdlc/scripts/lint:133`, `sdlc/scripts/surfaces:198-204`, `.github/workflows/gate.yml:180` and the library gates (for example `libraries/go/check.sh`, `libraries/zig/check.sh:20-21`, `libraries/ada/check.sh`, `libraries/cobol/check.sh`). More ceilings sit elsewhere: `conformance/library-surface-size-test.py`, the file cap `MAX_FILE_LINES` and the `MAX_FORMAT_SKIPS` ceiling in `sdlc/scripts/policy.py` (around lines 75, 88 and 2270), the source-size warnings 0457 added, and the sentence in the repo instructions and `AGENTS.md:10` that "a ceiling raise needs explicit reviewer acceptance".

The ceilings cause churn without finding defects. Since 2026-09-15, 74 commit subjects touching the nine C-interface and native libraries mention a ceiling, ratchet or budget. Examples: 133ce2d6d "Raise the C++ fixture ceiling one line for the guarded key read", dc155aec2 "Reconcile the C fixture ceiling with Windows output handling", 294992ad3, e4b206512 and cc4f0deb4 (all three named "Measure shared fixture source ceiling").

## Progress

- 2026-10-18 started
