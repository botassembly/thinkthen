# 0381 A accepted Windows C DLL design

Status: accepted by the fresh High design review. Accepted source SHA-256: `f5d6f3155e8a94747d4d13d7bb15eff740ed11db2c91f1adaa19b307668d68f3`. The implementation starts at landed main `6d26206aa8861490cc6540b6bafafc08e3249c3c` under the subsequent lane assignment. Historical baseline/unlanded statements below describe design review time; 0380 C source has since landed, with source and canonical receipts complete and native Windows proof still pending. No rebase or copied unlanded patch is needed.

The following accepted text is preserved exactly from the frozen proposal, including historical status wording. Implementation and platform review are separate from design acceptance.

**Corrected frozen design: ticket 0381, slice A — Windows x86-64 C DLL.**

Status: proposed for fresh independent acceptance. This replaces the earlier frozen proposal and corrects its recorded workflow and fixture migration gap. It preserves the remaining proposal choices.

Baseline remains main `110e6ac88ba3700cedaf504d6beb8b83655a5c27`. Ticket 0380 C at `51bad945a` has source acceptance and passing Linux tests/spec, but remains unlanded and has no native Windows proof. Implementation follows lane assignment and design acceptance. Rebase onto then-current main before implementation; do not copy unlanded 0380 C patches.

This design uses read-only inspection of workspace/repository instructions, [ticket 0381](sdlc/tickets/0381-windows-c-dll.md), the frozen proposal and finding, and the affected repository files. No repository edits, builds, tests, credential reads, provider calls, workflow dispatches, or helpers occurred.

1. **Authority and retained contracts**

   The [specification](specification/README.md) remains the behavioral contract. [The C header](libraries/c/include/thinkthen.h) and [C design](libraries/c/DESIGN.md) govern signatures, ABI and ownership. ADRs 0037, 0047 and 0111 retain the public-Rust-API dependency and private bundled SQLite rules.

   Follow the [Windows plan](sdlc/planning/windows.md), [current queue plan](sdlc/planning/cleanup-2026-09-30.md), [Rust standards](sdlc/planning/rust-standards.md) and [lane rules](sdlc/planning/worktrees.md). This correction introduces no public function, setting, installer option, dependency or stage 2 package.

2. **Ship the accepted three-file C archive**

   Derive the version from the existing manifests. For the current version, produce:

   ```text
   thinkthen-c-0.2.0-x86_64-pc-windows-msvc.zip
   thinkthen-c-0.2.0-x86_64-pc-windows-msvc.zip.sha256
   ```

   The ZIP contains exactly these regular files, in this order, without a wrapper directory:

   ```text
   include/thinkthen.h
   bin/thinkthen.dll
   lib/thinkthen.dll.lib
   ```

   `thinkthen.dll.lib` is an import library. Slice A ships no static implementation library, Windows `.pc`, PDB, EXP, generated DEF, runtime DLL or installer.

   Preserve the command ZIP’s one-file inventory. The PowerShell installer still installs only the command and its receipt.

   Preserve every Unix archive name, shared-library name, soname/install-name link, localized `libthinkthen.a`, `.pc` content and `Libs.private` value. [localize.sh](libraries/c/localize.sh) stays unchanged.

