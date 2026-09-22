# 0067: Command introductions and teaching order

Status: Landed on main as `be078b717f075827f34e206ad9af5de458e88a31` after hosted gate run `35745327707` passed on that exact revision. The coordinator fast-forwarded main, pushed it, and verified ancestry.

The first landing attempt stopped before changing main because the filesystem had zero available space and Git could not write its lock. Ian freed space; the coordinator observed 134 GB available and retried successfully. No lock, source, build directory, or registered worktree was removed by the coordinator.

## What changed

The eight judgment commands begin with their approved text-operation summaries. Root help teaches decide, filter, rank, choose, find, score, tag, annotate, with status before and cache after. Decide's record-exit warning, find's short whole-set disclosure and long bounds/one-request explanation, rank's local-sort cautions, every record-exit explanation, and annotate's exit 6 remain. Tag/annotate result-shape descriptions moved into long help; 0066's examples remain unchanged. Only wording-list items 3 and 4 are addressed, and the owning issue carries the marketing capture notice.

## Review and verification

- Independent Sol design review accepted the bounded level-2 ticket and preservation of safety advice over cosmetic placement suggestions.
- SWE-2 added a compiled matrix first. It reported sixteen introduction mismatches and the old root order before source changes. The focused five integration-test targets then passed all forty-seven tests; the executable decide page passed twenty-three checks.
- Coordinator pre-review caught a permissive sentence-prefix check and the loss of find's explicit one-request sentence. SWE-2 tightened the sentence boundary, restored the disclosure, and reran the focused tests.
- A fresh independent Sol code reviewer accepted the final public help, retained advice, exact-order/uniqueness proof, and ratchet increase.
- The coordinator ran `sdlc/scripts/install && sdlc/scripts/lint && sdlc/scripts/test && sdlc/scripts/spec && git diff --check` with the provider key empty and Cargo offline mode. Exit 0: policy, packaging, audit, format, clippy, docs, 548 Rust tests, doctests, replay checks, 27 specification checks, seven transform-page checks, and nineteen green how-tos.
- The exact Rust ceiling rose from 32,787 to 32,856: 64 nonblank test lines and five net source lines. One matrix covers every introduction and the command order, while existing retained-advice tests remain distinct. Enum reordering adds no separate ordering registry.

## Limits and next step

No judgment behavior, option parser, result/recording shape, dependency, package metadata, library, or website changed. No provider call or publication ran. Ticket 0065 and the library team's worktree retain their owners. Reconcile the remaining outcome vocabulary before broad prose replacement; the merged forty-item issue remains open. Engine settings/cache work remains behind 0065's landing, and the relation-method conflict remains unresolved.
