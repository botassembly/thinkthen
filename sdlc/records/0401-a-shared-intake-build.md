# 0401 slice A: Share named file and text-window intake

Status: candidate. Built in claude-0 from origin/main `115bb95b20111f3f73688157584ff2f4736171ff`. Fresh code review, the coordinator's full checkpoint and the actual 350 site sample replays remain pending.

The seven functions share one CLI intake iterator. Decide, filter, rank and annotate accept positional files. Choose, score and tag retain their positional labels and use repeated --input. The iterator opens every named source before admitting an item and initializes every table header first. It preserves file-local positions separately from global pipeline labels. Fixed text-line windows keep internal line feeds and file boundaries. Locations travel only in CLI result metadata.

Default-document decide, choose, score and tag runs associate each named file with its answer through JSONL. A completed multi-file document run exits 0. Empty later documents and failed later replies preserve completed output and return their actual error codes. Single-file and stdin scalar bytes, exits and requests remain unchanged. Find keeps its narrow single-file parser. Recognize and relate refuse the unsupported intake forms before sending and hide the window option in help.

## Focused proof

- The new per-ticket boundary files cover all seven functions, two-file ordering and locations, preserved blank lines, final short windows, exact 16 MiB joined-item bounds, table headers, document carriers, false/null run exits, late input/backend failures, zero-send refusals, batching/splits, existing-path labels and single-file/stdin request bytes. All 14 new tests pass.
- The existing keeping suite passed all 23 tests, including default framing, saved score rank, byte preservation, stable ties, input subsequences, permutations, top bounds and failure prefixes. Hints passed 3 tests; blank-line cases passed 2; choosing passed 11; annotate-related cases passed 70. The public JSON fixture replay passed its 63 shared comparisons after checking the CLI-only position and comparing every shared judgment byte. Total focused test cases: 124.
- Policy passed with all accepted boundaries and dependencies. Site flag data matches the debug command's help for all ten functions. Settings coverage passed all 12 planted cases and reports zero gaps against the current debug command. Tickets and diff whitespace checks pass. Final lint remains pending until its complete receipt is recorded.
- Tests use synthetic responses and counted loopback listeners. No live provider, credential file, registry, workflow dispatch or external service ran.

## Size and boundaries

The measured Rust total rises from 109496 to 110497 nonblank lines: 316 source lines and 685 test lines. The shared adapter replaces the separate asking and annotate source/table framing blocks. The new text reader and boxed table reader keep the iterator small without nesting suppressions. Moving Common's existing option-resolution methods to their own module keeps every file under 500 nonblank lines. I searched the asking and annotate framing blocks for duplication and removed the redundant adapters before raising the ratchet. The boundary tests earn their lines through CLI bytes, source identities, error exits and counted sends. The repository has no tests/test-file-caps.json registry; its existing policy measures every Rust file directly.

Core, engine, public APIs, binding code and provider wire formats are unchanged. The source-hash proof contract includes CLI and test files, so all 350 saved site proof receipts become stale. The coordinator owns their actual offline replay and strict verification. No source hash or saved output was rewritten by this build. Display flags, find display and multiquestion rank remain separate slices B, C and D.
