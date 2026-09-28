# 0216 independent design review handoff

Status: proposed B10 [ticket](../tickets/0216-annotate-record-batching.md), [preflight](0216-batching-preflight.md) and [ADR 0092](../planning/adr/0092-annotate-group-batch-metadata.md) for a fresh independent reviewer. No self-acceptance, product completion, runtime or paid result. Base main `d0dd6ad3`; B8 source `e5419ef9` and B9 accepted design `d5836c45` are separate unlanded branches.

## Review the exact boundaries

1. Follow one record through `QuestionSet::groups/group_evidence`, `cli/annotate.rs::plan_for`, `facade::split`, `cli/annotate_schedule.rs`, `engine/annotate_schedule.rs`, `facade/annotate.rs::take_answers/assemble` and `AnnotateMeta`. Check that independently closing group slices can fill requests without losing record order, question places, good siblings, failed markers, exit 6 or bounded scheduling.
2. Check the proposed per-record profile slice compatibility rule against `--batch 1` historical chunks, soft singleton versus hard profile limits, expanded tag wire counts, exact encoded body and one 413 member halving. The B8 helper at `e5419ef9` still uses a first-wire offset to index logical outcomes; the B9 design identifies that as a prerequisite correction, not B10 implementation evidence.
3. Check one top-level batch key and typed/environment/file/default precedence against ADR 0048 item 4. Keep the nested `questions.ok.batch` negative; preserve canonical question-set SHA and one-document bytes. Check all prose copies named in the preflight before authorizing runtime files.
4. Assess proposed ADR 0092's actual public gap: a row can ride several group/chunk batches, so singular `meta.batch` cannot truthfully describe every `records` and `position`. Review the additive `meta.batches` shape and batch-one omission. It remains a proposed outward choice for the coordinator to route; no ADR is accepted by this handoff.
5. Check the smallest listener and paid plan. Existing scheduling/splitting/request-identity/partial-failure tests must remain; new tests must cover distinct cross-record/group boundaries. The 182-case external suite is keyed but its existing set is tag/choose/choose, so the proposed yes/no plus choose set is a **new task**. Confirm offline identities, both token kinds and per-arm stop before any later paid execution.

No new choice is sought for batching, precedence, evidence, close rules or one halving; ADRs 0048/0051/0053/0055 already settle them. The only public proposal is multi-group metadata. Review may find a narrower truthful representation. Implementation still waits B8 completion/landing, B9 runtime dependency, exact file claims, metadata ruling and fresh source comparison. No item closes from design.

## Preparation lesson for review

The corrected B9/B10 inventory prevented a false nested-key edit and serial-scheduler assumption. Tracing all result constructors exposed a singular-metadata gap that a call-path-only list missed. The benchmark inventory prevented presenting an existing tag/choose set as the accepted yes/no/pick-one paid proof.
