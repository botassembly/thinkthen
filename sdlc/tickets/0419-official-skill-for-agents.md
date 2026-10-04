# 0419: Publish a small official ThinkThen skill for agents

Status: ready. Fresh independent ticket review accepted the corrected scope on 2026-10-04.

Milestone: 0.2

Lane: the next free documentation gap after separately reviewed scheduling. No product implementation precedes ticket acceptance.

Coordinator placement: item 22 moves from later to 0.2 because this bounded documentation outcome fits the approved docs ownership. Ian can overturn this placement. Other items in the umbrella issue retain their milestones.

Closes: item 22 of [Docs, how-tos and specification claims](../issues/2026-09-25-docs-how-tos-and-spec-claims-owed.md).
Related: 0401, 0402. Does not close their remaining slices.

## Outcome

Publish skills/thinkthen/SKILL.md and link it from the README. Teach agents when to use the ten verbs, scalar versus record exits, evidence framing, --plan, explicit spending limits and safe abstention. Use generic support records and passages.

## Evidence

- Starts from: docs item 22 and the submitted agent-skill prototype in shared mail `inbox/thinkthen/2026-10-04-kb-lead-a-tested-thinkthen-skill-draft-for-agents-from-k.md`. The prototype's one-round experiment motivates tested software guidance; it proves no causal, population, accuracy or performance guarantee. The experiment also changed its request cap, and shared usage counters prevent exact spend attribution. Public examples and text contain generic records and no private provenance.
- Keeps: all command, input, output, exit, cache and request contracts. Empty find remains exit 0 with no output or request, including 0401C. ThinkThen judges supplied evidence and never executes selected labels.
- Changes: publish one short skill with a ten-verb table and links to canonical contracts. Add generic examples for record ranking, explicit choose abstention, --plan and staged producer validation. Examples set --max-requests-total, --max-estimated-input-tokens-total and --max-retries explicitly. Retries consume attempted-send limits; defaults impose no request or input-token total cap; separate CLI processes have separate totals. These controls provide neither a dollar ceiling nor an output-token ceiling. --top limits output and does not save ThinkThen requests. Cache reuse requires matching identity and retained answers. Text windows preserve internal lines and file boundaries; neighbors affect display only. Label 0.2 examples with their minimum version, and exclude unfinished 0401C/D features until their reviewed landing.
- Proof: replay published examples without credentials or network. Pin scalar yes/no/unsure, completed record and multiple-document false/null success, ordinary choose-none label selection exit 0 versus unresolved exit 3, and find winner versus model-none. Finish and check the producer before extraction. Prove separate handling for nonzero producer failure, malformed JSON, missing or wrong-type results, invalid candidate records, zero candidates, one candidate and valid candidates. Reject bad producer output before invoking ThinkThen; handle zero and one outside find. Invoke find --none only for 2–254 valid candidates within 16 MiB. Capture exits safely under set -e. Retain existing parser, secrecy, cache and find regressions. Reuse request-counting coverage and close any actual zero-send coverage gap with a bounded loopback regression; plan preview alone is not that proof. Require fresh independent review and focused documentation checks.
- Defers: new flags or exits, choose --none, default empty-input warnings, installation automation for agent hosts, broader recipes, performance promises and new live experiments.
