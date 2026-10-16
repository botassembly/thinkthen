# 0597 — consistent-examples-and-batching-guide

Status: OPEN.

Milestone: 0.2.1

## Outcome

Every library, database and data-frame page shows the same short example in Ian's style. Every page teaches the batched way to judge many records on that surface. One site page explains batching and caching as the way to cut cost and time, with measured numbers.

## Evidence

- Ian, 2026-10-15, on the 0.2 public text: "This file needs a lot of work. We need to really clean up these examples, simplify them, and make them easier to read." His rules:
  - The fewest lines that carry the information.
  - Lines under 60 characters where possible.
  - Each assertion sits right beside its call and is explicit: `is True`, `is False`, `is None`, `== "shipping"`, or the host's equivalent.
  - The word "question" is fine, as in `question = "Does the customer ask for a refund?"`.
  - No engine object unless the example needs a setting.
- Ian, the same day: "We're not consistent in the refund probability example. We should put some probabilities in some of the examples." Zig asserts `probability == 0.99`; Python asserts none.
- Ian, the same day: "Make sure that we're maximizing the explanation of batching and really teaching people how to do batching both in programming languages and in the databases and data frames." He also asked how a table gains a judged column, as a view or by inserting 1,000 or 10,000 rows cheaply.
- A rewrite of every 0.2 page's example to these rules exists as a draft. The PM attaches it to the brief. It was checked against the 0.2 code, not replayed. Its findings:
  - Only Python, R and Ada have a default engine.
  - C and COBOL still need about 100 lines.
  - PHP and Dart asserts are off by default, and their pages do not say so.
  - The current COBOL example compares the whole 8,192-byte answer field and can fail.
  - Changed request text misses the saved answers on several pages.
- How each SQL host batches, read from the code on `ticket/0543-final`:
  - DuckDB's plain `thinkthen_decide(question, body)` receives a chunk of rows and judges it as one grouped call (`databases/duckdb/bridge/src/ffi/scalar/ffi.rs`).
  - SQLite and PostgreSQL scalars judge one row per call. Their `_many` table functions take one keyed object built with `json_group_object` or `jsonb_object_agg`, pack it, and join back on `key` (`databases/sqlite/README.md`, `databases/postgresql/README.md`).
  - The DuckDB page's only example uses the longer `_details` form, and no page shows adding a judged column to a table.
- Existing site pages: `site/src/pages/learn/caching.astro`, `learn/overhead.astro` and `learn/functional.astro`. The functional page still says "Bind a ThinkThen question once" though 0.2 removed binding.
- Earlier measurements to re-measure, not quote:
  - 306 titles cost 88,933 input tokens unbatched and 11,468 packed (ADR 0055's experiment).
  - Polars streaming over 200,000 rows made 16 engine calls of 12,500 rows (ADR 0107).

## Decisions

- PM decision, 2026-10-15: every page uses the same three cases. A refund message is true, a thanks message is false, and a parcel message chooses `shipping`. Each page adds one probability assertion after the first decide, written as `> 0.9` rather than an exact value.
- PM decision, 2026-10-15: each library and data-frame page adds one batched example: a list or column in one call, never a loop of single calls. Each database page's main example adds a judged column to a table. DuckDB uses the plain call in `CREATE TABLE ... AS SELECT`. SQLite and PostgreSQL use the `_many` gather-and-join form, with one sentence saying a plain scalar sends once per row.
- PM decision, 2026-10-15: one new site page, "Batching and cost", covers four things:
  - packing many records into few requests on every surface;
  - the cache, which never pays twice for the same question over the same text;
  - table or view, recommending a table filled once for large sets;
  - one measured before-and-after per surface family.
  The surface family pages link to it.
- PM decision, 2026-10-15: examples must pass saved-answer replay. Record new saved answers where the request text changed. A paid recording needs Ian's go and a token cap through `sdlc/scripts/live`.

## Scope

- Changes: `libraries/*/README.md`, `libraries/*/README-PUBLIC.md`, `libraries/*/examples.json`, `databases/*/README.md`, `databases/*/README-PUBLIC.md`, `site/src/pages/learn/` and its data files, and the saved-answer fixtures the examples replay. The owner narrows this before work starts.
- Owner: the 0.2.1 docs work. Under the milestones plan the docs team owns 0.2.1, not the 0.2 team.
- Ian, 2026-10-16: these documentation updates need no release bump. They update the website and the GitHub repository on their own, before 0.3 work begins in earnest. Registry package pages keep the 0.2 text until the next package release.
- Keeps: the approved prose, except sentences that describe a changed example; every API as 0.2 ships it.

## Proof

The routine suite's example replay passes for every changed page. A line-length check reports each example line over 60 characters, and each one left over has a reason. The site build passes with the new page linked from each surface family page.

## Defers

- Shorter C and COBOL examples need API changes, which go to a later ticket.
- Results that read as their answers are ticket 0595.
- The batching and cost presentation and its experiments are Ian's to schedule.
