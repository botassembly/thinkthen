# 0403: Build DuckDB 1.5.4 and 1.5.5

Each native-target archive now holds one extension per supported DuckDB version in its version/platform repository path. Builds use each version’s pinned source and static archives. Packaging retains checksums, footer checks and legal material from both versions. The public install consumer selects the member matching its actual stock host.

Both Linux versions passed genuine bridge/C++ builds, stock CLI/Python loads, repository installs, wrong-version refusals, settings and request-identity checks. Installed archives passed 53 conformance cases with zero failures and two retained exceptions: packed annotate groups and the private panic-boundary replacement. Existing SQL cases and the combined install/backend example passed. These results do not claim every conformance cell or another platform.

One whole-change High review found that the published consumer could accept an internal archive hardlink after extraction. It now calls the existing repository validator before extraction. The owning regression fails on the old code, rejects before unpack or DuckDB calls, and passes with the genuine archive. No new dependency, verifier or receipt system was added.

Landing gates: full tests and lint. Other targets remain for the approved hosted rehearsal. The extension stays unsigned; dbt v1 is the documented route. Signing, community listing and dbt v2 support remain deferred.
