# SQL/frame redesign settings and example corpus

Independent static acceptance data for tickets 0283–0298, transcribed from experiment 2038's `drafts/corpus-and-messages.md` under accepted ADRs 0105/0107. This record is in the repository so acceptance does not depend on the external experiment. It specifies intended fixtures; no runtime execution is claimed. The literal misspelling in I6 is intentional unknown-key input.

## The `thinkthen.settings/1` corpus (T1)

Valid objects and their wire effect (question keys enter the request bytes and
digest; call keys as marked):

| # | Object | Expectation |
| --- | --- | --- |
| V1 | `{}` | absent; identical request bytes to no-settings |
| V2 | `{"threshold": 0.7}` | decide cut 0.7; not in the digest |
| V3 | `{"threshold": "0.3:0.7"}` | decide band; NULL inside the band |
| V4 | `{"true": "A yes means a refund request"}` | rides the question bytes |
| V5 | `{"options": ["billing", "shipping"]}` | members, caller order |
| V6 | `{"options": {"billing": "Payments", "shipping": "Parcels"}}` | members with descriptions |
| V7 | `{"model": "jev-1.13.0"}` | per-request model |
| V8 | `{"context": "One shared reference text"}` | nonblank; `meta.context_sha256` set |
| V9 | `{"batch": "max"}` and `{"batch": 64}` | both valid |
| V10 | `{"deadline_ms": 5000}` | call key; `-1` none; `0` spent, sends nothing |
| V11 | `{"none": true}` | find only |

Invalid objects, each one intended failure:

| # | Object | Refusal |
| --- | --- | --- |
| I1 | `{"options": [...], "labels": [...]}` both given | usage |
| I2 | `{"threshold": 1.5}` | usage (above 1) |
| I3 | `{"threshold": "0.7:0.3"}` | usage (LOW ≥ HIGH) |
| I4 | `{"context": "  "}` | usage (blank) |
| I5 | `{"deadline_ms": 1.5}` | usage (not whole) |
| I6 | `{"unknwon": 1}` | usage (unknown key) |
| I7 | `{"none": true}` on decide | usage (key the verb cannot take) |
| I8 | settings repeating a question-file field | usage |
| I9 | members list beside settings members | usage (NULL members accepted) |
| I10 | not JSON, or not an object | usage |

PostgreSQL adds: I11 `threshold => '0.7', settings => '{"threshold":0.5}'`
(named parameter and settings duplicate) — usage.

## The removal messages (exact wording, T1–T6 tests)

| Old call | Message |
| --- | --- |
| `thinkthen_warm(...)` (all three) | `thinkthen usage: thinkthen_warm was removed; pack records with thinkthen_decide_many` |
| SQLite 3rd-arg deadline / 4th-arg context | `thinkthen usage: the deadline and context moved into the settings object; pass '{"deadline_ms": …, "context": …}'` |
| DuckDB trailing deadline/context slots | same sentence as above |
| PostgreSQL trailing context overload | `thinkthen usage: the context argument moved into the settings object or the context named parameter` |
| `find` positional `none`/`deadline_ms` | `thinkthen usage: find's none and deadline moved into the settings object` |
| SQLite `thinkthen_recognize_document` | `thinkthen usage: thinkthen_recognize_document was renamed thinkthen_relations` |
| Any of SQLite's twelve setting functions | `thinkthen usage: thinkthen_<name> was replaced by thinkthen_configure` |
| SQLite `thinkthen_relate('table', 'id', …)` | `thinkthen usage: relate takes a query and rules; pass 'SELECT id, name, kind FROM …'` |
| Python `tt.decide_many` (and siblings) | `thinkthen: decide_many was removed; apply the judge to a list: tt.decide(q)(rows)` |
| Python `true_=`/`false_=` | `thinkthen: true_ and false_ were renamed true and false` |
| Python `deadline=` (seconds) | `thinkthen: deadline was renamed deadline_ms, in milliseconds` |
| Python `descriptions=` on choose/score/tag | `thinkthen: descriptions moved into the options, labels or levels map` |
| R `deadline =` (partial match) | `thinkthen usage: deadline was renamed deadline_ms, in milliseconds` |
| `--dry-run` (asking verbs, check, find, annotate, recognize, relate) | `thinkthen: --dry-run was renamed --plan` |
| `Stream.value` | `thinkthen: a stream has no value; iterate it, or pass a list` |

