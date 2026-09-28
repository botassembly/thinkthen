# AGENTS.md still describes a backend profile

Status: Closed by ticket 0007. `AGENTS.md` names no profile, checked on 2026-09-21.

Found 2026-09-19 by the independent review of ticket 0007.

`AGENTS.md` says under "Credentials":

> A key is read from the environment variable its backend profile names. It is never committed, logged, hashed, echoed in a plan, written to a recording, or sent to a host other than its profile's.

Ticket 0007 removed profiles. The key is read from `THINKTHEN_API_KEY`, and it goes to the address the user named, as `specification/backends.md` now says. The two sentences contradict the contract.

`CLAUDE.md` is a symlink to `AGENTS.md`, so one edit fixes both. The file is not in ticket 0007's `opens:` list, so the ticket left it alone.

The fix is two sentences. Somebody with the file open should also read the rest of the page for anything else the smaller surface made stale.

Fixed on main on 2026-09-19, with the landing of ticket 0007.
