---
flow: build
priority: 47
opens: crates specification probes sdlc/issues sdlc/planning sdlc/ratchet.json
---

# 0038: Accept System One rounding at one hundredth

Status: landed

## Outcome

System One distributions whose decimal total is one hundredth above or below one are accepted. Binary parsing and addition cannot turn that decimal boundary into a refusal. Totals of `0.98`, `1.02`, `0.85`, and `1.15` remain refused. Every reported member stays unchanged, and another adapter keeps the generic strict rule unless its own evidence supports another tolerance.

## Evidence

Authorized live checks first saw five probability-total refusals among 58 requests. Three occurred under a temporary binary that compared the binary distance directly with `0.01`. That comparison was incomplete: `abs(0.99 - 1.0)` is `0.010000000000000009`, so a decimal total of `0.99` can still fail it.

Phase one added a safe diagnostic and ran the ticket's single capped capture job with retries disabled. The ledger moved from 18,440,118 to 18,524,118 charged tokens. Of 50 independent 17-option requests, 40 answered and 10 were refused. All ten refused totals were `0.9900000000000001`; all had the strict tolerance `3.774758283725532e-15`. No response body, member, label, evidence, or credential was retained. The result supports a System One rounding allowance of `0.01 + member count × f64::EPSILON`. It does not support a wider allowance.

## Contract

1. The generic `Distribution` keeps the invariant that members total one within a tolerance supplied by its caller. Its strict tolerance remains `member count × f64::EPSILON` for adapters with no rounding evidence.
2. The System One adapter supplies `0.01 + member count × f64::EPSILON`. The one-hundredth term covers its observed decimal rounding. The epsilon term covers binary parsing and addition at the inclusive boundary.
3. The adapter accepts totals of `0.99` and `1.01`, including sums whose binary representation falls just beyond one hundredth. It refuses `0.98`, `1.02`, `0.85`, `1.15`, zero, and three ones.
4. Validation preserves every probability the backend reported. It never renormalizes the members shown under `--details`. Score continues to divide its weighted sum by the accepted measured total, so its value remains inside the named scale. Choice order and cuts continue to use the reported members.
5. A refused distribution reports the computed total, member count, and active tolerance in Rust's shortest round-trip decimal text. It repeats no evidence, label, member probability, response body, key, address, or model.
6. Amend decided and implemented ADR 0019 in place. Preserve its former strict decision and record the later live evidence and the System One exception. Update `backends.md`, the issue, both plans, and the probe record. The main plan records Rust, Python, JavaScript/TypeScript, Ruby, R, and C libraries followed by the three accepted database extensions. It puts cache locking next, then `find`, then page 16 and transforms.

## Acceptance

- Generic core tests pin the strict default and exact boundaries. System One adapter tests pin both sides of the one-hundredth boundary, binary edge spellings, and the refused totals above.
- Existing missing/extra-label, probability-range, member-preservation, score-range, request, digest, and replay tests stay green. Every committed recording replays.
- Diagnostic tests pin the complete sentence above and below one. Compiled secrecy proof excludes labels, member values, response text, evidence, and keys.
- `probes/probability-total-0038/README.md` and its safe JSON result record the one command, ledger states, 50 attempts, 40 answers, 10 refusals, and one distinct total. The probe self-test remains in the test rung and no second paid run occurs.
- The four repository rungs and `git diff --check` pass. Commit, push, and the remote check belong to the coordinator's landing step.

## Excluded and following order

Excluded: rewriting members, a wider tolerance, retrying refused replies, recording refused bodies, cache locking, and changes to the paid-call door. This ticket completes the correction pass. Next is one bounded per-digest cache lock in engine-ready modules, then `find`, then page 16 and transforms.

## Complexity

- Contract: 2
- State and timing: 0
- Reach: 1
- Proof: 2
- Cost of error: 1
- Total: 6
- Minimum level floor: none
- Final level: 3
- Reasons: the ticket changes a public validity boundary shared by choice and score. Exact decimal and binary boundaries, adapter ownership, secrecy, replay, and score-range proof distinguish a live compatibility repair from silent acceptance of malformed answers.
- Selected model: `gpt-5.6-sol` with medium reasoning.

## Review

- Phase-one design review: accepted after one revision. It required a conditional measurement branch, the five-refusal evidence, deferred layer ownership, one fixed 50-attempt job at 84,000 tokens, and round-trip numeric text.
- Phase-one implementation: complete. The first core test failed because the error retained no measurement. The implementation added total, count, and tolerance; a safe 50-call probe; and free self-tests. The paid job then produced the evidence above.
- Phase-one code review: accepted after one remediation. The first review caught that default retries could turn 50 iterations into 150 requests. Remediation passed `--max-retries 0` and made the fake binary require it.
- Phase-two design review: accepted after one revision. The first review accepted the technical rule and required an in-place amendment to the decided and implemented ADR 0019 instead of a whole rewrite.
- Phase-two implementation: complete locally. The first adapter boundary test failed on total `0.99` under the strict generic tolerance. System One now supplies the measured decimal allowance plus binary summation tolerance; the generic constructor stays strict. Boundary, refusal, member-preservation, score-normalization, diagnostic, and secrecy tests pass. The free probe check, lint, test, spec, and diff check pass.
- Phase-two code review: accepted after one remediation. The first review accepted the code and found that the main plan omitted the 50 live calls. The plan now records the calls, states that actual usage was not retained, excludes it from the measured sum, and separately names the 84,000-token ledger charge. The same reviewer accepted the final diff.
