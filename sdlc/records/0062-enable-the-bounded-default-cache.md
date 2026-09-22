# 0062: Enable the bounded default cache

Date: 2026-09-22

Status: landed

## Result

Ordinary commands now reuse answers from the platform cache by default. `--no-cache` disables it for one run. `THINKTHEN_CACHE` and `--cache` select an explicit folder. Explicit record and replay remain separate and take precedence. The cache starts only when the first valid prepared request needs it, so rejected input and dry runs create nothing.

A closed, read-only JSON configuration supplies the backend address, model, cache switch, and prune target. Linux and macOS paths resolve independently. Only absolute XDG and home paths are usable. Existing default cache folders must have Unix mode `0700`; the tool never changes a wider mode silently. The manual and long help state that entries include the judged text and backups may copy it.

`thinkthen cache prune DIR` validates every recognized entry before mutation. It applies age and model selectors, then the 100,000,000-byte target, in modification-time and digest order. It counts allocated blocks, skips an active digest without waiting, reports truthful remaining counts, and leaves unknown files alone. A folder-wide directory lock keeps pruning outside active compliant record and replay work. Replay remains read-only.

The Rust ceiling rose from 29,242 to 30,856 nonblank lines. The closed configuration and path resolver, lazy folder gate, safe prune implementation, public command parsing, storage and process race tests, precedence matrix, documentation, and required fixture isolation account for the increase. The implementation reused the existing recording envelope, digest locks, recorder, backend validation, and command settings rather than adding a second cache format or engine.

## Review and proof

Independent design review rejected two drafts. The repairs added one folder gate, honest partial-deletion semantics, exact 100 MB accounting, model provenance, path and privacy rules, entry identity validation, and the correct level-4 route. The same reviewer accepted the final design.

Independent code review rejected the first implementation for blocking active locks, nondeterministic partial deletion, eager cache creation on rejected input, a missing-directory replay regression, incomplete proof, stale roadmap text, and incomplete help. The repair added nonblocking digest locks, stable deletion and fault proof, lazy recorder setup, the established replay miss, pure path functions, compact behavior matrices, and corrected documentation. Re-review found that a relative `HOME` could still create a cache beneath the working directory. The final repair accepts only absolute home paths. The reviewer reproduced each critical path and accepted the implementation with no remaining material finding.

The final install, lint, test, and specification rungs passed with the key and outside address variables unset. The test rung passed 195 library tests, 253 backend tests, every command-edge suite, demo-runner and question-file tests, doctests, and script self-tests. The specification rung passed 27 page checks, seven transform checks, every committed replay, and all 19 green demos. The exact 30,856-line ratchet and `git diff --check` passed. No paid or outside request ran.
