# The PostgreSQL `single_cancel` check has a wall-clock limit that fails under load

Status: open. Found by ticket 0336's first Linux check. Owner: ticket 0338, which removes fixed wall-clock limits from routine tests, or 0335 slice 2 if it moves this check first.

Kind: debt

Pay when: 0338 or 0335 slice 2 lands.

Keeping it risks a false red on a busy machine, which teaches builders to rerun reds.

## What happened

`single_cancel` in `databases/postgresql/check.sh:533` requires the cancel to finish within 200 ms (line 541). It took 308 ms once at load 17 on 16 cores. Three reruns passed.

## What should happen

The routine check proves the cancel and its message with no timing limit. A timing limit runs only under `test-stress --run` or the release suite.
