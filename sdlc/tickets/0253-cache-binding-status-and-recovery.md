---
flow: design
priority: 253
opens: sdlc/issues/2026-09-26-a-run-that-sends-nothing-binds-the-default-cache.md sdlc/records/0253-cache-binding-preflight.md
---

# 0253: Show cache binding and teach safe recovery

Status: proposed design for fresh independent High review; no runtime or public documentation is claimed. The issue remains open and post-0.1. [Preflight](../records/0253-cache-binding-preflight.md) traces main `4fc01f40`, source boundaries, states and proof. Root owns implementation claims and issue movement.

## Outcome

`thinkthen status` and `status --json` show whether the selected cache is bound to the **currently resolved** backend address. A default-cache mismatch refusal gives a concrete safe way to clear that binding: stop all processes using the folder, move the **whole** folder aside preserving it, and retry at the old path, or select a fresh cache folder. Status remains read-only, offline and useful with no key. The stored marker stays a private adapter/address hash published before any permitted send. No command removes a marker or folder, and no diagnostic reveals the bound address or marker bytes.

## Evidence

- **Starts from:** [the open issue](../issues/2026-09-26-a-run-that-sends-nothing-binds-the-default-cache.md) explicitly requires a status binding indicator and a refusal that teaches how to clear an existing binding. Its no-key example is fixed by 0228, but its `Done when` is not. Experiment 284/register 10's broader literal demands were separately declined after High review of `0327a13d`.
- **Keeps:** ticket 0065/ADR 0035's durable before-send, hash-only marker and first-writer rule; 0124's safely checked requested URL; ADR 0034's read-only, offline status and matching human/JSON facts; 0228/0246/0250's pre-admission refusals and gate precedence; Local/exit 5, no-send mismatch, key secrecy and existing entry bytes.
- **Changes:** add an advisory, closed `cache_binding` human line and `cache.binding` JSON string under additive `thinkthen.status/1`, using the existing selected endpoint and shared cache-inspection gate. Change only the default-cache mismatch sentence to teach whole-folder move after all users stop. No new mutating command, raw URL storage, marker migration, schema rename or library error change.
- **Proof:** focused outside-in status matrix for disabled/unavailable/missing/unbound/legacy/matching/mismatched and invalid marker, exact human/JSON strings, unchanged folders and no key/network; one default-cache mismatch plus safe move-and-rebind path with counted zero sends on refusal. Existing malformed/unsafe and first-writer tests remain. See preflight for exact state and file mapping.
- **Defers:** actual operator move, automatic cleanup, status of an explicit `--record`/`--replay` directory, recoverable bound URL, universal no-touch, per-address cache, active-holder detection and any public-page edits while marketing owns them. Status is an observation; the recorder gate remains the write authority.

## Design and closure

Use the current `Backend::resolve` result and `Environment::cache()` selection. Extend `cache_prune::inspect` under its existing shared folder gate with one read-only marker comparison via `recorder/identity.rs`'s strict reader; derive `legacy` from an absent marker and a digest-shaped final entry, not from counts alone. Return `disabled` for a configured-off default, `unavailable` for no usable path, `missing` for an absent selected path, `unbound` for an empty unmarked folder, `matching`/`mismatched` for a valid marker, and fail at fixed `StatusState`/exit 5 without partial output on an unsafe marker or folder. Keep old count fields, types, names and `thinkthen.status/1`; ticket 0163 is the additive-field precedent. The preflight records strict-reader compatibility limits.

The refusal must direct users to stop **all** users of that folder before moving it; otherwise path replacement can split active directory/digest lock inodes. Moving the whole folder preserves old entries and marker together. Pruning or deleting only the marker is not recovery. Use one fixed message and keep the requested URL already checked for key collision; do not print the bound URL. The preflight gives proposed exact words and later specification copy. No source or public-doc file is claimed for this design pass.

After implementation and focused proof of **both** status and recovery wording, root may close only the named source issue. Neither the accepted register 10 disposition nor broader no-touch claims change. Before building, obtain High design acceptance and source claims; after building, record actual file growth, deleted/retained tests and any revised assumptions here.

## What the build taught us

No build has begun. Preparation found that status already resolves the effective endpoint and shares a folder-inspection path, while the strict marker reader is private to recorder. The initial no-key anecdote is fixed; the status/recovery criteria remain. Read-only binding comparison and a whole-folder operator move can satisfy them without making status a new binding authority or weakening the before-send marker. Record fresh design review corrections and later build evidence here rather than treating this proposal as shipped behavior.
