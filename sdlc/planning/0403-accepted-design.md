Corrected complete design for ThinkThen 0403, based on main `6d26206aa8861490cc6540b6bafafc08e3249c3c`.

This replaces the prior design at `/tmp/thinkthen-0403-design-cli-eul13ov7/design.md`, whose SHA-256 matches `d077897316d2991968cd12a0c2c79656929aa992920dff2bbf78dd20e565386e`. It incorporates both findings in `/tmp/thinkthen-0403-design-review-cli-ykam0mfg/review.md`.

Implementation remains after 0402, in lane 3, on `ticket/0403-duckdb-extension-for-dbt-v2`. A and B land separately. The parent obtains fresh High design review before implementation.

This turn inspected instructions, contracts, tickets, the complete prior design and review, experiment 0011, ADR 0081, and the build, inventory, package, release, installed and site consumers. No helpers, builds, tests, native loads, network requests, credentials, initialization files, writes, dispatches or publication were used. HEAD and main still match the stated commit. All proof below describes future work.

The final release outcome remains one unsigned archive per existing DuckDB target, containing two separately compiled C++ extensions:

```text
v1.5.5/<platform>/thinkthen.duckdb_extension
v1.5.4/<platform>/thinkthen.duckdb_extension
LICENSE.thinkthen
LICENSE.duckdb
NOTICE
DEPENDENCIES.txt
LICENSES/...
```

Keep `thinkthen-duckdb-$V-$TARGET.tar.gz` and its checksum sidecar. `$V` is the ThinkThen release version, distinct from the DuckDB version.

| Release target | DuckDB repository platform |
|---|---|
| `x86_64-unknown-linux-gnu` | `linux_amd64` |
| `aarch64-unknown-linux-gnu` | `linux_arm64` |
| `aarch64-apple-darwin` | `osx_arm64` |
| `x86_64-apple-darwin` | `osx_amd64` |

Preserve every SQL function, overload, result type, setting, error sentence, removed-function refusal, bind validation, statement owner, cancellation boundary, secrecy rule, cache behavior and request limit. Preserve existing conformance exceptions and their named replacement evidence. ADR 0081’s C++ API and Rust bridge remain. This ticket introduces no core change or stable C API migration.

Stock unsigned DuckDB 1.5.4 compatibility establishes no ThinkThen load in dbt v2’s custom driver. dbt v1 with `duckdb==1.5.5` remains the documented 0.2 route. A successful stock load and a matching footer cannot establish signed dbt v2 compatibility.

The tracked repository already supplies the 1.5.5 source commit `d8cdaa33fda8df955cc76ef58a280f68f4cd43fa`, CLI ZIP and binary digests, static ZIP digests and extracted static manifests for all four targets. The manifests contain 22 members on each Linux target and 21 on each macOS target. It also supplies all four 1.5.4 CLI ZIP and binary pins under `DUCKDB_OLDER_*`, the newest Python requirements with `numpy==2.5.3`, and the pinned macOS CMake wheel.

Passive reads in this turn confirmed the Linux x86-64 cached source HEAD, 1.5.5 CLI and static ZIP digests, and 1.5.4 CLI and ZIP digests. The cache contains the source, extracted static archives, Python environment and `linux_amd64` platform file. These reads establish no native execution or complete offline readiness.

The cached older ZIP is named `duckdb_cli-v1.5.4.zip`. Preparation may reuse its bytes only after matching its pin. No `duckdb/v1.5.4` build cache exists locally. The newest CLI ZIP is absent from this cache under the filename `setup.sh` expects. The prior inventory reported no cached 1.5.4 Python wheel and reported the root, DuckDB surface and bridge crate archives, root and bridge extracted sources, and pinned Linux Rust toolchain as present. Those observations remain preparation evidence, not a passing resolver or release receipt.

Before Slice A can earn native proof, preparation must obtain and review:

