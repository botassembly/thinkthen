# ADR 0110: Omit explicit null descriptions from System One noul criteria

Status: **Proposed** 2026-09-29 for fresh High review and coordinator approval. [Ticket 0301](../../tickets/0301-noul-criteria-compatibility.md) implements this bounded revision to the Settled wire sections of `specification/backends.md` and `specification/check.md`; it does not authorize a provider call.

## Decision

The System One encoder sends `criteria.true` and `criteria.false` on a `noul` question only when that side holds a non-null description. It omits the whole `criteria` object if neither side has one. It keeps non-null string, object and list descriptions, member order, the parsed question-file value and the canonical question digest. An absent input key and an explicit JSON null remain distinct questions and may retain distinct `question_sha256` values. `choice` retains null-valued option entries, and `score` retains its existing empty-object form for a null level description. No public input grammar, response decoder or backend adapter changes.

## Evidence and consequences

Experiment 413 reports that Liquid `d1:free` refused the existing fixed `check` noul body with status 422 at `questions.q1.criteria.false`. The available experiment summary has no separately saved check body/status receipt, and no revised omitted-null request has been tried against that provider. Local exact-byte and loopback proofs will establish only what ThinkThen emits and sends; a later authorized hosted check remains necessary for the issue's done condition. Do not claim that TypeSafe answers the revised request equivalently without direct evidence.

The recording and cache key remains SHA-256 of adapter name, URL and exact body. An affected explicit-null request changes bytes and can miss its former entry. It can also become **byte-identical** to an already recorded absent-side request at the same URL, model, evidence and other question fields, so the ordinary lookup may hit that existing entry even though the two canonical question digests differ. This follows the current exchange contract; it does not prove provider-answer equivalence. Preserve old captured files and strict replay behavior without rewriting them or adding a legacy fallback. Check and documentation goldens, plus the R and TypeScript host assertions that pin null-bearing noul requests, must migrate with the encoder after ticket 0283 releases their shared files.
