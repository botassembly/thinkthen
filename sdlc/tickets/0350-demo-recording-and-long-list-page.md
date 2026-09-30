# 0350: Demo record scripts write a clean fixture, and a demo shows a first cut before a long list

Status: in progress. Plan: `sdlc/planning/issue-priorities-2026-09-30.md`, batch B1. Pays Debt 024, `sdlc/issues/closed/2026-09-30-demo-record-scripts-write-beside-their-fixture.md`, and page 11 of `sdlc/issues/2026-09-25-docs-how-tos-and-spec-claims-owed.md`, which closes row 9 of `sdlc/issues/2026-09-20-new-user-stumble-register.md`.

## Outcome

Every `demos/*/record.sh` records into a scratch folder, copies its `thinkthen.sqlite` into `recording/`, runs `thinkthen cache convert recording/`, and prints the convert summary, as demo 27 does. A new demo shows a list of more than 255 candidates narrowed with `grep` or a query, then `find` or `relate` on the survivors. It runs green under `--replay` in `sdlc/scripts/demos`.

## Evidence

- Starts from: the debt issue; `demos/27-test-with-no-network/record.sh`, the one script that already records into a scratch folder and converts; the other 14 scripts, which record or cache into their committed `recording/` folder directly (checked on main `d8018dd96`; demo 16 names the folder by its full path); `specification/recording.md` on `cache convert` and on a folder holding both `thinkthen.jsonl` and `thinkthen.sqlite`; page 11's text; `demos/README.md`, the one how-to list; ADR 0018.
- Keeps: each demo's committed `recording/thinkthen.jsonl`, its page and its expected output. `sdlc/scripts/demos` stays green with no network and a scratch usage folder. No record script runs in any gate.
- Changes: the 14 record scripts follow demo 27's pattern, and each closing `ls` line prints the convert summary instead of counting files. One new demo folder with its page, inputs, `record.sh`, `recording/thinkthen.jsonl` and expected output, listed in `demos/README.md`. Its fixture comes from one capped paid recording through `sdlc/scripts/live` (ruling 13), or from a synthetic loopback fixture if the builder shows that is simpler and honest about it. The two issue items close: the debt issue moves to `closed/`, page 11 leaves the docs issue, and row 9 leaves the stumble register, each citing the landing commit.
- Proof: `bash -n` on every changed script, and one script run end to end against the loopback conformance backend in a scratch copy, which leaves only `thinkthen.jsonl` in `recording/`. The new demo passes `sdlc/scripts/demos` under `--replay`, and fails once when its expected output is changed. `tickets` and `lint` in a clean checkout.
- Defers: the other docs pages, which follow 0.1.

## What the build taught us

- The fixture needed no paid call. Grep's twelve survivors equal how-to 15's policy byte for byte, so the `find` request is the same and how-to 48 copies that page's two recorded answers. The page says so.
- Demo 12's own EXIT trap would have fought `scratch_dir`, which takes the EXIT trap. The repaired input now lives in the scratch folder, so the trap went.
- Demos 40 and 41 used `--record recording/ --replay recording/` to reuse held answers across reruns. In a scratch folder that reuse lasts one run; a rerun sends every question again.
- A loopback run of demos 01 and 12 in a scratch copy, against `conformance-backend`'s generic arm, left only `thinkthen.jsonl` in `recording/`. Changing one expected sentence on the new page failed one block.
- The issue closures cite the landing merge in the record commit after landing, because the merge hash does not exist before it.
