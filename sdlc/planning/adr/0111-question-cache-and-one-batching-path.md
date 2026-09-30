# ADR 0111: One question cache and one batching path

- Status: **Accepted** by the coordinator, 2026-09-30, after two fresh design reviews. Ian can overturn each item.
- Date: 2026-09-29

This ADR builds Ian's rulings 2 to 6 and 8 of 2026-09-29 in `sdlc/planning/cleanup-2026-09-30.md`. It replaces ADR 0048 item 5 and amends the ADRs listed at the end. Ruling 7, one result schema generated from Rust, is its own ticket.

## Context

Today the cache key is the SHA-256 of the adapter name, the URL and the whole request body. One JSON file holds one exchange. Per-digest lock files coalesce concurrent misses, and a marker file binds each folder to one address. Because a batch is the unit of storage, a batch changes its key when any record in it changes. Content cuts exist only to keep batch edges stable. A run of 120 records after a run of 100 therefore resends most of the 100.

The batching work also grew separate planners and schedulers: `core/batch`, `engine/schedule.rs`, `engine/annotate_schedule`, `cli/asking/batched`, `cli/annotate/batching`, `public/batch/planned` and `public/native_batch`. Each surface packs, splits and orders on its own.

System One takes `{state, model, questions:{q1..qN}}`. Each question is `noul`, `choice` or `score`. The reply answers each `qN` and reports usage for the whole request only.

## Decision

### 1. Every record function quotes each record in its own question

`decide`, `filter`, `rank`, `choose`, `tag`, `score` and `annotate` send one fixed state for every request:

- Without `--context`, the state is the text `Each question quotes the text it asks about.`
- With `--context`, the state is the context text.

Each wire question carries its record. Its instructions are `The text is `, then the record's compact JSON, then `. `, then the question's instructions as today. `tag` expands one `noul` question per label, as today, and each expanded instruction starts with the same quote. An `annotate` group quotes the part its `on` pointer selects.

A batch of one uses the same form. The unquoted single-record request and the `{"records":[…]}` state both go away. So one record always makes the same questions, whatever else shares its request.

A question whose instructions are a JSON object or list cannot take the quote. Its record becomes the state and the question goes unquoted. Each such record then makes its own request. A context beside such a question stays a usage error, as today.

Experiment 275 found the records-list form ahead of the quoted form for `choose` and `tag` on short titles, and nobody has measured long records. Ian's ruling of 2026-09-29 takes the quoted form anyway. This ADR asks for no paid measurement.

### 2. The question key

The key is the SHA-256 of these bytes, joined by one line feed (0x0A) each:

1. The adapter name, `systemone`.
2. The resolved posting URL, as today's digest uses it.
3. The model as the body carries it: its compact JSON string, quotes included.
4. The state as encoded in the body: the compact JSON value after `"state":`.
5. One wire question as encoded in the body: the compact JSON object after `"qN":`, such as `{"type":"noul","instructions":"The text is \"Ringo\". Is this a drummer?"}`.

No part can hold a raw line feed. The URL is checked for control characters today, and compact JSON escapes line feeds. So the join is unambiguous. The key prints as 64 lowercase hex characters.

The encoder writes each state and each question once. It builds the body by joining those same bytes: `{"state":S,"model":M,"questions":{"q1":Q1,…}}`. The key therefore hashes exactly the bytes sent, and the byte count of a request is a sum.

The key leaves out the question name `qN`, the other questions in the request, the batch setting, the profile, the threshold and every header. The vendor shows no names to the model. Ruling 3 accepts that neighbours may move an answer. A threshold is applied after the answer, so retuning a threshold reuses every stored answer. Nothing else in the body can change an answer. Equal keys in one call are asked once.

### 3. The store

**One schema, two containers.** Live caches use one SQLite file, `thinkthen.sqlite`, inside the folder that `--cache`, `--record`, `THINKTHEN_CACHE` or the platform default names. Committed fixtures use `thinkthen.jsonl` in the same folder, with the same columns. The option spellings and folder rules stay.

Schema, `PRAGMA user_version = 1`, created with `auto_vacuum = INCREMENTAL` so a prune can shrink the file:

