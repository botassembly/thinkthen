# 0642: Compare every library against one core packet dump

Status: COMPLETE.

Milestone: 0.2.1

Phase: 2

Depends on: 0612, 0636

Reviews: revision 90d52a7, accept

Reviews: revision 38e6465, accept

## Outcome

- Changes: `conformance/fixtures/packets.json`, `crates/thinkthen/tests/packet_oracle.rs`, `conformance/children/packet_dump.py`, `libraries/rust/consumer/`, `libraries/rust/check.py`, `sdlc/scripts/lint`.
- The core writes one canonical, deterministic dump of each shared case's packets: rows, aggregates, terminal, answer, probability, pick, resolution, presence and facts. `conformance/fixtures/packets.json` is that dump and the one packet oracle. A Rust test, `crates/thinkthen/tests/packet_oracle.rs`, regenerates it on request and fails when it is not current; `lint` runs that check.
- A library's dumper reads each case's request, calls the library's public API, and prints the packet JSON that Rust returned, plus only the values, presence states and errors the host itself converted. It does not re-serialize every public type in its own code, so a new core field touches no language dumper.
- `conformance/children/packet_dump.py` compares parsed dumper JSON against the oracle, with numbers kept as decimal strings, not byte-for-byte text.
- The Rust library consumer becomes the first dumper and stops using `conformance/children/native_fixture.py`. Other languages keep `native_fixture.py` until their own phase 3 ticket adopts the dumper and deletes their old fixture consumer; the C dumper is 0640's. 0634 documents the dumper contract in `libraries/BINDING-AUTHOR.md`.

### Proof

- On the M5 natively: `pm job run --machine m5 0642 SHA --memory 8 -- sh -c 'cargo package --locked --offline --no-verify -p thinkthen && THINKTHEN_ARTIFACT="$(ls target/package/thinkthen-*.crate)" sh libraries/rust/check.sh'`. The installed Rust consumer's dump matches the oracle for every shared case. Before the change the consumer runs `native_fixture.py`, and no oracle file exists.
- On the M5 through pm job, `sdlc/scripts/lint` runs the currency check and passes on the branch.
- Done once by hand and not committed: changing one expected value in `packets.json` fails the Rust comparison with a diff, and changing a core packet field without regenerating fails the currency check.

## Amendment, 2026-10-19

PM decision, 2026-10-19: the oracle is a new file, `conformance/fixtures/packets.json`. `conformance/fixtures/complete.json` holds private result carrier fixtures with a `results` list and `native_presentation` questions, and three readers depend on that shape: the C# decoder run in `libraries/csharp/check.sh`, the Objective-C `Results` run in `libraries/objective-c/checks/foundation_installed.py`, and `transforms/sweep/human-label-test.sh`. Rewriting it in place would break those gates in phase 2. Their own tickets retire their use of it. The `BINDING-AUTHOR.md` bullet moved to 0634, which already claims that file and covers the dumper contract.

## Evidence

- Each language keeps a large fixture consumer that projects results its own way for `conformance/children/native_fixture.py` (389 lines), whose `run` builds requests with `request` and `native_request`, projects packets with `project`, and checks them with `assert_required`:
  - Go: `libraries/go/fixtures/owned_consumer.go` and `full_consumer.go`.
  - C++: `libraries/cpp/fixtures/owned/main.cpp` and `owned_calls.py`.
  - Zig: `libraries/zig/Tests/request_fixture.zig`.
  - Ada: `libraries/ada/checks/full_case.adb`.
  - COBOL: `libraries/cobol/checks/blocking.py`.
- `libraries/rust/check.py` `main` builds `libraries/rust/consumer` and calls `native_fixture.run('rust', ...)`; Python, TypeScript, R and Polars call the same `run`.
- 0612 put the settled answer, probability, pick and resolution in the final packet; 0636 (running) puts expected settled answers in `conformance/cases.json`.
- Fixture changes fan out to every language. Since 2026-09-15, 97 commit subjects across the nine scoped libraries mention fixtures. Examples: 6ee099210 changed 54 files, and 9f04b5c7a and 1a4b68b02 touched C, C++, Zig, Ada and COBOL.

## Progress

- 2026-10-19 started
- 2026-10-19 next: Fresh source1a84df8a7 awaits M5 proof and a PM ruling on shared case25-defect-fault: its private internal_invariant_failure operation has no admitted public input, so the proposed public oracle explicitly names that exclusion while retaining private defect coverage, or the PM can scope a separate internal injection test; every admitted public case proceeds.
- 2026-10-19 next: Full required-case inventory confirms private25-defect-fault was already outside Rust parity, so retaining that explicit boundary needs no new ruling; current sourcee5be1cf0a preserves all258 required cells through shared orchestration and awaits the next M5 slot for oracle generation, package comparison, lint and planted stale-output negatives.
- 2026-10-19 next: M5 job1792462197161-60612 passed generation and independent comparison of255 Rust-applicable cells from258 global cells, with only3 existing MCP-only exclusions, plus authored-value preservation and lossless fixture-storage probes; pushed498811adf needs final currency,full installed Rust,lint,test and the two planted stale-output checks under fresh review.
- 2026-10-19 next: M5 job1792462908099-82036 atdfb691f60 passed policy,backend,package,full255 installed Rust cases and routine tests, then exited2 before lint because native tool resolution selected Apple gcc instead of the Linux-pinned GCC; source remains unchanged, and the next slot uses existing Linux lint plus currency and both planted negatives before fresh acceptance.
- 2026-10-20 next: Fresh review accepts source 90d52a718: job 1792468804508-31699 passed Linux lint, both intended packet-diff negatives and restored native currency; prior job 1792467656278-89184 passed native/Linux currency and all 255 installed Rust cases at 4020fc933 before formatting/helper-only edits, preserving all three existing MCP-only exclusions, and the accepted lane is ready for the shared landing gate.
- 2026-10-20 next: Accepted standalone proof passed, but the combined landing gate at d8fcb38c859557a4266360d51385cd691478ace5 exited 101 on packet-currency differences after main gained 0639 core changes; the fixing reviewer is merging current main and classifying all refusal-facts diffs before any oracle regeneration, and no 0642 source has landed.
- 2026-10-20 next: Fresh fixing review accepts 38e646594: the integrated failure was a private cancellation scheduling race, fixed with a bounded public checkpoint; only case 23 and its alias changed, corrected exact-two inspection passed, and regeneration 1792472654599-36614 plus final native/Linux currency, installed 255-case comparison and full Linux lint in 1792472922745-43097 all exited 0 before another landing attempt.