3. **Use the header as deterministic export authority**

   Add one small declaration reader shared by [build.rs](libraries/c/build.rs) and the door contract tests. Strip comments, read function declarations and return sorted unique names. Reject duplicate declarations, malformed declarations and unterminated comments rather than silently discarding them. Independent parser fixtures state their expected declarations.

   The frozen baseline inventory is:

   ```text
   thinkthen_call
   thinkthen_call_opts
   thinkthen_cancel
   thinkthen_cancel_token_free
   thinkthen_cancel_token_new
   thinkthen_decide
   thinkthen_decide_many
   thinkthen_decide_many_opts
   thinkthen_decide_many_with_facts
   thinkthen_decide_many_with_facts_opts
   thinkthen_decide_opts
   thinkthen_decide_with_facts
   thinkthen_decide_with_facts_opts
   thinkthen_engine_free
   thinkthen_engine_new
   thinkthen_engine_new_with
   thinkthen_error_code
   thinkthen_error_facts_json
   thinkthen_error_message
   thinkthen_error_retryable
   thinkthen_free_string
   thinkthen_plan_json
   thinkthen_question_file
   thinkthen_recognize
   thinkthen_recognize_opts
   thinkthen_recognize_with_facts
   thinkthen_recognize_with_facts_opts
   thinkthen_relate
   thinkthen_relate_opts
   thinkthen_relate_with_facts
   thinkthen_relate_with_facts_opts
   ```

   Executable export checks derive their expected inventory and count from the header. The produced DLL never supplies its own expectation.

   In the Windows MSVC arm, `build.rs` registers the header as a rebuild input and writes this DEF under `OUT_DIR`:

   ```text
   LIBRARY thinkthen.dll
   EXPORTS
       <sorted header declarations>
   ```

   Pass it through a cdylib-only linker argument. Preserve the existing ELF and macOS arguments. Add no ordinals, calling-convention macros, signatures or public settings.

   The DEF remains a proposed linker input. Native inspection must establish how it interacts with Rust’s generated export handling and whether every unwanted export is excluded.

4. **Generate the public import library for the shipped DLL name**

   Cargo retains the internal name `thinkthen_c`. Copy its DLL to `bin/thinkthen.dll`.

   Generate the public import library explicitly with MSVC `lib.exe`, using a DEF derived from the same header authority and naming `thinkthen.dll`:

   ```text
   lib.exe /DEF:<def-path> /MACHINE:X64 /OUT:<stage>\lib\thinkthen.dll.lib
   ```

   The Windows pack helper and native door archive setup use the same import-library production operation. Merely renaming Cargo’s import library is insufficient because its descriptor may still name `thinkthen_c.dll`.

   Packaging owns final public import-library production; `build.rs` owns DLL export input. This preserves the earlier proposal’s refinement of the ticket wording.

   Native proof must inspect the import library and linked consumer, establish x64 identity and the `thinkthen.dll` descriptor, and run the consumer with only the public DLL available. Neither may require `thinkthen_c.dll`.

5. **Migrate packing, platform verification and collection together**

   In [release-pack](sdlc/scripts/release-pack), add `c` to the Windows allowlist and change the Windows default parts to:

   ```text
   command c first-run
   ```

   Preserve the existing profile, scratch, remapping, native-path conversion and `--reuse` rules. The Windows C branch builds the C workspace and delegates staging, explicit import-library generation and ZIP checking to one bounded helper. It never enters Unix localization or `.pc` production.

   ZIP packaging uses fixed member order and metadata and preserves supplied bytes. Identical supplied files must produce identical ZIP bytes; this makes no compilation reproducibility claim.

   Extend the existing `platform` action in [release-windows-command.py](sdlc/scripts/release-windows-command.py) to require exactly six regular files:

   ```text
   thinkthen-<version>-x86_64-pc-windows-msvc.zip
   thinkthen-<version>-x86_64-pc-windows-msvc.zip.sha256
   thinkthen-c-<version>-x86_64-pc-windows-msvc.zip
   thinkthen-c-<version>-x86_64-pc-windows-msvc.zip.sha256
   thinkthen-first-run.tar.gz
   thinkthen-first-run.tar.gz.sha256
   ```

   Validate the command ZIP, C ZIP and sample checksum separately. Keep command `pack` and `check` semantics unchanged. The C check requires the exact layout, committed header bytes, correct DLL identity and valid import-library structure. Portable validation does not substitute for native export or loader proof.

   Update [release-workflow](sdlc/scripts/release-workflow) collection diagnostics to describe the Windows command/C bundle. Its Windows platform verification must complete before collection creates output or copies files. Retain the five-target inventory, Unix family checks, duplicate handling and first-run equality rules.

   [release-windows-smoke.py](sdlc/scripts/release-windows-smoke.py) already calls the platform verifier. It therefore consumes the six-file platform contract while retaining its command, installer and packed-crate behavior. C archive execution remains a distinct required smoke route.

