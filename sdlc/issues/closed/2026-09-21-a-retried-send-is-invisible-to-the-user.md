# A retried send is invisible to the user

Status: Closed by ticket 0064

A judgment that needed three sends to arrive reports one send. The `--details` object after a retried run is byte-identical to the object after a clean single-send run. The user's bill shows three requests and the tool shows one judgment carrying one send's tokens.

ADR 0017's counters paragraph rules the other way: "The second send reaches the user in two places: `usage` carries the process's send count, and a judgment's `details` carries the number of sends that produced it, so a bill showing two requests never meets a tool showing one."

## Reproduction

A local stand-in answers the third request after two 500s. Commands from a scratch folder, 2026-09-21, with the stand-in's request log counting sends:

    $ thinkthen decide 'Does this convey urgency?' --details < evidence.txt
    {"schema":"thinkthen.result/1","value":true,...,"meta":{...,"usage":{"input_tokens":337,"output_tokens":48},"replayed":false}}

    $ thinkthen decide 'Does this convey urgency?' --details < evidence.txt   # after two 500s
    {"schema":"thinkthen.result/1","value":true,...,"meta":{...,"usage":{"input_tokens":337,"output_tokens":48},"replayed":false}}

The two objects are identical byte for byte. The stand-in saw 1 send for the first and 3 sends for the second. A sum would read 1011 input tokens and 144 output tokens. The details object carries no send count, the bare run prints nothing on a successful retry, and the command has no `usage` surface.

The orchestrator reproduced this independently on 2026-09-21: a clean run and a fail-twice-then-succeed run produced identical `--details` files, and the stand-in's log held exactly the expected sends.

## Where the gap lives

`specification/result.md` defines `meta.usage` as the backend's usage for the answer, and names no send count. The gap is against ADR 0017 and the quality brief, not against a settled specification page. Closing it needs either a spec page that carries the send count or the counters ADR 0017 rules in.

## How bad it is for a user

Major. The tool's own numbers disagree with the bill, and a user reconciling a spend has no lever that shows the retries.

Found by experiment 218, wave 1, area 7.

Ticket 0064 adds `meta.requests_sent` to every detailed result. A two-attempt success reports two while the persisted process total reports the same two sends. Cache and replay answers report zero, and a failed run remains visible only in the broader process total.
