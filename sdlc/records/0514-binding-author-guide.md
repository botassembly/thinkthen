# Binding author guide

Ticket: [0514](../tickets/0514-binding-author-guide-for-thin-first-class.md). Starting revision: `06c1271c30be8238a5f775b489b43ced02b8f761`.

## Change

[The guide](../../libraries/BINDING-AUTHOR.md) describes the approved 0.2 binding target. It replaces the plain-host-value and typed-result ban with Rust-owned generated types, explicit presence and complete observations. Twenty-one tables describe calls, inputs, typed results, absence, errors, async and cancellation, cleanup, editor support, installation and thinness for the languages and dataframe surfaces.

The guide links the ruling and existing contracts. It keeps public Request decoding at the edge and pure core admission inward. It describes the owned bounded session without inventing handle signatures, preserves frozen C compatibility, and names C# and Dart's approved result, scheduling and ownership forms. It separates existing enforcement from the extensions owned by admission, generation, session and packaging tickets. No product source or settled ADR policy changed.

## Evidence

- `CARGO_NET_OFFLINE=true python3 sdlc/scripts/policy.py` passed with the existing source-size warnings. The change adds no source lines or size ceiling.
- `cargo fmt --all -- --check` passed without compilation.
- `python3 sdlc/scripts/catalog.py` passed.
- `git diff --check` passed.
- Local link inspection checked 48 guide links and found no missing targets.
- The existing external private-name list supplied 35 names. Tracked-path inspection and the changed guide's tracked-content scan found no hits. The names were not printed or copied into the repository.

These focused checks cover the document change. They do not qualify session implementation, generated host results or installed packages. No full gate, paid call, hosted check, release action or build output was added. The fresh reviewer applies the guide to C# and Dart under the ticket's proof requirement.

## What the build taught us

A nullable host property cannot represent both missing and present null. A binding guide must name generated presence explicitly alongside its ordinary language null idiom. Linking the shared result contract avoids rebuilding the result schema in prose.

An enforcement table needs both a real check and its scope. Existing schema, ABI and conformance checks provide useful foundations, while the owning implementation tickets extend them for the newly approved session, generation and packaging behavior. Describing those extensions as existing evidence would mislead a binding author.
