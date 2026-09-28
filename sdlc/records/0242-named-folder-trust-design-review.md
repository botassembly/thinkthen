# 0242 named-folder trust design review

Status: **Awaiting fresh design review.** Candidate is the proposed [ticket](../tickets/0242-named-folder-trust-warning.md) and its [preflight](0242-named-folder-trust-preflight.md) on `ticket/0242-named-folder-trust-warning`. The author has not accepted this design or claimed runtime files.

Review the original experiment 283 finding and filed register-40 issue, the existing private-default gate, all named CLI folder selectors, and the deliberate non-Unix and library limits. Check that the warning can precede a real cache/replay hit, that owner mismatch and directory group-write are distinct from the configuration-file rule, and that no warning text discloses a path, URL, key, evidence or entry bytes. Verify that the proposed temporary listener table detects a missing warning without changing the cache/record/replay protocols. Record ACCEPT or concrete findings here after independent review; only then may the coordinator claim runtime work.
