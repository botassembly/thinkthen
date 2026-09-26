# The site's recognize accuracy claim overstates the shipped build

Status: Open. Filed 2026-09-26 by the queue owner for the marketing lead, who owns `site/` (`sdlc/planning/ownership.md`), from local experiment 273, report 10, finding 6. Blocks 0.1 under goal 4, because it is a public accuracy claim. Ian can overturn this placement.

## Where

`site/src/data/beatles.mjs` line 286, on the Beatles Bench page "What Jev knows" (`/learn/beatles-bench/what-jev-knows/`), reads: "The best is `recognize` at 0.96."

## What is wrong

Report 10 traced the 0.96. It is song precision on 48 easy template sentences, scored leniently, from the bench's `reports/results.md`. The shipped build scores lower on ordinary prose. On the 100-sentence edge-case key, experiment 270 measured 70.8% recall and 76.3% precision. The 87.5% figure in the recognize design comes from a simulation of a design that is not built yet. The bench's relate F1 of 0.72 is closed-book, so it measures what the model knows, not what it extracts.

A reader who sizes review work on 96% accuracy meets about 70% recall on prose with possessives, quotes and touching names.

## Checked on main

Verified: `beatles.mjs:286` reads as quoted. The trace of the 0.96 and the key scores come from the report. They were not rerun here.

## What would fix it

Name the measure in the claim: precision, on one kind of name, on template sentences. Put the shipped build's key score beside it. Label the relate figure as knowledge, not extraction. Backlog ticket R8 publishes the measured numbers for the shipped recognizer, so the page can cite R8's record once it lands.

## Done when

The page names what the 0.96 measures and shows the shipped build's score on the key beside it.
