# 0677: Package PHP for every supported platform without fixed paths

Status: OPEN.

Milestone: 0.2.1

Phase: 3

Depends on: 0615, 0618, 0619, 0678, 0688

## Outcome

- The package carries the native library for each supported platform from the shared inventory and picks the right one at runtime.
- The hard-coded `.so` name is deleted. The PHP path is 0615's.
- The archive is built by the shared packaging step, in a form a Composer path or VCS repository accepts.
- The PHP conformance runner becomes a packet dumper following the 0642 contract, and `fixtures/session_installed.py` and the other fixture consumers are deleted.

### Proof

- On the M5 through pm job, natively and in the pinned arm64 container, a clean Composer project installs the candidate archive and runs a named call with no library path.
- Linux x86_64 is proven in the pinned container on Yellow through pm job.

## Evidence

- **Fixed library path.** The default native path is `native/libthinkthen.so` (`libraries/php/src/client.php:15`).
- **Fixed PHP path.** `/usr/bin/php8.3` is hard-coded in `check.sh:9` and `fixtures/session_installed.py`.
- **One platform, silent skips.** `check.sh:7,8,13` exit 77 on anything except Linux x86_64.
- **Hand-made archive.** `fixtures/package.py` builds a tar file by hand, which Composer installs from an inline package repository. There is no multi-platform story.
