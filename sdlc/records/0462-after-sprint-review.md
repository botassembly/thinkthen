# 0462: After-sprint review and bug sweep

Reviewed source: 7ea661c1e, compared with rc/0.1.0-rc.1. Main 20576787b added release-hold documentation only; libraries, databases and product source are unchanged. The selected product folders changed 1,274 files, adding 149,066 and removing 6,366 lines. This review covered changed functional areas and their contracts, rather than promising an every-line certification or rerunning the complete installed campaign.

The review found real bugs despite the passing installed table. The highest-priority defect is npm assembly omitting imported modules. R silently accepts contradictory input sources, C reverses batch metadata values, persistent usage failure is invisible to SDK/SQL callers, and DuckDB buffers file descriptors before admission. The latter's memory growth follows directly from the source; an exhaustion crash was not run. No release workflow or paid call ran during this review.

## Findings and owners

| Severity | Finding | 0.2 owner and smallest correction |
| --- | --- | --- |
| High | npm assembly omits complete.js, _complete.js and their declaration files while index.js imports them. A dry-run pack succeeds without them. | [0465](../tickets/0465-package-assembly-inventory.md): include the modules and test an actual locally assembled archive import/types. |
| Medium | Optional source-package smoke requires Flutter, but its workflow container does not emit it. Flutter remains a private pilot. | 0465: align the optional bundle contract without dropping required public consumers. |
| Medium | R complete inputs accept both kinds of source fields, then ignore the fields that do not match kind. | [0466](../tickets/0466-r-complete-input-admission.md): refuse cross-kind fields before reads/sends, including explicit presence. |
| Medium | C batch metadata emits Max=1 and Records=2, opposite to the public constants; bindings following the header decode them backwards. | [0472](../tickets/0472-c-batch-kind-values.md): fix the native mapping and exercise both public values and warning sides. |
| Medium | SDK/SQL finalization discards persistent usage-write failure; in-memory facts remain correct. | [0468](../tickets/0468-usage-write-failure-reporting.md): expose explicit pending/written/failed/disabled persistence status without altering valid answers. |
| Medium | DuckDB collects all decoded file descriptors before engine admission; per-item and manifest-path limits do not cap retained content. | [0470](../tickets/0470-duckdb-file-admission-bounds.md): bounded streaming with reviewed ownership/cancellation, preserving native limits. |
| Medium | Linux spec/demo children still inherit ambient configuration despite isolated usage state. An owned conflicting configuration can change fixture results. | [0471](../tickets/0471-spec-demo-config-isolation.md): isolate the owning runner's configuration through existing scratch helpers. |
| Medium | The v2 cache specification omits the separate implemented image key domain; durable image validation uses that domain consistently. | [0463](../tickets/0463-image-cache-key-contract.md): document the implemented tag without changing stored keys. |
| Low | Find advertises unsupported image attachment help and repeats a sentence; MCP source schemas permit combinations native admission rejects. | [0464](../tickets/0464-public-input-help-and-schema.md): correct help/schema while retaining local refusals. |
| Low | Objective-C, Swift, Zig and Python carrier prose still says complete execution is pending. | [0467](../tickets/0467-sdk-documentation-adoption.md): remove stale adoption claims. |
| Low | Fixed issues remain open; closed drafts remain in the open folder and some metadata repeats. | [0469](../tickets/0469-issue-dispositions-after-sprint.md): reconcile against source and landing references. |

The relate distinct-pair guard cannot run under the existing 255-source-record limit. Retain it as harmless defensive code for now; this is not a runtime defect or a reason to expand 0.2 cleanup. Go batch Close cannot interrupt a concurrent Next, but context cancellation works and concurrent interruption by Close is not promised. That remains a later design option, not a confirmed bug.

## Coverage and checks

Fresh read-only Codex tasks coordinated fresh area reviewers. The coordinator used app tasks because native spawn_agent reached its thread limit. These were agents, not shell jobs. Medium was requested for ordinary review roles; the separate native ownership follow-up requested High for memory-safety risk. Family child reviewers inherited their parent task's model; no claim is made that a separate effort setting was changed for each child.