6. **Migrate the workflows and their existing gate explicitly**

   Inspection confirms that [release.yml](.github/workflows/release.yml) currently packs Windows `command first-run`, and [workflows](sdlc/scripts/workflows) requires that literal command. Change the Windows branch and its gate expectation together to:

   ```text
   sdlc/scripts/release-pack "$TARGET" release-files command c first-run
   ```

   Leave the separate Linux final-pack occurrence of `command first-run` unchanged.

   In Windows host setup, fetch both locked workspaces, in order:

   ```text
   cargo fetch --locked --manifest-path Cargo.toml
   cargo fetch --locked --manifest-path libraries/c/Cargo.toml
   ```

   Retain the Windows early return before Unix tool acquisition.

   In `release.yml`, establish an x64 MSVC developer environment before native C compilation, import-library production or inspection. Require root workspace checks and C workspace Clippy and tests, including the enabled C door cases, before packing and platform artifact upload. Keep all builds offline after dependency acquisition.

   In the Windows smoke job, establish the same MSVC prerequisite. After downloading `platform-${{ matrix.target }}`, validate and consume the downloaded C archive through the bounded archive-smoke entry point. Compile against its extracted header and import library and run its extracted DLL. A source-built door cannot satisfy this route.

   In [windows.yml](.github/workflows/windows.yml), add the MSVC prerequisite before its existing C checks. Retain its locked fetches, failure-reporting checks and cache behavior.

   Extend `workflows` to enforce these routes in their actual Windows execution context:

   - x64 MSVC setup precedes native C operations.
   - C Clippy and door tests execute before packing/upload.
   - The Windows pack includes `c`.
   - The six-file platform verifier runs before upload.
   - The downloaded C archive check and consumer run after artifact download.
   - The standalone Windows workflow establishes MSVC before C checks.

   Check job/step conditions and ordering. A command in a Unix branch, a comment, an echo, a disabled step, or a failure-tolerant route does not satisfy the requirement.

   Add independent negative plants to the existing workflow self-test for omitted C Clippy, omitted C tests, omitted or late MSVC setup, old Windows packing, omitted platform verification, omitted downloaded C validation and omitted archive consumption. Also plant wrong-target conditions and failure masking for the required C routes. Each plant must identify the intended missing boundary through the gate’s expected diagnostic.

   Retain all existing workflow plants, including Windows matrix, shell, command/crate smoke, Unix binding exclusion and draft exclusion cases. Preserve action pins, resolved-source checkout, LF bytes, scratch platform folders, workflow permissions, release-environment approvals, `RELEASE_ARMED` and outward-job approvals.

7. **Migrate the Windows command self-test without losing its regressions**

   [release-windows-command-self-test.py](sdlc/scripts/release-windows-command-self-test.py) currently asserts root-only fetching. Change its exact host call expectation to the pinned toolchain installation followed by the two locked fetches above. Continue refusing Python package acquisition in that fixture. Unexpected Unix tools, extra fetches or an omitted C fetch must fail.

   Its collection fixture currently copies the command archive and adds the sample, giving four Windows files. Add a synthetic, structurally valid C ZIP with the committed header, x64 DLL bytes and import-library fixture, plus its real checksum. Assert the exact six-file Windows contribution to the collected output.

   Keep command-only explicit packing checks command-only. Preserve deterministic bytes, malformed PE/ZIP refusals, wrong-name refusal, invalid-input non-overwrite, duplicate/unsafe/linked members, checksum failures, debug/release routing, scratch cleanup, native-path conversion, unsupported parts and absent-platform rejection.

   Update inventory diagnostics for the six-file bundle. Retain the extra unsupported binding plant; the accepted C ZIP must not make arbitrary C-named files admissible.

   Extend collection refusals to cover a missing C ZIP, missing C checksum, malformed C archive with a recomputed valid checksum, bad C checksum, wrong header and extra Windows asset. Restore fixture inputs between plants. Assert the exact refusal exit code and intended cause, and that no collection output exists after each pre-copy refusal.

