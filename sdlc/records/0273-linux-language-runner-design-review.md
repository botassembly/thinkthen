# 0273 Linux language runner design review

Fresh independent High review accepted design `131f065f` on its pushed ticket branch. The coordinator approves implementation within that scope. Ticket 0273 and its preparation remain on the branch until implementation lands. This adds one reviewed design, not a completed release issue.

Review found that selecting `javac` and `jar` through a JDK home did not bind Kotlin and Scala's transitive Java process. The corrected design sets both `JAVA_HOME` and `JAVACMD`, prepends the selected JDK binary directory, and checks inherited-wrong and selected-wrong Java cases. The reviewer read the actual Kotlin and Scala launchers to verify that `JAVACMD` takes precedence. The corrected design names the workflow checker and exact official Kotlin asset.

The reviewer also accepted a small standard-library `release-language-tools.py` helper and separate self-test. Managed and smoke modes keep package selection, digest verification and environment handoff out of the shell dispatcher. They can be built independently while 0272 holds the shared workflow files. Routine self-test registration and workflow integration must follow before landing; no isolated helper acceptance completes the ticket.

Reuse the independently verified signed Ubuntu snapshot and official SDK input records. No actual package download, installation, compiler, consumer, container, SQL/DataFrame or Actions run occurred in design review. Actual runner qualification remains a later checkpoint.
