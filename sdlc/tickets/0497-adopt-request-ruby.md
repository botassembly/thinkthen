# 0497: Make Ruby thin and first-class

Status: OPEN.

Milestone: 0.2

Depends on: 0511
Depends on: 0513

Reviews: revision 95fade3863f777142ecd49d4abca0efcf1790cce, accept

Reviews: revision c79e3f65e05eb4f12fa46d8f7ab003d477258c9c, accept

Reviews: revision 4d59e7c6a, accept

Reviews: revision a087f6dc3, accept

## Outcome

A Ruby caller installs the gem, calls the ten functions by name with ordinary Ruby values, and gets typed Ruby results and typed exceptions. Work honors the gem's declared scheduler and cancellation, and blocks clean up resources. Rust owns every rule and observation; Ruby keeps only naming, conversion, errors, scheduling and cleanup.

## Evidence

- Starts from: the [2026-10-09 binding decision](../decisions/2026-10-09-thin-first-class-bindings.md). The Ruby adapter repeats admission and result construction, including settings, batch, deadline and relate-rule checks. Its hand-copied reader lacked facts fields added by 0461 and 0468; 0520 repaired it as a narrow bridge.
- Retained defect evidence: 0487's Ruby child-home slice reproduced two failures with the original helper and backend wrapper from `42b22e719` on the same selected binaries. `TestSurface#test_named_recognition_and_relation_plans_retain_source_model_and_bounded_answers` disagrees on recognition request bodies, and shared case `56-recognize-caller-defined-amount` receives backend status 500 through the installed native gem. Normal offline builds did not resolve them, and the environment cleanup is not their cause. Diagnose the actual request differences here. Keep the failing cases, and change an expectation only if the reviewed contract shows it is stale.
- Keeps: Native engine ownership and safe errors. All ten functions and their input, result, error, cache and replay behavior. Missing stays distinct from null, and permitted unknown result fields are tolerated.
- Changes: Meet the caller acceptance and the Ruby section of `../../libraries/BINDING-AUTHOR.md`. This ticket owns:
  - the Ruby target template and generated outputs from 0513's common graph, with no second reader;
  - conversion of Ruby values into the shared Request, with no restated checks;
  - typed exceptions carrying Rust error kinds and facts;
  - the declared scheduler and cancellation behavior, with no new async framework;
  - block-scoped cleanup;
  - the gem README, with a short old-to-new call mapping;
  - removal of the old public names and copied readers after installed parity.
  One public API is one coherent family of named typed calls. Claim `libraries/ruby/**` and its installed typed consumer cases, narrowed per slice before coding.
- Proof: The full shared cases run through the installed gem's typed interface, including files and images, context and options, original positions, facts, failures, invalid input with zero sends and the two retained recognition cases. One installed held-provider case shows the declared scheduler progressing, cancellation stopping further reads and submissions, and cleanup returning before the provider is released. Raw JSON pass-through does not count. Record handwritten code removed and added, counting generator templates, in the landing record.
- Defers: The proxy and platform ruling changes need no 0.2 ticket.
