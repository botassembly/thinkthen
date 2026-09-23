# ADR 0033: Bounded default cache and read-only configuration

- Status: Accepted through ticket 0062 under Ian's ADR 0017 ruling
- Date: 2026-09-22

## Decision

Normal judgments use the platform cache by default. Linux uses `$XDG_CACHE_HOME/thinkthen` or `$HOME/.cache/thinkthen`; macOS uses `$HOME/Library/Caches/thinkthen`. `--cache DIR` outranks `THINKTHEN_CACHE`, which outranks the platform folder. Either explicit folder enables caching. `--no-cache` disables the answer cache for one run. Explicit record or replay stays separate and suppresses the default cache.

XDG homes and `HOME` must be absolute. A relative or blank value is unusable and never resolves against the working directory.

The read-only configuration file uses the matching platform configuration home and is absent by default. Its required schema is `thinkthen.config/1`; its only optional fields are `url`, `model`, `cache`, and positive `cache_bytes`. Address precedence is command line, environment, configuration, built-in. Model precedence is command line, single question file, configuration, built-in. A question set holds no model. Cache defaults on and its prune target defaults to 100,000,000 allocated bytes. The tool never creates or edits configuration.

Every recording, replay, and cache request holds a shared operating-system lock on the already-open directory handle. Explicit prune holds the exclusive lock. This gate creates no name and keeps replay read-only. Digest locks continue to serialize missing or damaged entries inside the shared gate.

`cache prune DIR` requires an explicit directory. Age and answering-model selectors form a union. Size is a target applied after those selectors, oldest modification time first with digest names breaking ties. Prune counts allocated blocks of recognized valid final entries. It validates the whole scan before mutation, ignores unrelated names, refuses digest-shaped non-regular objects without following them, removes matching inactive locks, and syncs changed directories. A later removal or sync failure may leave a deleted prefix.

The default cache is the sole exception to the rule that every written file has a user-named path. Its entries hold the judged text. A new default folder is private to its owner; an existing Unix default folder must already be mode `0700`. Explicit folders keep their established user-owned mode. No key enters an entry.

## Consequences

Repeated ordinary commands avoid another request. No judgment prunes another entry and no background worker runs. A user chooses when to prune and must name the folder, which prevents implicit selection of a committed recording. The 100 MB number is a maintenance target rather than a hard request-time cap.

Ian can overturn the schema, precedence, paths, default-on reading, allocated-byte measure, selectors, and output line.