```sql
CREATE TABLE states (
  id     INTEGER PRIMARY KEY,
  sha256 BLOB NOT NULL UNIQUE,  -- SHA-256 of the state bytes
  state  TEXT NOT NULL          -- the state as sent
);

CREATE TABLE answers (
  id            INTEGER PRIMARY KEY,
  key           BLOB NOT NULL UNIQUE,  -- the 32-byte question key
  url           TEXT NOT NULL,
  model         TEXT NOT NULL,         -- the model the request named
  state         INTEGER NOT NULL REFERENCES states(id),
  question      TEXT NOT NULL,         -- the wire question as sent
  answer        TEXT NOT NULL,         -- the wire answer, as received
  answered_by   TEXT NOT NULL,         -- the model the reply named
  input_tokens  INTEGER,               -- this question's share; NULL when unreported
  output_tokens INTEGER,
  taken_at      INTEGER NOT NULL,      -- Unix seconds, UTC, when the reply arrived
  origin        TEXT NOT NULL          -- 'live', 'converted' or 'quoted'
);
```

A state is stored once, so a relate text asked 400 times costs one copy. The stored parts let a later key version re-key the store without asking again. A row whose answer no longer decodes counts as a miss under a cache and as a local failure under `--replay`. Ordinary tables with unique blob keys suit these long text rows better than `WITHOUT ROWID`.

A question's token share is its request's reported usage split evenly over the request's questions, with any remainder to the earliest. A cached row reports the stored share with `cached: true`, as a replayed row reports its recorded usage today. Process usage totals still count only live replies.

**Fixtures.** `thinkthen.jsonl` holds one JSON object per line: first every state as `{"sha256":…,"state":…}`, sorted by digest, then every answer with the `answers` columns, sorted by key, with `state` naming the state digest. The bytes are a function of the entries, so reviews and merges read as text. `--replay DIR` loads `thinkthen.jsonl` into an in-memory SQLite database when the file exists, and otherwise opens `thinkthen.sqlite` read-only. `thinkthen cache convert DIR` writes `thinkthen.jsonl` from the union of what `DIR` holds: an existing `thinkthen.jsonl`, a `thinkthen.sqlite` and old `DIGEST.json` files. When two sources hold one key, the newer `taken_at` wins, and a tie keeps the fixture's entry. It sets every `taken_at` it cannot know to 0, so the same input always writes the same bytes. A folder holding both `thinkthen.jsonl` and `thinkthen.sqlite` is ambiguous, and `--replay` exits 5 and says to run `cache convert`. `policy.py` refuses a committed `thinkthen.sqlite`. So a new live recording goes to a scratch folder with `--record`, and `cache convert` merges it into the fixture.

**Dependency.** `rusqlite` with default features off and `bundled` on. Bundled SQLite gives every host the same SQLite with no system library skew, including wheels, the C door and the SQL extensions. It costs one C compile of SQLite and about 1 MB of binary. Pure Rust stores such as `redb` and `sled` lock their file to one process, so parallel shells and PostgreSQL backends could not share a cache. The SQLite extension will hold two SQLite copies in one process. That is safe because they never open the same file. The library must export no `sqlite3_` symbol, and slice 2 checks that. The 2026-09-30 amendment below corrects both sentences: the extension runs the store on the host's SQLite, and the static libraries export SQLite's symbols.

**Locking.** The file uses the rollback journal (`journal_mode=DELETE`) with `synchronous=FULL` and `secure_delete=ON`. The rules:

- Every write opens with `BEGIN IMMEDIATE`, so a writer takes its lock before it reads.
- A lookup finishes or resets its statement before any send, so no read lock outlives a lookup.
- Every connection installs one busy handler, for lookups and writes alike. It waits at most 30 seconds in all, checking the call's cancel token between waits. A longer wait is a storage failure with the fixed exit-5 sentence.
- A read-only replay that meets a leftover hot journal cannot roll it back. It exits 5 and says to open the folder once with write access.
- Network filesystems are unsupported, because SQLite's locks are not reliable there. The page says so.

