# 0254 DuckDB bridge design acceptance

Fresh independent High review accepted `6d3c90dc1be931a441db22de4659a125bf88198b`. The coordinator approves the bounded implementation under ADR 0098's existing opaque-payload disposal decision. The reviewer confirmed seven catches in the actual shipping bridge, the near-cap parent and feasible private child linkage. No runtime check or file mutation ran during design review.

The inner catch must forget an opaque payload before the bridge depth restores. A retained outer catch contains hook/depth setup failures and also forgets its opaque payload. Preserve all seven return fallbacks, the prior hook for unrelated threads, worker behavior and the documented allocation retained per caught panic. No new ADR or C++ production change is required.

One clarification limits the proof claim: the Rust child can invoke real private reply, guard and worker functions, but it cannot execute the C++ fallback mapping. The existing mapping at `cpp/src/bridge.hpp` remains source-review evidence. Any required C callback test stub belongs under a permitted `ffi.rs` basename and must remain test-only. The broader issue and platform qualification stay open until their own criteria are proved.
