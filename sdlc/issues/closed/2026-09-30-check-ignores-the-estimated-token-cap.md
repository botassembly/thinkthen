# check ignores the estimated input token cap

Status: closed 2026-09-30 by ticket 0364. Reported by the site team on 2026-09-30 from a capped `check` against Liquid, and confirmed by reading main `d75a4bdf1`. Owner: the queue owner, as batch C3 in `../planning/issue-priorities-2026-09-30.md`. It blocks 0.1, because the token cap is a spend guard. Resolution: `check` now reserves against the estimated input token cap and refuses a probe the cap cannot admit with exit 2. It retries a retried status three times, the shared default, so `check.md` and the code agree on sixteen attempts at most.
Kind: bug

1. **The cap does not stop `check`.** `THINKTHEN_MAX_ESTIMATED_INPUT_TOKENS_TOTAL=1 thinkthen check --backend liquid --timeout 90` sent four probes and 748 input tokens with no refusal. `crates/thinkthen/src/cli/check.rs:51-66` builds its engine without `.with_process_budget(...)`. The asking commands set it at `cli/asking.rs:99-104`. With no budget, `reserve_send` (`engine/send_budget.rs:91`) reserves nothing. `specification/settings.md:86` charges every live System One body, and `check` uses the production transport.
2. **`check.md` and the code disagree on retries.** `specification/check.md` says `--max-retries` keeps its default of 3 and a check makes at most sixteen attempts. The code uses `MAX_RETRIES = 2`, which gives at most twelve.

Not a bug: the same report saw `check` run more than five minutes past `--timeout 90`. `--timeout` bounds each attempt and each retry wait (`settings.md:77`), not the whole run. Probes run one after another, so the worst case at 90 seconds is about half an hour.

Fix: pass the process budget in `check.rs`, with a loopback test that a cap of 1 refuses before the first probe and counts zero requests. Make `check.md` match the code's retry count.