The rollback journal leaves no side files, so a read-only open writes nothing. Secure delete zeroes removed rows. Each reply is one small write transaction, so the single writer costs nothing measurable. The folder is mode `0700` when the tool creates it. The file is created with mode `0600` before SQLite opens it, and SQLite gives its journal the same mode.

**Modes.**

| Mode | Look up | Send | Write |
| --- | --- | --- | --- |
| `--replay DIR` | yes | nothing; a miss exits 5 | no |
| `--record DIR` | no | every question | yes, replacing |
| `--cache DIR`, both options on one folder, `THINKTHEN_CACHE`, the default cache | yes | misses | yes |
| A cache with `--refresh-cache`, or the model `jev-latest` | no | every question | yes, replacing |
| `--no-cache` | no | every question | no |

A replay miss names the missing key and says it is the SHA-256 of the adapter, address, model, shared state and question as sent. It prints no evidence. `--record` alone now replaces an entry and no longer stops at a conflict.

**Coalescing.** Within one call, a missing key already on its way is not sent again, and every item that needs it waits for that one answer. Across processes there is no coalescing. Two processes that miss the same question at the same moment both send, and the later write wins. This removes per-digest lock files, the folder gate, the backend marker and its admission probe.

**The lost marker guard.** ADR 0035's marker refused a folder pointed at a new address. The URL now sits inside every key, so answers from two addresses can never mix. A cache pointed at a new address simply misses and resends every question. The send budget and the token cap remain the guards against that cost.

Recording failures keep today's two fixed exit-5 sentences. SQLite's atomic commit replaces the temporary file, hard link, rename and directory sync. A crash keeps the last committed transaction.

### 4. The pipeline

One engine entry point serves every function and surface:

```rust
impl Engine {
    pub(crate) fn ask_all<A: Asker, E>(
        &self,
        asker: &A,
        host: Host<A::Input, A::Row, E>,
        cancel: &Cancel,
    ) -> Result<Outcome<E>, E>;
}

pub(crate) trait Asker: Sync {
    type Input: Send;
    type Row: Send;
    /// The wire questions one input needs, in order. Pure.
    fn asks(&self, input: &Self::Input) -> Result<Vec<Ask>, ItemError>;
    /// The row from the answers, in the same order. Pure.
    fn row(&self, input: Self::Input, answers: Vec<Answered>) -> Result<Self::Row, ItemError>;
}
```

`Ask` holds a shared `State` (bytes and SHA-256), one encoded `WireQuestion` with what its decoder needs, and its `QuestionKey`. `Item` is one input's row or `ItemError`, with its place, usage and `cached` flag. `Flow` is `Continue` or `Stop`. The host chooses: the command stops at the first failure, while `annotate --on-error continue` and the SQL hosts continue.

Stages, on one coordinator thread with `jobs` send workers from `engine/workers.rs`:

1. **Question stream.** `Host` is the demand bridge of today's scheduler. The coordinator sends the host events on one channel: `Ask` when it wants one more input, and `Row(Item)` for each finished row in order. The host answers `Ask` with one input, a failure or the end, on a second channel, and answers each `Row` with a `Flow`. The host pulls a record only after an `Ask`, so it never reads ahead. The same thread that feeds records also receives rows, so a host whose iterator is not `Send` works. The coordinator calls `asks` on each input.
2. **Lookup.** For each ask, the call's in-flight map answers first, then the store in one `SELECT … WHERE key IN (…)` per input. The lookup reads no key and opens no connection.
3. **Pack misses.** A pure core `Packer` adds each missing ask to the open request. The open request closes when the next input would pass the request-byte ceiling or a profile limit (`max_request_bytes`, `max_questions`, `max_evidence_bytes`), when it holds `--batch N` inputs or 4,096 inputs, when the next miss has another state, when input pauses 50 ms, when the window is full, or at end of input. `relate` and `recognize` step 3 also cap a request at 400 questions, from ADR 0057 item 4, which keeps a request well under the backend's 65,536 input tokens. One input whose asks pass a limit alone splits across requests with its state repeated. A lone question that passes a limit fails before any send, as today.
4. **Send.** A free worker takes the next closed request. It reads the key only now, for the first request of the call. It passes the pacer and the send budget, then the existing transport with its retries and per-address gate.
5. **Split.** The worker decodes the reply into one outcome per ask, plus the request's usage and attempts.
6. **Store.** The coordinator owns the call's one SQLite connection. It writes every good answer of the reply in one transaction.
7. **Reorder and emit.** An input is done when all its asks are resolved. The coordinator calls `row` and sends done inputs to the host as `Row` events in input order.

