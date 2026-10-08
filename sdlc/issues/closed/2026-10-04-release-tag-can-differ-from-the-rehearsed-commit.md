# The release tag can name a commit no rehearsal or checkpoint ran

Status: closed.
Resolution: 68abc60a4
Milestone: 0.2

For 0.1.2, checkpoint `checkpoint/surfaces/2026-10-03-1` and rehearsal 37126990511 ran on `abac3ce61`. Ticket 0395's cherry-pick then landed as `08328c9c0`, and `v0.1.2` was tagged there. The release run built and smoked `08328c9c0`, but the R package's published shape was first tested by R-universe after publishing. It passed, but nothing required that.

Release-process section 5 says the checkpoint and the rehearsal run on the bumped head of `release/0.1`. Nothing enforces it.

To evaluate:

1. Release mode refuses a tag whose commit has no passing rehearsal run and no checkpoint tag.
2. Or the release run counts as the rehearsal, so any commit can be tagged, and the checkpoint sweep joins the release run.
3. Issue `2026-10-03-doc-tests-gate-checkpoints-and-releases.md`, items 2 and 3, gates checkpoints and releases on the doc tests. A tag check here should share its mechanism.
4. Which surfaces a release run's smoke does not cover, as the R package's published shape was not covered.
