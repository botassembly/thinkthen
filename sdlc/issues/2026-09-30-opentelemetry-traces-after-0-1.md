# OpenTelemetry traces for backend calls, after 0.1

Status: open. Filed 2026-09-30 on Ian's request. Owner: none until 0.1 ships.
Kind: idea
When: after the 0.1 release
Milestone: later

OpenTelemetry support would give a user traces of every backend call, with spans and token counts that match the call facts thinkthen already returns.

Ian's ruling, 2026-09-30: thinkthen itself sends no telemetry. The command and the libraries keep the "No telemetry" anti-goal in `../planning/libraries/README.md`. Traces belong in the optional proxy service (`2026-09-30-proxy-service-for-shared-limits-and-traces.md`), which a user stands up and runs. This idea is a part of that one. Ian can overturn this.

Limits a design must keep:

- It lives in the service. The command and the libraries send no telemetry.
- It is opt-in. With no exporter named, the service sends nothing and does no extra work.
- Spans carry no key, header, question text or answer text. Key secrecy and the recording rule apply to every span.
- Span fields reuse the names in `facts` and `meta.usage`, so a trace and a run's facts agree.
- It sends only to the address the user names, like every other send.

Related: `2026-09-26-every-surface-should-give-back-run-facts.md` owns the facts a span would carry.
