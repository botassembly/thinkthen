---
flow: build
priority: 46
opens: crates specification demos README.md sdlc/planning sdlc/ratchet.json
---

# 0023: Close the correction pass

Status: landed

## Outcome

The remaining hands-on findings and review corrections have exact, unsurprising CLI behavior and truthful pages. This ticket changes no judgment, request, recording, or successful judgment output. It also brings the prospective command-and-library plan level with Ian's accepted engine, library, database, and release rulings.

## Current facts

The two hands-on passes found no additional wrong answer, leak, crash, or hang. Today's marketing update already corrected the built-command lists in `README.md` and `demos/README.md`. Pass two leaves eight message, help, and page findings. Pass one also leaves unsupported-view advice, the undocumented record-mode limit on quiet output, singular stopped-run grammar, and one page that uses the repository transform tree without naming that external input. Ticket 0033 made `--threshold=-0.5` reach the threshold parser, but spaced `-.5`, `-1e-1`, and `-inf` still fall into generic argument parsing. The address error `a base address has a host` has neither a settled page nor an exact compiled test. Four records call 493 recording distributions plus four standalone fixtures “497 recorded distributions.”

The prospective plan still assigns retries and concurrency to host languages and says R is open. Ian has ruled in one Rust engine, Rust, Python, JavaScript/TypeScript, Ruby, R, and C, followed by DuckDB, SQLite, and PostgreSQL. The experiment team owns the ADR 0017 rewrite; the build team reviews it before the one-crate merge.

## Corrections

1. A wrong-kind question file says `` `filter` reads a `decide` question, but the question file holds a `VERB` question `` or the same sentence with `rank`.
2. `--input` naming a directory says `` `--input` names a directory, and a directory is not an input file ``. It does not blame standard input or add a stopped-record line when no record was framed.
3. Shared framing and jobs help promises each command's output order. `rank` says that it sorts by probability and keeps input order only for ties. Shared field help describes whole-document behavior only for commands that accept a document.
4. `result.md` says that `filter --details` carries the cut's boolean under `value`, and that `question.verb` names the question kind. Filter and rank therefore carry `decide` there.
5. A rank dry run omits `threshold` from `from`, because rank takes no threshold. `channels.md` says `from` names only settings the verb takes.
6. Invalid UTF-8 in a streamed line or JSONL record says `the record is not valid UTF-8`. Whole-document invalid text keeps the evidence wording. Both remain exit 5 and repeat no byte.
7. Rank refuses negative, empty, and nonnumeric `--top` values with `` `--top` prints the first N of the order, and N is a whole number of 1 or more ``. Filter says `` `filter` keeps records and has no order to cut, so --top belongs to `rank` ``. Spaced and equals negative values reach these messages. These refusals happen before a key, input, or request.
8. How-to 43 shows the question text and its true and false meanings near the top, before the first `filter` run, while remaining inside the how-to limits.
9. All seven command homes send spaced `-0.5`, `-.5`, `-1e-1`, and `-inf` past Clap: `decide`, `choose`, `tag`, and `filter` reach the established threshold parser; `rank`, `score`, and `annotate` reach their established no-threshold refusals. Before `--`, only the token immediately after standalone `--threshold` is joined to that option, and only when the whole token starts with `-` and is either one value accepted by Rust's `f64` lexical parser or exactly two such values separated by one colon. Range, finiteness, ordering, and verb rules remain in the established threshold parser. `--dry-run`, `--bogus`, `-word`, `-.config`, `-infamous`, `-nanosecond`, and tokens after `--` remain options, unknown arguments, or positional text under Clap's existing rules.
10. The hostless-address rule and the exact sentence `a base address has a host` appear in `backends.md` and in core and compiled-command tests. The diagnostic repeats no address.
11. Ticket 0025, ADR 0019, record 0025, and the full-project review say `493 distributions in recordings plus four standalone fixtures`. No probability rule changes.
12. Unsupported views receive the tool's own safe messages everywhere. In particular, `decide --raw` says `` `decide` prints JSON; `choose --raw` prints a bare label ``, `score --raw` says `` `score` prints a JSON number; `choose --raw` prints a bare label ``, and `score --quiet` says `` `score` has no answer exit code, so --quiet would discard its result ``. Existing verb-specific refusals for tag, filter, rank, and annotate remain. `channels.md` and the verb pages say that quiet output cannot act in record mode.
13. Stopped-run counts use `record` at one and `records` otherwise, for both finished and replayed counts. Every current green page that reaches outside its folder into `transforms/` names the repository transform tree as an additional input; page 41 gains the missing statement, while pages that already declare it stay unchanged.
14. The prospective plan records the three layers: pure core rules; one Rust engine for sending, retries, process-wide scheduling, cache, record, and replay; thin CLI and language shims. It lists all six library interfaces after the command, the three database extensions as a fast follow, the shared 0.1.0 first release with 0.0.N builds before it, and the rule that a later surface joins the current shared version. It replaces stale detail to stay within budget. The plan records the four engine/one-crate steps, with ADR 0017 rewritten and reviewed before them. Cache locks, `find`, page 16, and transforms retain their accepted order.