| Area | Review scope and evidence |
| --- | --- |
| Core/engine | Changed modules, declarations, result assembly/IDs, adapters/images, routing, cache validation/migration, HTTP controls, cancellation, workers and usage lifecycle. Policy passed; focused cache-control 2, store 9, complete-result 8, usage 17, question-file 16 and question-set 9 tests passed. |
| CLI | Separate fresh first-pass and continuation reviewers traced argument/help, input/display, ten-function execution, rank sets, asking/rendering, failures, backend check and command dispatch. |
| C | Separate ownership and functional reviewers traced headers, counted inputs/readers, result/batch storage, ten complete-call branches, controls and failure snapshots. High follow-up inspected getters, layouts and semantic batch values. |
| Existing/dynamic SDKs | Fresh family reviewer covered Python, Ruby, R and PHP complete entrypoints, crossings, sources, carriers and selected tests. Python carrier 18 tests, Ruby 6 tests/241 assertions and R/PHP carrier fixtures passed. |
| Static/foreign SDKs | Fresh reviewers covered TypeScript, C#, Dart/Flutter, Java/Kotlin/Scala, Objective-C, Swift, Zig, Go, Ada, COBOL and C++ inputs/results/batches/compatibility and native ownership. TypeScript complete 6 tests and C helper syntax passed. High follow-up covered Go fields, Ada/COBOL counted layouts and C++ snapshot lifetimes; C header self-test passed. |
| SQL/dataframes | Fresh SQL and dataframe reviewers inspected changed complete calls, readers, images, questions/rank sets/settings, DuckDB bridge, Python accessors/frame dispatch and Rust Polars eager/lazy/typed input/collection. No full dataframe or platform build was claimed. |
| MCP | Fresh edge reviewer traced protocol, framing, admission, catalog, dispatch, output, cancellation and Unix/Windows pipe source. Separate family reviewer ran 9 client tests. No new Windows execution was performed. |
| Packaging/help/docs | Fresh packaging reviewer inspected changed assembly, archive/install/workflow paths and retained artifacts. The focused exact npm stage dry-run reproduced missing modules. Public installer copies match their sources; earlier archive revisions remain historical evidence. |

These checks complement the retained 29-consumer installed campaign; they do not requalify changed future packages or prove final Mac/ARM/Windows execution. Tests, minor helpers and every ABI field were not individually certified. The review's outcome is an actionable defect map, not a guarantee that no undiscovered bug exists.

## Historical sweep

The sweep inspected all 56 root issue files and all 78 changed record paths; 59 record groups contained failure or deferred-outcome language. Earlier failed build, downloader, platform and package checks have linked corrections. Candidate-five Unix setup failures have reviewed local corrections; candidate-six workflows are canceled. Current setup still needs eventual platform qualification when permitted.

Current source resolves named backend selection (0377), SQLite find's selected model (0433), dataframe Series and described DuckDB kinds (0410/0434), the Polars deadline fixture (0410), result/2 rank positions and exact-commit rehearsal routing (0398). 0469 verifies references and closes/narrows their old issues rather than treating stale statuses as fresh bugs. Ruby's 0.2 platform and diagnostic fallback gems were corrected and qualified at 60f0dcb9a; retiring old public inert placeholders remains an external registry decision under the release hold.

The later issues retain expansion such as annotate dynamic options, rank weights/fusion, additional recipes, relation distance/scale, evaluation tools, whole-job batches, proxy/telemetry, lazy-streaming extensions and Windows static libraries. Upstream Ollama/MLX description rendering and Zig alignment workarounds remain documented. Oversized-record stop behavior is explicitly specified; changing it is a product proposal, not a reproduced regression. 0468 promotes the actual silent usage defect instead of leaving it in later debt.

The old rank-threshold 0.2 idea conflicts with the settled pure-sort contract. CLI described score levels and the old scalar SQL filter name are still proposals; the complete SQL filter API is already implemented. Their remaining scope goes to the PM explicitly, without silently adding or dropping features. 0469 also reconciles transcript help, R install and search presentation against landed owners and examples. Four closed files in the open issue folder and duplicate headers need records-only cleanup. No clinical or biological TCGA interpretation was performed.

Ignored Rust tests are explicit stress cases or child fixtures called by parents. Python platform skips name unavailable fork/procfs/SIGINT/resource mechanisms and do not substitute for Windows tests. Legacy door exclusions name inaccessible private faults or obsolete grouped requests; the complete installed campaign has no skipped required public cells. Retain these distinctions, rather than deleting safety tests or claiming all skips are bugs.

## What the build taught us

Layout equality and successful packing do not prove semantic enum values or module completeness. Test the actual native getter meanings and the archive consumers use. Keep the shared cases, add only regressions for concrete missing behavior, and leave proof machinery alone. Source review found defects that the installed table did not cover, so table completion alone cannot establish absence of bugs.

The review and sweep have owners for every confirmed finding. Code fixes remain open under 0463–0472; their completion is separate from this review. Ian's release hold remains in force. 0461 stays ready on lane 2 pending the PM's ruling after the TCGA results. No final candidate is authorized by this record.
