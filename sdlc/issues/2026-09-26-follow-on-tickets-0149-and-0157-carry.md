# What follow-on tickets 0149 and 0157 carry

Status: Open. Filed 2026-09-26 by the coordinator, so that work other tickets defer to 0149 and 0157 is findable before those tickets exist.

Tickets 0148, 0154 and 0155 defer parts of their scope to two tickets that are not written yet. This issue lists what each must carry. The coordinator writes each ticket from this list and closes this issue when both land.

## Ticket 0149: settings on the SQL surfaces

- The settings rules in ticket 0148's section "Ticket 0149, the SQL settings". The address and key stay out of SQL. The profile is JSON text. `off` turns the cache off on DuckDB and PostgreSQL.
- ADR 0052 item 8, from ticket 0155. A retry counts against the SQL process request total. When the total is spent before a retry, the retry is not sent, and the call gets the spent-total refusal.
- It closes the SQL parts of `2026-09-26-settings-some-surfaces-cannot-reach.md` and `2026-09-26-libraries-cannot-replay-a-recording-strictly.md`.

## Ticket 0157: the size setting and the retries count on the libraries

- `max_request_bytes` from ADR 0051, ticket 0154, on every library and on SQL where it acts.
- `Counters::retries()` from ADR 0052, ticket 0155, on every library.
- Each new setting gets its row cells in `specification/settings.md`, which `sdlc/scripts/settings` checks.
- It builds after 0148 and 0149.
