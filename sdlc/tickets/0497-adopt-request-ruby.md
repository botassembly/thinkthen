# 0497: Move Ruby onto the shared request contract

Status: OPEN.

Milestone: 0.2

Reviews: revision 95fade3863f777142ecd49d4abca0efcf1790cce, accept

Reviews: revision c79e3f65e05eb4f12fa46d8f7ab003d477258c9c, accept

## Outcome

Adopt shared Request and generated results in Ruby through named typed public calls.

## Evidence

- Starts from: PM architecture asks2/5 requires one ticket per direct binding; current Ruby adapter repeats admission/result construction.
- Retained defect evidence: 0487's Ruby child-home slice reproduced two failures with the original helper and backend wrapper from `42b22e719` on the same selected binaries: `TestSurface#test_named_recognition_and_relation_plans_retain_source_model_and_bounded_answers` disagrees on recognition request bodies, and shared case `56-recognize-caller-defined-amount` receives backend status 500 through the installed native gem. The normal offline builds did not resolve them. Diagnose the actual request differences during this migration; preserve the failing cases and change expectations only if the reviewed contract establishes that they are stale. The environment cleanup is not their cause.
- Keeps: native engine ownership and safe errors; all ten functions, input/result/error and cache/replay behavior.
- Changes: Depends on0491, 0502 and the reviewed generation decision. Translate host arguments into Request, decode generated types, and remove the old copy after parity. Claim `libraries/ruby/**` and its installed typed consumer cases. Preserve absent versus null and tolerate permitted unknown result fields.
- Proof: Full shared cases execute through the installed host's typed interface, including files/images where supported, context/options, original positions, facts, failures, invalid-input zero sends and cancellation. Raw JSON pass-through is insufficient.
- Defers: Proxy and changing platform rulings without evidence. Size: medium surface migration.

## 2026-10-09 amendment

Follow 0511 shared admission, 0513 generated typed results and 0515's one API. Ruby owns naming, errors and cleanup idiom; Rust owns all rules and observations. Delete copied readers after installed typed cases pass, including the retained recognition failures. 0520 fixes the immediate facts-reader regression first. Review this amendment and narrow the Ruby slice files before coding.
