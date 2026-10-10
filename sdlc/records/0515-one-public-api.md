# Remove dead binding code and preserve frozen C exports

Language migrations now remove their old APIs through their installed consumer checks. This ticket removed unshipped PHP readers and JVM package demos, placed DuckDB helpers in their owning crate, and documented the frozen 0.1 C exports separately from the recommended 0.2 API.

The final change deletes the unused shared host packet module, its registration and an obsolete Rust-test helper importing the deleted Python reader. Caller inspection found no remaining users. It removes 900 physical lines, including 883 nonblank Rust lines, lowering the root ceiling from 190019 to 189136. These numbers count only this final deletion; earlier family deletions belong to their own records.

Fresh read-only review accepts `e300f4266f9cc6f9636bdf76f9b513b5300336ae` and the measured reduction. The obsolete helper failed with FileNotFoundError before deletion. Retained Rust and CLI recognition checks pass and preserve declines, duplicates, thresholds, seed strength 0.36, field absence and replay. The installed Python owner checks typed round trips and presence. Workspace and host strict Clippy, formatting, policy, child-process, generated-header and all 107 C-export checks pass. Existing routine evidence is reused for unchanged behavior; the remaining routine checks continue at integration. No release or load suite ran.

Shared source adapters, SQL preparation and every frozen C symbol remain. Final evidence is in the cleanup lane's `target/0515-*.log`.

## What the build taught us

Retiring a library API also requires removing tests that import its private implementation. Keep the distinct observable assertions in their current public consumer. A shared translator with no remaining caller is dead code; preserving the C ABI does not require preserving unrelated language packet readers.

Closure integration passes all 1996 routine Rust cases, one doctest, 21 external consumer cases and the remaining routine script and lint checks. Unchanged earlier passing checks are reused.
