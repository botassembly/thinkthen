# 0640: Test the installed blocking C call and its owned results

Status: OPEN.

Milestone: 0.2.1

Phase: 2

Depends on: 0612, 0618, 0638, 0642

Reviews: revision fd5b452, accept

## Outcome

- Changes: `libraries/c/check.sh`, `libraries/c/examples/session.c`, `libraries/c/README.md`, `libraries/c/tests/`, `conformance/children/packet_normalize.py`, `conformance/test_parity.py`.
- `examples/session.c` keeps calling `thinkthen_run_call` and prints the call's answer and probability read from the result, not the whole result JSON.
- The installed `thinkthen_run_call` consumer proves its owned Call JSON and owned error fields against the shared cases: answer, probability, complete results, facts, diagnostics, ownership and zero sends. Add no C packet dumper. Native packet envelopes, stopped positions and the deterministic cancellation checkpoint remain Rust proof.
- Delete tests only after the installed blocking-call proof replaces their behavior. Keep independent ABI ownership, threading, secrecy, held-provider cancellation and settlement assertions while those APIs ship.
- The C gate has no exit-77 path except platforms the `sdlc/tools.json` manifest marks unsupported.

### Proof

- On the M5 natively: `pm job run --machine m5 0640 SHA --memory 8 -- sh -c 'out=$(mktemp -d) && sh sdlc/scripts/release-pack aarch64-apple-darwin "$out" c && THINKTHEN_TEST_PROFILE=full THINKTHEN_ARTIFACT="$(ls "$out"/thinkthen-c-*.tar.gz)" sh libraries/c/check.sh'`. The installed check unpacks the archive, compiles the example and blocking consumer from outside the repository, and compares the observable owned result and error fields with the shared cases.
- Done once by hand and not committed: changing one expected result probability in `conformance/fixtures/packets.json` makes the same installed check fail with a field difference from the shared comparator. Restore the bytes afterward.
- Linux runs at the 0619 container checkpoint.

## Amendment, 2026-10-20 (C boundary proof)

The PM's second ruling in `repos/agents/inbox/thinkthen/2026-10-20-thinkthen-0640-needs-a-c-boundary-proof-ruling.md` supersedes the first seven-case packet amendment: add no C packet dumper. Prove the surviving installed blocking-call result and owned errors. The owned C error exposes code, message, retryability and optional native Facts JSON. A refusal before engine construction finishes remains explicit constructor-stage proof through the shipped diagnostic API, with no engine or call owners and zero sends. Do not infer native stopped positions, completed prefixes or other unexposed error fields. Preserve the existing malformed-request mapping and permit absent facts before invocation starts. Keep all shared C cases; prove their observable fields and request counts. Rust owns exact native envelopes and stop positions. The coordinator files the required pre-freeze error-position ticket for 0688; this ticket adds no API repair. Change no public API, diagnostic, shared oracle or deadline; the fifth PM ruling permits only the shared canonical helper change below.

## Evidence

- `libraries/c/check.sh` runs `tests/test_installed_consumers.py`, then with `THINKTHEN_ARTIFACT` set runs `tests/installed-session.py` on the archive and stops. Without it, it runs `generate-c-header.py --check`, `check-c-exports.py`, the crate's `cargo test` (the Rust door tests in `libraries/c/tests/door/*.rs`, about 1,400 lines), and compiles `examples/session.c` against the debug build.
- `tests/installed-session.py` and `tests/installed_cases.py` run the JSON session and plan cases against an extracted archive with the loopback `Backend`. `tests/c/` holds C programs such as `session.c`, `session_cases.c`, `run_call.c` and `error_views.c`.
- 0638 landed `thinkthen_run_call` and the `thinkthen_call_result` and `thinkthen_call_error` owners, and `examples/session.c` already calls `thinkthen_run_call`; it prints `thinkthen_call_result_json` whole.
- `sdlc/scripts/release-pack` `part_c` packs `thinkthen-c-VERSION-TARGET.tar.gz` with the header, both libraries and a `.pc` file.
- The Darwin linking defect in the source check, where `check.sh` copies `libthinkthen_c.so`, belongs to 0641, which also claims `libraries/c/check.sh`.

## Progress

- 2026-10-20 started
- 2026-10-20 next: Installed blocking-call checks are pushed at 9f429edf for all 254 applicable shared cases; native archive proof and the unchanged-oracle mutation check await a free M5 slot, and legacy deletions wait for replacement proof.
- 2026-10-20 next: M5 proof 1792484483598-40111 passed all 254 owned Call cases plus retained ABI checks, and the once-only oracle probability mutation failed with a diff and exact byte restoration; fresh fixing review now audits complete-result and facts coverage before the integrated closing gate.

### Fifth PM ruling: exact owned result and facts comparison

The fifth answer in the same PM mail permits one explicit expected-oracle mode in the existing shared canonical helper. It reindexes saved identity aliases only along the helper's existing schema paths; default native mode stays unchanged. Compare complete projected owned results and facts using that helper and the existing decimal JSON difference, retaining cross-step identity relationships and required error facts presence. Add exactly four regression classes in the existing conformance test owner: metadata and facts mismatches, required error facts presence, cross-step identity consistency, and opaque alias-shaped user values remaining unchanged. Add no private normalizer, fabricated packet fields or new comparison format. Any further comparator change needs another PM ruling.
