---
flow: build
priority: 60
opens: recipes transforms demos probes spec specification README.md AGENTS.md LICENSE Cargo.toml crates/thinkthen/Cargo.toml crates/thinkthen-core/Cargo.toml .github sdlc/scripts sdlc/planning/plan.md sdlc/planning/documentation-plan.md sdlc/tickets
---

# 0016: Names, pages, the license, and the push check

Status: in progress

## Outcome

The living pages use the names of ADR 0015, the specification says what the live probe measured, the repository carries the MIT license, and the gate ladder runs on every push. No behavior of the binary changes.

## Current Facts

Ian ruled on 2026-09-19, and ADR 0015 holds the rulings. ADR 0014 names the page edits that follow the live probe, and `sdlc/records/0011-the-live-probe.md` holds every number. `recipes/` holds seven `jq` files with an example each, and the rows they read. Green how-tos 04, 13, 24, 25, 28, and 38 call paths under `recipes/`, and so do scripts under `probes/`. The repository has no `.github` folder and no license file. `sdlc/scripts/install` needs `cargo`, `python3`, `node`, `mustmatch`, and `jq` on the path. `mustmatch` is public at `github.com/genomoncology/mustmatch`.

## Scope

- `git mv recipes transforms`. Every living page, script, and how-to that names the old path follows. The word "recipe" becomes "transform" in the living pages: `README.md`, `AGENTS.md`, `specification/`, `spec/`, `demos/`, `probes/README.md`, `transforms/`, `sdlc/planning/plan.md`, `sdlc/planning/documentation-plan.md`, and the tickets that have not landed (0013, 0014, 0015). `transforms/README.md` says that a transform is a metric or a policy, as ADR 0015 item 1 defines them, and that all seven today are metrics.
- History keeps its words. No ADR, no record under `sdlc/records/`, no landed ticket, no issue, and no recording changes. A recording is keyed by its request, and no request changes.
- The names table of ADR 0015 item 1 goes into `README.md` or `specification/README.md`, once, and other pages link to it.
- The page edits of ADR 0014, each with the numbers and the limits from record 0011:
  - `find.md` leaves Draft. It gains the `none` option and closes its first open point. It states the measured size of 11 to 14 lines and says that the ticket that builds `find` tests 100 to 250 lines first. `specification/README.md` and `plan.md` follow.
  - `score.md` and `rank.md` lose the sentence that calls rating the weakest thing the model does. `score.md` says what was measured: the order held well, the absolute level ran one step high on nine of forty, and a cut on the number is tuned on labeled cases.
  - `choose.md` loses its older sentence about fifty picks and gains this run's numbers. It says to keep the option order fixed once a cut is tuned, to put the catch-all last, and which kind of added option was measured. It also corrects `--options` to require `--jsonl`, because a line of text holds no pointer.
  - `decide.md` gains the caution about a claim planted in the evidence, with the three defenses ADR 0014 item 5 names.
  - The help text in the binary that repeats any changed sentence follows its page. This is the only source change allowed, and the ratchet moves only if the measured total moves.
- `LICENSE` holds the MIT text with the line `Copyright (c) 2026 Ian Maurer`. Each `Cargo.toml` package gains `license = "MIT"`. `README.md` names the license in one line.
- `.github/workflows/gate.yml` runs `sdlc/scripts/install`, `lint`, `test`, and `spec` on every push and every pull request, on Ubuntu, with the pinned toolchain of `rust-toolchain.toml`. It installs `mustmatch` and `jq` from public sources at pinned versions. It holds no secret, reads no key, and sets no `THINKTHEN_` variable. A comment in the file says that a live run never happens there. The first pushed run must be green, and the record gives its URL.

Excluded: any new command or option, the `transform list` and `transform show` commands, the `--from` option that ADR 0013 proposes, and any live call.

## Acceptance

- `grep -ri recipe` over the living pages listed above finds nothing, and the record lists every place the word stays and why.
- Every green how-to still runs green from its new paths with the key unset, and the demo count holds at 13 green and 8 red.
- `probes/replay-check.sh` still reproduces every probe byte for byte.
- Every number added to a specification page matches record 0011, and every sentence that carries one also carries its count and says the cases are few and made up.
- The push check is green on GitHub for the branch's last commit.
- The whole ladder is green, and the ratchet equals the measured total.
