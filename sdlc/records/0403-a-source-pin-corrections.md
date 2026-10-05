# 0403 A source and pin findings corrections

2026-10-05. Preparation remains partial. The whole-candidate High review at `/tmp/thinkthen-0403a-source-pin-review-cli-cvdje8kf/review.md` returned P1 and P2 against `1359cd469612e55352c99dcfbbd64d854239b15b`, based on main `26174408f620c1771e70e6fbe495eab4e6c07e67`. This correction is ready for fresh independent High source/pin review; the builder does not decide acceptance. No stock native work ran. A is incomplete, and B remains future work.

The frozen source is `1b04f9de33c8f8848d9dff18a4b77df787d87400`. All later repository changes contain records only. The new owned runner is `/tmp/thinkthen-0403a-pin-fixes-72zmNx/run-focused.sh`, SHA-256 `5351841784281cc6b0da8ee3c02ce339a7454869858b65981e9ddef32bee7eda`; its successor source guard is SHA-256 `eb38ecb4f8457feea2d6cf1303f519e901754dafca5b9427d55591769464802e`. The [new receipt manifest](0403-a-source-pin-correction-receipts.json) binds source, scripts, runtime and receipts. The owned `handoff.json` supplies the exact final candidate and final record hashes without a circular commit reference.

## Changes

P1: `release-archive-tree.py` uses one raw Git command/environment boundary. It removes inherited `GIT_*` location, object, replacement and configuration overrides, disables system/global configuration and replacement refs, and retains explicit owned checkout arguments. HEAD, tree and batch blob readers use the same boundary. Source preparation and CMake's commit reader reuse it; CMake children receive the same sanitized Git environment. Validation precedes compiler/tool checks and still runs after the fake CMake build. No global configuration or other checkout's refs changed.

The new successor preparation guard independently disables replacements for every HEAD/tree/blob reader and uses a fixed proof source commit. It retains the exact old executable, bytecode and directory-link baselines; it has no baseline rewrite operation. Commit/tree/blob and inherited override plants also exercise this guard. All real replacement refs and cleanup operations belong to fresh TemporaryDirectory roots; ownership checks and the lane cleanup refusal precede deletion.

P2: Node's bounded literal authority reader requires each supported version's source commit, requirements filename, CLI ZIP/binary digests, static ZIP digest and manifest filename for all four targets. Field formats admit only the bounded digest and filename forms. SQL smoke validates all pins before any tool resolution or child execution, including when another SQL surface is selected. No Python subprocess, circular import or second production support list enters the reader. The first supported version and all four independently retained newest seven-field setup expectations remain unchanged.

## Focused proof

| Check | Result |
| --- | --- |
| Original portable cases plus Node regression | 11 pass; zero failures/errors; zero counted loopback requests |
| Real commit/tree/blob replacement controls | Three validator and three build refusals, each exit 1 with the raw source mismatch cause; zero fake Cargo/CMake/compiler/host/tool starts; three restored raw positives exit 0 |
| Inherited Git override control | Validator and build refuse exit 1; zero tool starts; restored raw positive exits 0 |
| Node pin controls | 43 independent plants, including pinless v9.9.9, every missing supported field, missing older target pins, malformed hashes/filenames/assignments and duplicates; 43 reader plus 43 smoke refusals exit 1; zero counted child starts; complete supported list restored and accepted |
| Successor guard | Real commit/tree/blob replacements and inherited overrides refuse; restored positives pass; index/filter plants retain zero Git filter executions |
| Ordinary valid routing | Both fake builds pass; selected output and alias ownership, source mutation during fake CMake, footer refusal and old single-version scratch package cases remain passing |
| Ratchets and caps | All 53 exact; Rust file maximum 500; root Rust 118,107, DuckDB Rust 4,062, C++ 1,762, DuckDB Python 4,868 |
| Formatting/tickets/syntax | Root and bridge fmt exit 0; tickets exit 0; eight shell/two Node syntax checks pass |
| Source guards | All 7,007 frozen tracked members match raw bytes/modes/links before and after proof; 107 existing bytecodes, 2,975 executables and 23 enrolled links remain exact; additional owned bytecodes 0 |
| Policy and command status | Baseline policy 1, current policy 1, identical output; command driver 1 and outer runner 1; no policy PASS |

The inherited policy failure remains the recipe proof's HTTP connection-close finding. C/main correction is pending landing. The correction preserves its protected source and does not treat the inherited failure as a pass.

The runner holds the lane-0 HEAVY lock and matching HELD declaration, then shared Rustup, cache mutation and Cargo locks. It uses nonlogin `/bin/bash`, private HOME, allowlisted cached tools, Node 22.22.3, offline jobs 2, empty Rust wrapper, distinct empty npm configurations, offline UV with downloads disabled, declared PUB environment, null R profiles/environment and default SIGPIPE/SIGXFSZ. systemd limits are memory 12G, swap 1G, CPU 200%. No old runner, receipt, source cache, lock or build was changed or deleted.

DuckDB Python grows 126 measured nonblank lines in this fix, from 4,742 to 4,868; total growth from main is 644. The additional lines earn real replacement/environment and pre-child incomplete-pin regressions. The existing source, fake build and parser fixtures are reused; no giant parser is duplicated. The shared raw Git helper grows outside the DuckDB ratchet. Rust and C++ totals stay unchanged.

## Retention and limits

An independent digest audit retains both actual source inventories (14,601 and 14,539), all 16 official CLI/static ZIPs against saved official release metadata, all eight version/target sets, eight CLI binaries, 172 static archives, eight headers, exact extracted inventories and eight wheels. All source pins, newest manifest bytes, requirements and official asset records remain unchanged. No downloads were needed. All 41 prior owned scripts/JSON/log/config files, including failed and cancelled attempts, remain byte-identical; the original [receipts](0403-a-preparation-receipts.json) keep their original source attribution. Private text scanning reports only a count: zero matching files.

The 366-line accepted plan retains SHA-256 `904eb42d8922e1d9c0b4bd3c8090d1805b27a341381779df62f480634773b9bb`. Protected Docs A/B/C/D sources, six DRAFT recipes, 50 measured carriers, producer bounds, aliases, glossary, public 0.1.2 installation, saved 350 proofs, core SQL, C++ API/ABI sources, Rust bridge, exports, footer stripping and remaps remain unchanged. Slice A still packages one default root extension until B. dbt v1 with DuckDB 1.5.5 remains the documented 0.2 route. Unsigned stock 1.5.4 does not qualify dbt v2's custom driver or signed extension requirement.

No actual native compilation/load/host execution, SQL suites, real installed package smoke, actual site replay, full test/spec/surfaces/stress, canonical 350 replay, remote hardware, native Windows, CI/tag, release/rehearsal/publication, paid/provider call, experiment, sealed TEST, community submission or signing ran. Fresh High review must precede stock native work. A still needs its actual matching stock builds/loads, symmetric refusals, newest/older suites, old-format installed smoke and actual site replay. Ian can overturn the scope and deferred proof choices in the accepted plan.

The first final handoff audit on record candidate `83613b2753f323077630ce07afd7eba69cc25ef7` exited 1 when its count-only private-text scan detected duplicated host-specific paths in the new receipt's runtime block. The receipt now references the unchanged prior runtime manifest instead of repeating those paths. The failed log, scripts and status retain their original candidate attribution; new successor handoff scripts perform the corrected audit. No executable input changed and no portable receipt was rebound.