1. The official 1.5.4 source commit.
2. The official 1.5.4 static ZIP and exact digest for each target.
3. Each ZIP’s extracted archive manifest, including actual names, count and digests.
4. The 1.5.4 Python wheel and retained NumPy requirement for each proof host.
5. Both version caches for Linux ARM64, Apple Silicon and Intel macOS.
6. Any missing newest-version cached inputs needed by the declared checks.

Preparation occurs outside offline gates and outside this design turn. If a platform lacks official 1.5.4 static archives, stop and report it. Do not substitute 1.5.5 archives, relabel a binary, fabricate pins, or omit a version from a successful receipt.

`DUCKDB_VERSIONS="v1.5.5 v1.5.4"` in `databases/duckdb/tools/version.env` becomes the sole operational supported-version list. Its first member supplies the default. No second production support list or independently maintained default is added.

Move existing pins without changing their bytes into explicit version and target keys, such as `DUCKDB_V1_5_5_CPP_SOURCE_COMMIT` and corresponding CLI/static keys. Move the older CLI pins into the 1.5.4 keys. Keep existing 1.5.5 manifest names and contents; add four explicitly versioned 1.5.4 manifests. Keep the macOS CMake tool pin separate from the supported DuckDB list.

A bounded `tools/inputs.sh` owns the shell selector. Given one validated version and target, it resolves the source commit, assets, digests, manifest, requirements and cache locations. Setup, build, release packing and Linux container preparation use that selector. Reject unknown, malformed or duplicate supported-version entries and unsupported selectors before resolving build paths or configuring CMake. An unknown version never defaults to newest.

Python and Node readers consume the same tracked authority through bounded readers appropriate to their runtime. They parse the supported assignment form; they do not evaluate arbitrary file contents. Independently reviewed test expectations may contain literal versions, target mappings and digests as oracles.

Preserve `setup.sh --target`. Preserve the newest `--inputs` seven-field output exactly, including its existing manifest filename and all four literal expectations in `selftests.sh`. Internal version selection must not change the default output of these commands.

`setup.sh --fetch` prepares each listed version under `duckdb/<version>/`, with its own source, static archives, CLI, platform file and Python environment. Ordinary setup validates the prepared inputs and installs Python requirements offline. Missing prerequisites report exit 77; corrupt or mismatched prerequisites fail. Each version validates its own source identity, ZIP pins, extracted archive membership and digests, Python DuckDB version and reported platform. Do not assume 1.5.4 archive membership equals 1.5.5.

The new `requirements-v1.5.4.txt` pins `duckdb==1.5.4` and retains `numpy==2.5.3`. Both environments retain the supported Python preparation and pinned macOS CMake checks.

New consumers cease using `older-host/`. Do not automatically delete that legacy cache or other pre-existing paths. Later cleanup follows the owned-scratch rule.

Slice A introduces version-specific builds and stock-host proof **and adapts every existing checkout reader immediately**. Its release archive remains the existing single-version archive until B.

| Slice A files | Responsibility |
|---|---|
| `tools/version.env`, new `tools/inputs.sh` | One version list and selected input identity |
| `tools/setup.sh`, requirements, four new manifests | Prepare and validate both versions |
| `cpp/build.sh`, `cpp/CMakeLists.txt` | Separate compilation and version-qualified outputs |
| `cpp/verify_package.py` | Matching hosts, real cross-version refusals, patched-footer refusals and repository selection |
| `check.sh`, `tools/harness.py` | Explicit interpreter/artifact pairs in every affected branch |
| `tools/source_checks.py`, `tools/selftests.sh` | Both shipped-artifact scans and retained contract checks |
| `tools/release_pack_cases.py` | Read and alter scratch copies of the new canonical paths |
| `sdlc/scripts/release-pack` | Read the canonical default artifact while retaining the old archive shape |
| `sdlc/scripts/release-container` | Replace old pin-variable dependencies and prepare inputs by version |
| `cpp/package_notices.py` | Read the selected source and manifest without guessing the old inventory |
| `site/scripts/smoke-sql.mjs`, bounded version reader | Read the default version and final canonical artifact |
| DuckDB build documentation, `NOTES.md` as needed | Explain output paths and demonstrated header differences |

