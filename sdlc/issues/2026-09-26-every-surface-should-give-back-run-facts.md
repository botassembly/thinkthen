# Every surface should give back what Jev tells us about each run

Status: Sent by Ian to the main builder on 2026-09-26. Review findings and Ian's rulings of 2026-09-26 applied. Ticket 0151 settled ask 5 and the README part of ask 6 on 2026-09-26. Ticket 0170 settles the command total in gap 5 with opt-in `--facts`; 0212 and 0230 added Rust and C library facts, and 0214 adds reviewed Python Call values, facts, details and completion receipts at integrated source `dfc1b5fb`. Other libraries and SQL remain open. The site part of ask 6 is `2026-09-26-site-run-facts-after-0151.md`; direct site Python result consumers also need owner migration. Asks 1 to 4 remain open across all surfaces.

Filed 2026-09-26 from an audit of ThinkThen main at `cc51986b`, read-only, with no paid calls.

Ian's expectation: the command gives the full run facts when asked, and every library gives them on every call. The facts are input tokens, output tokens, cost, speed, request counts, and the full detail: probabilities, request IDs or digests, and model.

Ian ruled on 2026-09-26: library results carry `facts` on every call, with no setting and no second call. `--facts` controls only what the command line prints. Ian can overturn this ruling.

## What Jev sends back

The response body has three top-level fields and no others. A tally over 2,545 saved response bodies found only `model`, `answers`, and `usage`. The bodies came from the repo's recordings, experiment 265, and experiment 266.

- `model`: the answering version, such as `jev-1.13.0` (`specification/backends.md:111`, `:130`).
- `usage`: `input_tokens` and `output_tokens`, both whole numbers, in every body seen (`backends.md:111`).
- `answers.qN`:
  - `type`, and the probability: `noul` on a yes/no answer, or `probabilities` on a choice or score answer.
  - `confidence` on choice and score answers (`backends.md:119-124`).
  - The derived fields `choice`, `score`, and `legend`.

The adapter keeps every body field except the derived three, which it recomputes on purpose (`backends.md:134`). No body field is lost.

The body carries no timing, no request ID, and no price. Jev puts its request ID and server time in the headers `x-typesafe-request-id` and `x-envoy-upstream-service-time`. A recording never keeps headers (`specification/recording.md:60`). The open issue `2026-09-23-record-the-backends-own-time-for-each-call.md` already covers them.

## What each surface gave back at filing

The table and gap list below are the `cc51986b` audit snapshot, not a current support matrix. The dated C progress and current-source refresh below supersede its negative carrier claims; the original asks remain in force except for ADR 0101's explicit old-C-symbol compatibility exception.

"Details" means the full `thinkthen.result/1` line (`specification/result.md:26-38`, `:96-107`). That line holds:

- `meta.model`, `meta.usage`, `meta.requests_sent`, `meta.cached`, and `meta.requests`, the recording digests.
- `meta.question_sha256`, `meta.url`, and `meta.failed_questions`.
- Every probability, plus `confidence` when Jev sends it.

"Counters" means the process's running totals of `requests_sent`, `cache_answers`, `input_tokens`, and `output_tokens`. The Rust library defines them at `crates/thinkthen/src/public/results.rs:121-168` and returns them from `engine.rs:140`.