The two already-correct built-command paragraphs stay correct, and the pages check continues to hold them to the binary.

## Acceptance

- Compiled-command tests pin every changed sentence, exit code, and help paragraph. Local listeners prove that parser and option refusals send zero requests. Secrecy tests cover the new paths.
- Threshold cases cover all seven command homes, both `--option value` and `--option=value`, the whole-token numeric boundary, known-option precedence, unknown options, `-word`, `-.config`, `-infamous`, `-nanosecond`, and `--`. Top cases cover negative, word, empty, and filter ownership.
- Directory input is covered through an actual directory on the supported host. A deterministic handle test proves classification follows the opened object if its path changes. It produces no stopped-record summary. Existing missing-file, standard-input read, and later-record failure behavior stays pinned.
- Dry-run snapshots prove rank omits only the inapplicable threshold source. Existing request bytes, digests, line/JSONL output, and committed replay results remain unchanged.
- How-to 43 and every existing green page pass with no key and no network. The prospective plan stays within its 4,000-character budget.
- The four repository rungs and `git diff --check` pass. Commit, push, and the remote check belong to the coordinator's landing step.

## Excluded and following order

Excluded: engine or crate moves, cache locking, `find`, new command options, probability tolerance changes, a commit-message checker, and paid response-stability work. The newly observed live probability-total refusals get one focused evidence-and-fix ticket next and complete the correction pass. Then come one bounded per-digest cache lock in engine-ready modules, `find`, and page 16 with the transforms. The one-crate work waits for the experiment team's ADR 0017 rewrite and build-team review.

## Complexity

- Contract: 2
- State and timing: 0
- Reach: 1
- Proof: 2
- Cost of error: 1
- Total: 6
- Minimum level floor: none
- Final level: 3
- Reasons: argument routing changes across all seven command homes, public diagnostics and help must agree, and compatibility proof must preserve known and unknown option behavior alongside request bytes and replay. No new state or concurrency enters.
- Selected model: `gpt-5.6-sol` with medium reasoning.

## Review

- Design review: accepted after two revisions by a separate Sol Medium agent. The review added the four remaining pass-one findings, all seven threshold homes, exact public diagnostics, the later-surface version rule, and level 3 routing. Its final pass required an anchored whole-token `f64` boundary so threshold normalization cannot swallow similarly prefixed unknown arguments.
- Implementation: complete locally. The first spaced `-.5` test failed in Clap as expected. Focused compiled tests now cover every changed diagnostic, all seven threshold homes, listener request counts, input framing, help, secrecy, and unchanged replay behavior. Lint, test, and spec pass; final review and landing remain.
- Code review: accepted after one remediation pass by a separate Sol Medium agent. The first review found that spaced negative `--top` values stopped in Clap, the directory check inspected the path before opening it, and the filter result sentence described only kept rows. Remediation routes both `--top` spellings to the same safe command diagnostic with zero requests, classifies the one opened handle, and states the cut boolean directly. The review also placed a focused probability-total evidence-and-fix ticket next without expanding this ticket. The reviewer ran all four rungs and accepted the full current diff.