8. **Migrate the archived release self-test to six-file verification**

   [release-archive-self-test.py](sdlc/scripts/release-archive-self-test.py) independently constructs a four-file Windows collection fixture. Add the same required C archive structure and its checksum to that fixture, using the resolved version and appropriate committed header bytes. Extend `expected_files` by exactly the C ZIP and sidecar.

   Run the real collection route against all five platform folders. Do not bypass Windows verification or relax its inventory to make the fixture pass. Share bounded fixture construction where useful; retain separate collection assertions in both existing tests.

   Add a collection refusal for an omitted C asset and one for malformed C content with a valid checksum. Pin the refusal exit code and cause and assert that collection creates no output.

   Preserve the existing Unix family refusals, unexpected-platform checks, archive identity checks, source file/link/configuration plants, refusals before Cargo, stale C archive handling, archived `--reuse` rejection, deterministic first-run proof, container transport identity, tool checksum refusals and runner PATH/environment protections.

   Both existing fixtures migrate in the same implementation change as the six-file verifier. A new ticket-specific test cannot replace either migration.

9. **Port the existing door through small platform branches**

   Remove the blanket Unix exclusion from [door/main.rs](libraries/c/tests/door/main.rs). Extract archive setup, compilation and binary inspection into small Unix and Windows modules. Keep behavioral assertions shared.

   The Windows compiler route uses MSVC, with paths passed as process arguments: The Windows C test fixtures alone define `_CRT_SECURE_NO_WARNINGS` because their retained `scanf` calls trigger C4996 under `/WX`. This exact reviewer-requested adaptation preserves framing and keeps `/W4 /WX`; it does not suppress warnings in product libraries. Native driver compilation and execution remain required proof.

   ```text
   cl.exe /nologo /TC /std:c11 /W4 /WX /MD /D_CRT_SECURE_NO_WARNINGS
       /I<include-directory> <source.c>
       /Fo<object-path> /Fe<program.exe>
       /link <import-library-path>
   ```

   Stage `thinkthen.dll` beside each consumer executable. No Unix rpath, pthread linker option, soname link, `readelf` or localization command enters the Windows branch.

   Apply local fixture adaptations:

   - [threads.c](libraries/c/tests/c/threads.c), [cancel.c](libraries/c/tests/c/cancel.c) and [typed_facts.c](libraries/c/tests/c/typed_facts.c) use Windows thread operations that check creation, join and close owned handles. Preserve failure isolation and saved-pointer assertions. The Windows arrival barrier uses native atomic operations.
   - [driver.c](libraries/c/tests/c/driver.c) uses checked Windows `_putenv_s` and `_strdup` branches.
   - Byte-sensitive Windows fixtures set the relevant streams to binary mode. This includes driver framing, cancellation handshake output, [plan.c](libraries/c/tests/c/plan.c), and the pinned output of [slide.c](libraries/c/examples/slide.c) and [functions.c](libraries/c/examples/functions.c). Preserve LF expectations.
   - [fork.c](libraries/c/tests/c/fork.c) and its exact two-request assertion remain Unix-only.
   - [atexit.c](libraries/c/tests/c/atexit.c) runs as a Windows host-exit regression. Retain the stronger Unix teardown evidence without claiming identical Windows destructor ordering.
   - [usage.rs](libraries/c/tests/door/usage.rs) uses existing platform-folder helpers and the Windows usage location. XDG variables do not become Windows defaults.

   Compiler children receive a minimal explicit environment containing required tool paths and MSVC variables such as `INCLUDE`, `LIB` and `LIBPATH`. Runtime children retain required Windows system variables, owned scratch folders, explicit loopback addresses and fake keys. Do not forward ambient `CL`, `_CL_`, `LINK`, credentials, backend settings or the parent environment wholesale.

   Preserve bounded child lifetimes, owned-process cleanup and scratch locking.