`THINKTHEN_DUCKDB_VERSION` selects one build and defaults to the first supported version. Canonical artifacts become:

```text
databases/duckdb/build/artifacts/cpp/<version>/<target>/thinkthen.duckdb_extension
```

Treat `THINKTHEN_DUCKDB_CPP_BUILD` as a CMake output base. The actual CMake directory is:

```text
<base>/<version>/<target>/
```

The raw CMake artifact is beneath that directory at `extension/thinkthen/thinkthen.duckdb_extension`. No CMake cache switches between source versions. Preserve caller output isolation, including the packer’s per-part scratch base and the site runner’s separate CMake and bridge bases.

Pass the selected pinned source commit and manifest into CMake. Retain refusals for wrong source identity, missing or extra archives, malformed manifests and changed archive bytes. Existing absolute source/static overrides remain supported for one selected build, but must satisfy that version’s pins. A changed source or archive cannot enter a successful artifact merely because its footer matches.

Retain locked offline Cargo builds, remapping, host-target checks, architecture checks, macOS deployment-target checks, footer-preserving stripping and hidden-export checks. Process the artifact into the canonical destination first. Validate its complete footer before admitting it to a consumer. The site and packer consume this processed file, including on macOS.

Preserve the existing `build/thinkthen.duckdb_extension` newest-version compatibility alias with explicit ownership:

- `cpp/build.sh` is its only writer.
- Only a successful default-version build refreshes it, from the validated final canonical artifact.
- An older-version build leaves it untouched.
- Refresh it atomically; a failed build supplies no new alias or successful receipt.
- Repository-owned checks, scans, packing, installed proof and site proof never read it.
- Add no compatibility copy at the obsolete `build/artifacts/cpp/<target>/` path or former CMake path.

The alias preserves the accepted prior convenience contract. It is not an artifact selector or fallback. Fresh High review checks its ownership and byte identity. A missing canonical artifact must fail even when this alias or an obsolete output exists.

Start with unchanged C++ sources. A demonstrated header incompatibility may justify the smallest guard in the affected leaf, recorded in `NOTES.md`. Preserve lifecycle, bind validation, interruption and exception/panic ownership. Do not introduce speculative guards before compiling against the selected pinned headers.

The concrete reader transition is:

| Reader or consumer | Required behavior in A | Required behavior in B |
|---|---|---|
| `check.sh` normal, full and stress branches | Explicit canonical path and matching interpreter for each selected build | Same |
| `check.sh` replay smoke | Copy the validated default canonical artifact into fresh scratch | Retain replay semantics |
| `tools/harness.py` and imported suites | Receive the explicit artifact; any standalone default resolves version/target canonical path, never the legacy alias | Installed calls receive unpacked paths |
| `tools/source_checks.py` | Scan both canonical files for shipped test markers; retain feature-graph protection | Same |
| `tools/site_examples.py` | Copy the explicitly selected canonical file into owned scratch | Same |
| `cpp/verify_package.py`, `verify_interrupt.py` and other explicit-path verifiers | Callers pass canonical paths and correct hosts | Installed callers pass unpacked paths |
| `tools/release_pack_cases.py` | Use default version-qualified path in an isolated scratch source tree | Exercise both versions and repository layout |
| `release-pack` | Package only the default canonical file at the archive root | Package every supported canonical file at repository paths |
| `package_notices.py` | Use explicit default source/version/manifest | Use all selected source/version/manifest tuples |
| `release-container` | Use the shared selector and separate version caches | Same preparation supports the two-file pack |
| `site/scripts/smoke-sql.mjs` | Copy the final canonical default file; stop reading raw CMake output | Lay out repository members for the install sample |
| `check.sh` installed branch | Retain old root-member archive contract and matching default host | Require both repository members and matching hosts |
| `install_check_channels.py` | Retain root-member reading because A’s archive shape is unchanged | Select the real host’s matching repository member |

Fresh SQL scratch copies named `./thinkthen.duckdb_extension` remain valid where existing function examples explicitly load that name. They are made from a validated canonical input during the run. They do not justify keeping a backward checkout output reader.