| Surface | Input / output tokens | Cost | Speed or latency | Request counts | Probabilities and confidence | Request ID or digest | Model |
|---|---|---|---|---|---|---|---|
| Command, default output | No. Bare value only (`result.md:9-24`) | No | No | No | No | No | No |
| Command, `--details` | Yes, per row as `meta.usage`. Annotate and recognize sum it over their requests (`result.md:145`; `specification/recognize.md:45`) | No | No | Yes: `requests_sent` per row, plus `cached` | Yes. Every probability, plus `confidence` on choice and score (`result.md:46-48`) | Our recording digests in `meta.requests`. Not Jev's request ID | Yes, as Jev reported it |
| Command, `status` | Yes, as monthly and all-time totals for the command only (`crates/thinkthen/src/cli/status.rs:61-66`; `recording.md:20-22`) | No. The files hold "no price" (`recording.md:22`) | No | Yes: requests sent and cache answers | No | No | No |
| Rust library | Only through `Engine::details()`, which covers decide, choose, tag, and score for one text (`engine.rs:272-300`), or through the counters | No | No | `Details::requests_sent()` and the counters | Via `Details::probabilities()`. No `confidence` accessor, but `to_json()` carries it (`results.rs:172-240`) | `Details::requests()` gives digests | `Details::model()` |
| Polars (Rust feature) | No. Series and frame calls return values only (`crates/thinkthen/src/public/frame.rs:16-98`). No detail columns | No | No | Engine counters only | No | No | No |
| Python and its frames | Only via `details()`, which covers decide, choose, score, and tag for one text and refuses a column (`libraries/python/thinkthen/__init__.py:269-272`; `src/frame.rs:114-115`), and via `usage()` (`__init__.py:369-371`) | No | No | Via `details()` and `usage()` | Full line in `details()`. `rank` and `find` also carry a probability (`__init__.py:278-295`) | Via `details()` | Via `details()` |
| TypeScript | Same as Python: `details()` for one text and `usage()` (`libraries/typescript/index.d.ts:230`, `:233`, `:190-196`) | No | No | Via `details()` and `usage()` | Full line at run time. The declared `Details` type leaves out `confidence` (`index.d.ts:74-98`) | Via `details()` | Via `details()` |
| Ruby | Same: `details` for one text and `usage` (`libraries/ruby/lib/thinkthen.rb:161-165`, `:213-216`) | No | No | Via `details` and `usage` | Full line. `decide_many_with_probabilities` and `score_with_level` add some (`thinkthen.rb:115-117`, `:150-153`) | Via `details` | Via `details` |
| R | Same: `tt_details()` for one text and `tt_usage()` (`libraries/r/thinkthen/R/thinkthen.R:342-349`) | No | No | Via `tt_details()` and `tt_usage()` | Full line. `tt_rank` and `tt_find` carry a probability (`thinkthen.R:202-215`) | Via `tt_details()` | Via `tt_details()` |
| C | Via `"details": true` on the JSON door for decide, choose, score, and tag, and via `{"usage": true}` (`libraries/c/include/thinkthen.h:231-249`; `DESIGN.md:37-56`) | No | No | Via details and usage | Full line. `thinkthen_decide` fills the yes probability (`thinkthen.h:106-113`) | Via details | Via details |
| DuckDB | **No, even in details.** `thinkthen_details` returns a struct of yes probability, answer, value, nearest, model, question digest, `requests_sent`, and `cached` (`databases/duckdb/src/scalars/calls.rs:113-140`). `thinkthen_usage()` gives process totals (`README.md:22`) | No | No | Via `thinkthen_details` and `thinkthen_usage()` | Yes probability only. No choice, score, or tag probabilities, and no confidence | **No digests** | Yes |
| PostgreSQL | Via `thinkthen_details()` as jsonb for one text, and via `thinkthen_usage()` per backend process (`databases/postgresql/src/lib.rs:162-182`) | No | No | Via details and usage | Full line | Via details | Via details |
| SQLite | Via `thinkthen_details()` for one text, and via `thinkthen_usage()` (`databases/sqlite/src/scalars.rs:155-220`) | No | No | Via details and usage | Full line | Via details | Via details |
| Raw HTTP, as the docs show it | The docs have no curl example for the endpoint. The only wire example is a JSON body in `backends.md:103-112`, which shows `usage` | No | Not documented | Not applicable | Shown in the body | Not documented. The docs never mention the request-ID header | Shown in the body |

## The gaps at filing

1. **No surface gives latency.** No code records wall time, server time, or Jev's request ID. The only clocks in the engine and bindings serve deadlines and polling (`crates/thinkthen/src/engine/mod.rs:214-220`). The open issue from 2026-09-23 covers this. Ian ruled that day to keep the times. Nothing has landed.

2. **The libraries hide everything by default, and bulk calls have no details at all.** Every library verb returns a bare value. Run facts need a separate `details` call.
   - That call covers only decide, choose, tag, and score, over one text (`engine.rs:272-300`).
   - No library can get per-row details from filter, rank, find, annotate, recognize, relate, `decide_many`, a frame, or a vectorized SQL call. The command gives `--details` on all ten verbs.
   - A library user who wants tokens per call must make a second call. With the cache on, that second call is free. With the cache off, it is paid.
   - ADR 0017 set this shape on purpose. Section 6 item 1 lists `details` as a separate function, and its amendment keeps `details` and usage counters as supporting forms (`sdlc/planning/adr/0017-libraries-over-one-bound-core.md:95`, `:106`, `:211`). Ian's expectation contradicts that ADR.

3. **DuckDB's details are smaller than every other surface's.** The DuckDB struct drops:
   - input and output tokens,
   - the request digests and URL,
   - probabilities for choose, score, and tag,
   - `confidence`.

   The other surfaces return the full line.

4. **The typed views drop `confidence`.** The TypeScript `Details` type has no `confidence` field (`index.d.ts:74-98`). The Rust `Details` struct has no `confidence` or `url` accessor (`results.rs:172-182`). The JSON carries both, so the data is there, but a typed reader cannot see it.

5. **The command's `--details` hides the tokens of records it does not print.**
   - `filter --details` prints only kept records (`specification/filter.md:23`).
   - `rank --top N` prints N records but judges all of them (`specification/rank.md:27`).
   - The spend on dropped or cut records shows only in `status`, and there only as a monthly total mixed with other runs.
   - No surface prints a total line for the run.

