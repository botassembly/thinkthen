# 0272 managed workflow code review

Verdict: static CODE ACCEPT `bfa7c2e3` after fresh independent High review. The reviewer first found three helper defects at `ede90a0b`: rejection of genuine Git source symlinks, acceptance of missing mandatory nupkg files, and acceptance of extra ZIP directories or symlink-typed entries. The author corrected these with distinct assembly and verification refusals. Tests refresh the managed receipt and outer checksum before checking inventory/type failures.

The reviewer accepted the workflow integration at `417f5da8`, then confirmed it was byte-identical in the final candidate. Source and C receipts are captured at their producer handoffs and checked before both extractions and managed assembly. Fresh managed output is checked and hashed before wrapping. A separate internal provenance artifact reaches smoke and draft validation before release copying and never enters public assets. Existing target and family gates remain.

Final checks passed: the focused helper fixture, registered workflow checker, real Git source capture and tamper refusal, and diff checks. The reviewer also ran source-check, assemble and verify using a genuine `git archive 367c160f`, the retained 0262 C archive, and its actual nupkg and three JARs with independently captured receipts. No compiler or consumer was rerun. The coordinator merged the exact accepted code and passed pages, tickets and diff checks.

SDK provisioning, actual selected runner execution, installed consumer observations and Actions artifact behavior remain future evidence. No original consumer-release issue closes at this static checkpoint. Ticket 0273 owns the missing setup implementation; SQL/DataFrame work remains held.
