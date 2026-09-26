# probes/speed/

The speed test of ticket 0145, batching design ticket S1. It measures requests, wall time and tokens for every function and every Beatles Bench job. Ian's target is `filter` over the 306 Beatles titles in under half a second at the default throttle of 4, on a named build.

## Files

- `functions.jsonl`: one row per function. It names the verb, its arguments, its input under `workloads/`, `items`, `floor`, and a `list` entry while the function is still to batch.
- `workloads/`: made-up inputs. Nothing in them is real text.
- `measure.py`: the runner, with modes `gate`, `plan` and `live`.
- `job.sh`: the live job for `sdlc/scripts/live`.
- `runs/NAME/`: each authorized live run's `rows.jsonl` and `build.json`.

`items` is the requests today's command sends, one per item. `floor` is the fewest requests that carry the workload once the function batches. `list` holds `ticket`, the batching design's label, and `number`, the ticket's number once it has one.

## The gate

`crates/thinkthen/tests/speed.rs` runs `measure.py gate` in the `test` rung against the loopback backend's generic arm. It needs no engine and no real key: each command gets the made-up key `loopback-not-a-key`. Each command runs in a private home with `--no-cache`, and `thinkthen status --json` there gives its requests. The test checks that count against the socket.

The list rule, in order:

1. A listed row whose numbered ticket has landed fails.
2. A listed row fails when it sends at most `floor`, because it now batches. It fails when it sends more than `items`. A count between the two passes, so partial batching stays listed.
3. An unlisted row fails when it sends more than `floor`.

A batching ticket removes its function's `list` entry in the commit that makes it batch. A ticket that takes a number adds it to the entry. A ticket that changes a listed function's per-item count updates its `items`.

No workload record is a content cut. A cut is a record whose SHA-256 of compact JSON, first 8 bytes read big-endian, is 0 mod 4,096. The values mod 4,096, in file order:

- `titles.txt`: 1606 428 2799 593 2720 556 4026 3306 2633 3497 2425 162
- `sentences.txt`: 3555 2804 4061 167 1898 868 2138 839 1007 859 806 3518
- `records.jsonl`: 3717 3742 3112 4064 1070 2491 4026 1041 2793 276 4006 1208

For a line with no quote mark or backslash, `printf '"%s"' LINE | sha256sum` gives the same hash.

## The live part

Run it only with Ian's authorization, named for the run, and never from a gate:

```sh
env -u THINKTHEN_API_KEY python3 probes/speed/measure.py plan BENCH
sdlc/scripts/live --max-tokens 2000000 probes/speed/job.sh BENCH NAME
```

`BENCH` is a Beatles Bench checkout. `plan` builds the debug binary with no key, checks the preconditions and prints every command with an estimate of its input tokens. It sends nothing. Commit first, because the checkout must be clean.

`live` refuses at exit 2 when the checkout is dirty, the binary is missing or older than the last commit to the command's sources, `THINKTHEN_BASE_URL` is set, the bench titles do not hash to `3250fed3…`, or `runs/NAME/` exists. Each command's environment holds only `PATH`, a private `HOME` and the key the door gave it. So it measures the built-in address, and it passes no proxy variable. It runs one command at a time. It stops starting commands at 1,800,000 counted input tokens. A command that reports no tokens counts its estimate.

It measures the 306-title target three times, the same filter at `--jobs 16`, every function row, `recognize` by step, `annotate` by field, each bench case file matching `functions/*/*-cold.jsonl`, `functions/*/*-context.jsonl` or `functions/decide/decide-love.jsonl`, the bench's 1,501 questions grouped by function and question, and `decide` over one record at 8,000 to 56,000 bytes of evidence. It adds a `--batch 1` arm, `--boundary run` and the `--context` arms when the command's help names them. The bench's `diff` asks nothing, and `audit/rows-context.jsonl` is saved rows, not cases.

It writes `runs/NAME/rows.jsonl`, one row a command, and `build.json` with the commit, the binary's SHA-256, the version, the bench commit and the load. It keeps no output text, standard error, environment or key. At the end it prints one table per measurement and the target verdict: met when the median of the three default runs is under 0.5 s.
