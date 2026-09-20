# Database extensions are ruled in as a fast follow

Status: Open. A ruling and a study. No code is authorized.

Ian ruled on 2026-09-20: "I want to make extensions for all three using Rust. That'll be the next set of experiments after the libraries are done. I won't hold up the launch for the database extensions, but I definitely want to do them as a fast follow." The three are DuckDB, SQLite, and PostgreSQL. He pointed at a public post from 2026-09-16 in which a well-known DuckDB developer announced a DuckDB extension for Jev that classifies about a thousand rows in ten seconds.

## What was decided and recorded

- The three extensions are surfaces eight, nine, and ten over the one Rust engine. `sdlc/planning/databases/README.md` holds the study: eight SQL functions with the same names in all three, eight rules, the shared goals and anti-goals, and the order, DuckDB then SQLite then PostgreSQL. One page per database sits beside it.
- The experiments run in the workspace as experiment 207, after experiment 205 on the libraries, against the same stand-in engine and stub. They make no paid call.
- None of this enters version one. Nothing in ticket 0015 or the completion plan changes.

## What already exists, read by survey on 2026-09-20

Five days after Jev opened, a PostgreSQL project in PL/Python had 243 stars, two DuckDB extensions in C++ were public, and a data frame wrapper existed. Nothing exists for SQLite, and nothing exists for PostgreSQL in Rust. None of them records and replays, keeps answers on disk between sessions, or reports which model and request produced an answer. The planning page has the table.

## One finding the builders should hear now

Every one of those projects packs many rows into one request. That is where their speed comes from: one reports 2,000 rows in 3.5 seconds for 1.2 US cents. The same project measured that accuracy falls when more than 20 to 25 rows share a request. ThinkThen sends one request per record, and `specification/annotate.md` says records never share a request. Inside the vendor's documented limit that is about 980 short rows a minute.

Somebody will put those two numbers side by side in public. `2026-09-20-live-probe-findings-packing-tagging-status-and-cost.md` told the builders not to glue records together. That advice stands until a measurement says otherwise. The measurement is running now under Ian's standing go-ahead: the same 1,000 SMS messages at 5, 10, 20, and 40 rows a request, a stability arm that deals the rows into different groups, and BoolQ at 5 and 10. Its findings are in `2026-09-20-packing-rows-into-one-request-measured.md`: ten rows a request holds the totals, 40 destroys recall, and at any width a row's answer depends on its neighbors. If packing a small number of rows holds its accuracy, it is an engine option for an ADR, off by default, and it would serve `filter` in the command as much as any database.

## What Ian can overturn

The function list, the order of the three, and whether row packing is ever offered. His own ruling fixes Rust, the three databases, and that the launch does not wait.
