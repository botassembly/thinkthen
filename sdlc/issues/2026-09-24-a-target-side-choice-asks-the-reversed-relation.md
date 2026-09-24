# A target-side choice asks the reversed relation

Status: Open

When the target kind of a one-way cross-kind relation has more entities than the source kind, the shared planner asks from the target side. The question text still puts the asker before the relation words. The backend is asked the reverse of the declared relation, and the edge it produces is recorded in the declared direction.

## Reproduction

Found by the independent review of ticket 0088 (`sdlc/records/0088-review-claude.md`, finding 10) and reproduced on 2026-09-24 with a dry run, which sends nothing:

    $ printf '[{"name":"Ada","kind":"person"},{"name":"Acme","kind":"organization"},{"name":"Beta","kind":"organization"}]' \
        | thinkthen relate works_for=person:organization --dry-run --url http://127.0.0.1:9/v1 --model local-1 --no-cache

The first request's question is:

    {"type":"choice","instructions":"Which listed person fills the blank: Item 2 (organization \"Acme\") works for ___? Choose none if no listed person does.","criteria":{"i1":"Item 1 (person \"Ada\")","none":"No listed person."}}

The rule says a person works for an organization. The question asks which person Acme works for. When the backend picks Ada, the assembler emits `Ada works_for Acme`, the answer to a question nobody asked.

`relate` is not on main yet. `recognize` on main reaches the same planner through a one-way relation whose target kind has more recognized names than its source kind.

## Where it lives

`crates/thinkthen/src/core/relation.rs`, `choice_plan`: the `text` format writes `{asker reference} {reads} ___` for both directions. When `reversed` is true, the asker is the target, so the blank belongs before the relation words: `___ {reads} {asker reference}`.

## Effect on ticket 0088

Relate's target-side choice edges depend on this text. Ticket 0088 reports the asker role correctly and keeps the declared edge direction, but the answer behind each target-side edge comes from the reversed question. Relate output for one-way cross-kind rules cannot be trusted where the target side is larger until this is fixed. Same-kind yes/no questions and source-side choices are unaffected.

## Fix direction

Write the reversed question with the blank first, pin both wordings in the relation unit tests, and accept that the request bytes, recording digests, and cache identities of target-side choices change. Recognition's public output does not change shape.
