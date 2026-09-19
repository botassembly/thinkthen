---
flow: build
priority: 50
opens: crates spec specification demos sdlc/ratchet.json README.md
---

# 0007: Remove profiles and the configuration surface

Status: in progress

## Outcome

Version one has no configuration file, no profile, and no `config` command, as ADR 0010 rules. The backend is named by `THINKTHEN_API_KEY`, `THINKTHEN_BASE_URL`, and `--model`. The pages, the demos, the help, and the code all say the same thing.

## Current Facts

Ticket 0006 landed the two variables beside the older surface. The binary still accepts `--profile`, `--adapter`, and `--key-env`, a result still carries `meta.profile` and `meta.adapter`, and the plan document still carries `profile`. `specification/config.md` is Draft with a note that it waits on Ian, and he answered on 2026-09-19. Demo 10 exists for profiles and the configuration file.

## Scope

- The pages first. `specification/config.md` leaves, and `specification/roadmap.md` gains an entry: what the file held, why it left (two variables and `--model` say everything a profile said), and what would bring it back (a user with several endpoints for whom a variable in front of the command is not enough). `backends.md` loses its profile section and its ad-hoc rules and keeps one rule: the key goes to the address the user named. `result.md` drops `meta.profile` and `meta.adapter`. `channels.md` and `decide.md` drop the removed options. Every other page, both READMEs, `demos/FINDINGS.md`, and every demo are swept for profiles, `--profile`, `--adapter`, `--key-env`, `--config`, `THINKTHEN_PROFILE`, `THINKTHEN_CONFIG`, and `config path`, `show`, or `check`.
- Demo 10 leaves with a line in the roadmap entry that the git history keeps it. No other demo is renumbered.
- The code. `--profile`, `--adapter`, and `--key-env` are removed and exit 2 when given. `--url` stays hidden, and `--model` stays with its default. The profile type, the adapter selection, and the rules that served them are deleted. `meta` holds `url`, `model`, `usage`, and `replayed`. The plan document holds no `profile`.
- The request bytes do not change, so every pinned digest and the committed recording of demo 01 still hold. Demo 01 stays green.
- `spec/` pages and fixtures follow.
- Each point in `sdlc/issues/2026-09-19-review-leftovers-from-ticket-0006.md` is fixed, or the ticket's record says why it waits.

Excluded: any new option, and `THINKTHEN_MODEL`. A feature enters when a demo cannot be written without it.

## Acceptance

- Each removed option exits 2, with a test.
- A test pins the fields of `meta` and of the plan document.
- The pinned recording digest holds, and `sdlc/scripts/spec` still prints `demos: 1 green` with the key unset.
- A search of the repository for the swept words finds them only in ADRs, records, tickets, the roadmap entry, and tests of removed options.
- The ratchet falls, equals the measured total, and the commit says what was deleted.
- The whole ladder is green, and a second agent reviews the public surface change.
