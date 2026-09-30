# A proxy service in front of the backends, after 0.1

Status: open. Filed 2026-09-30 on Ian's request. Owner: none until 0.1 ships.
Kind: idea
When: after the 0.1 release.

Ian's idea: a user stands up a thinkthen service that forwards requests to the backends. Every command, library and SQL session points its address at the service.

What it would solve:

- Rate limits across processes and machines. Today the requests-a-minute pacer covers one process, so separate command runs, bench cases and PostgreSQL connections add their rates together. A service sees every request and can pace them all.
- A single place to export traces, which is where the OpenTelemetry idea (`2026-09-30-opentelemetry-traces-after-0-1.md`) fits best.
- A single place to hold the keys, so clients need none.
- A possible MCP server face, so an agent could call the verbs directly.

Limits a design must keep:

- It is optional. The command and libraries keep working with no service.
- Key secrecy, the recording rule and "sends only to the address the user names" apply to the service as well.
- The service serves the same wire shape the backends do, so clients change only their address.

A lighter step for one machine: a lock file in the cache folder that every process on the machine shares for pacing. Ticket 0308 deferred that limit across processes.