The broader inventory found these additional consumers:

- `release-workflow smoke-bundle` currently checks legal markers. A retains those checks; B adds exact repository validation.
- `release-smoke` routes by archive name, recursively scans unpacked contents and calls installed mode. Its routing, checksum checks and counts remain; B’s installed mode supplies both-version coverage.
- `surfaces` and `publish-builds` route the same archive name. No second DuckDB archive is introduced.
- `site/scripts/check-binding-proofs.mjs` checks archive names and saved proofs. Preserve its archive-name oracle and strict stale-proof behavior.
- `sdlc/scripts/inventory` inventories the Rust public API and has no DuckDB binary path assumption.
- `release-archive-tree.py` checks selected source-tree integrity and has no extension-member assumption.
- `install_check_tools.py` supplies the published channel’s stock CLI. Its runtime must remain pinned and must agree with the member-selection query.
- Historical `proof/0201` paths describe a separate probe and remain historical evidence, not current product output readers.

Repeat this search after rebasing onto completed 0402 and before each candidate review. Every remaining old path must be identified as an owned scratch filename, the explicit convenience alias, a historical record, or a reader corrected in that slice.

A’s `part_duckdb` explicitly selects the first supported version. It builds or reuses only:

```text
build/artifacts/cpp/<default>/<target>/thinkthen.duckdb_extension
```

It retains real architecture checks, footer checks, source validation, legal markers, existing notice content and the root archive extension. It does not silently change the packaged version because an inherited build selector names 1.5.4. Reject conflicting selector/override combinations rather than apply older inputs to the default package. Under `--reuse`, a missing canonical default file fails without building or consulting an obsolete path.

Update `package_notices.py` in A to receive the selected version and manifest explicitly. Its default-version output retains the single-version legal contract. This removes its old host-based manifest assumption before B needs two inventories.

Move Linux release preparation into A. The current container reads `DUCKDB_STATIC_ZIP_SHA256`, `DUCKDB_LINUX_ARM64_STATIC_ZIP_SHA256`, `DUCKDB_CPP_SOURCE_COMMIT` and `DUCKDB_VERSION`, then exports one global source/static pair. Those dependencies cannot survive the pin migration unchanged.

The container resolves each listed version through `inputs.sh`, caches downloads beneath version/target-qualified paths, and prepares separate source/static inputs under its declared toolchain root. Asset names alone cannot key two versions’ ZIPs. Validate source commits, ZIP digests and exact extracted membership; extracting over an old directory cannot establish a clean inventory.

The container exports the toolchain root, not one global source/static pair for all versions. The packer selects the correct inputs for its iteration. Preserve the container image pins, selected ThinkThen source archive identity, remaps, Cargo/toolchain locks, existing release routes and glibc compatibility checks. Container preparation remains outside offline validation.

For multi-version packing in B, existing generic source/static overrides apply only to the version named by `THINKTHEN_DUCKDB_VERSION`, or the default when unset. Resolve and validate that association before the loop. Other versions use their own prepared caches. Never forward one generic pair into both builds. The builder’s single-version override behavior remains intact.

The site’s actual reader changes in A. It reads the default from the shared version authority, validates the native target/platform, passes that version explicitly to `build.sh`, and reads the final canonical artifact. It stops reading:

```text
target/site-duckdb/cpp/extension/thinkthen/thinkthen.duckdb_extension
```

The CMake base is still isolated beneath `target/site-duckdb`; its new version/target dimensions are respected. The macOS runner uses and reports the pinned project-local CMake. Missing or failed final output fails the replay even if raw CMake output or legacy aliases exist.

A retains the current install sample and single-file archive wording. Its documentation distinguishes the two local builds from the one-version archive still shipped by this intermediate slice. Do not claim the archive serves 1.5.4 before B.

A’s check orchestration retains:

- Bridge format, Clippy and unit checks once.
- Both C++ builds and both shipped-marker scans.
- Explicit matching CLI, Python module, platform and extension for each version.
- The current newest suite sequence, selectors and named exceptions.
- Older conformance, rank, and the exact find and portable selections currently named by installed mode.
- Newest interruption checks and retained lifecycle evidence.
- Routine conformance selection; full and installed runs clear narrowing selectors.
- Smoke’s saved-answer contract.
- Stress campaigns only through `test-stress --run`.

Missing either version’s prerequisites produces no both-version pass. `CHECK_SETUP_ONLY` validates both prepared host environments while retaining its existing setup contract and invocation-from-three-folders regression.

A’s installed mode validates and loads only the default root member actually present in its single-version archive. It uses the newly prepared separate older CLI for the retained cross-version refusal. It does not load a checkout older artifact and label that as installed-package coverage. The full newest installed sequence remains, including panic-hook linkage, interruption, conformance, selected plan/find/portable/verbs/settings cases and rank.

A may assemble a two-version repository in fresh scratch from both canonical files to prove repository selection. That scratch repository does not change A’s archive contract.

Slice B completes the repository archive, its installed and published consumers, and public pages.

| Slice B files | Responsibility |
|---|---|
| `sdlc/scripts/release-pack` | Build/reuse, validate and stage every supported version |
| `cpp/package_notices.py` | Complete legal material and inventory for both pinned sources |
| New `cpp/verify_repository.py` | Validate archive/staging layout, regular members, footers and legal references |
| `tools/release_pack_cases.py` | Retained footer plants, new package plants and native offline consumer regression |
| `sdlc/scripts/release-workflow` | Validate repository members in `smoke-bundle` |
| `databases/duckdb/check.sh` | Load both unpacked members without rebuilding |
| `sdlc/scripts/install_check_channels.py` | Select the published member from real host identity |
| `sdlc/scripts/install_check_tools.py` | Preserve pinned runtime preparation using the shared input authority |
| `sdlc/scripts/install-check-self-test.py` | Offline consumer routing and refusal table |
| `sdlc/scripts/publish-builds` | Describe the archive’s version-specific contents |
| `site/src/data/catalog.mjs`, version reader and guard | Generated archive summary and independently checked support declarations |
| `site/package.json` | Run the version guard without dropping existing checks |
| `site/scripts/smoke-sql.mjs`, install SQL sample | Replay repository installation using processed artifacts |
| DuckDB READMEs, `site/README.md` | Explain supported versions, install forms and signing limits |
| `site/examples/bindings-proof.json` | Refresh only from successful executed replay |

`part_duckdb` iterates `DUCKDB_VERSIONS`. Before writing the final archive or sidecar, validate every selected file’s regular-file status, real ELF/Mach-O architecture, footer ABI, ABI version, platform, DuckDB version and ThinkThen version. Missing older input under `--reuse` fails without rebuilding, fallback or partial publication.

Stage exactly one extension per declared version at its version/platform repository path. The final archive contains no root extension duplicate, legacy alias or alternative version fallback. Keep one archive and sidecar per target, existing counts, four DuckDB routes and smoke routing. The Windows command route gains no DuckDB asset.

`verify_repository.py` validates staging and tar members before native loading. It derives required versions from the declared authority and the platform from the selected target. It does not discover expected coverage from files found in the package. Reject missing, duplicate, extra, wrongly placed, linked or escaping extension members. Tar metadata must identify extension members as regular files; generic extraction’s allowance for safe links must not weaken this package contract.

Collect legal material separately from both pinned sources. Deduplicate only byte-identical files. Retain root `LICENSE.duckdb`; when legal bytes differ, retain each version’s bytes at version-qualified legal paths and reference them in `NOTICE`. `NOTICE` names both versions and applicable legal paths. `DEPENDENCIES.txt` distinguishes each version’s selected static inventory and source notices, and records the common Rust inventory once. A one-version notice or an absent referenced legal file fails.

`release-workflow smoke-bundle` retains its legal markers and additionally validates the exact version/platform members and footers before assembling smoke inputs. Preserve sidecar verification, package counts and archive-name checks. Update affected workflow fixtures in the same slice.

