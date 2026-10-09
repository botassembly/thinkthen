# 0468: Expose shared usage persistence observations

Source `79d074c7f` implements the reviewed shared state and facts slice. One atomic failure latch owns writer and queue failure. Engine callers can observe persistence without waiting or explicitly finish usage under the existing lock deadline. Complete invocation facts freeze the state at their snapshot; aggregate tallies invent no persistence observation. Legacy facts serialization and void finalization remain compatible. Serializer-derived result schemas carry the additive observation.

The developer passed default all-target Clippy, supported library-only Clippy, policy, formatting, measured source size, the existing writer cases, public usage cases, inherited-held-queue fork regression and generated schema comparison. The public case retains good answers and exact request counts while showing pending writes and safe latched failure advice. The fork case retains exact durable child counts and avoids inherited parent locks. All owned jobs were reaped. No fresh code review, full qualification or landing is claimed.

The existing API inventory matched the usage declarations but failed on the already-declared, unlanded recognition APIs from 0479. Integrate that dependency after its repair and landing, reconcile the generated schemas and measured ceiling, and rerun the inventory before review. Host methods, the C status door and SQL status functions remain required in 0.2 under this ticket and the existing adoption tickets. Only usage-lock acquisition has a deadline; other filesystem work remains unbounded.

## What the build taught us

Preserve a single failure owner when making observation nonblocking; copying the queue failure flag into another state would create drift. A facts snapshot reports the state when observed, not a promise that an asynchronous write will later succeed. Observe and finalize only counters owned by the current process. Use supported feature profiles: bundled and host SQLite are mutually exclusive. Keep API inventory declarations on the implementation branch until their code can land, because the existing checker reads open tickets too.
