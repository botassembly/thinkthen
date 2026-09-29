# Tuning proposals: value and design review

Fresh independent Sol Medium review examined clean main `a16da4a5`, all seven named issue files, the landed 0256/0257 contracts, current cache/facts paths, and local experiments296/297. Ian asked whether the remaining requests are valuable enough to build. The review used the bounded adversarial-project-review workflow and made no source change, provider call or broad test run. This record preserves the independent conclusions and the coordinator's release ordering; Ian can overturn them.

| Original request | Assessment | Release decision |
| --- | --- | --- |
| Per-case audit evidence | A real measurement gap, now supplied by0257. Saved identity, grading and unknown usage remain explicit. | Complete; do not reopen or redesign. |
| Separate tuned output |0256 preserves the incumbent and refuses an existing output. | Complete. |
| Uncertain, hard or changing case selection | Useful workflow, but `audit --cases`, `diff`, and an external selector cover an initial consumer. Criterion identity, pairing and seeded selection still need a shared contract. Cohort gains are not a universal benefit guarantee. | Defer a built-in command until repeated consumers need the same policy. Keep the issue open. |
| Chained-question how-to | A runnable host sequence teaches a measured structural alternative without adding an engine feature. The brief's second command lacks options and does not consume the first command's answer. | Build a small replayable how-to and audit guidance before0.1. Handle an unresolved first hop explicitly. |
| Repeated runs | `--no-cache` supports an external loop. A built-in repeat changes output identity, packing and accounting beyond one option. | Defer; retain the original issue and use an external loop. |
| Score-adjacent run cost | Current `--facts` and `cost.jq` have different scopes. Missing usage, cached answers, allocation and caller prices prevent an actual bill from being reconstructed from arbitrary saved rows. | Defer new audit output until its scope and input contract are justified. Keep absent data unknown. |
| Adopt old recording folders | The refusal is real. Internal URL/digest agreement does not establish trusted origin, and adoption cannot repair a model-alias request mismatch. Old recordings remain readable. | Defer automatic adoption. Prefer an explicit bounded migration design if demand recurs; preserve current marker safety. |

## Probability and confidence

Do not add a new `--probability` flag or default result member for0.1. Existing detailed rows carry the yes probability or distribution;0257 exposes case probability, distributions and applicable choose top-two values. A script can select these with `jq`. Preserve scalar output and exit codes. Vendor confidence is distinct and its formula is unpublished; cohort accuracy and AUC do not establish calibrated confidence. This is a prioritization decision, not a claim that every verb has one interchangeable probability.

## Evidence corrections that affect a build

The source album-year cohort contains60 cases. One first hop, `multi-hop-album-year-017`, ties and produces null, so `pipeline.py` omits its second-hop row. Direct prediction is right on22/60 source cases, and22/59 on the retained subset because the omitted direct prediction was wrong. The pipeline is right on51/59 completed second hops; counting the unresolved hop as failure gives51/60 for the source cohort. The saved audit reports51/59. A new example must define missing-hop behavior rather than silently changing the denominator.

The two-hop cost comparison describes its retained cache state. It is not a promise of equal cold-run cost. Noise evidence contains50 cases times3 answer observations with one flip;150 observations are not150 transport requests, and710 is a cumulative ledger. The final experiment account is1,049 calls,1,019,902 input tokens and about$0.043. No new measurement was made for this review.

The label simulation has a228-case cohort, a76-case held set and300 trials. Ten-label uncertainty selection averaged0.723684 held accuracy versus0.698684 for random selection; the120-label comparison reverses their order. Preserve those experimental boundaries instead of making an empirical accuracy threshold part of feature acceptance.

## Next action

The coordinator assigns the chained-call how-to to the retained documentation author. It must use matching saved exchanges, execute both displayed calls, pass the first answer into the second, supply every option and retain a safe unresolved branch. Its focused replay proof and fresh review decide completion. The other four open proposals stay in the queue as later work; none is marked done, non-issue or externally blocked merely because it is deferred. Language integration and release correctness retain priority over new tuning runtime features.