B’s installed mode unpacks into fresh owned scratch, validates the repository layout, then selects each declared member with its matching prepared CLI and Python environment. Every extension loaded, scanned, checked for panic-hook linkage or passed to a suite comes from that unpacked archive.

Retain the complete newest installed sequence. Add older conformance, rank and the exact retained find/portable selections. Both files receive matching stock loads and cross-version refusal checks. No build occurs in installed mode. A missing member cannot cause a canonical checkout read, legacy root read or cached installed-extension load.

The published-install consumer is a required B change, separate from `check.sh` installed mode. `install-check.yml` dispatches this consumer after publication, so its current root-file assumption must be removed before the repository archive lands.

`install_check_channels.duckdb(check)` must:

1. Obtain the named ThinkThen release archive through existing `check.release`, preserving checksum verification and safe extraction.
2. Query the actual pinned CLI used for replay with `SELECT version(); PRAGMA platform;`, using an empty database and the clean consumer environment.
3. Require exactly one valid version and platform result. Validate the version against `DUCKDB_VERSIONS` and the platform against the channel’s supported native target.
4. Select precisely:
   ```text
   <unpacked>/<reported-version>/<reported-platform>/thinkthen.duckdb_extension
   ```
5. Require that regular archive member and validate its footer against the reported host and requested ThinkThen release version.
6. Load that exact unpacked path with unsigned extensions allowed at startup.
7. Retain `thinkthen_replay`, `thinkthen_cache='off'`, the recorded question/evidence, and the details call.
8. Read the loaded ThinkThen version from `duckdb_extensions()` and return it as the installed version.
9. Preserve the common `check_result` requirements: installed version equals the requested release, answer is boolean `true`, and `requests_sent` is integer zero.

The loaded-version query must identify one loaded ThinkThen extension. Do not infer installed version from the archive name, requested version or footer alone. Do not select by first glob, hard-coded newest folder, checkout default, root duplicate or cached installation.

`install_check_tools.py` retains stock CLI ZIP, extracted binary and runtime identity checks. Its preparation uses the same selected pins. A separately pinned test runtime is not another supported-version list. The existing Linux x86-64 workflow cell remains; widening the published workflow is unnecessary for this ticket.

Add offline regression through the **real DuckDB consumer**, not merely its path helper:

- A repository-shaped archive selects the independently expected member for the host’s reported version/platform.
- A missing matching member fails while another version’s valid member remains.
- Keep valid canonical checkout artifacts, a stale root file and an unrelated installed extension available during that negative. None may rescue it.
- Wrong folder/footer identity, unsupported host version/platform and linked matching members fail for their named cause.
- Wrong loaded ThinkThen version, wrong answer and nonzero request facts still fail through the common result validator.

Portable cases belong in `install-check-self-test.py`, already called by `workflows --self-test` from lint. They invoke the actual channel function and extraction path with local fixture transport and bounded host-process fixtures. Synthetic host fixtures establish routing and refusal behavior only.

Artifact-dependent regression runs after both real builds through the DuckDB package cases. Use the cached genuine stock CLI, real locally packed extension bytes and saved first-run recording. Replace only release acquisition with local fixture files; keep the actual consumer, host query, extraction, load SQL and result validation. Run the missing-member negative with genuine other-version bytes still present. Require consumer failure, the missing expected repository path, no replay success receipt and zero counted backend requests. This runs offline and dispatches no public workflow.

The site’s B install sample becomes:

```sql
INSTALL thinkthen FROM './';
LOAD thinkthen;
```

Its scratch working directory contains the repository layout. Provide a fresh owned extension installation directory at database startup, together with unsigned configuration. Installation must fetch the matching scratch member; an already installed extension cannot satisfy replay.

The runner continues consuming final processed canonical files. Existing function samples that load `./thinkthen.duckdb_extension` receive a fresh scratch copy from the selected canonical member. This scratch copy is not included in the archive. Preserve saved SQL output bytes.