10. **Require source and shipped-archive proof**

   Native inspection uses actual produced files and complete `dumpbin` inventories:

   | Boundary | Required observation |
   | --- | --- |
   | Archive | Exact names, members, checksums, regular files and committed header bytes |
   | DLL | PE32+ x86-64 DLL |
   | Exports | Complete named inventory equals header declarations; no missing, extra, decorated, forwarded or ordinal-only additions |
   | Import library | x64 imports targeting `thinkthen.dll` |
   | Consumer | Links shipped header/import library and loads only the shipped public DLL |
   | ABI | Existing argument types, answer layout, version macros, codes and sentinels |
   | Ownership | Owned allocations return through DLL free functions; failures remain borrowed |
   | Refusals | Exact codes/messages, unchanged outputs and counted zero requests |
   | Cancellation | Held replies, pinned fire order, no later sends and retained cache/counters |
   | Semantics | Existing requests, results, byte comparisons, row counts and failure facts |

   Use `dumpbin /exports`, `/headers` and appropriate import/dependency inspection. Do not filter exports to `thinkthen_` before comparison.

   Retain nulls, options equivalence, engine/thread isolation, typed facts, plan, named-file refusals, batching, golden cases, usage and cancellation. Missing Windows proof never justifies deleting Unix regressions.

   The downloaded-archive consumer reproduces the installed spend refusal and counted successful call from [check.sh](libraries/c/check.sh): exact refusal code/message with zero loopback requests, then a successful call with exactly one request. Its header, import library and DLL all come from the downloaded archive. This detects a correct source build paired with stale or incorrect packed files.

11. **Keep portable evidence and sanitizer evidence separate**

   Add one ticket-specific portable test file and invoke it through the existing `workflows --self-test` route, already reached by [lint](sdlc/scripts/lint). It covers declaration parsing, deterministic ZIP bytes, archive/member validation, PE/COFF structural refusals, saved export-output parsing, C pack routing, import-library tool failure propagation and owned scratch cleanup.

   The migrated existing workflow gate and both collection fixtures remain required alongside it. Update self-test reporting for added cases.

   Stubbed tools establish routing and refusal behavior only. Synthetic PE/COFF bytes and saved `dumpbin` output establish no native linker, loader or execution result.

   Retain cleanup plants. Any new or changed cleanup guard must reject a checkout/lane path before deletion and remove only scratch paths created by that invocation. Exercise tool failures during staging and consumer setup without deleting caller-owned input.

   Keep the proposed separate MSVC ASan check using `/fsanitize=address` on C consumers. Before claiming coverage, an owned negative consumer must produce the expected ASan diagnostic, establishing instrumentation and runtime availability. Remove its generated files afterward.

   Then run applicable ownership and concurrency fixtures under ASan. C-consumer instrumentation does not instrument the Rust DLL and provides no Unix LSan evidence. Do not carry `detect_leaks=1` claims into Windows. If unavailable, record the exact toolchain failure and coverage limitation for review. Ordinary native C execution remains required.

12. **Preserve privacy and the slice boundary**

   Slice A adds no Rust dependency, native feature expansion, unsafe allowance, console handler, ACL implementation or core change. Retain the public API dependency and private bundled SQLite.

   Keep key lookup, named-backend routing, header-free recordings, secret-free diagnostics, count-only usage, read-only configuration and spend limits unchanged. Manual live-call authority remains unchanged.

   The accepted 0380 C source and Linux results do not establish native privacy or cancellation proof. Its pending evidence and W1/W2 status remain separate.

   Slice B owns COFF static localization, static-link coexistence and Windows static-link metadata. Cargo’s raw static output does not ship in A. B must investigate actual tool behavior before declaring localization unsupported. A clean static result must keep Rust runtime and bundled SQLite symbols nonglobal. If no clean solution emerges, retain DLL-only distribution and file the ticket-required stage 3 issue with evidence and reopening conditions.

   Inspect runtime dependencies before release. This design adds no runtime bundling, installer change, signing purchase or public distribution commitment.

