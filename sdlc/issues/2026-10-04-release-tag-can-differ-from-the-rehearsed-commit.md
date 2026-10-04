# The release tag can name a commit no rehearsal or checkpoint ran

Status: open. Filed 2026-10-04 by the queue owner from the 0.1 releases. Owner: the queue owner.
Milestone: 0.2

For 0.1.2, checkpoint `checkpoint/surfaces/2026-10-03-1` and rehearsal 37126990511 ran on `abac3ce61`. Ticket 0395's cherry-pick then landed as `08328c9c0`, and `v0.1.2` was tagged there. The release run built and smoked `08328c9c0`, but the R package's published shape was first tested by R-universe after publishing. It passed, but nothing required that.

Release-process section 5 says the checkpoint and the rehearsal run on the bumped head of `release/0.1`. Nothing enforces it.

To evaluate:

1. Release mode refuses a tag whose commit has no passing rehearsal run and no checkpoint tag.
2. Or the release run counts as the rehearsal, so any commit can be tagged, and the checkpoint sweep joins the release run.
3. Which surfaces a release run's smoke does not cover, as the R package's published shape was not covered.
