# 0236 code review

Status: Fresh read-only High review ACCEPT at `b84ff2de17b8d7d8cf40483d3fe5a3f3b1bffd83`. High review was used for async native ownership, early cancellation and final account lifetime.

The reviewer traced JavaScript invoke, native result conversion and original index mapping across every verb and delegate. Lazy batches drain before final facts; dynamic-label many calls preserve null choice, numeric score and empty tags. It checked full JSON validation and ordered parsing, missing versus null, unsafe nested host values and Map refusal.

The reviewer read napi 2.16.17's TSFN implementation: clones share reference state; early stop unrefs without aborting; an observed wait refs; the final callback stores one terminal report and detaches. Existing functional tests cover unobserved exit and observed joined completion. The installed addon exports the expected call, engine, usage and handle symbols.

Independent checks passed all four measured ratchets: Rust 932, JavaScript 407, MJS 1019 and TypeScript 398, with clean diff and worktree. The reviewer retained the author's 37 affected Node cases, eight exact old conformance cases, three annotate cases, four native units and strict lint/type results because the inputs were unchanged. No provider, stress or broad gate ran. The captured-body test checks parsed integer-key order and actual digests rather than a literal raw key-order substring; the ordered source path supports the contract. No blocking finding remains. Site migration is outside this ticket and remains tracked.
