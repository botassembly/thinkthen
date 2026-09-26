# Surfaces write judged text to disk without saying so

Status: Open. Filed 2026-09-26 by the queue owner from local experiment 273, report 12, finding 2.1. Blocks 0.1: it is a data-boundary claim, and every library and SQL reader meets it.

## What happens

The answer cache is on by default for the command, the libraries and the SQL extensions. Each entry holds the full evidence text, in plain text, with no expiry. The main README warns about this. No library or database README does.

- In PostgreSQL the folder belongs to the server's operating-system user. Every role that can call the functions shares it. That places row text outside the database's own access control, row-level security included.
- The folder marker binds only the adapter and the address. So two keys or two roles share answers, and `meta.cached` tells a caller that someone else already judged the same text.

A database administrator who adds `thinkthen_decide(body)` to a query over customer rows later finds a growing folder of those rows under the server's home.

## Checked on main

Verified: no `libraries/*/README.md` or `databases/*/README.md` says that entries hold the judged text. `databases/postgresql/README.md` names evidence only as a parameter. The PostgreSQL sharing comes from the report and was not rerun.

## What would fix it

1. State the default and what an entry holds in every library and database README and in the settings table.
2. Consider turning the cache off by default for the SQL extensions, or on only when an administrator names a folder. That choice changes a default on three surfaces, so the queue owner records it in a ticket.
3. Consider an expiry setting later.

Report 08, finding 11, adds a related gap: `Engine::builder()` ignores the configuration file's `cache: false`. The severity 3 roll-up lists it.

## Done when

Every surface's README and the settings table say where judged text is written and how to turn it off.
