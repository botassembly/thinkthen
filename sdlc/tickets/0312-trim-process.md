# 0312: Trim process pedantry

Status: landed

Lane claude-1. Branch `ticket/0312-trim-process`. Plan: `sdlc/planning/cleanup-2026-09-30.md`, ruling 8.

## Outcome

The process docs are short and hold only rules that prevented real, recurring defects. Checks that only police prose wording or page shape are gone. Checks that protect behavior, secrecy, spend, boundaries, and ticket evidence stay.

## Evidence

- Starts from: main `5e6631a83`. `ticket-preparation.md` runs 4,056 words, mostly case narratives. `AGENTS.md` forbids weakening any lint table. `sdlc/scripts/demos` scans pages and help for banned words and measures ADR 0016 page limits. `sdlc/scripts/pages` holds three copies of the how-to list to the same titles and states.
- Keeps: the private-name scan, `policy.py` (including the adapter-word seam and recording header words), the Rust ratchet, the tickets evidence check, `catalog.py`, `named-answers.mjs`, and in `demos`: the `like ""` refusal, the `--replay` folder guard, the `set +e` refusal, and the rule that every green page and every `bash` block asserts. `pages` keeps its broken-link check. AGENTS keeps boundaries, secrecy, spend and live rules, gates, file cap, lanes, and the public-repo rule.
- Changes: rewrite `ticket-preparation.md` to about 800 words; tighten `AGENTS.md`; state the short ticket form in `sdlc/tickets/README.md`; remove the wording and page-shape checks listed below and their self-test cases.
- Proof: `sdlc/scripts/lint` passes; `python3 sdlc/scripts/tickets` passes; `sh sdlc/scripts/demos-self-test` and `python3 sdlc/scripts/pages --self-test` pass. The spec rung's real demo run needs the built binary and `mustmatch`; see the build result.
- Defers: ADR 0016's limits stay as writing guidance without a script. The how-to list still has two copies, `demos/README.md` and `documentation-plan.md`, kept in step by hand; merge them into one. `site/scripts/named-answers.mjs` is marketing's and unchanged. Shortening `review-lessons.md` and other planning pages.

## Checks reviewed

| Check | What it polices | Decision |
| --- | --- | --- |
| `demos` vocabulary scan | banned phrases (`decider model`, `rating`, `judgment`, `unresolved`, ...) in pages, README, and every command's help | remove: wording only |
| `demos` ADR 0016 rules 1, 2 (line positions), 3 (count), 4, 5, 7, 8 | page length, word count, where the first block sits, steps, closing links, title names the commands | remove: page shape only |
| `demos` "How to" title, "What can go wrong" heading, index lists the folder | page headings and index wording | remove: wording only |
| `demos` asserting block and silent `bash` block | a green page proves something; a shown command is tested | keep: proof |
| `demos` `like ""`, `--replay` folder, `set +e` | loose assertions, network reach, hidden failures | keep: proof and no-network |
| `pages` list, title, state, front-window, heading, and "Absorbed by" agreement | three copies of the how-to list say the same words | remove: wording only |
| `pages` broken relative links | a link goes somewhere | keep; fold its self-test into `pages --self-test` |
| `named-answers.mjs` | sample code names answers and reads exit codes safely | keep: code samples, shared with the site |
| `tickets` | five Evidence bullets | keep: workspace rule |
| `policy.py` seam and catalog words | vendor and effect names stay behind their boundaries | keep: architecture |

## Build result

| Check | Result |
| --- | --- |
| `sdlc/scripts/lint` | pass, exit 0; `CLAUDE.md` 3,351/5,000 characters |
| `python3 sdlc/scripts/tickets` | 0 evidence failures |
| `sh sdlc/scripts/demos-self-test` | 11 cases pass |
| `python3 sdlc/scripts/pages --self-test`, `pages` | pass; 26 pages, every relative link resolves |
| `sdlc/scripts/demos` over the real pages | 23 green, 0 red, with the lane's existing `target/debug/thinkthen` |

Word counts: `ticket-preparation.md` 4,056 to 706. `AGENTS.md` 637 to 457 words, 4,572 to 3,351 characters. `sdlc/tickets/README.md` 77 to 159 words, because it now states the short form.

Removed: `sdlc/scripts/pages-self-test`; the vocabulary scan, help scan, and ADR 0016 layout rules in `demos`; the list, title, state, and front-window agreement in `pages`. Scripts lost about 590 lines. ADR 0016, `demos/README.md`, `documentation-plan.md`, and `sdlc/scripts/README.md` now describe what the scripts check.

Code review accepted with five fixes: the documentation plan names the amendment; `demos` fails when the index lists a page as green and the page does not say `Status: green`, with a self-test case; AGENTS keeps the shared toolchain and cache lock rule; ticket preparation keeps the validate-before-first-write rule; `quality-plan.md` checklist items 10 and 18 mark the vocabulary lint superseded.

## What the build taught us

- Most of the demo runner's rules measured page shape. Only four protect a proof: an asserting block, no silent `bash` block, no `like ""`, and no `set +e`. The `--replay` folder guard keeps demos off the network.
- The three-way how-to list agreement existed because the list has three copies. The check policed a duplication instead of removing it.
- `quality-plan.md` still proposes a vocabulary lint over the marketing word list. It is a proposal; ruling 8 now weighs against it.
- The real demo run used the lane's prebuilt binary. The spec rung rebuilds first; this change touches no Rust, so the result stands.
