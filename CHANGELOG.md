# Changelog

Every release of every surface shares one version number.

## Unreleased: 0.1.0

The first release. The command, the Rust, C, Python, TypeScript, Ruby, and R libraries, and the SQLite, DuckDB, and PostgreSQL extensions.

The command handles SIGTERM like Ctrl-C, reports a signal as the cause when a sent request later fails, and gives the finished count without naming a record that may not exist.

The SQL extensions add `thinkthen_try_details` so a recoverable bad row yields a safe typed JSON failure and later good rows continue. SQLite adds a connection-scoped ThinkThen time budget. DuckDB retires idle engine plans while retaining cumulative usage and its 16-plan cap. PostgreSQL refuses a changed explicit throttle. DuckDB's whole-query budget follows in ticket 0201.

Every surface paces its HTTP attempts to 1,000 a minute for each `https://` address, under the vendor's published 1,200. `THINKTHEN_REQUESTS_PER_MINUTE` sets another rate. The pacer counts within one process (ticket 0308).

The default model is the pinned version `jev-1.13.0`, not the alias `jev-latest`, so a vendor's move of its alias moves no default answer (ticket 0159).

Python and Ruby calls now raise cancellation when the caller's token fires before a held reply reaches the call, including when both happen in one wait tick (ticket 0168).

`recognize` finds names in three steps, as ADR 0056 decides. Each name prints `text`, `start`, `end`, `length`, `kind` and `strength` on every surface. The default kinds are gone, so a run with no kinds prints every name as `ENTITY`. `--max-text-bytes` refuses a text over 600,000 bytes before any request (ticket 0147).

`relate` asks one yes/no question per allowed pair and keeps every edge at the cut. All rules share one entity state and requests of at most 400 questions; this changes relate request bodies and recording digests. Its version-one plan keeps one entry per rule with the requests carrying that rule's questions. Wildcard edges now print in question order (ticket 0167).

`decide`, `filter`, `rank`, `choose`, `tag`, `score` and `annotate` send one quoted form on every surface. The state is the fixed sentence `Each question quotes the text it asks about.` or the context, and each question quotes its own record, a batch of one included. The record-list state and the unquoted single-record request are gone, so request bodies and recording digests change and an old cache entry is never found again. A profile's evidence limit still bounds each record, and it now also counts the fixed sentence or the context, so a limit below 44 bytes refuses every such request (ticket 0304, ADR 0111).

`diff` compares two `recognize` or `relate` runs, or two cuts on one. Each changed record lists the names or edges it gained, lost, or changed in kind, and a key runs McNemar on the key names or edges only one side matched. `--match strict|overlap` pairs names as `audit` does (ticket 0165).

### Breaking changes

None. This is the first release.
