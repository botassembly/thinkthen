# Correct the R counter case and Dart negative diagnostics

Date: 2026-09-29. Quick Fix in codex-4, claimed and pushed at `f1802619d` after importing the independent final verification report `ccf436c90`. Review pending. Source runtime APIs are unchanged.

## Cause and change

Experiment302's final `4c0ef210` package run independently proves the original five fixes hold. Two newly reported harness causes remained on current main. R case40 calls `tt_engine(batch="max")`, performs ordinary answer checks, then attempts to choose a different cache folder. The current session contract refuses any changed settings. Merely moving the cache argument earlier would make the counter phase start warm and invalidate its one-send/one-cache-answer oracle. The corrected case reuses `helper.R::child` for an isolated counter measurement. It passes the same question file and evidence, requires a successful child plus exactly one counters record, compares actual counters with the corpus's literal expectations, and includes the child's sends in the final backend count. No engine lifecycle behavior or corpus expectation changes.

Dart0279 changed `decide` to return a `CallResult`. The three consumers already assert its value correctly but printed the carrier object on failure. Their label now prints `answer.value.outcome` and `answer.value.probability`. The two planted-negative checkers retain their exact `Bad state: scalar yes Outcome.no/0.1` marker. Product `toString`, failure classes and result parsing are unchanged.

## Focused evidence

Root source at `f1802619d` has identical R/core product inputs to reviewed0301 `8286b99d7`. The retained source R install from that review was reused read-only in codex-7. Its `thinkthen.so` SHA-256 is `80087dc9e6ead9fd09d542cce4616518d80b119428d1053fdb474e951254f2ba`; its matching conformance backend is `b6688e0ec140592f330c37b3573e1aed774ba667622cfc4e5672d896f0795c85`. The selected case ran through `libraries/r/tests/with-backend.sh` under `env -i` with explicit PATH, HOME, UTF-8 locale, R library paths and an absolute one-ID selector. Before the edit it failed with the reported session-settings sentence. Afterwards: selected1, pass1, fail0, not_run0; the backend observed exactly two requests, one for the ordinary answer checks and one for the isolated two-call counter check. This is a real installed source package and a loopback fixture, not a mocked counter or a full R package gate.

The current local C debug library was rebuilt with `cargo build --locked --offline --manifest-path libraries/c/Cargo.toml`, explicit wrapper overrides and four jobs. The incremental compile completed in5.80s. Its SHA-256 is `bd2656295fc10bb80147189f88284253e43b225b5ffa30ac245bc782c27a4bd3`. Only this built file was copied to the Dart runner's existing scratch-library path. Dart3.13.4 and Flutter3.47.5 used cached dependencies with `pub get --offline --enforce-lockfile`; the package, two Dart consumers and Flutter example kept their shipped lockfiles. This did not qualify the distinct Flutter wrapper lock mentioned in the verifier's report.

Both Dart consumers and the Flutter test consumer ran their existing `run.py` fixtures under an explicit minimal environment, with loopback addresses and synthetic keys. Alpha and bravo's wrong-probability cases each exited255 after one observed request; Flutter exited1 after two. All three showed the exact expected marker and gate failure. The corresponding positive runs exited0 with exact body multisets and16,16,17 observed requests. The Flutter test consumer ran; its Linux GUI embedder did not need rebuilding for these three diagnostic strings. Raw receipts remain under the claimed worktree's ignored `target/codex-builds/package-harness`, `libraries/dart/checks/logs` and `libraries/dart/flutter/logs`.

Dart formatting passed with zero changes. R's measured source counter rises1863→1869; Dart stays2950. The six R lines of net growth reuse the existing child/cache isolation rather than adding a second engine or backend harness. `children` reports zero findings; pages, tickets and diff checks pass. No new test, broad suite, stress campaign, provider call or full24 qualification was added or run. Existing counter and negative fixtures now prove their intended behavior; none was removed.

## What the build taught us

- An immutable-session test must isolate phases that require distinct cache state. Moving configuration alone can turn a red setup into a false cached result.
- An envelope migration changes diagnostic interpolation as well as successful-value access. Read failure labels and exact negative-check markers together when preparing the next host family.
- A clean environment must name its locale and build overrides; a minimal variable list does not remove Cargo's ancestor-directory configuration.
- The final verifier's Dart report also found silent offline lock rewriting. Preserve that separate qualification gap. Enforcing the example's lock here does not prove the wrapper lock or all release dependencies.
- These focused receipts repair three reported failing surfaces. Historical14/10 counts remain pinned to their source revision. Whole-package and final release qualification remain separate.