6. **No surface computes cost.**
   - The only cost math is the `cost` transform (`crates/thinkthen/transforms/cost.jq:1-53`). `thinkthen transform show cost` prints it, and the user runs it with `jq`.
   - The user passes the price as `--argjson usd_per_million_input`. The transform prices input tokens only and never assumes a price (`cost.jq:6-10`, `:17-19`). Its example price of 0.042 comes from `sdlc/records/0011-the-live-probe.md`.
   - The site shows fixed measured figures, not a calculation (`site/src/data/catalog.mjs:540-546`).
   - `status` stores "no price" on purpose (`recording.md:22`).
   - No price setting exists in the configuration file (`recording.md:32`).

7. **Nothing holds the surfaces to one set of run facts.** The shared conformance cases check only four details fields: `answer`, `model`, `question_sha256`, and `requests` (`conformance/cases.json`). They never check `usage`, `requests_sent`, `cached`, `confidence`, or `url`. Gap 3 and gap 4 slipped through for that reason.

8. **The docs do not say where run facts live.**
   - The Python README and the Rust README never mention `details` or `usage`.
   - No site page says what a library or SQL call returns for usage or details.
   - The site calls R "ten `tt_` functions" (`catalog.mjs:362`). R has 14, including `tt_details` and `tt_usage`.
   - No page shows a raw curl call and its reply.

9. **`status` sees only the command's spend.** This is the known issue `2026-09-25-status-sees-only-command-spend-and-the-sql-total-has-three-leaks.md`. It is not refiled here. Gap 2 makes it worse: a library user has neither per-call facts by default nor a lasting total.

## What is asked

Outcomes, not a design. Each item names what Ian can overturn.

1. **Every library call gives back its run facts without a second call, on every verb.** Ian ruled this on 2026-09-26. Library results carry `facts` on every call, with no setting and no second call. The run facts are the batching design's fields: `records`, `requests_sent`, `cache_answers`, `input_tokens`, `output_tokens`, `seconds` and `model`. The fuller detail, request digests and every probability with `confidence`, reaches every verb, bulk verbs and frames included. This amends ADR 0017 section 6. The amendment needs an ADR, because it changes the return shape on ten surfaces. The trade-off: a bare value keeps the `if tt.decide(...)` form short, and a result object makes every call carry more. The ADR picks the spelling. It does not make `facts` optional. SQL per-call facts stay deferred below.
2. **The command reports every request a run sent when asked.** That includes records `filter` drops and records `rank --top` leaves off. A run can report its total tokens and requests sent without `status`. Ian ruled that a finished run stays silent on standard error by default, as `specification/records.md` line 109 says. So the totals print only under `--facts`, as "Agreement with the batching design" below fixes. `--facts` controls only what the command line prints. It changes nothing a library returns. `--details` keeps its per-row lines on every verb.
3. **Every surface reports speed.** For each request, report the wall time, the server time, and Jev's request ID, as the 2026-09-23 issue and Ian's ruling ask. This issue asks only that the result reach every surface, not only `--details`.
4. **A run can report its cost.** Cost is tokens times a price the user supplies. The price comes from configuration, never a guess. With no price set, the run shows tokens and no money. A decision records the price source and whether output tokens are priced. Today's transform prices input tokens only. The vendor's output price is not recorded in the repo.
5. **One set of run facts, one set of names, on every surface.** DuckDB's details match the others. The typed views in TypeScript and Rust expose `confidence` and `url`. The conformance cases check `usage`, `requests_sent`, `cached`, and `confidence`, so drift fails a gate.
6. **Each surface's README and the site say where run facts come from.** The site shows one raw HTTP exchange with its `usage`.

Any ticket that comes from this issue and adds or changes a setting updates that setting's row in `specification/settings.md` in the same commit. Batching ticket C1 creates that page.

## Agreement with the batching design

`2026-09-26-batching-design.md`, "Output and run facts", fixes the run facts' shape and names. This issue and that design agree on these points. Ian can overturn each.

- **One name.** The name is `facts` on every surface.
- **Libraries.** Library results carry `facts` on every call, with no setting and no second call. A result over many records carries `.facts`, or the surface's own spelling. So does a single call.
- **Command.** `--facts` controls only what the command line prints. The command takes `--facts` and writes one `thinkthen.run/1` line to standard error at the end of the run. Without it, a finished run prints nothing there.
- **One set of names.** The fields are `records`, `requests_sent`, `cache_answers`, `input_tokens`, `output_tokens`, `seconds` and `model`. They match `thinkthen status`, the library counters, and the per-row `meta.requests_sent` and `meta.cached`.
- **Tokens.** `input_tokens` and `output_tokens` sum live replies, as `status` does. A field the backend did not report stays absent. It is never written as 0.
- **Time.** `seconds` is wall time. It never enters a byte-identity check.
- **Single calls.** A single call's facts have the same fields with `records` of 1. The ADR from item 1 picks how a bare-value call carries them. Every call carries them.
- **SQL.** Per-call facts in SQL are deferred below. SQL keeps `thinkthen_details` and `thinkthen_usage()`.

