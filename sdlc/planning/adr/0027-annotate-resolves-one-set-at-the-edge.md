# ADR 0027: `annotate` resolves one set at the edge

- Status: Accepted by Ian's rulings on 2026-09-20. Ian can overturn every item
- Date: 2026-09-20

`annotate` applies several questions to one document or record stream. Four details in settled pages conflict with the command Ian approved for ticket 0015. This decision replaces those details before code depends on them.

## Decision

1. **A question set holds no model.** `--model` applies to the run. This replaces item 7 of ADR 0013, which allowed one top-level model in a set. A reusable set describes judgments. The selected service model remains a runtime setting. A single question file may still hold `model` under ADR 0013.
2. **An empty input gives `annotate --dry-run` no plan to print.** The command validates the set, succeeds, and prints nothing. With evidence it prints the first request and the complete question-to-pointer map. This replaces the sentence in `annotate.md` that implied a plan without evidence.
3. **The set digest names resolved behavior.** It digests a specified canonical form containing the version, ordered names, each existing canonical resolved question, and normalized pointers. It does not digest source-file bytes. This replaces the definition-file digest accepted in ADR 0010 and `result.md`. A library caller can build the same set without a file and reach the same digest.
4. **Mixed response models fail one record.** A row never combines measurements from different reported model versions. Replies already recorded remain reusable. A diagnostic names versions only under the narrow safe rule in ticket 0015; otherwise it prints no reply field. This closes the open point in `result.md`.

`annotate` also lets `--jobs` bound independent groups within one document. Other commands continue to accept `--jobs` only for record streams. Groups enter one deterministic queue in record order and group order. Once a failure is observed, queued work stops and already-billed work finishes and is recorded.

## Consequences

Ticket 0015 updates `annotate.md`, `question-file.md`, `records.md`, and `result.md` with the exact contract and fixed digest example. The command stays compatible with the existing question parser, request format, recording entries, and single-question digests. A later public library can construct a question set without inventing a file identity.