Generate the archive summary from `DUCKDB_VERSIONS`, but retain separately authored support declarations in the catalog and DuckDB README. `check-duckdb-versions.mjs` checks those declarations independently. A generated summary alone would automatically follow a changed list and could conceal stale prose.

The guard runs from DuckDB checks and `npm run build`; include it in the relevant site check path without removing or reordering existing checks unnecessarily. Plant a third supported version while leaving declarations unchanged and require both omissions to be reported. Also test each declaration separately. Run these plants directly against the declaration guard so missing build pins cannot mask the intended failure.

Public wording must communicate:

> The archive contains unsigned extensions for DuckDB v1.5.5 and v1.5.4. Use dbt v1 with duckdb 1.5.5 for the documented 0.2 dbt route. dbt v2 requires signed extensions; this archive does not enable ThinkThen in dbt v2.

Document direct loading by the matching repository member as an alternative. Preserve 0402’s completed page structure, redirects, terminology and guards when rebasing. Follow `site/WRITING.md`. Refresh `bindings-proof.json` only through successful replay. Missing toolchains, `--allow-missing` results, synthetic binaries or edited proof entries cannot qualify strict site proof.

Independent proof must protect the actual failure causes.

| Contract | Required evidence |
|---|---|
| Pin routing | Retain all four literal newest `--inputs` expectations and unsupported-host exit-77 cases; add reviewed older expectations after real pins exist |
| Selected inputs | Wrong commit, wrong-version source/static pair, missing/extra archive and changed bytes fail before artifact admission |
| Footer | Wrong platform, DuckDB version, ThinkThen version, ABI and ABI version; missing/truncated footer |
| Canonical readers | Missing canonical output fails despite valid obsolete outputs; final processed output is consumed |
| Alias ownership | Only default build refreshes it; older build leaves it unchanged; no proof reader consults it |
| A package | Default canonical input produces old single-version shape; obsolete target output cannot rescue missing canonical input |
| B package | Exactly one member per required version/platform; missing older file, swapped folders, wrong platform, duplicates, extras, links and escapes fail |
| Notices | Both versions and applicable legal references are required |
| Packer | Real `--reuse` entry point rejects plants with the intended diagnostic and creates no archive or sidecar |
| Installed consumer | Missing unpacked member fails without any checkout or installation-cache fallback |
| Published consumer | Actual channel function selects real host identity and retains loaded-version, answer and zero-request assertions |
| Site | Catalog and README omissions fail independently; replay consumes processed output and preserves saved bytes |

The footer fixture writer independently encodes the 534-byte format and fixed field positions. It imports neither production parser constants nor the parser. Synthetic binaries prove metadata and layout only.

Preserve both existing refusal regressions:

1. Matching 1.5.5 Python refuses the 1.5.5 binary with its footer patched to 1.5.4.
2. Stock 1.5.4 CLI refuses the unchanged 1.5.5 binary.

Add symmetric cases for the genuine 1.5.4 build. Patch only the footer’s fixed version field; replace whole-binary `rfind`.

For actual and patched wrong-version loads, pin exit 1 and DuckDB’s sentence containing both identities:

```text
The file was built specifically for DuckDB version '<built>' and can only be loaded with that version of DuckDB. (this version of DuckDB is '<running>')
```

Allow surrounding host diagnostics while requiring that sentence. A crash, signature refusal, absent dependency or unrelated load error cannot satisfy the oracle.

Both stock versions execute repository `INSTALL ... FROM '<folder>'; LOAD thinkthen;` in fresh extension directories with unsigned configuration established at startup. A repository containing only the newer member must fail installation on the older host and identify the missing older repository path. Keep the matching host’s install cache empty in that case.

Run real packer plants in an owned scratch source tree containing copied canonical artifacts and tracked packer inputs. Invoke that tree’s actual `release-pack`, whose repository root follows its script location. Do not mutate and restore the lane’s real canonical artifact as the current test does. Shared source/static caches remain read-only inputs. Assert exact captured exit codes and stable diagnostic causes. Failed plants leave neither archive nor checksum sidecar in fresh output directories.