Memory stays bounded. The coordinator sends no `Ask` while it holds W unemitted inputs, where W = (`jobs` + 1) × the inputs-per-request cap, which is 4,096 or `--batch N`. When the window fills, the open request closes, so the head of the window can finish. That is ADR 0053 item 2's bound. `rank` without `--top` still holds every score until the end, as today, because a ranking needs them all.

Surfaces:

- **Command.** One host thread frames a record on each `Ask` and prints each `Row`, or keeps the top rows for `rank`.
- **Public Rust API.** `public/batch.rs` keeps its lazy `Batch` iterator. `ask_all` runs on a background thread. The calling thread is the host: `next()` pulls a caller record on each `Ask` and returns on each `Row`.
- **Polars, eager and lazy.** Each column or morsel is one `ask_all` call. A morsel evaluated twice finds its answers in the store.
- **C door and SQL hosts.** They keep calling the public `*_with` methods, which call `ask_all`. A recoverable per-row failure is an `ItemError` the host continues past.
- **`--plan` and dry runs.** They run the pure `Packer` with no lookup and show the upper bound, as today.

The packer, key, encoder, decoder and assembly stay in `core`. The channel, clock, threads and SQLite stay in `engine`.

**Model checks.** ADR 0100's `filter` and `rank` run check compares the models of live replies only. A cached row's `answered_by` takes no part, so an old stored answer never stops a run. The per-record `annotate` check follows the same rule.

### 5. How the ten functions map to questions

| Function | State | One wire question per | Input |
| --- | --- | --- | --- |
| `decide`, `filter`, `rank` | fixed sentence or context | record: `noul`, or rank's graded `score` as today | record |
| `choose` | fixed sentence or context | record: `choice` | record |
| `score` | fixed sentence or context | record: `score` | record |
| `tag` | fixed sentence or context | record and label: `noul` | record |
| `annotate` | fixed sentence or context | record, group and question: that question's kind | record |
| `find` | the unit set, as today | the whole set: one `choice` | the set |
| `recognize` step 1 | the text window of at most 40 pieces, as today | piece: the BIO `choice` | window |
| `recognize` step 2 | its window, as today | name: its `choice`, as today | window |
| `relate`, `recognize` step 3 | the whole text or entity set, as today | entity pair and relation: `noul` | pair |

`find`, `recognize` and `relate` keep their wire form, so their recordings convert without loss. `find` asks one question over the whole set, so adding a unit changes that question and gets no reuse. `recognize` and `relate` call `ask_all` once per step and compose the steps in their own function code. `annotate` needs no group planner: all its questions share one state, so records and groups pack together.

### 6. Partial replies, refusals and retries

- **Complete reply.** Every answer is stored.
- **Partial reply.** Good answers are stored and used. A failed question fails its input, which stops the command as today. Failed answers are never stored, so the next run asks only them. A replay of such a question is a miss at exit 5, not a replayed exit 6.
- **Whole refusal.** A reply with no valid answer, or one that does not decode, fails every input that waits on it. Nothing is stored.
- **Halving.** Status 413, or status 400 naming `max_tokens_exceeded`, on a request of two or more asks splits the asks in half once. The worker sends the first half, then the second, in the same slot. A half refused again fails its inputs and is not split again. The parent stores nothing. Other statuses do not split.
- **Retries.** They are unchanged. A retried status resends the same body under the per-address gate. A transport failure is never resent.

### 7. Accounting

