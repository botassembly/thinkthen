# 0381 A Windows C DLL build

Status: clean source candidate ready for fresh High ABI/platform review. Required lint and focused Linux C checks passed. The parent-named full checkpoint remains pending. Native Windows linker, loader, execution, sanitizer availability and runtime dependency proof are explicitly pending.

The accepted design's source SHA-256 is `f5d6f3155e8a94747d4d13d7bb15eff740ed11db2c91f1adaa19b307668d68f3`. The historical design baseline predates 0380 C landing; this implementation starts from landed main `6d26206aa8861490cc6540b6bafafc08e3249c3c`. The lane and branch are assigned in ticket 0381. No unlanded patch was copied or rebase performed. The parent supplies the fresh reviewer; this builder uses no helpers.

The Windows C ZIP contains exactly the committed header, x64 public DLL and x64 MSVC import library, with its checksum beside it. Final import-library production invokes `lib.exe` against the header-derived DEF naming `thinkthen.dll`. Portable validation checks regular members, exact order/bytes, PE identity, COFF short imports and both MSVC linker-table references. Native inspection compares the full dumpbin export inventory against header declarations and inspects both import library and linked consumer. Unix static localization, sonames, metadata and archive routing remain unchanged.

The Windows source door uses shared behavior assertions with checked native threads/handles, interlocked arrival ordering, checked CRT environment/string operations and binary framing. Fork stays Unix-only. The native ASan test first requires an actual diagnosed owned use-after-free before checking C ownership and concurrency; no Rust-DLL or Windows leak instrumentation claim is made. The downloaded-archive consumer independently links the extracted header/import library, supplies only the public DLL and pins the installed spend-refusal sentence/code with zero loopback requests, then the success bytes and exactly one request.

Workflow checks enforce MSVC setup, root/C checks before packing, six-file verification before upload, and downloaded validation/consumption after download, including conditions, ordering and failure propagation. Existing workflow protections and sibling 0398 routes remain. Both existing collection fixtures now include C and preserve their independent assertions; missing/malformed C assets refuse before output creation. All earlier Unix, command-only, archive-source and installer regression plants remain.

Focused checks completed offline under an owned 12 GiB memory/1 GiB swap scope, two Cargo jobs, lane lock and shared toolchain/cache mutation locks, with an explicit scratch environment. Policy passed. C Clippy denied warnings. The Unix C door passed 36 Rust tests, including the header parser and version contract, ownership, cancellation, usage, plan, golden bytes and static exports. The shared C corpus selected 55 cases: 53 passed and the retained two internal-injection exclusions were reported. Portable archive/header/export checks and 39 workflow plants passed. These synthetic files and saved tool output establish routing/refusals, not native execution. The archived-release test initially refused dirty source bytes as designed; it must rerun against the whole committed candidate.

Initial policy rejected an ignored binding sanitizer test; it is now a separate normal native door test. Initial Clippy rejected nested parser scanning and an unused moved import; both were corrected without lint suppression. No dependency lock, root product source, core boundary or root ratchet change is needed.

The C Rust ratchet and C fixture ratchet are measured exactly in the candidate. Growth buys the shared fail-closed declaration parser, the Windows source/ASan proof arm and local C portability branches. The Unix platform code was moved intact and the old permissive declaration reader removed. The header/thread operations and two fixture constructors reuse existing authorities rather than creating a second function inventory. No scaffolding is added to product interfaces. Final measured totals and immutable local receipt digests follow once required checks finish.

0380 C source and canonical receipts are complete and landed. Its native console/privacy execution and W1/W2 closure remain pending. COFF static localization, coexistence and metadata belong to slice B; stage 2, signing, publishing, provider calls and new dispatches are outside this build. The parent owns the new full checkpoint and fresh independent review.

## Frozen source and final receipts

Reviewed-ready source commit: `fd98fbbaaab7db4de3a892dedd7425251bac1478`. Its non-Markdown source patch against landed main `6d26206aa8861490cc6540b6bafafc08e3249c3c` has SHA-256 `ee217e1cf293758139be8239fd3dd34249cfdbb8f34cc1238ce26243d1843415`. The final record commit changes documentation only; the source patch fingerprint is retained. Immutable local copies of the source patch, per-file source digest inventory and logs are kept under the source commit's receipt names. All whole fixes are pushed to the ticket branch. No landing or rebase occurred.