## Deferred

- Persisting library spend for `status` stays with the 2026-09-25 issue.
- Keeping header fields in recordings stays with the 2026-09-23 issue, which keeps them beside the entry and not inside it.
- **SQL run facts.** A per-call or per-query facts shape for DuckDB, PostgreSQL and SQLite waits for its own design. A scalar returns one value a row, so the facts need another function or a details column. Gap 3's repair of DuckDB's details struct is not deferred.

## Factual C boundary refresh, 2026-09-28

At main `e8789b76`, accepted ticket 0230 gives successful C JSON-door calls owned `value` and `facts` together, and a started failure's facts through the borrowed engine/thread error accessor. The original table and C run-facts settings cell above predate that landing. They must not be read as a claim that the JSON door still lacks facts. The eight typed C exports are a different carrier: plain and `_opts` decide, decide-many, recognize and relate still return a bare judgment or JSON value. Their Rust calls capture final facts, but the FFI path discards them on success. No accepted C ABI exception to the original every-call wording was found in ADR 0089 or ticket 0230. A future reviewed design must preserve existing compiled callers and either supply call-owned facts through an additive form or explicitly rule on a success accessor and the legacy names. The separate latency, request-ID, user-priced cost and SQL criteria above remain open. See `sdlc/records/2026-09-28-c-facts-and-process-replay-preflight.md` for exact paths, options and proof limits.

## Typed C progress, 2026-09-28

Ticket 0255 passed fresh High code/API review at `41114d20` and landed eight typed `*_with_facts` forms. They return owned facts from the same asking call and retain the existing C layout and symbols. ADR 0101 explicitly preserves the eight old bare symbols as compatibility forms; they do not gain facts. The installed Linux C driver, exact exports and failure/ownership checks passed. Other target packages and this issue's remaining surfaces, cost, server timing and request-ID criteria stay open.

## Factual preparation after typed C facts, 2026-09-28

At main `23371cc9`, `public/results/call.rs` fixes final `Call<T>::facts` at completion; `public/results/observation.rs` supplies ordered question/row detail, and `public/frame.rs` wraps Polars values. Python `src/result.rs`/frame stop, TypeScript `src/door/result.rs`, Ruby `src/call.rs`/`ffi/result.rs`, R `rust/src/calls/account.rs`, and C's JSON and eight additive typed doors convert facts into host carriers. A pre-account refusal has no invented facts; a started failure can carry a final receipt, and absent provider usage stays absent. `cli/facts.rs` prints opt-in whole-command totals, including work omitted from printed filter/rank rows. `conformance/cases.json` already checks usage, requests sent, cached and confidence; DuckDB's current `thinkthen_details` uses full result JSON. These source paths and the 0255 Linux installed driver do not qualify every target package, native Intel or macOS 15, and do not close full-detail/failure parity without each host's exact boundary proof. The original eight C typed names remain bare by ADR 0101, while the eight new names return owned facts. Wall `seconds` is a call/run interval, not the requested per-request backend/server time. Vendor request IDs, caller-priced money, SQL per-call facts (explicitly deferred above) and the public wording/raw exchange remain distinct. Register 61's `meta.usage` field-name criterion is also still open even though its even-share algorithm and prose are settled. See `sdlc/records/2026-09-28-accounting-after-c-facts.md` for the bounded next batches and proof limits.

## Go and PHP typed calls, 2026-09-29

Ticket 0277 passed fresh High code review at `0d714cfee`. Each wrapper’s four typed methods now return the former value with owned facts from one existing C facts call. Focused source and copied installed-source consumers pass, including absent usage, empty bulk, Go calls forced to overlap, and typed failure facts retained across recovery and close. JSON calls and the old C ABI remain unchanged. These are wrapper source and selected local consumer receipts; clean release archives and actual runners remain separate. The other surface, full-detail, cost, vendor timing and request-ID criteria keep this umbrella open.

## C# and JVM typed calls, 2026-09-29

Ticket 0280 passed fresh High code review at `3b74fa121`. The four typed C# and JVM routes now return their former values with independently owned facts through the existing C ABI. Kotlin and Scala forward the same carrier. The focused source and copied installed consumers preserve exact request inventories, typed failure lifetime and controlled overlap. An identical three-record packed replay reports one cached response and zero sends; held calls prove elapsed facts. The JSON routes and prior host lifetime limits remain. Other wrappers, richer details, cost, per-request vendor timing, request IDs and clean release/runner qualification remain open.
