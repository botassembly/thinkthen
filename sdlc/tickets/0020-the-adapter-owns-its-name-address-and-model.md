---
flow: build
priority: 48
opens: crates specification/backends.md sdlc/ratchet.json
---

# 0020: The adapter owns its name, its address, and its model

Status: in progress

## Outcome

Everything that belongs to one vendor lives in that vendor's adapter module. No behavior changes, no request changes by a byte, and every committed recording still replays.

## Current Facts

Ian ruled on 2026-09-19 that other backends will come, and ADR 0010's clarification holds the seam rules. `sdlc/planning/sdk-design-study.md` searched the code and found the vendor's words outside `crates/thinkthen-core/src/systemone/` in four places: `backend.rs` holds the vendor's address as `DEFAULT_BASE` and the vendor's model as `DEFAULT_MODEL`, and it builds the URL by appending the adapter's name; `recording.rs` names `systemone::NAME` in the digest, the entry, and the replay check; `plan_document.rs` calls the adapter's encoder by name; and tests in several files write the vendor's URL and model names as bare literals. Ticket 0017 adds a lint check for the seam that allows these sites by name for now.

## Scope

- The default address, the default model, and the path of the endpoint move into the adapter's module as that adapter's defaults.
- The adapter's name reaches the recording key, the recording entry, the replay check, and the plan as a value handed in. No call site outside the adapter's module names the constant.
- The tests take the vendor's URL and model names from one fixture that the adapter's module owns.
- The seam check of ticket 0017 drops its allowed sites, and it then passes with none.
- `backends.md` says in one sentence that an adapter owns its name, its default address, its default model, and its endpoint path.

Excluded: a `Backend` trait, a second adapter, any option that selects an adapter, and any `adapter` field in `meta`. ADR 0017 holds each until a second adapter or the library split.

## Acceptance

- The pinned `decide` digest holds, and every committed recording replays with no recording changed.
- A test sends one request for each question type and compares the body byte for byte with the body from before the move.
- The seam check passes with no allowed site outside the adapter's module, the default address aside if the binary must still name it.
- The ratchet equals the measured total. The ceiling should not rise, and the commit says why if it does.
- The whole ladder is green.
