# relate help lists --jobs and relate refuses it

Status: Open

`thinkthen relate --help` lists `--jobs` from the shared options, with the line `How many requests are in flight at once, from 1 to 32. [default: 4]`. A `relate` run given `--jobs` exits 2. The ticket 0082 code review observed both, and the mismatch predates 0082 (`sdlc/records/0082-code-review.md`, follow-up F2).

A user who reads the help and passes the option gets a usage error. The fix is either to hide `--jobs` from `relate` help or to give `relate` its own option set without it, as `find` has. Either change touches the parsed surface, so it needs its own ticket.
