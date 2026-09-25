# Triage of the open issues, by layer

Status: Closed on 2026-09-22. Replaced by `sdlc/planning/open-issues-for-the-architect-2026-09-22.md`.

Ian asked on 2026-09-21 how many issues remain and which layer owns each. This folder held 49 files that day. Ten were already closed. A read-only sweep checked the other 39 against the tickets, the records, the ADRs, the specification, and the code. The sweep was quick. The builder confirms a row before acting on it, and closes each stale issue with its evidence.

Ian's frame, and it holds: a rule or a behavior in the command becomes the Rust functions, and every library and database inherits it. A fix made before the engine merge lands once. The same fix after the libraries ship lands ten times. The rows marked "every surface" go first for that reason.

## Stale: close with the evidence (7)

| Issue | Evidence |
| --- | --- |
| `2026-09-18-two-lint-rules-do-not-hold-as-written` | `rust-standards.md` already carries the corrected rule |
| `2026-09-19-agents-md-still-describes-profiles` | Ticket 0007 |
| `2026-09-19-replay-check-fails-on-meta-tool` | `probes/replay-check.sh` drops `meta.tool` on both sides |
| `2026-09-19-review-leftovers-from-ticket-0008` | `meta.tool` on every row, and `repeated_ids` in the comparison |
| `2026-09-19-the-live-script-does-not-exist` | Ticket 0006 |
| `2026-09-20-feedback-on-the-annotate-plan-for-ticket-0015` | Ticket 0015 and its record |
| `2026-09-20-tag-a-fourth-question-type-for-many-labels` | Ticket 0036 and ADR 0029 |

## Rulings, measurements, and registers: no work in them (8)

The design-capture ideas, the label-tuning survey, the triage-pipeline question, the database ruling, the libraries ruling, the live-probe findings, the release-number ruling, and the new-user stumble register. They stay as reference. The stumble register stays open until launch by design.

## Open, and every surface inherits it

| Ask | Issue | Layer |
| --- | --- | --- |
| A cap on a user's spend, and a request count in `--dry-run` | `2026-09-21-where-a-user-could-lose-trust-a-first-list`, item 1 | Engine setting, command option |
| One bad record ends a run: decide whether a skip-and-log mode enters | The same issue, item 8 | Engine |
| The cache: a prune command, `THINKTHEN_CACHE`, no removal on the request path, and a measured store for a million entries | `2026-09-21-the-disk-cache-is-never-on-unless-the-user-names-a-folder` | Engine, needs an ADR |
| An error that says whether a second try could help, a deadline for one call, and the whole ranking from `find` | `2026-09-21-what-a-tool-search-feature-asks-of-find-as-a-function` | Engine, feeds the ADR 0017 rewrite |
| One request sent twice moves by up to 0.08: state it, size the band by it, and give the comparison transform a floor | `2026-09-21-the-same-request-answers-differently-twice-measured` | Pages, one transform |
| A control-character check on `--field` and on options | `2026-09-19-small-leftovers-from-the-security-ticket` | Core |
| An input-size cap and one shared path to exit code 70 | `2026-09-19-review-leftovers-from-the-core-tickets`, items 7 to 13 | Core and command |
| The fork hang, the width gate, the cancel token, the fast stop | `2026-09-20-what-the-two-experiments-ask-of-the-engine-and-the-order-to-build-it`, `2026-09-20-a-process-that-forks-after-its-first-call-hangs`, `2026-09-20-steering-on-the-version-one-completion-plan` | Engine, already in the plan |

## Open, and only the command carries it

| Ask | Issue |
| --- | --- |
| The first lines of the help, the three public words, one layout for all eight | `2026-09-21-the-help-first-lines-and-the-public-words` |
| Findings 2 to 10 of the second hands-on pass, all help and message wording | `2026-09-19-hands-on-test-pass-two` |
| `transforms/cost/cost.jq:32` fails on a row with no `input.id`. It was hit live in the accuracy round | `2026-09-20-accuracy-round-on-three-public-sets-and-a-speed-rerun` |
| `sdlc/scripts/live` refuses a job that is not a shell script | `2026-09-20-packing-rows-into-one-request-measured` |
| A `status` view of configuration and usage. No code, page, or ticket exists | `2026-09-20-a-status-command-for-configuration-and-usage` |
| A stale `interface-audit.md` | `2026-09-20-are-we-using-everything-the-service-offers`, `2026-09-20-what-the-launch-needs-from-the-build` |
| Two review leftovers the sweep could not confirm either way | `2026-09-19-review-leftovers-from-ticket-0006`, `2026-09-19-review-leftovers-from-ticket-0010` |

## Open, and it is release and pages work

| Ask | Issue |
| --- | --- |
| A release workflow, an installer, and trusted publishing. `.github/workflows/` holds `gate.yml` alone | `2026-09-20-launch-gaps-found-in-marketing-prep`, `2026-09-20-lessons-from-biomcp-for-release-install-ci-and-docs` |
| Fifteen how-to pages that no ticket owns: six from the coverage pass and nine from the notes | `2026-09-20-second-coverage-pass-no-new-verb-and-six-soft-spots`, `2026-09-20-use-cases-from-the-notes-that-no-page-teaches` |
| A mechanical check for the ceiling rule on commit messages | `2026-09-20-full-project-review-and-follow-up`, if the builder still wants it |

## The experiment team's

`2026-09-20-feedback-to-the-experiment-team-after-both-harvests`. It now also carries the million-entry cache measurement from the cache issue.

## A suggested order, after the transforms

1. Close the seven stale issues.
2. The two small defects: `cost.jq` and the live script's refusal.
3. One ticket for the help and message wording: the help issue and findings 2 to 10 together.
4. `thinkthen status`, the request count in `--dry-run`, and the stateless `--max-requests`, per the recommendation in the status issue. No ledger.
5. The ADRs the engine needs before the merge: ADR 0017, and one for the cache.
6. The engine's four steps, with the error shape, the deadline, and the ranking from `find` folded into step 2.
7. The release workflow and the how-to pages, in release preparation.

## What Ian can overturn

All of it. The order is a suggestion to the builder, who owns the plan.
