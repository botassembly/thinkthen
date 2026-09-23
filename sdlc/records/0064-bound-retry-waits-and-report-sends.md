# 0064: Bound retry waits and report sends

Date: 2026-09-22

Status: landed

## Result

Every exponential and header-selected retry wait now stops at the attempt timeout. `Retry-After` keeps its existing 60-second ceiling. Help states that `--timeout` bounds one attempt and each retry wait; the total run may still contain every configured attempt and the waits between them.

Detailed ordinary and annotation results now carry `meta.requests_sent` immediately before `replayed`. A first-attempt live answer reports one, retries add their actual attempts, and cache or replay hits report zero. Packed annotation sums sends across its group requests. Provider token counts and logical request digests keep their prior meanings, while persistent status counts remain broader because they also include failed runs.

A clean peer close before response headers already returned promptly through the pinned HTTP client, so no client workaround or dependency was needed. The exact regression reads the complete request, closes every accepted handle, and pins the premature-close message. A separate open peer proves the configured timeout still yields the timeout message.

The Rust ceiling rose from 32,464 to 32,713 nonblank lines. The HTTP answer count, result plumbing, wait-bound checks, raw close and open-peer harnesses, storage-mode accounting matrix, exact JSON assertions, and help proof account for the increase. The implementation reused the existing HTTP loop, `Answered`, `RequestMeta`, serializers, listener, and one shared result-normalization helper rather than duplicating result comparisons across test modules.

## Review and proof

Independent design review rejected the first draft because it did not freeze the field position, preserve both wait ceilings, define every recording mode, or treat the early-close report as an unconfirmed hypothesis. The whole rewrite fixed those points and was accepted at level 3 with Sol Medium.

Independent code review rejected the first implementation because it lacked the open-peer timeout counterpart and the two help homes described retry waits differently. Remediation added a distinct accepted-socket timeout test with the exact safe message and one identical help sentence. The reviewer accepted the implementation with no remaining material finding.

The final install, lint, test, and specification rungs passed with the key and outside address variables unset. The lint rung measured the exact 32,713-line ratchet. The specification rung passed all 19 green demos and every committed replay. `git diff --check` passed. No paid or outside request ran.
