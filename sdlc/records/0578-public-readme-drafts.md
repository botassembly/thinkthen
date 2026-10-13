# Public package text drafts

These texts and package descriptions are drafts for marketing and Ian's approval before publication. Comparing source and package bytes establishes assembly, not approval or registry availability. This revision starts from `c35341668`, aligned with the combined wave at `1295f71c1`, and follows the PM amendment in `457bba8fa`.

The canonical pages are the root `README.md`, `crates/thinkthen/README.md`, `libraries/{typescript,go,ruby}/README.md`, `libraries/{python,c,cpp,csharp,jvm,swift,zig,php,dart,ada,objective-c,cobol,r}/README-PUBLIC.md`, `libraries/dart/flutter/README-PUBLIC.md`, and `databases/{sqlite,postgresql,duckdb}/README-PUBLIC.md`. Every page has a capitalized title, explains branchable answers, caching and offline replay, names Jev, SystemOne, OpenAI and local models, shows key setting and a typed answer read, and stays below 40 lines. Cost and installation wording remain public drafts. No example disables the cache.

Packagist reads the exact root README and root Composer description. That page combines the command and PHP examples. The full root guide is preserved byte for byte from `c35341668` as `README-DEVELOPMENT.md`. npm, Go and Ruby retain their existing separate developer guides. Other developer READMEs remain separate from their public pages.

Release assembly selects the public pages in `release-pack`, including database, C, command and source wrapper archives. The Windows C and command packers include the matching public README as a regular ZIP member. JVM staging maps its public page to `README.md`; the R source tarball does the same. The C# package inventory maps the public source to its actual NuGet member name. Python's pyproject already names the public source. Maven's javadoc JAR, the assembled NuGet file, the final npm tarball and Dart publication directory are checked after assembly. Family checks and the draft operation check archive bytes before upload; source checks cover Go and Packagist, which publish repository text. No publishing or hosted workflow ran here.

The checker compares actual README and description bytes to those canonical sources. It rejects ADRs, ticket numbers, internal paths, internal crate/project names, stale ThinkThen versions and oversized pages, while allowing dependency and compiler versions. When the local private-name file exists, its matches produce only a refusal category. Python metadata payloads are decoded as bytes to preserve UTF-8 punctuation. The source check alone does not establish built-package text.

The DuckDB public draft derives from `9009a74ae:sdlc/records/0577-duckdb-readme-draft.md` and applies the PM amendment. Release assembly selects its canonical public bytes for the archive's `README.md`. The matching verifier allowance is at `805d790255cf2528b73bc2bd7b869e0b82bae737` in `databases/duckdb/cpp/verify_repository.py`; this text revision changed no DuckDB product, configuration, C++ or developer guide.

Examples use APIs read in the current source and shipped examples. TypeScript reads a typed row through iteration because its current completed object has no scalar accessor. Python and Ruby use `.value`; R uses its typed `$answer`; C++, C#, Dart, PHP, Swift, Go, Java, Ada and Objective-C use their current named answer readers. C, Zig and COBOL show explicitly identified typed packet excerpts; complete examples own construction and cleanup. No installed candidate SDK example was executed here. The public root replay retains the recorded question from the prior guide and passed against the retained 0.2.0 command at `5b65d99345f0633f0350aa97bd297d179b9c9bac`: exit 0, `true`, no key and zero counted loopback requests. The recording, recording specification, core recording implementation, engine and CLI asking source are unchanged from that checkpoint; changes to the generated result schema do not change this saved recording.

Focused evidence:

- `python3 sdlc/scripts/registry-readmes.py sources` passes for all public sources and metadata descriptions.
- `python3 sdlc/scripts/registry-readmes-self-test.py` passes five tests. It retains the original eleven negative plants, adds an internal-project plant, and exercises real npm packing plus Windows command archive assembly with structurally valid synthetic PE bytes. Both assembled archives refuse a planted ADR for the intended cause.
- `python3 sdlc/scripts/release-windows-c-self-test.py` passes the portable archive and workflow rejection checks. `python3 sdlc/scripts/release-windows-command-self-test.py` passes. Their native bytes remain synthetic, and every prior rejection retains its intended cause.
- `python3 sdlc/scripts/release-registry-self-test.py` passes all 41 cases, including actual registry output assembly from supplied fixture archives and Apple source-wrapper assembly. These fixtures establish assembly and rejection behavior, not native runtime support.
- `sh sdlc/scripts/release-smoke-command-test` passes its existing replay, checksum, output and zero-request rejection cases.
- Shell syntax checks pass for `release-pack`, `release-workflow`, `package` and R `make-tarball.sh`. Python compilation passes for changed Python scripts. `git diff --check` passes.
- `CARGO_NET_OFFLINE=true python3 sdlc/scripts/policy.py` passes. Its file-size warnings concern unchanged Rust, database and binding sources; this work changes no Rust source or dependency lock.
- An explicit comparison confirms the preserved root developer guide equals the original bytes at `c35341668`.

Actual tool-produced package text was checked with an isolated scratch home and output directory: `cargo package --locked --offline --allow-dirty --no-verify --package thinkthen`, `npm pack --offline --ignore-scripts`, and `THINKTHEN_RUBY_FALLBACK=1 gem build`. The checker passed each archive. Artifact identities from that run were:

- `thinkthen-0.2.0.crate`: SHA-256 `a42ac8e761106e570c387932ac892a18197aea68a2f4664f6ea737e40a135c86`.
- `thinkthen-0.2.0.tgz`: SHA-256 `5122ceb1d5db8200674d1922445000a9d6bbc6b7e4c2a55c68aa475ab1d31c13`.
- `thinkthen-0.2.0.gem`: SHA-256 `580fe78e0a4e688d9d465d9ce05664a0de529ea928f877833f6eb74067717b2b`.

Cargo verification was disabled. The npm archive has no compiled addon, and the Ruby diagnostic fallback cannot judge text. These checks establish public text assembly only. The owned scratch directory was removed after checking.
