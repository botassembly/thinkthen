# 0748 — Dart packing checks its generated manifest against the C archive

Status: OPEN.

Milestone: 0.2.1

Phase: 2

## Outcome

- Changes: `sdlc/scripts/release-pack`, `sdlc/scripts/release-pack.d/dart`, `sdlc/scripts/release-archive-self-test.py`.
- `release-pack`'s archived-source check compares every copied Dart source file with the archived commit, except `native-assets.json`, which `release-pack.d/dart` generates from the paired C library. That generated manifest is checked instead: its sha256 for the target equals the sha256 of `lib/libthinkthen.so` in the paired C archive, as the Swift native library already is.
- Every existing assertion stays, including the self-test's check that the Dart manifest identifies the packed C bytes.

### Proof

- `pm job run --machine gator 0748 SHA --memory 7 -- sdlc/scripts/linux-check lint test` passes; it failed on main's source in Gator receipt 0744/1792529452380-1255765.
- A planted Dart source edit still fails the archived-source check, and a planted manifest checksum that differs from the C archive fails the new check.
- The configured landing gate passes through `pm job run --machine m5 0748 SHA --disk 20 --memory 8 -- sdlc/scripts/gate`.

## Evidence

- The attempted focused command `pm job run --machine gator 0748 cd18107da --memory 7 -- sdlc/scripts/job python3 sdlc/scripts/release-archive-self-test.py` failed before fixture execution in receipt 1792531904700-1471139 because the host lacked its pinned tools and could not provision `/opt/thinkthen-tools/rustup`. The required container proof above owns those tools and runs the archive fixture; the standalone attempt proves no product behavior.
- Gator 0744/1792529452380-1255765 on fa84bfa exited 1 after advisory checks passed: `release-archive-self-test.py` reports "release-pack: dart source differs from archived commit: native-assets.json". `release-pack.d/dart` rewrites the development manifest's sha256 from the paired C library, then `release-pack` compares that rewritten file with the original source archive, while the self-test expects the rewritten checksum. 0744 touches none of these paths, so main carries the defect; it first showed on x86 under rule 3 of `sdlc/decisions/2026-10-20-freeze-names-the-contract.md`.
- It does not hold 0744 or the freeze: per standing ruling 6 and the amended rule 3, a Gator failure the batch did not cause gets its own repair ticket. It gates 0687.

## Progress

- 2026-10-20 started
- 2026-10-20 next: setup: focused Gator job 1792531904700-1471139 exited 2 before product execution because the host standard job resolver could not provision its pinned tools; cd18107da is pushed, no unchanged retry or host workaround will run, and the required container lint/test and configured gate wait for accepted 0744 main integration.
- 2026-10-20 next: setup: Gator 1792532412154-1497053 passed the repaired manifest and planted guards before a PHP prerequisite blocked the later SDK refusal fixture, repaired with isolated version and FFI stubs at 9d5ff63b7 under the same unchanged assertions; the required Gator check is running on that source and the unadmitted M5 waiter was cancelled for freeze priority.
- 2026-10-20 next: Gator 1792533319143-1664346 passed lint and the full routine suite on 9d5ff63b7, including both Dart guard plants and the preserved packed-C checksum assertion; fresh source review is clear at source-equivalent 941842a06 and final paired proof waits for the freeze parents to avoid invalidating reuse.
- 2026-10-20 next: Stopped at Ian’s request with all work pushed at 89d03e420; both earlier checks failed the instruction size guard, the prose fix is committed, and final checks were cancelled before acceptance.
