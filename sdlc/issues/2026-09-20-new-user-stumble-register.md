# The new-user stumble register

Status: open until launch. Shortened 2026-09-30: rows 1 to 17 are closed, row 9 by ticket 0350 (`8f397e963`), and git history holds their record. Row 19 closed 2026-10-01 by site ticket 0042 (`8c5aabc00`): the Liquid d1 page tells a reader to pass `--timeout 90` on a first `check`. Row 18 belongs to the release ticket 0128, whose Phase 4 README commit closes it. The register holds no pre-0.1 work for the queue owner. It stays open until launch by design, so it blocks no release. Moved from 0.1 to later on 2026-10-01. Add a row when a stumble is seen. Close a row when its fix lands, and name the commit.

Milestone: later

Ian asked on 2026-09-20 for one list of every place a new user stumbles in the first hour with no clear next step, so that the documents and the tool fix each one before the public push.

| No. | The stumble | What was seen | The fix | Kind |
| --- | --- | --- | --- | --- |
| 18 | A new user waits for a key | Ian's key took about a day on 2026-09-20. TypeSafe's [2026-09-15 announcement](https://typesafe.ai/blog/introducing-system-one-models-and-jev) described a waitlist. The [public home page](https://typesafe.ai/), checked 2026-09-29, gives no current wait time | The README's replay example already needs no key and no network. The current wait time and registry onboarding stay open. Do not publish the old one-day observation as present guidance. Ticket 0128 closes this row with its Phase 4 README commit | Page and packaging |

After 0.1 ships, marketing runs a clean-machine install of each package from its registry. New stumbles from that run go in this register.
