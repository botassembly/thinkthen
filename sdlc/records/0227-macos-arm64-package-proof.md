# 0227 native macOS ARM64 Ruby package proof

Status: accepted after fresh independent High package review of `bfc7b48bf2c4e61cdf0eccc4e20be730e3bf749d`. This is an installed-package supplement to the accepted [0227 source and Linux proof](0227-language-panic-build.md), not a new caught-panic source test or a blanket platform closure.

The native Apple Silicon M5 ran macOS 26.4. An isolated clean worktree at source `ca6e6625244254c429d503bad457d2440042ab91` used pinned Ruby 3.4.11 (`arm64-darwin25`), official Rust 1.95.0 `aarch64-apple-darwin`, the Xcode Command Line Tools libclang and macOS deployment target 15.0. The build used existing `libraries/ruby/build.sh` with a separate `/tmp/thinkthen-0227-ruby-m5-target`; remote `BUILD_STATUS=0`. A fresh isolated `GEM_HOME=/tmp/thinkthen-0227-ruby-m5-gems` received `gem install --local --ignore-dependencies --no-document`; remote `INSTALL_STATUS=0`. The source worktree has no tracked edits. Logs and outputs stay on M5 under `/tmp/thinkthen-0227-ruby-m5-*` and the isolated worktree.

| Artifact | SHA-256 |
| --- | --- |
| `libraries/ruby/thinkthen-0.0.1-arm64-darwin-25.gem` | `5a7854c9d57e810339d05e31d97a2e464b1c1878c1e36f0fefe5f2bf067fd97c` |
| Built and installed `thinkthen.bundle` | `ff22714c4916a162bd23d53c8280526b9c7e6654e8af22c15d29f0fb663177aa` |

The installed bundle is Mach-O arm64 with `LC_ID_DYLIB @rpath/thinkthen.bundle`, deployment minimum 15.0, and links only system `libiconv` and `libSystem` beyond its own install name. `nm` shows local `std::panicking::HOOK`. Raw bundle bytes contain neither the builder's home path nor the synthetic panic marker. These inspections describe the built artifact; they do not prove execution on macOS 15.

The focused installed invocation used pinned Ruby from the fresh gem home, unset `THINKTHEN_API_KEY`, `RUBYLIB` and `RUBYOPT`, disabled the cache, and used unused loopback port 9. `ThinkThen::Engine.new(...).decide("Is it urgent?", "text", deadline: 0)` returned `ThinkThen::DeadlineError` with kind `deadline`, `retryable=false` and final `requests_sent=0`. Two later `engine.usage` reads agreed and still reported zero sends. `$LOADED_FEATURES` resolved the native bundle inside the installed gem, rather than the checkout. Remote `REMOTE_STATUS=0`; the concise result is in `/tmp/thinkthen-0227-ruby-m5-smoke.log`.

The first smoke script exited 1 only because it compared the `/private/tmp` loaded path to a literal `/tmp` gem home. Its Deadline and usage assertions had passed. The corrected script canonicalized both paths with `File.realpath` and exited 0. This is a harness path-alias correction, not a product change. SSH's local exit status has previously failed to carry M5 remote failures; the recorded statuses above were emitted inside the remote command.

The accepted 0227 Ruby source child supplied the synthetic panic, marker secrecy, prior-hook and later-success evidence. This installed invocation exercises ordinary no-send error handling and package linkage, not a real caught panic in the shipped gem. The M5 ran macOS 26.4; actual macOS 15 execution and native Intel remain unproved. The artifact differs from 0227's earlier Linux gem and requires fresh independent review before acceptance.

## Independent review

The fresh reviewer verified the M5 source and toolchains, both artifact hashes, Mach-O architecture and deployment minimum, system linkage, local panic-hook symbol, actual installed load path and explicit smoke status. Ruby and engine source did not change between the built source and the reviewed candidate. ACCEPT covers the stated installed no-fault proof only. A caught panic in the shipped gem, macOS 15 execution and native Intel hardware remain unproved.
