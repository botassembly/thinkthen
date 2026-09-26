# Ticket 0142 names a private project

Status: Closed 2026-09-26 by a Quick Fix that removed the path from the ticket. Main's history still holds the line, and Ian decides how history is treated before launch.

`sdlc/tickets/0142-pool-keeps-jobs-connections.md` line 135 names a name from the private-name list. Commit `71a26685` brought it to main. `sdlc/scripts/lint` fails on it when the list at `~/.config/thinkthen/private-names.txt` is present.

## Fix

Replace the name with a generic consumer and rerun `lint` with the list. The line also sits in main's history, which needs the same treatment as any other private-name leak before launch.
