# 0224 DuckDB find on Apple Silicon: installed package proof

Status: accepted at `37b57014` after fresh independent High artifact review; see [review](0224-macos-arm64-find-code-review.md). This is native Apple Silicon execution of the accepted Linux find source, not a new runtime or package-script change and not completion of ticket 0224. Linux ARM64 and Intel macOS packages, actual macOS 15 execution, and the wider release rehearsal remain open.

## Exact source and host

The M5's dedicated `$WORKSPACE/worktrees/thinkthen-codex-m5-duckdb` checkout fetched and detached at pushed main `c2db47e9db41c064fe13211c4907631cde967f34`, which contains independently High-accepted DuckDB find candidate `fb9009aac575063cb885f157b2d636c8fcabcfea`. Its only worktree change before and after the build was the known unrelated deletion of `site/examples/beatles/BENCH`; it was neither restored nor used as a source input. The previous 0222 extension was copied to the isolated 0224 output before `cpp/build.sh` replaced its default built-file path; its preserved SHA-256 is `332458264b319c4a44d4a69436cefb31e5068d4e70207b488645ddba860d6073`.

This host is `Darwin arm64`, macOS 26.4 build `25E246`. Before the build, load was 0.71 over 18 logical CPUs, memory pressure reported 95% free, and 447 GiB of disk was free. No toolchain install or shared cache update ran. The pinned DuckDB v1.5.5 C++ source was `d8cdaa33fda8df955cc76ef58a280f68f4cd43fa`. The stock v1.5.5 CLI SHA-256 was `d0610710dd30667aa6c76709299b6822e55dc9199803350aa2e1b06e3346943b`; the genuine stock v1.5.4 CLI was `6c5abaff49f07ba3f6b2e41ed1adf338d10fcb2d98777331b285cc97938fb00a`. The official cached Rust 1.95.0 `rustc` and `cargo` SHA-256 values were `b829b733131d4e1673eeebd1f34d06ae1e9ff4977b051313cf42e2a9e79ecf1c` and `c512bff73c86143b557463f021d0c3d5b0490d97d65040ba59ea2b3427784758`; the official `libstd` objects report `minos 11.0`.

## Build and package

With the official binaries prefixed on `PATH`, `CARGO_NET_OFFLINE=true`, a private `CARGO_TARGET_DIR` and `THINKTHEN_DUCKDB_CPP_BUILD` under `databases/duckdb/build/0224-macos-find`, the lane-specific Python `fcntl` lock `/tmp/thinkthen-codex-3.lock` guarded `sh databases/duckdb/cpp/build.sh`. It exited 0. Under the same lock and toolchain, `sh sdlc/scripts/release-pack --reuse aarch64-apple-darwin databases/duckdb/build/0224-macos-find/package duckdb` exited 0. Logs remain in that isolated output as `build.log` and `package.log`. Source was not changed between build and package.

| Exact output | SHA-256 |
| --- | --- |
| Rust bridge static archive | `a2fe70b2390c34cdf4612fddcceca8bec610efb19c21d850fec309a1b9e41d62` |
| `thinkthen-duckdb-0.0.1-aarch64-apple-darwin.tar.gz` | `7b986eb8accc6db3b545153717a5e94f327eda6e30731253f746c415481be11a` |
| Freshly extracted `thinkthen.duckdb_extension` | `26f79addfae6045161cb498b1bb97a8479c804b3f44e78bbf1769d587ed3ada0` |
| Built `build/thinkthen.duckdb_extension` | `26f79addfae6045161cb498b1bb97a8479c804b3f44e78bbf1769d587ed3ada0` |
| Source-matched conformance backend, rebuilt in isolated output | `e40ea9113654b3c1b732bf24a1ee7786879e5f3b63667a5636c484faafaf1940` |

The extracted file is a single arm64 Mach-O, 54,511,686 bytes, with `LC_ID_DYLIB @rpath/thinkthen.duckdb_extension` and `LC_BUILD_VERSION minos 15.0` (SDK 26.5). Its exact 534-byte DuckDB footer has the required signature and `CPP`, `osx_arm64`, ABI `4` fields. A raw byte scan found zero occurrences of the builder's home path. The build's official standard-library floor check passed. These deployment fields and inputs do not prove execution on macOS 15: this machine ran macOS 26.4.

## Bounded installed proof

All following checks loaded the **extracted archive file**, not the CMake output. `cpp/verify_package.py` exited 0: stock v1.5.5 loaded it, the same host refused a changed-version footer, and the genuine stock v1.5.4 host refused the unchanged file. The pinned stock v1.5.5 CLI also loaded it and reported the accepted typed find struct for a NULL call. `tools/find_suite.py` passed its selected three cases: original duplicate index, first-real and none ties, top-level NULL and empty list, NULL child and invalid inputs with zero sends, complete typed result, held cancellation, and spent statement budget with zero sends.

The first conformance attempt used a retained pre-capture backend binary and reported **0/2**: both cases had zero captured bodies. It was not counted as proof. Rebuilding only `conformance-backend` from `c2db47e9` into the isolated target completed successfully. With that binary, the installed selected conformance cases `18-find-second` and `19-find-none` passed **2/2**. Each counted exactly one listener request, compared the complete captured body against independent corpus bytes, and checked the digest from the actual served URL. The full bodies captured on M5 were:

```json
{"state":"[{\"id\":\"u001\",\"evidence\":\"First passage.\"},{\"id\":\"u002\",\"evidence\":\"Second passage.\"},{\"id\":\"u003\",\"evidence\":\"Third passage.\"}]","model":"jev-1.13.0","questions":{"q1":{"type":"choice","instructions":"Which passage answers the question?","criteria":{"u001":null,"u002":null,"u003":null,"none":null}}}}
{"state":"[{\"id\":\"u001\",\"evidence\":\"First passage.\"},{\"id\":\"u002\",\"evidence\":\"Second passage.\"}]","model":"jev-1.13.0","questions":{"q1":{"type":"choice","instructions":"Which passage answers the question?","criteria":{"u001":null,"u002":null,"none":null}}}}
```

Case 18's captured body SHA-256 was `6f2db9a5d733178470a4f58ebc2495dd66ebe2609c6c1bec8f534e9e8a6a77fe`. At `http://127.0.0.1:60156/case/18-find-second/capture/v1/systemone`, its actual recording digest was `fc8e9b5eaa4464d4396422147267be161ffa9097f4bfdeaccaebdc495098e760`; the literal result selected original index 1, value `Second passage.`, probability 0.8, with ordered candidate probabilities 0.1, 0.8, 0.05 and 0.05 for none. Case 19's body SHA-256 was `d2e0ca3f5223f468512160e22784684fbc812306fa948b17dfc11d9098970a20`. At `http://127.0.0.1:60158/case/19-find-none/capture/v1/systemone`, its digest was `90ac043b58c58aa1b8b8838a920463ecd0093224898175b24ac41efb29d6dc1f`; the literal result selected none with null index/value and probability 0.7, after original-index candidates 0.1 and 0.2. Both body hashes match the independently captured Linux find proof; the port-specific digest changes with the actual loopback URL.

## What the build taught us

The retained M5 extension and backend were from older source. Packaging the newly built extension succeeded, but the stale backend made capture-based conformance fail until its isolated binary was rebuilt from the exact pushed revision. Keeping the first 0/2 result and the successful 2/2 rerun distinguishes a test-input mismatch from a native find failure. The prior M5 package remains preserved. No product test, runtime, checker or package script was edited for this proof.