13. **Complete implementation footprint and proof responsibility**

   The candidate must include every affected file below. New helper filenames are internal implementation details; they introduce no public API.

   | Files | Change and required proof |
   | --- | --- |
   | [build.rs](libraries/c/build.rs); new shared header-declaration module | Windows DEF generation, rebuild input and parser contract cases |
   | [door/main.rs](libraries/c/tests/door/main.rs); new Unix/Windows door platform modules | Enable Windows cases; preserve Unix archive/compiler/inspection behavior; native MSVC and complete export proof |
   | [door/usage.rs](libraries/c/tests/door/usage.rs) | Platform-folder mapping and retained send/cache counts |
   | [threads.c](libraries/c/tests/c/threads.c), [cancel.c](libraries/c/tests/c/cancel.c), [typed_facts.c](libraries/c/tests/c/typed_facts.c), [driver.c](libraries/c/tests/c/driver.c) | Windows threading, checked environment/string operations, framing and retained behavioral assertions |
   | [plan.c](libraries/c/tests/c/plan.c), [slide.c](libraries/c/examples/slide.c), [functions.c](libraries/c/examples/functions.c) | Windows binary output and unchanged pinned bytes |
   | [release-pack](sdlc/scripts/release-pack); new bounded Windows C pack/check helper | Windows C allowlist/default, staging, public import-library generation, deterministic packaging and refusal/cleanup cases |
   | [release-windows-command.py](sdlc/scripts/release-windows-command.py) | Exact six-file platform verification and independent asset checks |
   | [release-workflow](sdlc/scripts/release-workflow) | Root/C locked fetches and verified collection before copying |
   | [release.yml](.github/workflows/release.yml), [windows.yml](.github/workflows/windows.yml) | MSVC prerequisite, native C checks, packing and downloaded-archive consumption |
   | [workflows](sdlc/scripts/workflows) | Updated literal pack contract, required native/archive routes, negative plants and portable-test wiring |
   | [release-windows-command-self-test.py](sdlc/scripts/release-windows-command-self-test.py) | Two-fetch expectation, six-file collection fixture and retained/extended refusals and cleanup |
   | [release-archive-self-test.py](sdlc/scripts/release-archive-self-test.py) | Six-file fixture, exact collection inventory and retained/extended archive refusals |
   | New bounded Windows C archive-smoke entry point; new ticket-specific portable test file | Actual shipped consumer proof versus explicitly limited synthetic routing proof |
   | [C DESIGN.md](libraries/c/DESIGN.md), [C README.md](libraries/c/README.md), [root README.md](README.md) | DLL/import-library usage, DLL-only slice A and correction of the current command-only Windows binding statement |
   | [C Rust ratchet](libraries/c/ratchet.json), [C fixture ratchet](libraries/c/ratchet.c.json), [root ratchet](sdlc/ratchet.json) where measured totals change | Exact measured totals, explained growth and unchanged Rust file cap |
   | [Ticket 0381](sdlc/tickets/0381-windows-c-dll.md) and its design/build/review records | Record accepted production responsibility and actual proof status |

   [release-windows-smoke.py](sdlc/scripts/release-windows-smoke.py), [check.sh](libraries/c/check.sh), `fork.c`, `atexit.c` and the other shared door assertions are proof inputs. Preserve their contracts; this design does not require unrelated edits to them.

   Current C Rust and C fixture ceilings are 4,953 and 1,245. These are baselines, not approved increases. Preserve the 500-nonblank-line Rust cap, exact measured totals, dependency locks, lane outputs and shared toolchain/cache mutation locks.

   After lane assignment and implementation, run focused portable workflow/collection/packing checks and affected C checks. Run offline policy before fresh Rust code review. The coordinator names checkpoints before full `test`, `spec` or `surfaces`. Required Linux/macOS checks protect unchanged Unix behavior. Native Windows evidence requires separately approved dispatch of the exact reviewed commit and recorded outputs.

Ian can overturn the archive layout, explicit import-library production, sanitizer limitation acceptance and DLL-only staging choice. **This corrected design is frozen for fresh acceptance; implementation and native proof remain pending.**

Implementation note: the assigned baseline is the landed 0380 C commit named above. The native source door retains a separate normal MSVC sanitizer test under the binding policy; it requires its owned negative before claiming C-consumer coverage. Portable receipts remain separate from pending native Windows execution.

Current implementation status: source `fd98fbbaaab7db4de3a892dedd7425251bac1478` implements slice A against the assigned landed baseline. Focused Linux C checks and required lint pass with exact ratchets 114995, 5223 and 1323. [The build record](0381-a-windows-c-build.md) holds source fingerprints, growth and actual receipt digests. Fresh High ABI/platform review, the parent checkpoint and native authorization/execution remain pending; the accepted proposal text above is preserved exactly.
