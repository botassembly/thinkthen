# 0.2 SDK rollout

Ian's 2026-10-08 binding architecture ruling puts all SDK architecture in 0.2. The proxy belongs to 0.3. This replaces earlier architecture deferrals. Keep all languages and named typed public functions. Preserve the frozen 0.1 C ABI and accepted `thinkthen_call` grammar through compatibility translation. Release management remains held until Ian authorizes it.

Sizes describe work breadth, not duration. Small changes touch one existing behavior or documentation seam. Medium changes migrate one public surface or run one bounded experiment. Large changes cross shared execution, memory ownership or several languages. A family slice gets one review and its installed shared cases; no additional per-language reports.

| Order | Tickets and outcome | Size | Dependencies and division of work |
| --- | --- | --- | --- |
| 1 | 0474,0480: shared cache admission, safe conversion and replay contention | Large storage work | Land confirmed fixes without claiming the private reported failure is reproduced. Give TCGA a revision-specific main binary and wheel for its jobs-eight rerun. |
| 1 | 0482: C extent guards and borrowed lifetimes | Small bounded memory-safety fix | Land before0492 header generation and new C consumers; retain ordinary null/zero behavior. |
| 1 | 0483: recognize custom wording and original-scalar spans | Small or medium bounded fix | Land before new custom-mode recordings. Refresh only affected owned request fixtures; preserve request-count and secrecy checks. |
| 2 | 0489: recognize context per record across existing context-capable surfaces | Large shared behavior change | Every emitted stage receives the selected context; empty context clears it. Settle identity and admission before the new Request contract. |
| 3 | 0491: one Rust Request and generated schema | Large shared contract change | One writer owns Request/schema outputs. Review its ADR; keep legacy C grammar through translation. Native calls use typed Request without serializing to JSON. |
| 3 | 0492 with0485: generated C header and Rust-owned values | Medium contract generation change | Follows Request/0482 ownership wording. Preserve the frozen ABI; dependency review covers cbindgen. Generate rather than maintain a second values table. |
| 3 | 0502: C#,Dart,PHP,Python generation experiment | Medium bounded experiment | Separate lane alongside shared-contract work, after must-fix caches/context. Use the reviewed draft schema and saved generic exchanges. Its stop rule selects mechanics; it never removes a language. |
| 4 | 0461 slice B,0475–0479: recognize size, snippet, proposals, traces, stage context and boundary-only controls | Medium per control; stage semantics need contract review | Incorporate settled fields before wide language adoption. Existing threshold controls final strength; add another cut only for a demonstrated distinct omission. |
| 4 | 0490: tagged examples | Large conditional behavior change | Review now. Build only after experiment469 demonstrates benefit and PM receives its incremental work assessment. It adds to0489 rather than replacing context. |
| 4 | 0468: safe usage persistence state in shared facts | Medium accounting change | Settle the snapshot observation point before generated readers. Keep nonblocking status and explicit finalization; Pending is not durable success. |
| 5 | 0493: derived MCP schemas;0481/0488 resource and protocol corrections | Medium surface migration plus bounded fixes | Preserve MCP-specific file authority, framing and cancellation. Ordinary admission derives from Request. |
| 5 | 0494 R,0497 Ruby,0498 TypeScript,0499 SQLite,0500 PostgreSQL | Medium per direct surface | Adopt the reviewed Request/result generation together with every remaining feature. Retain host conventions and PostgreSQL file-authority ruling. |
| 5 | 0495 DuckDB with0470 bounded feed;0496 Python and its dataframes | Large per direct surface | Preserve calling-thread filesystem access, cancellation, dataframe row identity and native engine ownership. |
| 5 | 0503: narrow bounded JSON session interface | Large ownership change | Depends on0491 and the0502 decision. Reuse0470 lifetime/backpressure design; no whole-input staging to simulate streaming. |
| 5 | 0504: C#/JVM, Dart/Flutter, Go/C++, Swift/Objective-C/PHP and Rust/dataframe family slices | Large overall; independently reviewed family slices | After shared interfaces settle, claim distinct paths and run families in parallel. Preserve every typed function and input/result/error/cache behavior. |
| 5 | 0505: C,Zig,Ada,COBOL generated access | Large constrained-host migration | Preserve frozen ABI and documented host representation limits. Retire post-0.1 mirrors only after typed installed consumers pass. |
| 6 | 0501: generated npm/JAR product manifest | Medium packaging change | Derive from the final generated file layout; keep existing complete import-graph and compiled-output checks. |
| 6 | 0463/0464/0467 documentation/help | Small per repair | Describe the settled public interfaces and actual supported routes. Schema repair can precede0493 where needed. |
| 6 | 0484/0487/0469: proven test duplication, shared test environments and stale issue reconciliation | Medium bounded cleanup | Retain distinct secrecy, cancellation, parser, memory safety and compatibility cases. No new proof machinery or arbitrary pruning target. |
| 7 | Shared installed typed conformance and the parity table | Large final integration run | Every required consumer runs against its built package. A required not-run case fails. Record explicit platform/representation rulings. This is product verification, not permission to start release management. |

Use the existing three lanes. Keep one writer for shared Request/schema/header/session paths. Run the bounded generation experiment and independent language families in separate lanes. A family adopts all settled features in one pass. Run builds concurrently unless measurement shows contention; retain per-lane resource limits and locks.

The experiment measures handwritten lines, typed cases passed, missing versus null, retained failures, unknown output fields, added tools and per-row cost over100000 cache-hit rows. Stop when all four hosts pass, two generated hosts cannot preserve missing/null or failures, or handwritten glue exceeds half of today's reader code. Record the chosen generator/fallback before production migration.

No preview-only carrier marking substitutes for settled implementation. No paid calls are needed for this rollout. No tags, candidates, hosted release workflows, rehearsal or publishing follows from this plan. Existing external DuckDB listing approval remains independent of completing local SDK behavior.