- `requests_sent` and `retries` count HTTP attempts at the transport, as today. A halved parent counts one attempt.
- Input and output tokens count once for each live reply in the process totals, as today.
- `cache_answers` now counts questions answered from the store, not requests. The usage files keep the field name.
- A row's usage sums its questions' shares, live or stored. It is absent when any of its questions lacks a share.
- `meta.cached` is true when every answer of the row came from the store.
- `meta.requests` lists the row's question keys in answer order. Attempt observations keep the digest of the body they sent.
- `meta.batch` and annotate's `meta.batches` go away. A row's request now depends on which neighbours missed, so it no longer describes the row.
- The estimated input cap reserves each packed body of misses only, as today.

### 8. The requests-per-minute pacer

The pacer lives in the send stage, in `engine/backoff.rs` beside the per-address 429 gate. Every HTTP attempt, first sends and retries alike, takes one slot for its posting URL before it goes. Stored answers never wait. Every surface passes through this one send stage, so every surface gets the pacer. The rate setting and its default belong to the pacer issue's own ticket.

### 9. Recordings and tests

Ruling 6 asks for the simpler option. That is one schema: tests replay the same entries a cache holds, from a text file.

`thinkthen cache convert DIR [--quote]` reads every `DIGEST.json` entry of schema `thinkthen.recording/1` in `DIR`. It decodes each response against its request and writes each good answer as a question entry with its token share, `taken_at` 0 and `origin` `converted`. It always writes a `{"records":[…]}` exchange under the fixed-sentence state, because its questions already carry the quote. With `--quote`, it also writes each single-record exchange in the quoted form, with the record's compact JSON quoted into each string instruction and `origin` `quoted`. An entry whose envelope carries `"quoted": true` also gets `origin` `quoted`. That reuses an answer taken under the old form, which ruling 3 accepts. The changelog names both origins and says that a user's old cache is ignored until the user runs `cache convert`. The converter leaves the old files in place and can run again.

In the repository:

- Slice 1 rewrites committed record-function recordings into the quoted form at the request level. Slice 2 converts every committed folder to `thinkthen.jsonl`. The demo pages keep their exact outputs, token counts included, because each question keeps its recorded answer and share.
- Old files stay until slice 5, so a function not yet moved still reads them.
- Strict replay stays at the question level. `--replay` reads only and sends nothing, and a test proves "sends nothing" by counting loopback requests.
- Slice 1 stops the `spec` gate running `probes/replay-check.sh`, because the probes replay the old wire form. The probes are history of the forms they measured. Their rows and old files stay unchanged. `probes/find-0040/recording` is converted, because `find_edge.rs` replays it.
- The check that ties a captured conformance case to its recording reads the question entries.

### 10. What the path deletes and what survives

Deleted, about 8,900 lines today:

- `core/batch.rs`, `core/batch/groups.rs`, `core/batch/questions.rs` and `core/batch/tests*`. The content cut goes with them.
- `core/recording_identity.rs`. `core/recording.rs` shrinks to the old-entry reader the converter needs.
- `engine/recorder.rs`, `engine/recorder/fault.rs`, `engine/recorder/identity.rs` and its tests, `engine/cache_lock.rs`, `engine/cache_prune.rs`, `engine/cache_prune/binding.rs` and `engine/cache_prune/scan.rs`.
- `engine/schedule.rs`, `engine/annotate_schedule.rs` and `engine/annotate_schedule/*`, `engine/prepared_request.rs`, `engine/facade/split.rs`, `engine/facade/native_batch.rs` and `engine/facade/annotate/batch.rs`.
- `cli/asking/batched.rs`, `cli/asking/batched/*`, `cli/asking/batch_meta.rs`, `cli/annotate/batching.rs`, `cli/annotate/batching/*` and `cli/annotate_schedule.rs`.
- `public/batch/planned.rs`, `public/batch/planned/*` and `public/batch/annotation.rs`.
- Shrunk rather than deleted: the recording logic in `engine/request.rs` and the planning in `public/native_batch.rs`, which keeps only `RecoverableDetails` and its mapping.
- The folder marker, `.locks`, temporary entry files, record conflicts, the one-millisecond sleep loops in the annotate former, `RecordFlow`, and the `status` fields for bad and temporary entries. `status` becomes `thinkthen.status/2`.

