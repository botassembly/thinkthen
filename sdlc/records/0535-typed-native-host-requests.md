# Typed native requests for Ruby, JavaScript and R

Ruby, JavaScript and R now construct native typed requests instead of assembling whole-request JSON envelopes or keeping host copies of request grammar. Native admission owns the question, input and settings rules. Existing public host calls, result readers, error carriers and producer cleanup remain intact. Ruby and JavaScript landed as reviewed slices; the final R conversion completes the change.

`RequestDefinition::from_authored_json` retains detailed authored file-selector diagnostics. The shared `RequestDefinition::from_request_json` boundary reuses the existing canonical-request error mapping for R and JavaScript, including the unsupported-feature exception. The declarations and API inventory describe both methods. Authored bytes, JSON identity, ordered questions, row positions and capacity-before-decode feed behavior remain covered through installed callers.

Fresh code reviews accepted all three conversions and explicitly accepted their necessary native ceiling increases. Ruby host source fell from 1815 to 1805 while its native adapter grew from 1458 to 1578. JavaScript source fell from 2211 to 2209 while its native adapter grew from 133 to 217. R production host code fell by three lines; ten retained regression lines raised its combined host/test ceiling from 1945 to 1952. Its native adapter grew from 3485 to 3620. The shared canonical-request mapping adds five root Rust lines; the earlier authored factory documentation added four. These are measured costs of replacing host grammar and preserving distinct refusals, not deletions.

Installed Ruby checks passed, including its routine shared cases and focused error regressions. Both installed JavaScript module forms passed their routine consumers and focused feed and authored-error checks. Installed R passed its 31 routine shared cases, 60 canonical-request checks, remaining function and feed scripts, and five named backends. Each check process exited and was awaited. The final closure lint passed policy, source ceilings, release fixtures, Clippy, documentation generation and the 1959-declaration API inventory. The uninterrupted routine suite passed all 1995 selected cases, doctests, 21 external consumer cases, conformance fixtures, schema, child isolation, transforms, triage and live-runner self-tests. Release and stress cases remain with the candidate suite.

`target/0535-typed-host-requests/` in the main checkout retains original and corrected Ruby, JavaScript and R logs under their original folder names, plus closure lint and test logs. The logs retain package builds and installed artifact identities. Build output is not pushed.

## What the build taught us

The Ruby review caught a lost invalid-UTF-8 error carrier and a changed per-item context diagnostic. The correction restored `UsageError` and the existing sentence, while retaining true Enumerable producer cleanup and cancellation. Focused installed cases passed after the correction.

The JavaScript review caught admission before capacity checks and loss of a pending native failure during producer closure. The correction checks capacity before decoding feed descriptors and preserves completed rows, final facts and the original failure. The focused installed checks passed in both module forms.

R package generation initially used the reserved R argument name `function`; the native parameter was renamed and wrappers regenerated. A request-count assertion was briefly changed after confusing assertion count with provider sends, then restored to its original 27 sends. R's authored-error check also exposed the same diagnostic regression in JavaScript. Both bindings now use one shared native error boundary instead of copying its exception rule.

The fresh R review found that an authored selector ignored unknown fields. `AuthoredSelector` now denies them. The installed regression first reproduced the admission, then passed with the exact `invalid canonical request` error and zero sends. The reviewer accepted the correction and its one native line and four regression lines.

The coordinator initially placed future API declarations in the main inventory before their code landed, and one declaration heading had prose before its required fence. Those metadata defects were corrected. Each implementation branch now promotes its declarations when the code is present, keeping other branches' inventories meaningful.
