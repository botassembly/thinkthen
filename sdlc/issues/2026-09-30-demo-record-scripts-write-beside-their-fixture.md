# Demo record scripts write a live store beside their fixture

Status: open. Found by ticket 0304 slice 5 while it deleted the old entries beside each demo's `thinkthen.jsonl`.

Ticket: 0350, ready since 2026-09-30; batch order in `../planning/issue-priorities-2026-09-30.md`.

Kind: debt

Pay when: a demo is next recorded live, before 0.1.

Debt: 024

Severity: low

Keeping it means the next live run of these scripts leaves a folder that `--replay` refuses, because it holds both `thinkthen.jsonl` and `thinkthen.sqlite`.

Fourteen of the fifteen `demos/*/record.sh` scripts still record with `--record recording/` or `--cache recording/` into the committed folder and then list its files. Since 0304 slice 2 that writes `recording/thinkthen.sqlite` beside the fixture. Only `demos/27-test-with-no-network/record.sh` records into a scratch folder and runs `thinkthen cache convert recording/`, as `specification/recording.md` asks. Each script's closing `ls` line also counts files that no longer track exchanges.

Done when each script records into a scratch folder, copies its `thinkthen.sqlite` into `recording/`, runs `thinkthen cache convert recording/`, and prints the convert summary, as demo 27 does. The change needs no live run to review. Its first live run proves it.
