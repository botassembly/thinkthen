# 0016: Names, pages, the license, and the push check

Branch `ticket/0016-names-and-pages`. Built 2026-09-19.

## What landed

`recipes/` is `transforms/`. The living pages use the four names of ADR 0015 item 1, and `README.md` holds the names table once. The specification pages say what the live probe measured, with every number's count beside it. `LICENSE` holds the MIT text and each package declares `license = "MIT"`. `.github/workflows/gate.yml` runs the four rungs on every push and every pull request.

No behavior of the binary changed. The only source edit is two help paragraphs in `crates/thinkthen/src/args.rs` that repeat sentences their pages changed. Both keep their line counts, so the ratchet stands at 7440 and `sdlc/ratchet.json` was not touched.

## The commits

| Commit | What it did |
| --- | --- |
| `b7b64dc` | Set the ticket to in progress |
| `5bc8120` | `git mv recipes transforms` and every path that follows: six green how-tos, `demos/README.md`, nine files under `probes/`, `transforms/README.md`, `plan.md`, and `documentation-plan.md` |
| `730d523` | The word "recipe" became "transform" on the living pages, and `README.md` gained the names table |
| `0524f9a` | The specification edits of ADR 0014, and the two help paragraphs |
| `65e54af` | `LICENSE`, the two `license = "MIT"` lines, the `policy.py` check, and the `README.md` line |
| `8363383` | `.github/workflows/gate.yml` |
| `9836bb7` | The cache action moved off the deprecated Node 20 runtime |
| `e9809c3` | An issue for the `replay-check.sh` failure found while checking acceptance |
| `dc6e5cc` | The word "recipe" became "transform" in four probe script comments |

## Where the word "recipe" stays, and why

`grep -ri recipe` over the living pages of the ticket finds nothing: `README.md`, `AGENTS.md`, `specification/`, `spec/`, `demos/`, `probes/`, `transforms/`, `plan.md`, `documentation-plan.md`, and tickets 0013, 0014, and 0015. Tickets 0013, 0014, and 0015 never held the word.

The word stays in these places.

- **History.** `sdlc/planning/adr/0008`, `0009`, `0010`, `0012`, `0013`, `0014`, and `0015`; `sdlc/records/0008`, `0011`, and `0012`; `sdlc/issues/` for the three ticket-0008 and design-capture issues; and tickets 0008, 0011, and 0012, which have landed. ADR 0015 rules that history keeps its words.
- **Ticket 0016 itself.** Its Current Facts and Scope name `recipes/` as the folder that existed when it was written. A ticket is its own contract and it is not on the list of living pages.
- **`sdlc/planning/open-concerns.md`.** Its own first line says the text below is kept as it was written. It is a record of what Ian ruled on.
- **`sdlc/planning/design-study.md`, one line.** "the vendor's own reranking recipe" is the everyday word for a method and never this repository's noun. `specification/rank.md` carried the same phrase and now reads "the vendor's own reranking method", because a specification page must not use the retired word at all.
- **The probe data.** `probes/01-find-vs-rank/docs.jsonl`, its three run files, and four recordings hold the word inside made-up documents about cooking. A recording is keyed by its request, and no request changed.
- **`sdlc/issues/2026-09-19-replay-check-fails-on-meta-tool.md`, one line**, which names the rename that did not cause the failure.

## The pages the numbers went onto

Every number comes from `sdlc/records/0011-the-live-probe.md`, carries its count, and sits in a sentence that says the cases are few and made up.

- **`specification/find.md`** left Draft and is Settled. It gained "What one run measured" (16 of 16 against 15 of 16, 20 requests and 11,063 input tokens against 239 and 69,143, documents of 11 to 14 lines with 12 to 14 options) and "Saying that nothing fits" (`none` on 4 of 4 blank documents and 0 of the 16 answerable, 20 requests and 11,183 input tokens). It says the ticket that builds `find` first repeats the comparison on documents of 100 to 250 lines. `--none` joins the options table and exit 3 joins the exit codes. The first open point is closed and removed. `specification/README.md` and `sdlc/planning/plan.md` follow.
- **`specification/score.md`** lost "Rating is the weakest thing a decider model does." It keeps the rubric sentence and gained the split ADR 0014 item 3 names: a rank correlation of 0.9703, within one level on 40 of 40, the exact level on 31 of 40, one step high on 9 of 40, and a cut tuned on labeled cases.
- **`specification/rank.md`** lost the same sentence about rating.
- **`specification/choose.md`** lost "Reversing the option order changed none of fifty picks." It gained 2 of 60 reversed, 1 of 60 shuffled, every change on the catch-all `other`, the rule to keep the order fixed once a cut is tuned, the rule to put the catch-all last, and the added option that fits nothing at 0 of 60 picks and a probability of 0.0 on all 60 rows. It says a label that overlaps a real one is untested. `--options` now requires `--jsonl`, in the options table and in its own section, because a line of text holds no pointer.
- **`specification/decide.md`** gained the caution about a planted claim, with the three defenses of ADR 0014 item 5: `--field`, a band, and an eval with hostile cases. The numbers are 0.04 or less for a command aimed at the judge in seventeen wordings, as much as 0.57 for a planted claim, 0.02 for a third one, and 18 of 20 holding under the band with 2 turning unresolved.

## The two help paragraphs

`Choose` says "type a catch-all such as other yourself, last" and "Keep the option order fixed once a cut is tuned, because a run with a reordered list is a different measurement." `Score` lost the sentence about rating and gained "A later run ordered forty made-up reports well and ran one level high on 9 of 40, so tune a cut on labeled cases." Each rewrite holds its old line count. No test or page asserts this text.

