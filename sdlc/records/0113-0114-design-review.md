# Design review: tickets 0113 (audit) and 0114 (diff)

Reviewer: a fresh, read-only Claude session. Reviewed 0113 at 4e075921 and 0114 at 9058699b against repo `CLAUDE.md`, `sdlc/README.md`, Ian's ruling of 2026-09-24, `specification/channels.md` exit codes, ticket 0082, ADR 0017, `core/threshold.rs`, `core/json.rs`, `core/pointer.rs`, `sdlc/scripts/policy.py`, `sdlc/scripts/demos`, and the Beatles Bench prototype (`measure.py`, `test_measure.py`, `README.md`, goldens, and the McNemar issue at 18c0dc8).

## Verdicts

- 0113 audit: **FINDINGS** (2 high, 2 medium, 4 low)
- 0114 diff: **FINDINGS** (1 high, 2 medium, 2 low)

The design is sound, and the math matches the prototype. The findings are a private-repository problem, planted bugs that no named test catches, and the shared-reader seam 0114 needs.

## What I checked by command

- The prototype suite passes: `python3 -m unittest tests.test_measure` ran 31 tests, all OK.
- I wrote the statistics again from the ticket text alone, without importing `measure.py`, and matched the goldens:
  - SplitMix64 seed 0 gives `0xE220A8397B1DCDAF, 0x6E789E6AA1B965F4, 0x06C45D188009454F`.
  - Wilson for 4 of 6 gives `[0.299993, 0.903229]`, matching `audit-decide.jsonl`.
  - AUC by the pairwise sum gives 7/9 = 0.777778. The calibration error is 0.375.
  - `audit-249-seed.jsonl` matches in full from the ticket's split, shuffle, bootstrap, and quantile text. The error is 0.070772, the interval `[0.047862, 0.137796]`, the cut 0.42 on 136/136, and held right 89 at the cut.
  - McNemar gives 1.0 for (1,0), 0.5 for (2,0), 0.221549 for (39,28) as in `diff-249-cuts`, and 0.904975 for (36,34) as in `diff-249-soft`.
- The log-space McNemar in 0114 agrees with exact integer sums for every split up to n = 120. The worst relative error is 3.0e-14, well inside the stated 1e-12.
- I mutated a scratch copy of `measure.py` with each planted bug. For each one I ran the tests the tickets name: the goldens with the 1e-6 tolerance, the ported `test_measure` classes, the table captures, the diff-annotate capture, and the McNemar unit pins. Results are in 0113-2 and 0114-2.
- `gh repo view botassembly/beatles-bench` reports `isPrivate: true`.

## 0113 findings

1. **High. The source repository is private, and the ticket calls it public.** GitHub reports `botassembly/beatles-bench` as private, and `repos/README.md` says "private until Ian makes it public". The ticket copies about 420 KB of its fixtures into `crates/thinkthen/tests/fixtures/measure/`. `cargo package` ships those files, and a README there names the repository and commit. thinkthen will go public. Publishing another repo's content is outward-facing, so this goes to Ian. Fix: replace "public" with "private" in both tickets, and add a precondition. Options:
   - (a) Ian makes beatles-bench public before 0113 lands. This is the recommendation, because it costs nothing and the ruling already names the repo as the definition.
   - (b) Land now, with an `sdlc/issues/` entry that blocks the release build until the repo is public.
   - (c) Describe the source generically and drop the repository name.

   Put the choice in a `notes/todos/` item for Ian.
2. **High. Four planted bugs survive the named tests. A fifth is caught only by accident.** Bugs 5, 9, 10, and 13 turn nothing red. Bug 8 turns only `audit-annotate` red, through its 3-id `kind` group. The 249 key has 272 ids, which is even, and its control file is already in code point order. Name these tests in the ticket now. Each value below comes from the prototype, and each mutation turns it red:
   - Bug 5: calibration error over `[(1.0, no), (0.0, no)]` is 0.5. The bug gives 0.
   - Bug 9: audit `small/decide.jsonl` with its lines reversed and its key parts stripped, at seed 0. The suggested cut is 0.71. The bug gives 0.45.
   - Bug 10: two tuning answers, p 0.45 keyed yes and p 0.54 keyed no. Cuts 0.45 and 0.55 tie, and the cut is 0.45. The bug gives 0.55.
   - Bug 13: take `249/control.jsonl` with alternate lines given two question texts, and use `249/key.jsonl`. The second group's calibration interval is `[0.049765, 0.164636]`. A shared generator gives an upper bound of 0.161989. An alternative test asserts each group equals the same group audited alone.
   - Bug 8: a seeded split over an odd count, such as 5 ids, pins a tune n of 2.
3. **Medium. The vocabulary check never sees audit help.** `sdlc/scripts/demos` scans help only for the ten verbs and `transform`, so "passes the vocabulary check" proves nothing for `audit`. Fix: add `audit` (and, in 0114, `diff`) to the `scan_help` loop, and give it a line in the scripts budget. Also record a decision on the table words. `--table` prints "`0 unresolved`" byte for byte from the prototype, and `spec/audit.md` pins that line. Either keep the formal term in data output, or print "not sure" and make the table captures port-owned. Ian can overturn either choice.
4. **Medium. The shared seam for 0114 is not designed.** The reader, the key reader, and the failure sentences sit in `cli/audit.rs` with the words `audit:`, `results`, and `key` fixed. 0114 then imports from another command module and reworks those messages. Fix:
   - Put line reading, the id pointer, key loading, and a failure type that takes the command name and the input's role (`results`, `first run`, and so on) in a neutral module, for example `cli/measure.rs`, with the key parsing in core.
   - Keep the duplicate rule per command, because 0113 checks (name, id, text) and 0114 checks (name, id).
   - Name the golden comparison helper's home now, for example `tests/support/measure.rs`, since integration tests are separate crates.
   - Have the policy entry hold the neutral module.
