# 0121 design review

Ticket: `sdlc/tickets/0121-add-the-backend-check-command.md`. Reviewer: a fresh, read-only Claude session (Opus) that did not write the ticket. It read the issue, the house rules, the specification pages, the engine, adapter, and command code on main, the 0086 branch at `09d81bc9`, and the second-backend evidence.

## First pass, at `795d8320`: ten findings

1. The `model` warning fires on the reference backend. The hosted service answers `jev-latest` as `jev-1.13.0` (`demos/40-what-yes-and-no-mean/recording/`). Fixed: the `model` row, its arm, and its test are gone. Decision 6 says why.
2. The `score` probe could not be built through the grammar. A level map keeps `null` and never sends a bare name. Fixed: the probe uses `fair`, `good`, and `excellent` in a map. The `mixed` probe's level list covers bare names.
3. A failed tag cannot name one wire question. The decoder folds `q4` and `q5` into one failure. Fixed: a failed logical question is named by its wire range.
4. The rows after a stop were under-specified for 404 and the first probe's row. Fixed: a stop table pins the lines for transport, 401 to 403, and 404.
5. The refusal would print `thinkthen: thinkthen check`. Fixed: the message begins at `check`, and the ticket pins the whole standard error line.
6. The SIGINT claim was wrong and untested. Fixed: the ticket cites the channels rule and the shared engine cancel, and claims no new test.
7. Tests could write usage totals into the real cache. Fixed: tests run through the backend harness, which clears the environment and sets a temporary `HOME`.
8. The dry run proved nothing about the live bodies. Fixed: the dry run prints the chunk bodies from the same `Engine::split` call the live path sends.
9. Minor: exit 4 gains a clause in `specification/channels.md`. The status arm takes only 401 to 404, apart from the drift status and the existing 429 and 503 arms.
10. Style: trailing "so" clauses split and the model clause dropped from Evidence.

The reviewer confirmed as sound: the Evidence format, the key rule, the address refusal through `Backend::resolve`, the stop counts of 1 and the refuse count of 4 under the retry rule, one-question failures surfacing as whole-reply errors, and the pinned warnings on the generic and malformed arms.
