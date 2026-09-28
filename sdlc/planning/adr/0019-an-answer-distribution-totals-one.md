# ADR 0019: An answer distribution totals one

- Status: Decided by the agent on 2026-09-20. Ian can overturn any line
- Date: 2026-09-20

`choose` and `score` treat the backend's numbers as one probability distribution. The decoder checks each number and checks that every requested label appears. It does not check the total and ignores labels that were never requested. A reply of `[1, 1, 1]` therefore becomes a score of 3 on a three-level scale whose largest valid position is 2.

## Decision

- A distribution carries exactly one probability for every option or level sent, and no probability for another label.
- Every member remains inside zero through one.
- The members total one within `member count × f64::EPSILON`. Parsing and adding at most 255 binary floating-point probabilities can introduce error at that scale. The largest tolerance is about `5.7e-14`; it admits representation noise and no percent-scale missing or excess mass.
- The tool validates and preserves the reported members. It never rewrites or renormalizes the distribution it reports. A score divides its weighted sum by the accepted measured total, then applies its existing rounding. That keeps the score inside the level scale when the total differs from one only by admitted representation noise.
- A refused distribution names the question and the rule. It never repeats reply values.
- A yes/no answer carries one probability rather than a distribution, so this rule does not apply to it.

ADR 0057 makes every relation question yes/no. A relation no longer asks a choice, so this total-one rule does not limit how many relation edges reach the cut.

## Evidence and consequences

The repository holds 493 distributions in recordings plus four standalone fixtures. Their largest measured distance from one is `1.1102230246251565e-16`, so every saved reply remains valid. At the largest choice list of 255 members, the admitted total error is at most about `5.7e-14`. A score has at most ten levels, so its tolerance is at most about `2.3e-15`; at position 9, an uncorrected total-mass drift is at most about `2.1e-14`. Dividing by the measured total removes that drift before the existing twelve-decimal rounding.

The generic answer model owns the total-one invariant. An adapter owns the exact match between its wire keys and the labels it sent. A later adapter therefore cannot construct an invalid distribution, while each wire format remains responsible for its own names.

## Amendment on 2026-09-20: System One decimal rounding

The strict generic decision above remains the default. Later live evidence showed System One returning 17-member distributions whose computed total was `0.9900000000000001`. A capped 50-request capture saw that same total in all ten refusals; 40 requests answered. The ledger moved from 18,440,118 to 18,524,118 for the authorized reservation. The safe result is under `probes/probability-total-0038/`.

System One therefore supplies a tolerance of `0.01 + member count × f64::EPSILON`. The hundredth covers the observed decimal rounding, and the epsilon term makes decimal `0.99` and `1.01` inclusive after binary parsing and addition. The adapter still refuses `0.98`, `1.02`, `0.85`, `1.15`, zero, and three ones. It preserves every member. Score still divides by the measured accepted total. Another adapter keeps the strict default until evidence supports its own exception.