New code is `core/pack.rs` (packer and key), `engine/store.rs`, `engine/pipeline.rs` and the converter.

Survives: `engine/http.rs`, `engine/backoff.rs`, `engine/send_budget.rs`, `engine/usage*`, `engine/workers.rs` with its panic diagnostics, the System One adapter, the question, answer, threshold and result types, command framing and output (`cli/schedule.rs`), the function code of `judge`, `find`, `recognize` and `relate` as `Asker`s, `public/batch.rs` and `public/frame*`. `cache prune` and `cache unused` keep their selectors as short SQL over `taken_at`, `answered_by` and the key list, followed by deleting states no answer uses and `PRAGMA incremental_vacuum`. Clearing and expiry wait for their own ticket.

## Build order

Each slice lands green: `cargo test --workspace`, `policy.py`, fresh code review.

1. **The quoted wire form on every surface.** The encoder takes section 1's form and writes per-question bytes and the joined body. Today's batcher, request-level cache and schedulers stay, so only wire bytes change. The slice rewrites the shared `conformance/` cases. A one-off script rewrites each committed request-level recording into the quoted form by the rule `cache convert --quote` uses, renames it under its new digest, keeps its response, and adds `"quoted": true` to its envelope. The `spec` gate stops running `probes/replay-check.sh`. Proof: `sdlc/scripts/spec` passes with the probes out of it; every conformance runner passes on the rewritten cases, including the Polars, facade contract and `tests/backend/public_json.rs` tests; every demo replays with unchanged output, including `demos/28-what-a-run-cost`'s token counts.
2. **Key, store, fixtures and pipeline for the record functions on the command.** The slice adds the key, the store, the JSON Lines reader and `cache convert`, and converts every committed folder to `thinkthen.jsonl`. `decide`, `filter`, `rank`, `choose`, `tag`, `score` and `annotate` use `ask_all`. No wire bytes change. Proof: a key vector is pinned from bytes hashed outside the program; converting the same folder twice writes identical bytes; converting a folder that holds a fixture, a scratch `thinkthen.sqlite` and old files keeps the newer answer of a shared key and the fixture's on a tie; `--replay` of a folder holding both files exits 5; demos replay with unchanged output, including any demo that pins a batched `annotate` row's token counts under the new remainder rule; a lookup during another process's commit waits and then answers. On the loopback backend, which gains a count of questions received: a batch of 100 records, then a run of 120 that includes them, sends one request holding exactly the 20 new questions, and the second run reports 100 cache answers; a partial reply stores its good answers and the rerun sends only the failed question; a 413 on a request of two or more asks makes exactly three attempts and stores both halves; adding one label to a `tag` run sends only that label's questions; a slow pipe closes a request at the pause; the unemitted window never passes W; the store edge table covers hit, miss, replace, a read-only open that writes nothing, a hot journal under read-only replay, a 30-second busy limit, and an answer that no longer decodes; two child processes write one store at once and both succeed; `nm` shows no exported `sqlite3_` symbol in the C door or the SQLite extension.
3. **Public Rust API, Polars, the C door and the SQL hosts on `ask_all`.** Proof: every surface passes the conformance cases; a Polars lazy frame collected twice sends nothing the second time; an SQL row with a missing pointer still fails alone while its neighbours answer.
4. **`find`, `recognize` and `relate` on `ask_all`.** The relation splitter and `ask_chunks` go. Proof: conformance cases 41 to 50 give identical results from the converted fixtures; a relate run of 401 questions sends two requests, and the same run again sends none.
5. **Remove the old store.** Delete the recorder, locks, marker, request-level prune, both old schedulers and every old `DIGEST.json` outside probe history. Rewrite `specification/recording.md` and the size and splitting sections of `specification/backends.md`. Proof: the `cache prune` and `status` edge tables pass on the store, and prune shrinks the file; the full suite passes.

Each slice updates the specification pages its behavior changes.

## What this amends

