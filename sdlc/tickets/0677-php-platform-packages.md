# 0677: Package PHP for every supported platform without fixed paths

Status: OPEN.

Milestone: 0.2.1

Phase: 3

Depends on: 0615, 0618, 0619, 0688, 0728

## Outcome

- Changes: `libraries/php/src/session/client.php`, `libraries/php/composer.json`, `libraries/php/check.sh`, `libraries/php/fixtures/`, `libraries/php/README.md`, `sdlc/tools/php.json`, `sdlc/scripts/release-pack.d/php`.
- The package carries the native library for each platform the `php` entry under `libraries` in `sdlc/tools.json` lists, and `Client` picks the right one at runtime.
- The hard-coded `native/libthinkthen.so` default in `client.php` and the `/usr/bin/php8.3` paths in `fixtures/session_installed.py` are deleted. PHP comes from the manifest on every platform it checks; `sdlc/tools.json` declares PHP for macOS and adds `darwin-arm64` to `check_platforms`.
- `release-pack` builds the archive in a form a Composer path or VCS repository accepts. `fixtures/package.py` is deleted.

### Proof

- On the M5 through `pm job run --machine m5 0677 SHA --memory 8 -- sh libraries/php/check.sh`, a clean Composer project installs the candidate archive and runs a named call with no library path. Before the change the check exits 77 on the M5 because only `linux-x86_64` is declared.
- Linux arm64 and x86_64 run at the 0619 container checkpoint.

## Evidence

- Order (coordinator, 2026-10-19): the packaging ticket 0677 lands before the call ticket 0678, so 0678 can prove on the M5. The packet dumper belongs to 0678, written once against the run call.
- **Fixed library path.** `Client::__construct` in `libraries/php/src/session/client.php` defaults the native path to `native/libthinkthen.so`.
- **Fixed PHP path.** `libraries/php/fixtures/session_installed.py` runs `/usr/bin/php8.3` directly. `check.sh` already takes PHP from the manifest after 0615.
- **One platform.** `sdlc/tools.json` lists four PHP platforms but `check_platforms` holds only `linux-x86_64`, and the `php` tool is declared for Linux only.
- **Hand-made archive.** `fixtures/package.py` builds a tar file by hand, which Composer installs from an inline package repository. `release-pack` copies the source wrapper without a native library.
