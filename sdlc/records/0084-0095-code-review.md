# Code review: 0084 and 0095

Reviewer: fresh read-only Claude session, 2026-09-24. I did not write this work.

Read: repo `CLAUDE.md`, `sdlc/README.md`, both tickets, both build records, `sdlc/records/2026-09-24-spine-review-contract.md`, `sdlc/records/2026-09-24-rereview-contract.md` (including the builder amendment confirmation), the ADR 0017 and `rust.md` diffs, main `9eb26289`, 0078 `fafbe009`, 0086, 0098, and the surface tickets 0105 to 0112 and 0118 on their branches.

Reviewed commits: 0084 `0304a6e4`, 0095 `d886fec6`. Both branch from main `9f47bd18`.

What I ran, on scratch copies under `/tmp/claude-1000/`:
- Merged main `9eb26289`, then 0084, then 0095, in a scratch clone. Both merges are clean. The merged diff against main touches only 8 files under `sdlc/`.
- On that merged tree: `policy.py`, `catalog.py`, `pages-self-test`, `pages`, and `ratchet.mjs` exit 0. The ratchet reads `48475/48475`. `git diff --check` is clean.
- Extracted the one `rust` block of each ticket. `rustfmt --edition 2024 --emit stdout` exits 0 on both. A planted missing `)` exits 1.
- A small owner check over both blocks found 215 members, 54 types, and no duplicate. The planted rename `Question::kind` to `rank` reports one duplicate.
- `wc -m`: 0084 is 29,983 (cap 30,000). 0095 is 11,732 (cap 16,000).

## Verdicts

- **0084: findings.** One small blocking fix (B1) in ADR 0017. Two landing edits (L1, L2).
- **0095: ACCEPT.** Its own two files are clean. It takes B1 through a merge of the fixed 0084, and it needs the same two landing edits.

## Blocking finding

**B1. 0084: the two proposed ADR 0017 amendments sit in the wrong place, and the ADR does not name `EngineBuilder::from_env`.**

The branch inserts "Proposed amendment, 2026-09-23" and "Proposed amendment, 2026-09-24" between "Consequences" and "### Ruled after acceptance, 2026-09-21: the data frame is Polars". That `###` heading now falls under the proposed interrupt-check amendment. A reader sees Ian's accepted Polars ruling as part of a proposal he can still overturn. The two sections also sit above "What Ian can overturn" and above the dated 2026-09-21 and 2026-09-22 amendments, so the file no longer reads in date order.

The same ADR lists every 0095 member, as spine review B2 required. It does not name `EngineBuilder::from_env`, which the 2026-09-24 builder amendment added to the public inventory. `rust.md` says the public API grows only with an ADR. Main's port guide already builds every surface on `EngineBuilder::from_env()`.

Smallest fix: move both "Proposed amendment" sections, unchanged, to the end of the file after "Amendment, 2026-09-24: one width for the process". In the 2026-09-23 section, add one sentence: "`EngineBuilder::from_env` seeds a builder with the section 5 settings; `Engine::from_env` equals `EngineBuilder::from_env()?.build()`." The ticket text does not change, so its cap is untouched. Then merge the fixed 0084 into 0095.

## Landing edits (both tickets)

**L1. The code review line becomes false once this review is recorded.** 0084 says "Code review: none; design only." 0095 says "Code review: not applicable; design records only." Each must name this record, as 0117 does. For 0084 this needs about 22 more characters, and only 17 remain. Drop `sdlc/records/` from the 0084 status line to free 13 characters. The line can then read "- Code review: `sdlc/records/0084-0095-code-review.md`."

**L2. Neither build record states its ladder result.** Both say "The builder reports that result with the commit." A result that lives only in a chat report cannot be found later. Record the ladder on the merged tree in each build record: the commit, each rung's exit code, and the ratchet reading. 0078 and 0117 do this.

## Answers

**(1) Scope.** 0084 changes only its ticket, ADR 0017, `rust.md`, and records. All three design files are in its `opens`. No code, dependency, feature, or ratchet changes. 0095's own diff over 0084 is exactly its ticket and its record. Neither ticket delivers less than its acceptance: both blocks parse, each name has one owner, the caps hold, `git diff --check` passes, and 0095's five "Why 0084 changed" items all appear in the 0084 inventory. One thing crosses over: the ADR paragraph that lists 0095's members lives on the 0084 branch, and 0084's prose points at 0095. This is why the two must land back to back (see 5).