5. **Low. The ticket leaves out rules the port must copy.**
   - The `--by question` group name is `name or text or verb`. Empty text falls back too, and `audit-decide-bare` pins `"decide"`.
   - A group where every answer failed prints `verb: null`, `disagreements: []`, and null directions.
   - The table formats the six-place-rounded values. For example, 0.6874996 prints 0.688, not 0.687.
   - Cuts, rules, and `--target` print as Python's `str(float)` does (`1.0`, `1e-05`). Name one helper for this, because Rust prints `1`.
   - A typed band prints verbatim. `Threshold`'s `Display` and `Serialize` normalize `0.40:0.60` to `0.4:0.6`, so add a test with `--threshold 0.40:0.60`.
6. **Low. The departures list is incomplete.** None of these reaches a golden, but each should be listed:
   - An empty RESULTS makes the prototype print one blank line.
   - `core/json.rs` refuses duplicate member names and NaN.
   - Python treats a decide key value of `1` or `0` as yes or no, because `True == 1`. The port leaves it unlabeled.
   - An option literally named `tied` or `unresolved` reads as that state in the prototype.
   - A `probability` outside 0 to 1 or not a number, or an empty `probabilities`, crashes the prototype. Add a failure row at exit 2, because `Threshold::judge` takes a checked `Probability`.
7. **Low. The policy wording overclaims, and the ban list is short.**
   - A token check cannot prove "opens only paths it was handed". State what the check refuses.
   - Add the clock, thread, and signal bans that the catalog policy already carries.
   - Consider a check that the `Audit` early return precedes `Environment::read` in `cli/mod.rs`.
   - The catalog check alone runs about 85 lines. Parameterize `catalog_policy_failures` to fit the 60-line budget.
8. **Low. Traceability.** Give a table of golden file, command line, and test name, as the prototype README does. For the table captures, run `git show be7cea2:scripts/tools/measure.py` rather than the working tree, which sits at 18c0dc8. Record the Python version beside the capture commands.

Items that pass:
- Exit codes follow `channels.md`: 2 for usage and input, 5 for a local file.
- The pure-core split follows ADR 0017. The cli already reads user files elsewhere.
- The tolerance of 1e-6 + 1e-12, with exact integers and matching types, fits Python's `round` against Rust's.
- No dependency is added. `sha2` and `serde_json` are already direct dependencies, and `Json::Number` keeps integers apart from floats.
- The no-request test counts connections on the listener and plants a key canary.
- The budgets are tight but realistic.
- The fixture count of 28 and the ruling's count of 15 golden files reconcile.

## 0114 findings

1. **High. The private repository.** This is the same issue as 0113-1, and the same fix applies.
2. **Medium. Planted bug 9 survives.** In `diff-decide-nokey` the moves are yes→no 1 and no→yes 1, so p is 1.0 under either rule. Fix: capture `golden/extra/diff-249-cuts-nokey.jsonl` from `diff 249/control.jsonl --compare-threshold 0.42`. It shows 67 moves from no to yes and `mcnemar_p` 0.0. The bug gives 1.0. A core test works too. Every other planted bug turns a named test red.
3. **Medium. The McNemar switch is not isolated.** The design runs the test "on (gained, lost)", which ties the test to the effect counts. The upstream fix keeps `gained` and `lost` as effects and widens only the discordant pairs. Fix:
   - Compute two discordant counters through one predicate, `discordant(oa, ob)`, in `core/measure/diff.rs`. Today it returns only wrong→right and right→wrong.
   - Write down the switch steps: the upstream goldens are rewritten; the pinned commit and SHA table are bumped; `diff-annotate` and the choose table are recaptured (a3 right→unresolved turns discordant, and `diff-choose` p becomes 0.5); the predicate flips; the spec sentence and the hand test update.
   - Cite `sdlc/issues/2026-09-24-diff-mcnemar-leaves-out-pairs-that-become-right-from-not-sure.md` in beatles-bench.

   Decision 4 is sound as the agent's call, because the ruling says the goldens decide.
4. **Low. Reuse.** Reuse is clean only once 0113-4 lands. Drop "`cli/audit.rs` for a shared reader" from the touched files and name the neutral module. Say whether `--key` refuses a bad `part` value. The prototype refuses one, and "ignored" is ambiguous.
5. **Low. Writing and vocabulary.**
   - The help sentence "Two cuts on one run cost nothing, because the probabilities are already saved." carries a trailing clause. Split it: "Two cuts on one run cost nothing. The probabilities are already saved."
   - Add `diff` to the `scan_help` loop, as in 0113-3.

Items that pass:
- Pairing, the effect table, the moves order, the summary order, and the table count line match the prototype. The pinned `spec/diff.md` line matches the prototype byte for byte.
- The diff-annotate capture pins pairing by answer name and two `withdrawn` effects, and it catches bugs 8 and 10.
- The log-space McNemar needs no big-integer crate.
- Decision 6, which refuses a repeated (name, id), breaks no golden. Every fixture has unique ids.

## Writing and names

Both tickets otherwise follow subject, verb, object, with no cleft sentences and no dash glosses. Neither names a customer or a private project other than beatles-bench, which 0113-1 covers. The fixtures carry `api.typesafe.ai` and `jev-1.13.0` in `meta`. `README.md` on main already names both.
