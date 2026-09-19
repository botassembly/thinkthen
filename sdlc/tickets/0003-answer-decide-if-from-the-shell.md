---
flow: build
priority: 30
opens: crates/thinkthen spec Cargo.toml Cargo.lock sdlc/ratchet.json sdlc/scripts/policy.py
---

# 0003: Answer `decide if` from the shell

Status: ready

## Outcome

`thinkthen decide if CONDITION` works end to end as `specification/decide.md`, `channels.md`, `result.md`, and `backends.md` describe. Evidence comes from standard input. The result prints as JSON. The exit code follows the table.

## Current Facts

Tickets 0001 and 0002 supply the types, the assessment, the result, the plan, and the `systemone` adapter. The binary knows only `--version`. It prints through a locked writer passed as a value. No gate may touch the network.

## Scope

- The `decide if` command with `--min-prob`, `--status`, `--plan`, `--backend`, `--url`, `--adapter`, `--model`, `--key-env`, `--timeout`, and `--max-retries`. `--status` without `--min-prob` is a usage error.
- Profile resolution as `backends.md` orders it: flag, then environment variable, then the built-in `jev` profile. An ad-hoc backend needs a URL, an adapter, and a model together, and it takes no key variable from any profile.
- `Meta` in the core gains `url` and a `backend` that may be `null`, as `result.md` now shows. Ticket 0001 built `Meta` before ADR 0006 changed it.
- `--plan` prints the six fields `channels.md` lists, with `key_env` as `null` when no key would be sent.
- The environment and standard input are read once at the edge and handed inward as typed values.
- One blocking HTTP client, `ureq`, with the timeout and the retry rule of `backends.md`. The wait between retries is injectable so tests never sleep for real seconds.
- One function maps every error to its exit code and its message. No message holds a key or the evidence.
- Dependency added: `ureq`. Update `policy.py` in the same commit, and extend the license list only if its tree needs it, saying which license and why.

Excluded: a configuration file, recording and replay, record framing, every other verb, and any live call.

## Acceptance

- Integration tests run the compiled binary against a loopback listener built from the standard library. They assert the request line, the content type, the bearer header, and the body by value. They cover: a yes, a no, an unsure, an unassessed result, each `--status` exit code, a 401, a retry after 429 that then succeeds, retries exhausted, a refused reply, a missing key, an ad-hoc backend that sends no key, a URL with no adapter as a usage error, empty input, and input that is not UTF-8.
- `--plan` prints the plan, holds no key, and opens no connection. A test proves no connection by pointing the URL at a closed port.
- Executable specs under `spec/` cover `--help` for `decide if`, `--plan`, and the usage errors. They need no network.
- The whole ladder is green, and a second agent reviews the dependency addition.
- After landing, the steering agent makes one live call by hand and records the result in `sdlc/records/`.
