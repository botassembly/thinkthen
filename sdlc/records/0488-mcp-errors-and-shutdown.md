# Preserve MCP error correlation and specify stdio shutdown

The MCP 2025-11-25 [message contract](https://modelcontextprotocol.io/specification/2025-11-25/basic/index#messages) requires an error to retain a readable request ID. Its error schema permits an absent ID, with string or integer values when present. The older [JSON-RPC response contract](https://www.jsonrpc.org/specification#response_object) requires null when the ID cannot be read. The admitted MCP version controls this stdio surface.

The parser discarded every ID when an envelope failed admission, including readable IDs on a wrong protocol version, a nonstring method and an unknown envelope member. Output also encoded unreadable IDs as null. The parser now recovers one typed ID independently of envelope admission. Duplicate IDs remain ambiguous and receive no ID; malformed JSON, nonobjects, null, boolean and fractional IDs likewise receive no ID. The existing output door omits an absent ID. Error text remains fixed and withholds request payloads.

Closing stdin initiates shutdown in the admitted [stdio lifecycle](https://modelcontextprotocol.io/specification/2025-11-25/basic/lifecycle#shutdown). The [transport](https://modelcontextprotocol.io/specification/2025-11-25/basic/transports#stdio) requires newline-delimited messages. The current EOF behavior therefore needs no correction: stop pending protocol work, cancel active dispatch and join native attempts already sent. A JSON object lacking LF is an incomplete transport frame. Pending responses are not guaranteed after EOF; an interrupted output write reports local I/O failure. Callers must await responses before closing stdin.

The installed regression sends malformed envelopes through the owned subprocess, checks error code and exact ID type or absence, checks safe output and proves zero loopback requests. The EOF case holds an already-sent attempt, checks a complete second call receives busy, then closes stdin; its partial-frame variant omits LF and checks local I/O failure. The existing runtime backpressure case supplies queued protocol messages behind full output and verifies EOF joins owned threads. The focused runtime cases also cover cancellation and broken output.

## Verification

Source revision `9333013f99d7957eda6f14e1c7aa24d68ea96242` passed `sdlc/scripts/lint`, `sdlc/scripts/test`, `sdlc/scripts/spec` and `libraries/mcp/check.sh` with exit 0. The existing successful `sdlc/scripts/install` result was reused after the internal serialization correction. Policy passed before review. The test gate passed routine, library-only and external consumer cases; its ordinary skipped cases were not claimed as executed stress checks. The specification gate passed its executable pages and 24 demos. The complete MCP row passed 258 canonical cases, nine client checks and 17 installed behavior groups, including error IDs and both EOF variants.

The first gate launch lacked the existing `mustmatch` path under the service manager; providing the installed tool path resolved that prerequisite. A later workflow self-test failed when its process-status reader observed a disappearing process and raised `ProcessLookupError`. That unchanged self-test passed on retry. Its failure remains in the owned gate log; this ticket changed no workflow or process-cleanup code.

The gate job used the existing lane targets, two build jobs, an 8 GiB memory cap and a 1 GiB swap cap. It completed and reaped its owned subprocesses. Native Windows and explicit stress qualification were not run.

## Lesson

The initial failing caller table established a protocol defect in correlation and a version-specific error shape. It did not establish a defect in shutdown. Protocol versions can narrow base JSON-RPC rules, so a plausible generic response expectation is not enough. The first focused compile found two mistaken test adaptations after changing the parser error type; those adaptations were corrected before behavioral checks. The first full lint found forbidden JSON indexing in error output. A typed response now serializes the optional ID directly and removes that panic-shaped operation.

No paid call, production data or private key was used. Loopback fixtures cannot establish compatibility with every external MCP client or Windows runner behavior; those remain separate qualification boundaries.
