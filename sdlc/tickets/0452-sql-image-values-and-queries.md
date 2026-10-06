# 0452: Carry typed image values through SQL

Status: ready. Planning only; implementation follows accepted ticket review.

Milestone: 0.2
Owner: builder.
Ticket review: ACCEPT, 2026-10-06; fresh read-only review. Conflicting image deferrals corrected where found.

## Outcome

DuckDB, SQLite and PostgreSQL accept explicit image values and ordered multi-image questions for decide/choose/score with complete results, facts and zero-send replay.

## Evidence

- Starts from: Main 399c6c7c7 and the existing SDK plan. PM message `2026-10-06-pm-vision-ships-in-0-2-and-the-sdk-stays-one-endpoint-while-the-proxy-owns-the.md`, ask 1; experiment 0034 design, recorded spike at 8ec4fbffc0dbf8ceaa0fea5607749ff2f683a2a0 and subsequent OpenRouter controls. 0036 reports are pending.
- Keeps: Existing text behavior, typed SDK parity, six errors, cancellation, secrecy, spend limits and zero-send strict replay. Core remains free of I/O; the one Rust engine and native file reader remain shared.
- Changes: Add reviewed image constructors and judgment overloads using the shared native engine. Proposed thinkthen_image(bytes,mime) plus ordered image-list input: validated DuckDB struct, PostgreSQL composite and SQLite tagged value revalidated after storage. Ordinary BLOB/bytea/text/path values retain current meaning.
- Proof: Public SQL consumers store and reload two-image values, preserve order/duplicates, execute named scalar/image-file routes and compare shared answers/details/facts. Test malformed tags/media, SQL NULL/empty values, implicit-BLOB refusal, text-only function refusals and zero-send limits/replay.
- Defers: Proxy business logic and screens, other modalities and unmeasured function combinations.

## Dependencies and ownership

0447 native input/reader and 0448 admission first. Coordinate 0434 input grammar and 0435 facts in the same SQL folders; no competing parsers. DuckDB/SQLite image readers preserve host file permissions. PostgreSQL uses the 0434 client-reader/keyed-table workaround; it adds no server image-path reader.

## Design notes

Record exact SQL public signatures and null semantics before code. Image-file results preserve file identity and absent text positions. Never resolve an arbitrary SQL path or URL as an image. SQL fact objects come from the same call, never a second judgment.

## Family build contract (2026-10-06)

Risk: High: counted compressed binary values cross DuckDB FFI, stored values
must be revalidated, and evidence must remain confidential on every failure.
Use the existing native pixel validator, reader, engine, cache and scheduler;
no new dependencies or PostgreSQL server image reader. Ian can overturn these
additive spellings. This joins 0434 in one build and one fresh whole review.

Public signatures, fixed before implementation:

- DuckDB `thinkthen_image(bytes BLOB, mime VARCHAR)` returns
  `STRUCT(media VARCHAR, data BLOB, file VARCHAR)`; PostgreSQL's same `(bytea,text)`
  constructor returns composite `thinkthen_image_value(media text,data bytea,file text)`.
- SQLite `thinkthen_image(bytes BLOB, mime TEXT)` returns a versioned tagged
  BLOB, not transient SQLite subtype metadata. `thinkthen_images(image, ...)`
  constructs a persistent ordered tagged BLOB for 1–8 images.
- All three expose `thinkthen_decide_images`, `thinkthen_choose_images`,
  `thinkthen_score_images`, `thinkthen_details_images` with arguments
  `(question, images, text := NULL, settings := NULL)`. DuckDB images is a
  list of its image structs; PostgreSQL is `thinkthen_image_value[]`; SQLite
  is the ordered tagged BLOB. SQLite accepts the trailing arguments by arity.
  Choices/rubrics use existing question JSON or settings, never a new parser.
- DuckDB/SQLite `thinkthen_image_file(path)` explicitly reads one image using
  the native reader and returns the host image value. PostgreSQL clients read
  files in a native SDK and insert bytes/media with retained file identities.
  Paths and URLs are never implicit image inputs.

NULL constructor operands return NULL. NULL question or image collection
returns NULL without sends. Ancillary NULL text means absent text; NULL
settings means defaults. NULL members of a non-NULL collection, empty
collections, malformed tags/media/pixels and over-limit inputs are Usage,
before transport. Ordinary text/BLOB/bytea APIs keep their existing meaning.
Unsure decide/choose stays SQL NULL; errors stay errors. Stored values are
validated again at judgment, preserving order and duplicates.

Remaining dependencies: lane 0 owns the native complete-result/facts APIs
(0442/0445/0450) and 0300 exact caller prices. 0435 must adopt those APIs for
isolated invocation facts and started-failure facts; these image calls expose
the landed native details only until then. 0417 requires the shared complete rank-set
observation/carrier API at the SQL boundary. `Engine::rank_set_with`, `RankSet`
and `SetRanked` are already landed; 0417 still needs their SQL adapter with
member observations, selecting names, retained host keys and combined facts. No full parity claim is made
while those dependent public calls and checks are absent. 0456's named input
design remains pending; no schema or `@NAME` semantics are invented here.

Image values retain an optional `file` source name (NULL for bytes). SQLite
exposes `thinkthen_image_file_name(image)`; DuckDB/composite callers read the
`file` field. This metadata is excluded from native evidence and identity.
No line/span fields are invented. Clients join stored file identities beside
judgments; complete located result adoption awaits the shared result API.

Build note: the baseline already has `Call::facts`, `Error::facts`,
`EngineBuilder::prices_usd_per_million` and `Engine::rank_set_with`. The pending
0435 adoption needs finalized call_id/answer identity and complete observations
from lane 0, plus the reviewed SQL price and started-failure carrier shapes.
The builder does not replace these APIs or derive facts from usage snapshots.

Test incident: an initial image test explicitly selected the built-in Liquid
backend over a captured loopback URL, causing three rejected 401 requests with
a fake key to the built-in endpoint. No real key or paid authorization was
used. The tests now capture backend and the explicit loopback URL in the same
environment tier; no SQL backend setter can replace that address. This is
recorded for the whole High review, not represented as an offline test pass.