Wire portable checks into existing cheap gates. Wire artifact-dependent packer and native published-consumer cases into the DuckDB check after both builds. A script present on disk without a caller is not proof.

A and B each require their own passing candidate evidence:

| Slice | Passing evidence before landing |
|---|---|
| A | Complete selected pins; portable routing/input/reader regressions; both native builds and matching stock loads; symmetric refusals; retained newest suites and older selected suites; old-format pack plus installed smoke from canonical default input; actual site replay from final canonical output |
| B | Exact repository/layout/legal plants; real two-version pack; both unpacked installed loads and selected suites; offline published-consumer positive/negative regressions; site version guard and repository-install replay; retained release routing, counts and proof checks |

Use fresh owned source/output locations for clean-transition proof. No pre-existing CMake output, canonical artifact, alias or install cache qualifies an omitted step. Stale-file plants belong in scratch and must demonstrate rejection.

All future proof uses explicit minimal environments, declared UTF-8 locales, pinned tools, fake keys and owned home/config/cache/state/extension directories. Count no-send assertions at a loopback backend. Keep provider credentials absent. Preparation stays outside offline gates.

For each native target, both unchanged real binaries must load by path and repository installation in matching stock CLI and Python hosts. CLI decide smoke prints exactly `true` and exits 0. Require both actual cross-version refusals and both patched-footer refusals.

Retain the full newest suite sequence. Run older conformance, rank and the retained find/portable selections against older Python and older bytes. Reconcile selected case counts and named exceptions. Linux x86-64 and Apple Silicon archives require both-version installed proof. All four targets require release-runner proof in the separately authorized rehearsal; Rosetta evidence remains explicitly labeled and does not establish native Intel or macOS 15 qualification.

Before each candidate review, run:

```sh
CARGO_NET_OFFLINE=true python3 sdlc/scripts/policy.py
```

Run focused checks for each correction. The coordinator names checkpoints before full `test`, `spec` or `surfaces`. Keep required landing gates and the all-surface release checkpoint. Passing focused DuckDB checks alone does not qualify a release. Preserve valid receipts only while their source and inputs remain unchanged.

No Rust change is anticipated. Any necessary Rust change requires its own explanation, the 500-nonblank-line cap, exact measured ratchet accounting and fresh review. Keep new readers, validators and fixtures bounded, preferably below 250 nonblank lines each. Preserve checks protecting behavior, secrecy, spend, boundaries and ticket evidence.

Fresh High review must specifically assess:

- C++ headers, source, static archives and footer belong to the selected release.
- Version/target CMake isolation and container input routing prevent cross-version contamination.
- Every consumer follows its slice’s actual artifact/archive contract.
- Alias ownership cannot conceal an obsolete reader.
- Package layout, architecture, footer and legal checks fail before archive publication.
- Installed and published checks use unpacked bytes and retain loaded-version and zero-request evidence.
- Header guards preserve lifecycle, interruption and exception boundaries.
- Negative oracles establish the intended cause independently.
- macOS processing preserves footers, hides exports and reaches actual target proof.
- Public wording separates stock unsigned compatibility from dbt v2’s remaining signature and custom-driver limits.

A separately authorized release rehearsal must build and smoke all four DuckDB targets with zero missing prerequisites or skipped target checks. Record the exact run, commit and archive digests. Preserve existing release scans, sidecar checks, package counts, strict site-proof rules and documented conformance exceptions. This design authorizes no dispatch.

The experiments team’s later manual P0e rerun should determine whether version refusal disappears and signature refusal remains. Record the actual result; do not predict it as success. It cannot replace a signed ThinkThen load in dbt v2.

Ticket 0415 remains separately reviewed post-core community-listing preparation. Signed dbt v2 loading, external submission, enrollment, spend, a dbt package, additional DuckDB versions and Windows DuckDB builds remain deferred. `release/0.1` stays frozen.

Ian can overturn the combined archive choice, two-version policy, C++ API choice and older-suite scope already named in 0403. Until then they remain constraints. The parent should freeze this complete design and obtain fresh High review. The builder then rebases it against completed 0402 before beginning A.