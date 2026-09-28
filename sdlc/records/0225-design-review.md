# 0225 design review

Status: pending fresh independent design review. Review [ticket 0225](../tickets/0225-bound-usage-lock-waits.md), [ADR 0097](../planning/adr/0097-bound-advisory-usage-lock-acquisition.md), and the [source preflight](0225-usage-lock-preflight.md). No runtime or Settled page has changed; this note records no acceptance.

The reviewer should trace `finish`, the writer loop, `update` and `Drop` together. A finish-only timeout leaves the joining destructor blocked; an independent per-month deadline can extend finalization beyond the proposed one-second foreign-lock budget. Check whether the writer's one deadline, nonblocking lock attempt and existing failure/queue-clear path let the command return with one warning and possible advisory count loss. The source must not promise a total exit bound across arbitrary filesystem operations. Confirm the proposed regression holds a scratch usage lock through a complete command and independently checks answer, warning, facts and unchanged count. ADR 0097 would amend only ADR 0049 item 3 and the settled recording page; the live paid ledger remains untouched.

## What the build taught us

Preparation found that the accepted write-behind queue has a separate destructor join and a multi-month pending vector. Those details set the small design's actual boundary. Fresh review must correct the proposal before source work if its deadline or cleanup is incomplete.
