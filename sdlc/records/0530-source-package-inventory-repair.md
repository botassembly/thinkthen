# 0530: Repair current Go and C++ source package inventories

The local archive self-test at starting revision `b2c652a6270912527c82f66fbb8626808f95d3ea` failed with `release-go-cpp-pair: Go source members differ`. The self-test invokes the real source archive builder and substitutes only native build outputs. The Go builder already copied every root Go source file, while its gate retained a stale handwritten inventory missing `link_linux_amd64.go`, `owned_results_generated.go`, `owned_values.go` and `session.go`.

`release-go-cpp-pair` now derives root Go source names from committed HEAD and retains the fixed document, example, directory and manifest expectations and exact archive member comparison. It reads committed source independently of the package under test. The existing altered source identity rejection keeps its original failure ordering and cause.

The same inspection found that `release-pack` omitted C++ `client.hpp`, `inputs_generated.hpp`, `result_value.hpp` and `results_generated.hpp`. A consumer of the current client could not compile from that archive. The builder now copies the public header directory's `.hpp` files, following the existing CMake directory installation. It keeps the archived source byte checks and native pairing checks.

## Checks

The existing `release-archive-self-test.py` passed using local synthetic native artifacts and fake transport tools. Its existing missing archive, checksum, extra family member, altered source identity, private native bytes, source mutation and copied source refusal cases remain unchanged. During that run, direct inspection of the actual C++ synthetic archive found all nine canonical public headers and compared each header's bytes with committed HEAD. This includes all four previously omitted headers.

Offline policy passed for 268 resolved packages. Existing size warnings concern unchanged files. Shell syntax and whitespace checks passed. The Rust source ceiling remains 183253/183253; the Python source ceiling remains 9052/9052. Nonblank shell source changed from 266 to 265 lines in `release-go-cpp-pair` and from 578 to 576 lines in `release-pack`. The Python archive self-test remains 751 lines. No ceiling changed.

This repair runs no real native build, full parity check, final distribution assembly, workflow dispatch, candidate, publication or paid call. Installed consumer qualification and final distribution assembly remain with their existing owning tickets and authorized release checks.

## What the build taught us

A fresh read-only reviewer accepted `73065ed38601fc3c3373a6d3cee7d85feb207841`. The reviewer checked all 31 root Go files and nine public C++ headers against committed source and confirmed that the existing source identity, zero-SHA and archived byte rejection checks remain intact. Integration changes no source ceiling.

The apparent synthetic fixture failure exposed two stale copies of package contents in real scripts. Deriving Go source names from committed source and C++ headers from the public directory removes those copies while preserving independent package identity and rejection checks.

## Bound managed package identity to its XML parent

At starting revision `73af2637d581be49248ed8e4531e8d22682b9f33`, the managed package self-test used the current JVM POM and encountered `ambiguous XML groupId`. The package checker searched every descendant for project identity. The POM's valid Kotlin dependency also declares group, artifact and version fields.

`release-managed-pair.py` now reads Maven identity only from direct children of the project root. NuGet identity comes only from direct children of the package's single metadata element. The existing ElementTree parser handles namespace prefixes. Missing, duplicate and blank fields refuse; nested dependency identity cannot replace project identity. The root and each selected parent must also match uniquely.

The existing managed pair self-test retains its package and provenance refusals. Its successful assembly and verification use the real POM with dependency coordinates. Three added assembly cases reject duplicate, missing and wrong project group identifiers even when a dependency supplies the expected identifier. This change adds no checker, receipt mechanism, native build or release action. Neither source ceiling changes; the scripts remain below the existing file size limit.

A fresh read-only reviewer accepted `ea740b17b7853f17403a12a728b96c2316fe2fdc` and reproduced the existing managed self-test, including the three dependency boundary cases. Integration preserves the independently reviewed root Rust ceiling of 183258. Reading a document's descendants does not identify its owner: project and dependency coordinates require their own semantic parents.

## Remove the obsolete Flutter library lock from version inputs

At starting revision `377e3fe9aba186aff78bc5ded3f2030a517d83ff`, `versions --self-test` raised `FileNotFoundError` while copying `libraries/dart/flutter/pubspec.lock`. Ticket 0522 had removed that library lock as part of native asset packaging. The version input inventory still required it, so the same stale entry also made the real version check reject the checkout.

`sdlc/scripts/versions` removes that single obsolete entry. The fixture continues to copy every required version input. The three existing Dart and Flutter manifests, the example's two package lock entries and both existing standalone consumer lock entries remain required. All existing negative cases remain unchanged, including the Flutter consumer lock mismatch and refused writes.

The existing self-test passed all 29 cases. Its planted C header parse failure printed the expected generator diagnostic. The real read-only version check passed with `73 places read 0.2.0`. Offline policy passed for 268 resolved packages; its size warnings concern unchanged files. The existing external privacy list found no matches in tracked paths or files, and whitespace checks passed. Handwritten Python source removes one line and adds none, reducing nonblank source in `versions` from 389 to 388. No source ceiling changes. This repair uses no native or release build.

Fresh read-only review accepts `eabbfeb0f4f9e5191ebf872cbfac00f33a8b6c04`. The reviewer reproduced all 29 self-tests and the 73-place version check and confirmed that the three manifests, four consumer lock entries and meaningful refusal cases remain. The repair removes a stale library-lock requirement rather than weakening consumer version coverage.


## Derive current Dart and Flutter source members

At starting revision `0399d25ec2e4350872859173031551100d6a4741`, the existing fake archive self-test refused the PHP/Dart pair because its Dart inventory omitted the committed complete and session sources, build hook and native asset manifest. The real builder already copied those inputs. The shared helper supplies both the PHP/Dart and Dart/Flutter consumer routes.

`sdlc/scripts/release-go-cpp-pair` now derives the recursively copied Dart and Flutter source trees from committed Git. It keeps fixed package metadata, document, example and provenance requirements and compares the exact sorted members independently of the archive under test. The Git selection excludes ancestor directories. Flutter no longer requires its removed library lock; the wrapper dependency guard follows the current registry version while both example path dependency guards remain. The same archive test exposed a stale COBOL inventory: the checker now derives the root copybook names and requires the session source, generated request source and session example already selected by the builder. Ada needs no change.

`sdlc/scripts/release-archive-self-test.py` exercises the Flutter route through the real builder with synthetic native outputs. Four planted Dart archives independently remove the hook, native asset manifest or session client, or add unexpected source. Each plant regenerates its checksum so the exact member comparison owns the rejection. The existing source identity, altered bytes, missing archives, sidecars, extra family entries and secrecy refusals remain unchanged.

The complete fake archive self-test passed at `87f11beb0`. The focused source package smoke refusals passed. Shell syntax, scratch and usage lint, tracked path and content privacy checks, and whitespace checks passed. Offline policy passed for 268 resolved packages; its existing file size warnings concern unchanged files. Neither source ceiling changes. Full routine closure checks belong to the coordinator. Installed consumer qualification, final distribution assembly and platform proof remain with their owning tickets and authorized release checks. This repair uses no real native build, full parity, release action or paid call.

Fresh review accepted `fe45b8d3f00f1400e013b96e781590f5b06a0af3`. The reviewer checked both consumer routes and confirmed that current archives pass while missing assets, unexpected or duplicate members, traversal, wrong wrapper versions and wrong example paths refuse. Existing tamper and secrecy guards remain unchanged.

## What the build taught us

The structural lesson repeats the original inventory repair: a builder that copies a directory must not depend on a second handwritten file list. Derive those inputs from the committed source and preserve independent malformed archive plants. Adding the missing names alone would leave the same drift mechanism in place.
