# 0443: Complete native SDK calls and cache policy

The native SDK now implements complete typed results for all ten functions, result/2, ordered question sets and rank sets, named and explicit loaders, retained file locations, authored declarations and route controls. The same engine exposes hosted and explicitly declared local image inputs. OpenAI text decisions use the pure adapter and per-question projected names. Call identities, request timing, partial usage, actual model facts, stable observations and priced tallies survive the complete result and cache paths. Existing v1 request and recording bytes remain separate from strict v2 model-matching replay.

Whole High review and follow-up review found two admission defects. CLI rank sets checked only the first member's item declaration; they now check every member before lookup or sending. Native authored pointers reparsed literal text as JSON; they now preserve the carrier and select fields only from explicitly parsed records. Counted regressions cover both corrections. The prior 577 affected native/store/catalog/OpenAI cases pass, the final CLI rank refusal case passes, and all six native consumer cases pass. Workspace all-target Clippy, offline dependency policy, formatting, public inventory and the exact source ceiling pass. No paid calls ran.

Native code is pushed through fbee4ea7c. Confirmation of the last admission correction and full tests and lint on the landing commit remain with the coordinator. Host SDK adoption, the shared parity table and final image-admission fixture execution remain open in their owning tickets. This record does not claim complete surface parity or release qualification.

## What the build taught us

A cached priced call costs zero without inventing token counts. Reported model and token availability come from the actual saved answer, while current send cost and request count describe this invocation. Partial usage remains partial through storage and replay. Literal text and explicitly parsed JSON are different admitted inputs even when their bytes look alike. Every member's declaration must be checked before a rank set reaches request lookup or sending.
