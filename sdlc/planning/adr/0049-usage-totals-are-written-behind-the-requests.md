# ADR 0049: Usage totals are written behind the requests

- Status: Accepted 2026-09-26 through ticket 0141. Ian can overturn it.
- Date: 2026-09-26

## Context

Each request rewrote the usage file and forced it to disk before it was sent and again when its reply came. Every worker queued on one lock, so the disk set the pace. Experiment 268 measured a 306-title `filter` run at 6.6 to 7.2 s with the file on disk and 3.5 to 4.0 s with it in memory. A second `thinkthen` process holding the usage lock stopped every request. `sdlc/issues/closed/2026-09-26-usage-file-writes-serialize-requests-in-flight.md` holds the evidence.

Ticket 0063 wrote the count of each request to disk before sending it, so a crash could overcount one request and never undercount one. The totals enforce no budget, as ADR 0034 says. The paid-call authority is the live ledger, which keeps its own precharge under ADR 0022.

## Decision

1. A request counts in memory and never waits on the usage file. One writer thread per process writes every count that piled up since its last write. It uses the same folder, lock, file, and checks as before.
2. Each count keeps the month it was counted in.
3. The command waits for the writer after its results, then prints the one warning if a write failed. It waits as long as another process holds the lock.
4. The guarantee changes. `specification/recording.md` said "A crash can undercount tokens or overcount one precharged request." ADR 0034 said "A crash can leave a conservative overcount of one request or an undercount of later tokens." Both now read: "A crash can undercount the requests and tokens counted after the last write that finished." Ticket 0063's precharge rule is retired.

Amendment on 2026-09-28: [ADR 0097](0097-bound-advisory-usage-lock-acquisition.md) replaces item 3's unlimited advisory usage-lock wait with one shared finalization deadline. The write-behind queue and the rest of this decision remain.

## Consequences

`--jobs N` puts N requests in flight whatever the usage file is doing. A killed process loses the counts it had not yet written. Ticket 0141 weighed one write at the end of the run, which needs no thread. It was set aside because `status` would show nothing during a long run and a kill would lose the whole run.
