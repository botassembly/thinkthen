---
flow: build
priority: 50
opens: crates spec specification/fixtures sdlc/scripts sdlc/ratchet.json demos/01-refund-gate
---

# 0006: The two variables, the live script, and the first green demo

Status: in progress

## Outcome

`thinkthen` reads its key from `THINKTHEN_API_KEY` and its address from `THINKTHEN_BASE_URL`, as ADR 0010 rules. `sdlc/scripts/live` is the one door for a paid call. Demo 01 is recorded against the hosted model and replays green in the `spec` rung.

## Current Facts

Ticket 0005 landed the flat `decide`. The built-in profile still names an older key variable, and the address is fixed unless the hidden `--url` is given. `AGENTS.md` names `sdlc/scripts/live`, and the script does not exist (`sdlc/issues/2026-09-19-the-live-script-does-not-exist.md`). The key in `THINKTHEN_API_KEY` works: four calls by hand returned answers on 2026-09-19. Every demo is red.

## Scope

- The key is read from `THINKTHEN_API_KEY` unless the hidden `--key-env` names another variable.
- The address comes from the hidden `--url`, then `THINKTHEN_BASE_URL`, then the default base `https://api.typesafe.ai/v1`. The tool posts to `BASE/systemone`. A base with a trailing slash is accepted. A base that is not an `http` or `https` address is a usage error before any request. An empty variable counts as absent.
- The request bytes do not change, so every pinned digest holds.
- `--dry-run` shows the address the run would use and still reads no key.
- `sdlc/scripts/live` runs one named live job by hand. It refuses to run when `THINKTHEN_API_KEY` is absent. It reads the spend limit and the tokens spent so far from one small file under `sdlc/`, refuses to start when the limit is reached, and adds the input tokens of every call it made. It never prints the key, and it never runs from another rung.
- `demos/01-refund-gate/record.sh` runs through the live script and writes the recording folder. The recording is committed. Demo 01's status line turns green, and the `spec` rung replays it with no key and no network.
- The issue about the missing live script is closed by a line in it that names this ticket.

Excluded: the configuration file, profiles, and the removal of `--profile`, `--adapter`, and `--key-env`. Those wait on Ian's answer to the Proposed section of ADR 0010. Build Settled sections only.

## Acceptance

- Integration tests against a local listener prove the order of the three address sources, the path `BASE/systemone` with and without a trailing slash, the refusal of a bad base with zero requests, and that an empty variable counts as absent.
- A test proves that the key is read from `THINKTHEN_API_KEY` and that no error, plan, recording, or Debug output carries it.
- The pinned recording digest from ticket 0004 still holds.
- `sdlc/scripts/spec` prints `demos: 1 green` and touches no network. Run it once with the network variable unset and the key unset to prove it.
- The live script's refusal at the limit has a test that needs no network.
- `sdlc/planning/plan.md` gains the tokens that the recording spent.
- The ratchet equals the measured total, and the commit that moves it says what grew and why.
- The whole ladder is green, and a second agent reviews the public surface change.
