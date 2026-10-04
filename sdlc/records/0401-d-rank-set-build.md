# 0401D: CLI and Rust rank question sets

Status: implementation candidate. Fresh independent code review and coordinator checkpoints remain pending. SQL sets belong to 0417 for 0.2; C and language sets belong to 0418 later. No landing or checkpoint dispatch is claimed.

## Authority and source

Lane0 started clean on `ticket/0401-rank-search-flags` at landed C main `fea3a6fbc46b68ccc33a28e35eb0c4eb48329df9`. The [owning design](0401-d-rank-set-design.md) adopts frozen SHA256 `ace1ddd3deec8ef962920c06804f1b66d55826cdb66d4e68e504241cfb014a87` and its fresh independent ACCEPT. The review accepted duplicate-consumes-visit turns and the additive RankSet/SetRanked API. Historical preparation baselines did not replace landed C. There is no API deviation. Ian can overturn the accepted choices; the parent owns review, latest-main rebase and full checkpoints.

RankSet parses the existing closed, ordered set grammar through rank admission before normalization. It refuses authored threshold/on and non-decide members while leaving annotate defaults and canonicalization intact. CLI dispatch reads the named file once through the existing capped loader and chooses the grammar by the parsed questions key. Parse errors never fall back. CLI meanings conflicts retain an exact safe usage sentence.

The CLI plans each member independently, packs through the existing engine, decodes every member with its own receipt, and retains member-local stable lists. The pure core merge visits saved member order at each depth. Duplicate identities consume visits; equal text at distinct positions survives; top follows first appearances. With top K, candidates remain at most M×K and merge deduplication at most K. All records and members are judged. Shared intake, snapshots, locations and display emission remain the owning paths.

Rust uses one pipeline and Stop for the complete eager call. Original caller items stay on the caller thread, move once, and need no Clone/Send/Serialize implementation. SetRanked owns the selecting probability and name without changing existing Ranked/RankedRow. Observations retain all named probabilities and receipts, with one row event per original. Success facts count N records; errors retain final facts after joining workers. CLI details add only question_name to the selecting member's ordinary rank detail. No schema, fingerprint, cache migration, backend, default width or paid call is added.

## Focused evidence

Owned complete logs are in `/tmp/thinkthen-0401d-build-cli-o1P52IbY`. Every shell tool call used `/bin/bash` with login disabled. Builds ran through a sourced allow-list, clean owned HOME/Cargo configuration, offline Cargo, two jobs and an empty compiler wrapper, inside a user systemd scope capped at 12 GiB memory and 1 GiB swap. The lane lock isolates warm outputs. Cargo retains shared cache/toolchain exclusion; no installation or mutation of the shared advisory snapshot was requested. Warm outputs were preserved and no process was stopped.

- `final-correction.log`: 16 CLI and 9 public API rank-set cases pass, followed by both rank-set executable blocks and the Result shapes block. Cases independently declare member order and attribution, cross-scale probabilities, stable ties, original identities and top prefixes. They pin N-record facts, selecting receipts, non-Clone/non-Send ownership, named backend model/address/key selection, shared cancellation/budget, preflight refusals, late member failure, capped loading, secrecy, single-member wire/detail parity, individual-member replay, rename/reorder, missing strict replay, normal cache misses, and mixed structured/text identities.
- `focused-final.log`: 2 pure merge/retention witnesses, all 23 retained keeping cases, 26 shared display cases including C's find views, the retained annotate batch-tier case, and all 9 question-set parser/canonicalization cases pass. Saved-score ordinary rank, filter subsequences, find none/positions, immutable snapshots, replay and display failures remain protected. The 24 initial rank-set cases also passed; the strengthened final correction supersedes that rank-set selection with 25 cases.
- Settings coverage passes: 66 rows, 68 flags, 9 environment names and 15 question-file keys, zero failures. Result compatibility now admits optional CLI question_name and checks both a replayed set row and a literal illustrative set row, retaining strict unknown-member checks.
- `policy-correction.log`: offline policy passes on final product source, checking 254 resolved packages. Required lint is pending at this preparation commit; its actual receipt follows below.

Across those retained and corrected selections, 86 distinct Rust cases pass. This is focused evidence, not a full test/spec/surfaces checkpoint or native/packaging/site proof. No helper, mail, remote dispatch or paid provider was used; the live ledger was untouched.

## Lessons and corrections

Planning a mixed structured/text set as one Plan would make the core's all-or-none quoting rule unquote every member. Each member now uses its own production plan before shared packing. Individual structured and text recordings replay in either saved order. A stronger selecting-member case then failed red: a text member selected after a structured first member reported batch 1 instead of max. Set runs now retain the actual shared batch setting, and structured member rows report their own one-record setting. `mixed-metadata-red.log` pins the original mismatch; the final corrected case passes.

The initial fixtures confused TCP connection counts with request counts, used an answer field named yes instead of the established probability field, and combined conflicting cache flags. The corrected witnesses count actual requests for judgments and persistent listener connection totals for runtime no-send comparisons. The shared request-total control is process-wide; its test allocates one additional send above the observed process total and pins the one-record/error facts. These changes correct test premises without weakening the expected behavior.

Each executable bash block gets its own environment. The second rank-set block initially referenced the first block's temporary question file and returned exit 5 instead of the intended meanings-conflict exit 2. It now declares its own file and passes with the exact sentence. Clippy led to small parser/admission and detail-attribution helpers, without adding production lint suppressions.

## Measured growth

The exact ratchet rises from 112872 to 114848 nonblank Rust lines: +1976, comprising +647 production and +1329 tests, including the two pure boundary modules and registration. Production stays just below the design's 650–900 estimate; tests exceed its 500–750 estimate because the independent CLI/API evidence separately pins complete receipts, failures, mixed-wording behavior, backend selection, ownership and memory bounds. Every source/test file stays below 500. New CLI tests have 242, 69, 347 and 162 lines; public tests have 231 and 186. The core merge witness has 31 and retention witness 42.

The shared parser extraction replaces 75 old parent lines with one reused grammar and rank admission. The implementation reuses the scheduler, production planner, ordinary row/detail builder, display emitter, Decisions asker, capped loaders, batch precedence and pull controls. I inspected those paths for duplication before raising the ratchet. No dependency, source/test cap, schema or lint policy was weakened. The repository measures files directly and has no tests/test-file-caps.json registry, as the earlier A/B records establish.

## Deferred proof and boundaries

Fresh independent candidate review remains required. Parent-named full test/spec/surfaces, C-door/Polars/public compatibility, canonical actual350 replay/strict/site, native host and release proof remain deferred. The parent controls rebase onto newer main. This lane does not cherry-pick 0400 or Windows work and does not change default concurrency. SQL 0417 remains 0.2; foreign 0418 remains later; 0406 single-question surface parity is separate. Score members, filter sets, alternate merges, fingerprint changes and new providers remain outside D.