| Receipt | Source | Exit | SHA-256 |
| --- | --- | --- | --- |
| `fd98fbbaa-c-final.log` | `fd98fbbaa` | 0 | `5ca649e5bf911ba33562ac7bf93a856a68e81e6b29fa9b08a0ecb85b8dcdf07d` |
| `fd98fbbaa-required5.log` | `fd98fbbaa` | 0 | `36f988e35d025d1824e89e7f1ce0f848d0902af1d75731923388e9d6367472be` |

The renewed focused receipt passes policy, C formatting and C Clippy with denied warnings, then all 36 Unix Rust door tests. Its shared C corpus selects all 55 cases: 53 pass, zero fail and the two existing internal-injection exclusions remain. The full selected door run takes 9.43 seconds on the warm lane. This is focused Linux C execution, not the full root test, spec or surfaces rung.

The required receipt passes the exact root and C ratchets, all 113 workflow self-test cases, the 45 new Windows C route plants, both migrated collection fixtures, the existing 0398 installation routes, and complete required lint. Root Clippy and documentation deny warnings; the API inventory checks 569 declared items and refuses its four plants. The registry's expected owned timeout negative prints `Killed` and the scope exits zero. The external private-name scan reports skipped because its variable is absent in the minimal environment; no external configuration, credential or shell initialization file was read.

| Measured scope | Landed baseline | Candidate | Growth |
| --- | --- | --- | --- |
| Root Rust ratchet | 114995 | 114995 | 0 |
| C Rust `src` and `tests` | 4953 | 5223 | 270 |
| C fixture/example `.c` files | 1245 | 1323 | 78 |
| C `build.rs`, outside that ratchet's existing scope | 12 | 33 | 21 |
| Shared C fixture `platform.h`, outside the `.c` ratchet | 0 | 39 | 39 |

Every Rust file stays below the 500-nonblank-line cap. The largest C Rust file remains the existing `tests/door/settings.rs` at 487. The new declaration reader has 121 lines, and the Windows door arm has 151. Growth protects header-derived complete exports, native setup/import identity, ownership and concurrency, and byte framing. The existing Unix platform code moved intact; the permissive declaration reader was replaced and shared import-library production and fixture construction remove parallel inventories. The root source, dependency manifests/locks, localization script and static Unix metadata remain unchanged.

A prior-failing portable regression pinned `0381_C_ZIP_HOST_METADATA_DIFFERS`: ZipInfo's default creator varied by host. Setting it explicitly makes identical supplied files produce identical ZIP bytes under simulated Unix and Windows hosts. Independent MSVC setup fixtures pin the exact cmd quote framing, x64 selection, 60-second bound, system/tool environment and output filter. These are routing evidence, never native Windows execution. The owned ASan negative now imports the public DLL through the existing null-engine error accessor, so it satisfies ordinary consumer inspection before its diagnosed use-after-free. Both its object and executable are removed after the proof.

All final checks used the lane lock, shared toolchain/cache mutation exclusion, a 12 GiB memory/1 GiB swap scope, two Cargo jobs and explicit scratch home/state/tool variables. Ordinary Cargo was offline. Only the existing narrow adapter allowed the validated owned sibling `file://` source-not-allowed negative; no remote gate acquisition was admitted. Every owned check scope is inactive. Earlier failure logs remain retained separately.

Fresh High ABI/platform source review and the parent's named full checkpoint remain required. Native MSVC compilation, complete real dumpbin/COFF/loader proof, downloaded-archive execution, sanitizer availability, runtime dependencies and macOS execution remain unrun here. Native dispatch authorization remains pending. No root full test/spec/surfaces, canonical site run, stress, live call, signing, publishing, dispatch, environment approval or rehearsal ran for this slice. 0380 C source/canonical receipts stay complete and landed, while its native W1/W2 evidence remains pending. Slice B owns COFF static localization/coexistence/metadata and any evidence-backed stage 3 debt.
