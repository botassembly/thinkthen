# A release needs two environment approvals and an 80-minute wait

Status: open. Filed 2026-10-04 by the queue owner from the 0.1 releases. Owner: the queue owner.
Milestone: 0.2

Release mode waits for one `release` approval before the eight publish jobs. Then `publish` waits for a second approval before it makes the GitHub release public and creates the Go tag. The approval comes about 80 minutes after the dispatch, once the builds and smokes finish. For 0.1.2, Ian's first approval did not register, and the run waited an hour before anyone noticed.

To evaluate:

1. Whether `publish` can run without a second approval once the publish jobs pass, because it publishes nothing new to a registry.
2. A notice when the run reaches the approval, so the approver does not have to watch for it.
3. Whether the release run can reuse the rehearsal's built and smoked files for the same commit, so the approval comes minutes after the dispatch.