## The E1–E9 inputs (T8 corpus)

Fixed inputs, all replayed, no key, no network:

- **Songs table** (E1, E4 uses `messages`, E2/E3/E5 use `tickets`): the
  Beatles Bench `data/songs.tsv` slice at `ed8ffe49` — take six songs
  **with id gaps**, e.g. ids 1, 2, 5, 9, 14, 17: `Here Comes the Sun`,
  `Yellow Submarine`, `Octopus's Garden`, `Penny Lane`,
  `A Day in the Life`, `Hey Jude`. The gaps prove the keyed join.
- **E1**: the Abbey Road question with `{"threshold": "0.3:0.7"}`; expected:
  one send on every surface; `A Day in the Life` NULL (not sure) with the
  band; order by probability descending.
- **E2**: `tickets.body` with the refund question; packed filter; one send.
- **E3**: the team question with the descriptions map; `(key, value,
  probability)`; one send.
- **E4**: `messages.message` with levels calm/frustrated/angry;
  `(key, value)` only; score probability refusal is a separate corpus row.
- **E5**: labels refund/shipping; `(key, value)`.
- **E6**: five passages, `{"none": true}`; one request for the set.
- **E7** (F2): a 1,000-line generator over the tickets bodies; `pipe(...,
  tt.filter(q), take(20), list)` sends within the look-ahead bound.
- **E8** (F4): a 20,000-row parquet, 10% `lang == "en"`; the safe form judges
  2,000; the unsafe spelling's count is pinned as whatever the pinned Polars
  produces, with the page stating the hazard.
- **E9** (F6): the R `mutate` with the judge on a grouped frame; one packed
  call per group; dbplyr pinned to DuckDB with the PostgreSQL per-row count
  stated.

The fixture producer must preserve exact input bytes and checked expected answers. E1–E6's stated one-send outcomes are for their fixed small inputs and specified cache/replay setup; E7 uses an upper bound from bounded look-ahead, not a guessed 1,000-record send count. E8's safe form judges exactly 2,000 of 20,000 rows; the unsafe form records the pinned Polars observation and states its version and query shape. For every executable replay, capture the exact listener request bodies and derive counts from those bodies; neither a row count nor the parser under test is an independent request oracle. Do not duplicate the full corpus per host: 0283 owns V/I grammar, 0290 owns E1–E9, and each SQL/host ticket takes a small conversion table plus its distinctive case.

## P1: independent one-record plan fixture for each SQL host

Use `thinkthen_plan('asks for a refund', '{"7":"Refund me please."}', '{}')` on SQLite and DuckDB, and `thinkthen_plan('asks for a refund', '{"7":"Refund me please."}'::jsonb, '{}'::json)` on PostgreSQL, with the built-in model `jev-1.13.0`, no context and default batch. The keyed singleton must produce the same body as the existing command's one-text plan fixture in `spec/decide.md`, **not** an expected body obtained from the new SQL parser:

```json
{"state":"Refund me please.","model":"jev-1.13.0","questions":{"q1":{"type":"noul","instructions":"asks for a refund"}}}
```

This literal is 120 UTF-8 bytes. Independently expected summary: `records=1`, `requests=1`, `estimated_bytes=120`, `estimated_input_tokens={"lower":61,"upper":109}` with the conservative whole-token rule `floor(120 × 0.516)` and `ceil(120 × 0.908)`, `upper_bound=false`, and **zero** loopback requests accepted. The host returns its native shape: SQLite JSON text, PostgreSQL `jsonb`, DuckDB `STRUCT`. For each host, compare that shape's decoded fields to these literals and also compare the body to the recorded request bytes; one invalid settings object or duplicate conflict must refuse before any send. The prepared bodies and token rates are preview estimates, not a claim about a provider's actual token usage. The fixture becomes executable only when the three host plan functions land.
