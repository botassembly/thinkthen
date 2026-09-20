# Full-project review and follow-up

Status: foundation defects closed; follow-up risks planned.

## Scope and snapshot

The adversarial review examined commit `2c32524` on 2026-09-20 across the command-line parser, pure core, adapter, record scheduler, recordings and cache, transforms, live-spend wrapper, specifications, ADRs, tickets, demos, installation, and release gates. It made no paid call. The four repository rungs and the committed replay corpus supplied executable evidence. Five focused issue files hold each reproduced foundation defect.

A follow-up at `5f452ff` checked the repairs and the independent feedback about them. Three read-only reviewers separately traced the live-spend wrapper, paid-response failure paths, and project records. This file consolidates the review that the five issue files previously cited only by name.

## Foundation findings

1. Ticket 0024 closed the scheduler hang and the unbounded queue of paid rows behind a slow first row.
2. Ticket 0025 added total and extra-key validation to answer distributions while preserving the existing missing-key refusal.
3. Ticket 0026 keeps the first complete recording entry and refuses a different later response for the same request.
4. Ticket 0027 compares modern complete rows by question digest. Older, mixed, or malformed rows fall back to the printed question text and say that they did so.
5. Ticket 0028 serializes and durably precharges live spending authority.

These were real correctness or budget-control defects. The hands-on passes still found no secret leak, crash, or wrong accepted answer outside the defects above.

## Follow-up findings

### Confirmed maintainability and operating problems

- Ticket 0028 grew the live wrapper from 87 to 449 lines and its Rust proof from 124 to 1,158 lines. Its child wait has no deadline and deliberately holds the lock until the recorded child disappears. Recovery requires manual PID, ledger, allowlist, sync, and cleanup judgments. Keep the fail-closed behavior and durable precharge. Add a checked recovery command and consistent charged-authority wording before the next paid feature test. Ian can instead overturn ADR 0022's durable child-identity requirement; that would permit a smaller design with weaker interruption evidence.
- `BackendError::NotAnAddress` prints `a base address has a host`, but no settled page or exact test owns that sentence. Add both with the remaining address repairs.
- The main plan and ticket 0023 carried stale state after tickets 0022 and 0033 landed. This plan update corrects them.
- Commits `be2c9a3`, `cab3922`, and `3f8d9bc` raised the Rust ceiling without the explanation required in the commit message. Their records explain the growth, but published commits cannot be repaired. Later commits comply. Add a mechanical push check for the rule.

### Accepted contracts with missing operating evidence

- A live response is received and paid for before a divergent recording conflict is known. Parallel duplicate cache misses can both run and then conflict. The first entry remains intact, as ADR 0020 requires. Document the consequence and measure exact response stability before considering single-flight cache misses or another contract.
- Probability members must total one within `member_count * f64::EPSILON`. All 497 choice and score objects in the 1,059 tracked recordings pass, and none totals 0.99 or 1.01. The repository contains no provider rounding promise. Keep the strict rule until an authoritative source or live response establishes a valid rounded distribution, then define an adapter-specific tolerance instead of guessing one.

## Feedback not accepted as defects

- Records 0024 through 0033 contain substantive independent design and code review sections. A separate reviewer artifact or name is not required.
- Future tickets remain unwritten until work begins by project rule. Their absence is not a planning gap.
- Every ceiling-raising commit from ticket 0028 onward explains growth and duplication. The rule failure is limited to the three historical commits named above.

## Work order

The [prospective plan](../planning/prospective-bash-rust-python-plan.md) carries the order: `annotate`, the long-document measurement and `find`, workflows and transforms, release preparation, then libraries only after ADR 0017 is decided. The companion track adds checked recovery before `annotate` makes its paid measurement, gathers operating evidence during authorized probes, and closes the small repairs before release.