| ADR | Change |
| --- | --- |
| 0048 item 1 | Every record function takes the quoted form, a batch of one included. Equal keys are asked once per call |
| 0048 item 2 | The content cut goes. A request closes at a limit, the size, 4,096 inputs, a new state, the pause, the window or the end |
| 0048 item 5 | Replaced. The key is one question, and replay and cache work per question |
| 0048 items 6 and 9 | Partial replies store good answers. `meta.batch` goes and usage shares split by question |
| 0053 items 5 and 6 | No record is in another record's state. A cache stores the good answers of a partial reply |
| 0055 items 3 and 4 | A batch of one is quoted, and `choose`, `tag` and `score` take the fixed sentence |
| 0035, 0099 | Withdrawn. No folder binds to one address. Section 3 names the lost guard |
| 0092 | Withdrawn. `meta.batches` goes |
| 0100 | The alias refresh stays. Its lock and rename steps give way to one SQLite transaction. Model checks compare live replies only |
| 0053, the ticket 0212 amendment | Superseded. The library closes the open request after 50 ms with no new record, including while a caller's `Iterator::next()` blocks, by section 4. `Records(1)` still returns each row before the next pull |
| 0089, the 0212 source-boundary clarification | Superseded. Its no-idle-timer rule goes with ADR 0053's amendment |

## Amendment, 2026-09-30: two facts from ticket 0304 slice 3a

Section 3's dependency paragraph assumed each host holds one private bundled SQLite. Two facts from the slice 3a build and its reviews correct it. Slice 3a fixes the first, and slice 3b finishes both.

1. **The SQLite extension runs the cache on the host's SQLite.** The extension pins `rusqlite` with `loadable_extension`, and Cargo resolves one `libsqlite3-sys` with every crate's features. The first slice 3a build left `bundled` on in `thinkthen`, so `libsqlite3-sys` generated its bindings from the bundled SQLite 3.53.2 header. It still skipped the bundled build and routed every call through the host's function table. `rusqlite`'s loadable entry then refused every host older than 3.53.2, so the extension failed to load on its pinned 3.50.0 host. Its error path also aborted the process, because it reached `sqlite3_malloc64` before the table was read. Slice 3a fixes this. `thinkthen` gains two features: `bundled-sqlite` bundles SQLite, and `host-sqlite` bundles nothing. The library refuses to compile with neither. Every binding names `bundled-sqlite` except the SQLite extension, which names `host-sqlite`, and `policy.py` enforces the choice. The extension's bindings now come from the build machine's SQLite header, as they did before slice 3a, and the question store runs on the host's SQLite. The store needs `INSERT … ON CONFLICT DO UPDATE`, from SQLite 3.24, and the extension's floor is already 3.50.0. The store also opens connections on the pipeline's background thread, which a host built single-threaded forbids. Slice 3b checks the host's thread mode when the extension loads and refuses a host that cannot carry the store.
2. **The static libraries export SQLite's symbols.** `libthinkthen.a` from the C door and the R binding's static library export about 285 global `sqlite3_` symbols from the bundled SQLite. A program that links one of them beside its own SQLite meets duplicate or mixed symbols. The slice 2 `nm -D` check reads only the shared library, which keeps them private. The C door now pins the static library's leak with an inverted test, and `sdlc/issues/2026-09-30-static-library-exports-sqlite-symbols.md` tracks it. Slice 3b localizes the symbols with a partial link before archiving, then flips the test to require zero.

## What Ian can overturn

Ian's rulings built here: the per-question key (2), accepting neighbour effects (3), the three question kinds as the cache unit (4), taken-at on each entry (5), the simpler test recordings (6).

The design author's calls:

1. SQLite through bundled `rusqlite` for caches, with the rollback journal, and sorted JSON Lines for committed fixtures.
2. No coalescing across processes, so a concurrent identical miss can be paid twice.
3. A cache pointed at a new address resends everything, with the send budget as the guard.
4. A replay of a question whose live answer failed is a miss, not a replayed exit 6.
5. `--record` replaces an entry and never reports a conflict.
6. `meta.batch` and `meta.batches` go, and `meta.requests` lists question keys.
7. `cache_answers` counts questions.
8. Probe replay leaves the `spec` gate.
9. No automatic conversion of a user's old cache.
