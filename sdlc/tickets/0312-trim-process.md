# 0312: Trim process pedantry

Status: in progress

Lane claude-1. Branch `ticket/0312-trim-process`. Plan: `sdlc/planning/cleanup-2026-09-30.md`, ruling 8.

## Outcome

The process docs are short and hold only rules that prevented real, recurring defects. Checks that only police prose wording or page shape are gone. Checks that protect behavior, secrecy, spend, boundaries, and ticket evidence stay.

## Evidence

- Starts from: main `5e6631a83`. `ticket-preparation.md` runs 4,056 words, mostly case narratives. `AGENTS.md` forbids weakening any lint table. `sdlc/scripts/demos` scans pages and help for banned words and measures ADR 0016 page limits. `sdlc/scripts/pages` holds three copies of the how-to list to the same titles and states.
- Keeps: the private-name scan, `policy.py` (including the adapter-word seam and recording header words), the Rust ratchet, the tickets evidence check, `catalog.py`, `named-answers.mjs`, and in `demos`: the `like ""` refusal, the `--replay` folder guard, the `set +e` refusal, and the rule that every green page and every `bash` block asserts. `pages` keeps its broken-link check. AGENTS keeps boundaries, secrecy, spend and live rules, gates, file cap, lanes, and the public-repo rule.
- Changes: rewrite `ticket-preparation.md` to about 800 words; tighten `AGENTS.md`; state the short ticket form in `sdlc/tickets/README.md`; remove the wording and page-shape checks listed below and their self-test cases.
- Proof: `sdlc/scripts/lint` passes; `python3 sdlc/scripts/tickets` passes; `sh sdlc/scripts/demos-self-test` and `python3 sdlc/scripts/pages --self-test` pass. The spec rung's real demo run needs the built binary and `mustmatch`; see the build result.
- Defers: ADR 0016's limits stay as writing guidance without a script. `site/scripts/named-answers.mjs` is marketing's and unchanged. Shortening `review-lessons.md` and other planning pages.

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
