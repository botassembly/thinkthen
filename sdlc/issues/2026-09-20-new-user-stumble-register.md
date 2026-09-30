# The new-user stumble register

Status: open until launch. Shortened 2026-09-30: rows 1 to 8 and 10 to 17 are closed, and git history holds their record. Add a row when a stumble is seen. Close a row when its fix lands, and name the commit.

Ian asked on 2026-09-20 for one list of every place a new user stumbles in the first hour with no clear next step, so that the documents and the tool fix each one before the public push.

| No. | The stumble | What was seen | The fix | Kind |
| --- | --- | --- | --- | --- |
| 9 | Matching against a list stops at 255 entries | `choose` and `find` refuse more | Page 11 of `2026-09-25-docs-how-tos-and-spec-claims-owed.md`: a cheap first cut with `grep` or a database | Page |
| 18 | A new user waits for a key | Ian's key took about a day on 2026-09-20. TypeSafe's [2026-09-15 announcement](https://typesafe.ai/blog/introducing-system-one-models-and-jev) described a waitlist. The [public home page](https://typesafe.ai/), checked 2026-09-29, gives no current wait time | The README's replay example already needs no key and no network. The current wait time and registry onboarding stay open. Do not publish the old one-day observation as present guidance. Ticket 0128 closes this row with its Phase 4 README commit | Page and packaging |
| 19 | A first `check` against Liquid d1 times out | The Beatles Bench team's first `check` against d1 timed out at the default 30 s on 2026-09-30 and passed with `--timeout 90` | The Liquid d1 page says to pass `--timeout 90` on a first run, per `2026-09-29-docs-page-for-the-liquid-d1-backend.md` | Page |

After 0.1 ships, marketing runs a clean-machine install of each package from its registry. New stumbles from that run go in this register.
