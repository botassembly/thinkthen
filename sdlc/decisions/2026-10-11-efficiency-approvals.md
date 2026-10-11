# Efficiency approvals for 0.2

Date: 2026-10-11

## Ruling

Ian approved both proposals in the [0.2 adversarial review](../records/2026-10-11-0-2-adversarial-review.md): "Yes, to both of your things. Whatever we have to do to make everything more efficient and better, I approve."

1. **Slim the binding parity cases.** Core limit and refusal cases run on cli, rust and c, as each consumer's contract resolves them. Each binding runs a representative subset. This replaces the rule that every consumer runs every required case. Ticket 0555 carries it.
2. **Add a blocking session read to the C API before 0.2 publishes.** Every binding deletes its sleep-and-poll loop. Ticket 0556 carries it.

## Standing approval

Ian also approved, in general terms, changes that make the build or its tests more efficient or the code better. Within 0.2, the PM and coordinator may file and build such work without asking first, when it does three things:

- it keeps public behavior or follows the reviewed deletion rules;
- it stays inside the existing outcomes and rulings;
- it publishes nothing.

Publication, paid calls, credential changes and changes to read-only configuration still need Ian's explicit go.

## Why

Polling cost CPU and latency in every binding. The full campaign ran each core limit case once per language. Rust admission makes those repeats prove the same thing each time.

## Replaces

The required-cell rule in `conformance/parity.py` that every consumer asserts every required case. The scope freeze in the 2026-10-10 ruling still holds for new features.
