# 0238 code review

Verdict: ACCEPT `2ab0fe65fa76b59227c682656956131437af3997`, runtime source `98e26511efbf065963b1a13e8f72ecd1b27fa9e1`, after correction of the fresh High review's three findings. The reviewer was independent of design and implementation and rechecked the same candidate family read only.

The first review found that unlinking a completed digest lock could leave a waiter on the old inode while a third caller locks a new inode. The correction retains lock paths through ordinary misses, valid-entry rechecks, failed writes and refresh completion. Only folder-exclusive prune removes selected matching locks. One controlled three-process test observes the same inode and sequential sends; existing failed-owner, fault and prune tests cover adjacent boundaries. The contract states that orphan lock files remain, lock bytes are excluded from entry counts, and immutable older writers do not gain this guarantee.

The correction also rejects refresh with a disabled cache during dry runs and records the source growth and reuse rationale. The reviewer verified the frozen binary hash, found no remaining correctness issue, and retained earlier valid proof rather than rerunning broad gates. The build record holds exact functional checks and old-reader interoperability evidence.

Integration matched the accepted runtime, tests, specification pages and source counter byte for byte. The settings checker reported 46 rows, 55 flags, six environment names, 15 question-file keys and zero failures. Pages, tickets, scratch cleanup policy and whitespace passed. No provider or production cache was used. Register 40 and 105 remain outside this landing.
