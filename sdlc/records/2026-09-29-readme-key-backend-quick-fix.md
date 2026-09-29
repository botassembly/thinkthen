# README key and backend Quick Fix

Status: Candidate for fresh review, 2026-09-29. The issue remains open for measured overhead.

## Result

The root README links to [TypeSafe's official homepage](https://typesafe.ai/) for obtaining a user's own key. It says which key and base address to put in `THINKTHEN_API_KEY` and `THINKTHEN_BASE_URL` when using another System One backend. It names `--model`, the `jev-1.13.0` default, and the exact loopback hosts that accept an absent key. The replay sample remains explicitly offline and key-free.

## Evidence

- TypeSafe's homepage, checked 2026-09-29, identifies TypeSafe AI and links to its own sign-in and docs. The README points to the homepage and makes no claim about an account or signup path.
- `specification/backends.md` defines the key, address precedence, System One shape, default model, and keyless loopback exception. `crates/thinkthen/src/core/adapters/systemone.rs` defines `DEFAULT_MODEL` as `jev-1.13.0`; `crates/thinkthen/src/cli/args.rs` uses it in the `--model` help.
- The README's measured-overhead sentence remains unwritten. Issue criterion 3 depends on marketing's named benchmark run.

## Checks

- `sdlc/scripts/tickets`: exit 0, no evidence failures.
- `sdlc/scripts/pages`: exit 0, 1 coming and 23 green.
- `RUSTC_WRAPPER= CARGO_NET_OFFLINE=true python3 sdlc/scripts/policy.py`: exit 0, 189 resolved packages and accepted dependency rules match. The first invocation inherited a blocked `sccache` wrapper; clearing it let the same policy check finish.
- `git diff --check`: exit 0. No provider call, benchmark, or runtime test ran.

## What the build taught us

The existing README already explained the wire shape and offline replay, but did not connect the key to its source or tell a user which key belongs at a different address. The contract also permits no key at three exact loopback hosts, so a blanket key instruction would misstate local use. This documentation-only Quick Fix adds no test or runtime path.
