# 0496: Move Python and its dataframes onto the shared contract

Status: OPEN.

Milestone: 0.2

Depends on: 0511
Depends on: 0513

Reviews: revision 95fade3863f777142ecd49d4abca0efcf1790cce, accept

Reviews: revision c79e3f65e05eb4f12fa46d8f7ab003d477258c9c, accept

## Outcome

Adopt shared Request and generated results in Python, pandas and Python Polars through named typed public calls.

## Evidence

- Starts from: PM architecture asks2/5 requires one ticket per direct binding; current Python, pandas and Python Polars adapter repeats admission/result construction.
- Keeps: native engine handles, mapping access, dataframe row identity; all ten functions, input/result/error and cache/replay behavior.
- Changes: Depends on0491, 0502 and the reviewed generation decision. Translate host arguments into Request, decode generated types, and remove the old copy after parity. Claim `libraries/python/**` and its installed typed consumer cases. Preserve absent versus null and tolerate permitted unknown result fields.
- Proof: Full shared cases execute through the installed host's typed interface, including files/images where supported, context/options, original positions, facts, failures, invalid-input zero sends and cancellation. Raw JSON pass-through is insufficient.
- Defers: Proxy and changing platform rulings without evidence. Size: large surface migration.

## 2026-10-09 amendment

Follow 0511 admission, 0513 Rust-owned generated typed results and 0515's one API. Python results support repr, equality, to_dict, pickling and bool without raising, preserving distinct absence, null and failures. Python owns its naming and dataframe idiom, not copied validation or cache logic. Keep pandas and Polars row identity and all ten functions. 0520 fixes the immediate facts-reader regression first. Review this amendment and narrow each actual Python/dataframe slice before coding.

## Surface assessment amendment

This ticket owns Python target generation, async execution, task cancellation and context-manager cleanup as well as result behavior. Releasing the interpreter while waiting on the calling thread does not prove that an asyncio loop stays responsive. Add one installed held-provider case in the existing runner: another coroutine progresses, cancelling the caller stops further reads/submissions, and cleanup returns before the provider is released. Preserve the native distinction between cancellation and final settlement.

For successful Call values, bool follows the corresponding ordinary Python value, including False, None, numeric zero and empty collections. Top-level failures remain typed exceptions; embedded failures retain their explicit failure kind and facts and are not converted into successful False or None values. Define the truth behavior of any separate failure carrier in the reviewed result design. Printing, equality, mapping conversion and pickling preserve presence and failures and do not retain an engine handle. Generated stubs describe the actual installed objects.

Python pandas and Python Polars belong here. Rust Polars belongs to 0504. Preserve original row identity, null masks and whole-set semantics rather than issuing per-row aggregate calls. 0515's removal follows installed replacement parity and does not block this migration.