**(2) The scratch checker is acceptable. Nothing needs a rung now.** It checks design text once, at acceptance. The lasting proof belongs in a rung that checks code. 0086 extracts the 0084 block into signature fixtures and normalized `cargo public-api` output, and 0098 extends that inventory check to 0095. A rung that parses ticket prose and enforces ticket caps would lock a record that the post-0078 reconciliation must still edit. It would also add a script that both tickets exclude. The builder's line "the inventory check becomes real when 0098 lands" is half right. 0086 makes the 0084 half real, and 0098 adds the 0095 half. Smallest fix (non-blocking): give the extraction and check commands in the build records, so the post-0078 reconciliation can rerun them. The `awk` extraction, the `rustfmt` line, and a one-sentence owner rule are enough. Today the planted-failure tables cite a tool nobody else can run.

**(3) The shorter code review line lost nothing.** "None; design only" and "not applicable; design records only" say the same thing. The status edit also dropped "design and its 2026-09-24 amendment accepted". The Review line still names the re-review record, and that record holds the builder amendment confirmation, so that loss is harmless too. Both wordings of the line become false once this review exists (L1).

**(4) Agreement holds.**
- The two branches agree. 0095 at `d886fec6` holds 0084 at `0304a6e4` byte for byte plus two files. The shared re-review record kept the longer 0084 copy.
- They agree with main. The merge is clean and the cheap rungs pass on the merged tree. Main's `Width::new` and `args.rs` both accept 1 through 32, which matches 0095's "refuses 0 and anything above 32" and 0084's `width(u8)`. Main's port guide already cites `EngineBuilder::from_env()` as "0084, amended 2026-09-24".
- They agree with the surface tickets. 0086, 0105, 0107, 0108, 0109, 0110, 0111, 0112, and 0118 all use `EngineBuilder::from_env() -> Result<EngineBuilder, Error>`, setters that override, and `Engine::from_env() == EngineBuilder::from_env()?.build()`. 0105 and 0111 read width as 1 to 32 with 0 refused as `usage`. 0105 handles out-of-range host numbers (300, -1) in the binding before the `u8` setter.
- The only gap is the missing ADR line for `from_env` (B1).

**(5) Landing order is safe.** 0084 first, then 0095 right after. 0095 is 0084 plus two new files, so once 0084 is on main, the 0095 merge adds only those files. The B1 fix merges cleanly into 0095, because 0095 never edits ADR 0017 itself. The merge needs no re-measure. Neither branch touches `ratchet.json` or any `.rs` file, so the merge keeps main's 48475, and the merged tree measures 48475. No script or test reads the changed files. Land them back to back. If 0084 sits alone on main, its prose and its ADR paragraph point at a 0095 that is not there. Each land commit is a main merge on top of the reviewed commit. Record the ladder on that tree (L2).

**(6) 0078 changes one sentence in 0084. It does not change the inventory.** 0078 at `fafbe009` makes `nix` (feature `signal`) a non-optional Unix library dependency. It also makes `signal-hook` optional and selected by `cli`. Its `policy.py` now fails when `nix` is missing from the default-features-off graph. 0084's package-proof sentence says the proof "rejects every package activated only by `cli`, including `clap`, `csv-core`, and `nix`" and rejects `signal-hook` "only if 0078 made it CLI-only". After 0078, that proof must allow `nix` and reject `signal-hook`. 0086 line 41 says the same thing and needs the same edit. This is the "signal dependency placement" item on 0084's reconciliation list, so it does not reopen full review. 0078 does not touch construction, `Engine`'s traits, implicit initialization, or omitted width. Its worker mask keeps the host's mask on the calling thread, which agrees with 0095's rule that the interrupt check runs only there. Do not make this edit now. 0078's code review may still change the dependency split. Make it in the post-0078 reconciliation, before 0085 or 0086 starts. The edit makes the sentence shorter, so the 0084 cap holds.
