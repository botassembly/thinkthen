# Quick Fix qf-intake-and-readme-hygiene: generic evidence wording and one README sentence

Status: fresh Medium review accepted `6438ea17120c118287dab14a11576a0efe8d0b8e`; integrated as a bounded wording cleanup of commit `e8789b76` and one authorized root README sentence. Preparation began at main `7e1c3033`. No issue changes status and none counts as completed. Ian can overturn the wording of any replacement.

## Why

This repository goes public, so no tracked file may hold a name from the external private-name guard. Commit `e8789b76` ("Add the chain finding and the error-shape evidence to the tuning issues") reintroduced the machine-specific prefix of three evidence pointers after the earlier cleanup landed as `5ccf06b1`. Separately, the root `README.md` sentence "The model returns structured judgments, not generated prose." used a word the demo-standard vocabulary guard rejects; Ian authorized replacing exactly that sentence.

## Change

- 4 files touched, one line each. `git diff --numstat` shows exactly 1 insertion and 1 deletion per file.
- Three issue files, one guard match each: the machine-specific prefix immediately before `/experiments/` inside a backticked evidence pointer. The two follow-up pointers use local experiment 297's `LESSONS.md`; the pipeline issue names the local experiment 297 folder with `pipeline.py` and `runs/pipeline/` unchanged beside it. The chained-question status line also says "Evidence lives in the workspace at", matching how the clean issue files cite workspace experiments. The prefix was fully removable inside each pointer, so no edit hit the stop boundary.
- Preserved without change: every filing date, the GEPA tuning experiment numbers 296 and 297, `LESSONS.md` sections 11 to 13, both experiment folder names, the `notes/2026-09-27-optimization-lessons.md` write-up pointer, every numeric measurement (counts, accuracies, probabilities), every status, criterion, original finding, and every other line of every file. A programmatic digit-sequence comparison of each changed line against `HEAD` found no difference.
- Verified pointers: local experiment 297's `LESSONS.md`, `pipeline.py`, and `runs/pipeline/` all exist under the workspace root, so the local experiment folders hold the same artifacts. No artifact was renamed and no numeric evidence altered.
- Root `README.md`: replaced only "The model returns structured judgments, not generated prose." with "The model returns structured values." Wording only; no product claim, vocabulary guard, or `site/` change.

## Checks

- Root independently ran the actual lint guard: 30 external entries, zero tracked path matches and zero tracked text matches across 7005 files. Both match commands exited 1, meaning no match. A separate raw-byte scan also found none. The Pi report's proposed binary-image finding was false: lossy UTF-8 decoding removed bytes and created a match absent from the file. The original report and root correction remain in experiment 2037; no site edit is needed.
- `sdlc/scripts/demos-self-test`: exit 0, "28 cases pass". Log preserved with exit status in local experiment 2037's job folder outside the repo; the log itself scans 0 guard matches.
- `sdlc/scripts/pages`: exit 0, "1 coming, 22 green". `sdlc/scripts/tickets`: exit 0, "0 evidence failures from ticket 0120 on". `git diff --check`: clean.
- Not run, per scope: compilation, provider calls, full gates, stress. The Pi preparation itself made no commit or push and edited nothing outside the four files plus this record. Root subsequently committed the verified changes and obtained independent review.

## Root verification and review

Root verified that only the named line changed in each file, every digit sequence and evidence tail is identical to `HEAD`, and the cited workspace artifacts exist. A fresh independent review remains required before landing. This normalizes references without renaming evidence or changing an outcome. No history rewrite is claimed.

The independent reviewer checked the merged candidate across 7013 tracked files with the same 30-entry guard and found zero path or text matches; raw bytes also had no match. It verified the four one-line edits, identical evidence tails and numeric sequences, existing artifacts and retained 28-case demo-standard pass. Later merges may add unrelated tracked files. No runtime build or issue closure follows from this wording correction.
