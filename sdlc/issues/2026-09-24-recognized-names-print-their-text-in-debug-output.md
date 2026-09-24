# Recognized names print their text in Debug output

Status: closed by ticket 0102 (`sdlc/tickets/0102-redact-recognized-names-in-debug.md`). Found 2026-09-24 by the 0088 builder while adding relate to the shared Debug secrecy suite.

The 0088 fix made relation entities redact their text in `Debug`. `recognize`'s own name type, `RecognizedName`, still derives `Debug` and prints the entity text. The secrecy rule in `CLAUDE.md` says the secrecy test reads every `Debug` line. A panic message or a log line that formats a recognized name would carry user text.

Fix: give `RecognizedName` the same redacting `Debug` that 0088 gave relation entities, and add recognize to the Debug row of the shared secrecy suite. Owner: Claude, queued after 0088 lands.
