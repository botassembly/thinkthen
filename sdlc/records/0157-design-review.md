# Accept the library size and retry settings design

Status: design accepted 2026-09-27. Owner: Codex. No runtime change is claimed by this record.

A fresh read-only Codex reviewer returned **ACCEPT** on ticket 0157 at `5f3b943e`, contingent on corrected ticket 0149 landing first. The coordinator accepts the design within the existing 0148 settings map, ADR 0051 request-size rule and ADR 0052 retry-count rule. Ian can overturn those accepted choices through their owning records.

Ticket 0157 propagates the already settled positive request-byte ceiling through the Rust builder, six library bindings and SQL where it acts. It reads the command's environment variable through the library builder, keeps the 96,000-byte default and the smaller-of profile rule, and adds one shared settings case. It exposes the engine's retry count through `Counters::retries()` and each library's existing usage shape without recomputing it from sends. A retry remains a subset of `requests_sent`; a lone request that cannot split still goes alone. The ticket adds no new credential channel, transport retry rule or campaign gate.

The ticket evidence self-test passed 13/13, the checker found zero failures, and the design diff check passed at `5f3b943e`. No library or SQL implementation, fixture run, lint gate or issue closure is claimed. Implementation waits for 0155, 0154 and corrected 0149 to land, then takes exact runtime file claims and fresh code review.