## The license

`LICENSE` holds the MIT text with "Copyright (c) 2026 Ian Maurer", as ADR 0015 item 4 rules. `crates/thinkthen/Cargo.toml` and `crates/thinkthen-core/Cargo.toml` each declare `license = "MIT"`. `README.md` names it in one line. `publish = false` stays on both packages, because ADR 0015 ruled on the license and not on publishing.

`sdlc/scripts/policy.py` refused both files at first: it failed a package that declared any license, "while the repository is private". That check now requires `license = "MIT"` and names ADR 0015. A rule that Ian overturned became the check for the new rule rather than a deleted one.

## The workflow

`.github/workflows/gate.yml` triggers on `push` and `pull_request`, runs on `ubuntu-24.04`, and holds `permissions: contents: read`. It checks out the commit, installs the toolchain that `rust-toolchain.toml` pins, caches the cargo registry and `target/` on `Cargo.lock` and `rust-toolchain.toml`, installs `jq` and `mustmatch`, then runs `install`, `lint`, `test`, and `spec` as four named steps. Its header comment says that a live call to the paid backend never happens there. No step holds a secret, reads a key, or sets a `THINKTHEN_` variable.

`mustmatch` v0.1.0 is installed on the Linux box as a `uv` tool. The public repository `github.com/genomoncology/mustmatch` publishes v0.1.0 to PyPI as wheels that carry the binary, and `pipx` is already on the runner image, so the workflow runs `pipx install mustmatch==0.1.0`. That is one pinned line and adds no action. `jq` is downloaded from the pinned public release `jq-1.7.1/jq-linux-amd64` into `$HOME/.local/bin`, which the step adds to `$GITHUB_PATH`. The Linux box runs jq-1.7 and the runner runs 1.7.1. Every demo passed on both.

The first pushed run was green: https://github.com/botassembly/thinkthen/actions/runs/35465792739. It printed `demos: 13 green, 8 red` on rung 3. It also raised one annotation: `actions/cache@v4` targets Node 20, which GitHub deprecated. Commit `9836bb7` moved it to `actions/cache@v5`, and that run was green with no annotation: https://github.com/botassembly/thinkthen/actions/runs/35465888948.

## The ladder

| Rung | Script | Result |
| --- | --- | --- |
| 0 | `sdlc/scripts/install` | exit 0 |
| 1 | `sdlc/scripts/lint` | exit 0, ratchet `crates 7440/7440` |
| 2 | `sdlc/scripts/test` | exit 0 |
| 3 | `sdlc/scripts/spec` | exit 0, `demos: 13 green, 8 red` |

Every rung ran as `env -u THINKTHEN_API_KEY -u THINKTHEN_BASE_URL flock -w 1800 /tmp/thinkthen-gate.lock sdlc/scripts/<rung>`. No live call of any kind went out, and `sdlc/scripts/live` never ran.

## `probes/replay-check.sh`

It fails, and it failed before this ticket. All six probes exit 1 and all 18 run files are reported as different. The cause is `meta.tool`, which ticket 0012 added to every result after ticket 0011 wrote these rows. Setting `meta.tool` aside as well as `meta.replayed` reproduces all 639 rows across all 18 files, with no other difference anywhere.

The failure reproduces on the commit before the rename, checked by stashing this branch's work and running the check again. `sdlc/issues/2026-09-19-replay-check-fails-on-meta-tool.md` holds the finding and three levers. This ticket did not take one, because it changes words, pages, a license file, and a workflow, and it changes no check.

## What was chosen where the ticket was silent

Each of these is cheap for Ian to overturn.

1. **`--none` is the spelling** of `find`'s `none` option on `find.md`. The ticket says the page gains the option and excludes new options from being built. No code was written.
2. **`find.md` is Settled, not Draft.** ADR 0015 accepted `find`, and a Settled page is the one code may be built against.
3. **The names table went into `README.md`**, not `specification/README.md`. The ticket allowed either. `README.md` is the tutorial and the first public page. `specification/README.md`, `demos/README.md`, and `transforms/README.md` link to it.
4. **`policy.py` now requires `license = "MIT"`** rather than losing its license check.
5. **`ubuntu-24.04`, `pipx install mustmatch==0.1.0`, the pinned `jq-1.7.1` binary, `actions/checkout@v5`, and `actions/cache@v5`.** The ticket named Ubuntu, pinned versions, and public sources, and left the rest open.
6. **`sdlc/planning/rust-standards.md` followed.** Its "When the repository goes public" section said `LICENSE` and a CI workflow still waited. It now says both are here and that `CHANGELOG.md` and `deny.toml` still wait.
7. **`sdlc/scripts/install` and four probe script comments** use the new word. The ticket listed them under neither the living pages nor history. A comment that says "recipes" beside a path that reads `transforms/` is a slip, so the word followed the folder.
8. **`sdlc/planning/open-concerns.md` and `sdlc/planning/design-study.md` keep the word,** for the reasons above.
9. **The `replay-check.sh` failure became an issue** rather than a fix in this ticket.

## Left for another ticket

- The `replay-check.sh` fix, in the issue above.
- ADR 0015 item 7: a second agent reads the whole repository once for coherence and reports where two living pages disagree.
- `CHANGELOG.md` and `deny.toml` with `cargo deny` in `lint`, which `rust-standards.md` still lists as waiting.
- `find` itself, slice 11, whose ticket first repeats the comparison on documents of 100 to 250 lines.
