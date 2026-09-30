# ADR 0111: One question cache and one batching path

- Status: Proposed 2026-09-29 for ticket 0304. A fresh reviewer returns ACCEPT or findings before any build. Ian can overturn each item.
- Date: 2026-09-29

This ADR builds Ian's rulings 2 to 6 and 8 of 2026-09-29 in `sdlc/planning/cleanup-2026-09-30.md`. It replaces ADR 0048 item 5 and amends the ADRs listed at the end.

## Context

Today the cache key is the SHA-256 of the adapter name, the URL and the whole request body. One JSON file holds one exchange. Per-digest lock files coalesce concurrent misses, and a marker file binds each folder to one address. Because a batch is the unit of storage, a batch changes its key when any record in it changes. Content cuts exist only to keep batch edges stable. A run of 120 records after a run of 100 therefore resends most of the 100.

The batching work also grew five schedulers and planners: `core/batch`, `engine/schedule.rs`, `engine/annotate_schedule`, `cli/asking/batched`, `cli/annotate/batching`, `public/batch/planned` and `public/native_batch`. Each surface packs, splits and orders on its own.

System One takes `{state, model, questions:{q1..qN}}`. Each question is `noul`, `choice` or `score`. The reply answers each `qN` and reports usage for the whole request only.

## Decision

### 1. Every record function quotes each record in its own question

`decide`, `filter`, `rank`, `choose`, `tag`, `score` and `annotate` send one fixed state for every request:

- Without `--context`, the state is the text `Each question quotes the text it asks about.`
- With `--context`, the state is the context text.

Each wire question carries its record. Its instructions are `The text is `, then the record's compact JSON, then `. `, then the question's instructions as today. `tag` expands one `noul` question per label, as today, and each expanded instruction starts with the same quote. An `annotate` group quotes the part its `on` pointer selects.

A batch of one uses the same form. The unquoted single-record request and the `{"records":[…]}` state both go away. So one record always makes the same questions, whatever else shares its request.

Experiment 275 found the records-list form ahead of the quoted form for `choose` and `tag` on short titles, and nobody has measured long records. Ian's ruling of 2026-09-29 takes the quoted form anyway. This ADR asks for no paid measurement.

A question whose instructions are a JSON object or list cannot take the quote. Its record becomes the state and the question goes unquoted. Each such record then makes its own request. A context beside such a question stays a usage error, as today.

### 2. The question key

The key is the SHA-256 of these bytes, joined by one line feed (0x0A) each:

1. The adapter name, `systemone`.
2. The resolved posting URL, as today's digest uses it.
3. The model string exactly as the body's `model` member carries it, without JSON quotes.
4. The state as encoded in the body: the compact JSON value after `"state":`.
5. One wire question as encoded in the body: the compact JSON object after `"qN":`, such as `{"type":"noul","instructions":"The text is \"Ringo\". Is this a drummer?"}`.

No part can hold a raw line feed. The URL and model are checked for control characters today, and compact JSON escapes line feeds. So the join is unambiguous. The key prints as 64 lowercase hex characters.

The encoder writes each state and each question once. It builds the body by joining those same bytes: `{"state":S,"model":M,"questions":{"q1":Q1,…}}`. The key therefore hashes exactly the bytes sent, and the byte count of a request is a sum. No skeleton or probe encoding remains.

The key leaves out the question name `qN`, the other questions in the request, the batch setting, the profile, the threshold and every header. The vendor shows no names to the model. Ruling 3 accepts that neighbours may move an answer. A threshold is applied after the answer, so retuning a threshold reuses every stored answer. Nothing else in the body can change an answer.

Equal keys in one call are asked once. That replaces the equal-evidence rule of ADR 0048 item 1.

### 3. The store

One SQLite file named `thinkthen.sqlite` sits inside the folder that `--cache`, `--record`, `--replay`, `THINKTHEN_CACHE` or the platform default names. The option spellings and folder rules stay. The folder is mode `0700` when the tool creates it. The file is created with mode `0600` before SQLite opens it, and SQLite gives its journal the same mode.

Dependency: `rusqlite` with default features off and `bundled` on. Bundled SQLite gives every host the same SQLite with no system library skew, including wheels, the C door and the SQL extensions. It costs one C compile of SQLite and about 1 MB of binary. Pure Rust stores such as `redb` and `sled` lock their file to one process, so parallel shells and PostgreSQL backends could not share a cache. The SQLite extension will hold two SQLite copies in one process. That is safe because they never open the same file. The library must export no `sqlite3_` symbol, and slice 1 checks that.

The file uses the rollback journal (`journal_mode=DELETE`) with `synchronous=FULL` and `secure_delete=ON`. Each reply is one small write transaction, so the single writer costs nothing measurable. The rollback journal leaves no side files, so `--replay` can open a committed file with `mode=ro` in a read-only checkout and write nothing. Write-ahead logging would leave `-wal` and `-shm` files and break that. Secure delete zeroes removed rows, so a committed file holds no deleted evidence. A busy handler waits for another writer and checks the call's cancel token between waits.

Schema, `PRAGMA user_version = 1`:

```sql
CREATE TABLE states (
  sha256 BLOB PRIMARY KEY,   -- SHA-256 of the state bytes
  state  TEXT NOT NULL       -- the state as sent
) WITHOUT ROWID;

CREATE TABLE answers (
  key         BLOB PRIMARY KEY,  -- the 32-byte question key
  url         TEXT NOT NULL,
  model       TEXT NOT NULL,     -- the model the request named
  state       BLOB NOT NULL,     -- states.sha256
  question    TEXT NOT NULL,     -- the wire question as sent
  answer      TEXT NOT NULL,     -- the wire answer for that question, as received
  answered_by TEXT NOT NULL,     -- the model the reply named
  taken_at    INTEGER NOT NULL   -- Unix seconds, UTC, when the reply arrived
) WITHOUT ROWID;
```

A state is stored once, so a relate text asked 400 times costs one copy. The stored parts let a later key version re-key the store without asking again. A row whose answer no longer decodes counts as a miss under a cache and as a local failure under `--replay`.

Modes:

| Mode | Look up | Send | Write |
| --- | --- | --- | --- |
| `--replay DIR` | yes | nothing; a miss exits 5 | no |
| `--record DIR` | no | every question | yes, replacing |
| `--cache DIR`, both options on one folder, `THINKTHEN_CACHE`, the default cache | yes | misses | yes |
| A cache with `--refresh-cache`, or the model `jev-latest` | no | every question | yes, replacing |
| `--no-cache` | no | every question | no |

A replay miss names the missing key and says it is the SHA-256 of the adapter, address, model, shared state and question as sent. It prints no evidence. `--record` alone now replaces an entry with a new answer and no longer stops at a conflict.

Coalescing: within one call, a missing key already on its way is not sent again, and every item that needs it waits for that one answer. Across processes there is no coalescing. Two processes that miss the same question at the same moment both send, and the later write wins. This removes per-digest lock files, the folder gate, the backend marker and its admission probe. The URL sits inside every key, so one store can safely hold answers from several addresses.

Recording failures keep today's two fixed exit-5 sentences. SQLite's atomic commit replaces the temporary file, hard link, rename and directory sync. A crash keeps the last committed transaction.

### 4. The pipeline

One engine entry point serves every function and surface:

```rust
impl Engine {
    pub(crate) fn ask_all<A: Asker, E>(
        &self,
        asker: &A,
        inputs: Receiver<Result<A::Input, E>>,
        cancel: &Cancel,
        emit: impl FnMut(Item<A::Row>) -> Result<Flow, E>,
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

`Ask` holds a shared `State` (bytes and SHA-256), one encoded `WireQuestion` with what its decoder needs, and its `QuestionKey`. `Item` is one input's row or `ItemError`, with its place, usage share and `cached` flag. `Flow` is `Continue` or `Stop`. The caller's `emit` chooses: the command stops at the first failure, `annotate --on-error continue` and the SQL hosts continue.

Stages, on one coordinator thread with `jobs` send workers from `engine/workers.rs`:

1. **Question stream.** The coordinator takes the next input from the channel and calls `asks`. A host reader thread feeds the channel through a rendezvous `sync_channel(0)`, so the reader reads only when asked.
2. **Lookup.** For each ask, the call's in-flight map answers first, then the store in one `SELECT … WHERE key IN (…)` per input. The lookup reads no key and opens no connection.
3. **Pack misses.** A pure core `Packer` adds each missing ask to the open request. The open request closes when the next input would pass the request-byte ceiling or a profile limit (`max_request_bytes`, `max_questions`, `max_evidence_bytes`), when it holds `--batch N` inputs or 4,096 inputs, when the next miss has another state, when input pauses 50 ms, when the window is full, or at end of input. One input whose asks pass a limit alone splits across requests with its state repeated, as relate does today. A lone question that passes a limit fails before any send, as today.
4. **Send.** A free worker takes the next closed request. It reads the key only now, for the first request of the call. It passes the pacer and the send budget, then the existing transport with its retries and per-address gate.
5. **Split.** The worker decodes the reply into one outcome per ask, plus the request's usage and attempts.
6. **Store.** The coordinator owns the call's one SQLite connection. It writes every good answer of the reply in one transaction.
7. **Reorder and emit.** An input is done when all its asks are resolved. The coordinator calls `row` and emits done inputs in input order.

Memory stays bounded. The coordinator reads no new input while it holds W unemitted inputs, where W = (`jobs` + 1) × the inputs-per-request cap, which is 4,096 or `--batch N`. When the window fills, the open request closes, so the head of the window can finish. That is ADR 0053 item 2's bound.

Surfaces:

- **Command.** The reader thread frames records into the channel. `emit` prints, or keeps the top rows for `rank`.
- **Public Rust API.** `public/batch.rs` keeps its lazy `Batch` iterator. It runs `ask_all` on a background thread, feeds the caller's records from the calling thread, and returns rows through a bounded channel.
- **Polars, eager and lazy.** Each column or morsel is one `ask_all` call. A morsel evaluated twice finds its answers in the store.
- **C door and SQL hosts.** They keep calling the public `*_with` methods, which call `ask_all`. A recoverable per-row failure is an `ItemError` that `emit` continues past.
- **`--plan` and dry runs.** They run the pure `Packer` with no lookup and show the upper bound, as today.

The packer, key, encoder, decoder and assembly stay in `core`. The channel, clock, threads and SQLite stay in `engine`.

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
| `relate` | the whole text or entity set, as today | entity pair and relation: `noul` | pair |

`find`, `recognize` and `relate` keep their wire form, so their recordings convert without loss. `recognize` and `relate` call `ask_all` once per step and compose the steps in their own function code. `annotate` needs no group planner: all its questions share one state, so records and groups pack together.

### 6. Partial replies, refusals and retries

- **Complete reply.** Every answer is stored.
- **Partial reply.** Good answers are stored and used. A failed question fails its input, which stops the command as today. Failed answers are never stored, so the next run asks only them. `--record` no longer writes a failed answer. A replay of that question is a miss at exit 5, not a replayed exit 6.
- **Whole refusal.** A reply with no valid answer, or one that does not decode, fails every input that waits on it. Nothing is stored.
- **Halving.** Status 413, or status 400 naming `max_tokens_exceeded`, on a request of two or more asks splits the asks in half once. The worker sends the first half, then the second, in the same slot. A half refused again fails its inputs and is not split again. The parent stores nothing. Other statuses do not split.
- **Retries.** They are unchanged. A retried status resends the same body under the per-address gate. A transport failure is never resent.

### 7. Accounting

- `requests_sent` and `retries` count HTTP attempts at the transport, as today. A halved parent counts one attempt.
- Input and output tokens count once for each live reply, as today.
- `cache_answers` now counts questions answered from the store, not requests. The usage files keep the field name.
- A row's usage is its share of each live request it used. A request's usage is split evenly over its asks, with any remainder to the earliest. Stored answers add nothing. The row's usage is absent when any contributing reply lacked it.
- `meta.cached` is true when every answer of the row came from the store.
- `meta.requests` lists the row's question keys in answer order. Attempt observations keep the digest of the body they sent.
- `meta.batch` and annotate's `meta.batches` go away. A row's request now depends on which neighbours missed, so it no longer describes the row.
- The estimated input cap reserves each packed body of misses only, as today.

### 8. The requests-per-minute pacer

The pacer lives in the send stage, in `engine/backoff.rs` beside the per-address 429 gate. Every HTTP attempt, first sends and retries alike, takes one slot for its posting URL before it goes. Stored answers never wait. Every surface passes through this one send stage, so every surface gets the pacer. The rate setting and its default belong to the pacer issue's own ticket. This ADR fixes only where it sits.

### 9. Recordings and tests

Ruling 6 asks for the simpler option. That is one store: tests replay the same SQLite file the cache writes.

`thinkthen cache convert DIR [--quote]` reads every `DIGEST.json` entry of schema `thinkthen.recording/1` in `DIR`. It decodes each response against its request and writes each good answer as a question entry, with `taken_at` set to the file's modification time. It always writes a `{"records":[…]}` exchange under the fixed-sentence state, because its questions already carry the quote. With `--quote`, it also writes each single-record exchange in the quoted form, with the record's compact JSON quoted into each string instruction. That reuses an answer taken under the old form, which ruling 3 accepts. The converter leaves the old files in place and can run again.

In the repository:

- Each slice converts the committed folders its functions replay, with `--quote` for record-function folders. The demo pages keep their exact outputs, because each question keeps its recorded answer.
- Old files stay until slice 6, so a folder shared by a moved and an unmoved function works in between.
- Strict replay stays at the question level. `--replay` reads only and sends nothing, and a test proves "sends nothing" by counting loopback requests.
- The `spec` gate stops running `probes/replay-check.sh`. The probes are history of the forms they measured. Their rows and old files stay unchanged. `probes/find-0040/recording` is converted, because `find_edge.rs` replays it.
- Conformance cases whose request bytes change are rewritten to the quoted form. The check that ties a captured case to its recording reads the question entries.
- A user's old cache is ignored until the user runs `cache convert`. The changelog says so.

### 10. What the path deletes and what survives

Deleted:

- `core/batch.rs`, `core/batch/groups.rs`, `core/batch/questions.rs` and `core/batch/tests*`. The content cut goes with them.
- `core/recording_identity.rs`. `core/recording.rs` shrinks to the old-entry reader the converter needs.
- `engine/recorder.rs`, `engine/recorder/fault.rs`, `engine/recorder/identity.rs` and its tests, `engine/cache_lock.rs`, `engine/cache_prune.rs`, `engine/cache_prune/binding.rs` and `engine/cache_prune/scan.rs`.
- `engine/schedule.rs`, `engine/annotate_schedule.rs` and `engine/annotate_schedule/*`, `engine/prepared_request.rs`, `engine/facade/split.rs`, `engine/facade/native_batch.rs` and `engine/facade/annotate/batch.rs`.
- The `ask_prepared` and `first_use_key` recording logic in `engine/request.rs`.
- `cli/asking/batched.rs`, `cli/asking/batched/*`, `cli/asking/batch_meta.rs`, `cli/annotate/batching.rs`, `cli/annotate/batching/*` and `cli/annotate_schedule.rs`.
- `public/batch/planned.rs`, `public/batch/planned/*` and `public/batch/annotation.rs`. `public/native_batch.rs` keeps only `RecoverableDetails` and its mapping.
- The folder marker, `.locks`, temporary entry files, record conflicts, the one-millisecond sleep loops in the annotate former, `RecordFlow`, and the `status` fields for bad and temporary entries. `status` becomes `thinkthen.status/2`.

These files hold about 11,000 lines today. New code is `core/pack.rs` (packer and key), `engine/store.rs`, `engine/pipeline.rs` and the converter.

Survives: `engine/http.rs`, `engine/backoff.rs`, `engine/send_budget.rs`, `engine/usage*`, `engine/workers.rs` with its panic diagnostics, the System One adapter, the question, answer, threshold and result types, command framing and output (`cli/schedule.rs`), the function code of `judge`, `find`, `recognize` and `relate` as `Asker`s, `public/batch.rs` and `public/frame*`. `cache prune` and `cache unused` keep their selectors as short SQL over `taken_at`, `answered_by` and the key list. Clearing and expiry wait for their own ticket.

## Build order

Each slice lands green: `cargo test --workspace`, `policy.py`, fresh code review.

1. **Key, store and converter, beside today's path.** The encoder gains per-question bytes and the joined body. The slice adds the key, `engine/store.rs`, the `rusqlite` dependency and `cache convert`. Proof: every fixture request re-encodes byte for byte through the joined body; a key vector is pinned from bytes hashed outside the program; a store edge table covers hit, miss, replace, a read-only replay that creates no file, and an answer that does not decode; two child processes write one store at once and both succeed; converting a recognize fixture folder yields one entry per good answer; `nm` shows no exported `sqlite3_` symbol in the C door or the SQLite extension.
2. **Pipeline and packer for `decide`, `filter` and `rank` on the command.** A batch of one takes the quoted form. The slice converts their committed folders. Proof, counted on the loopback backend, which gains a count of questions received: a batch of 100 records, then a run of 120 that includes them, sends one request holding exactly the 20 new questions, and the second run reports 100 cache answers; a partial reply stores its good answers, and the rerun sends only the failed question; a 413 on a request of two or more asks makes exactly three attempts and stores both halves; a slow pipe closes a request at the pause; the unemitted window never passes W; demos replay with unchanged output.
3. **`choose`, `tag`, `score` and `annotate` on the pipeline.** The records-list state and the annotate group planner go. Proof: adding one label to a `tag` run sends only that label's questions; an `annotate` run with two groups over 50 records sends one request, not one per group; adding one record sends only its questions; demos replay unchanged.
4. **Public Rust API, Polars, the C door and the SQL hosts on `ask_all`.** Proof: every surface passes the conformance cases; a Polars lazy frame collected twice sends nothing the second time; an SQL row with a missing pointer still fails alone while its neighbours answer.
5. **`find`, `recognize` and `relate` on `ask_all`.** The relation splitter and `ask_chunks` go, and their fixtures convert. Proof: conformance cases 41 to 50 give identical results from the converted stores; a relate run of 401 questions sends two requests, and the same run again sends none.
6. **Remove the old store.** Delete the recorder, locks, marker, request-level prune, both schedulers and every old `DIGEST.json` in the repository. Rewrite `specification/recording.md` and the size and splitting sections of `specification/backends.md`. Proof: the `cache prune` and `status` edge tables pass on the store; `find . -regex '.*/[0-9a-f]\{64\}\.json'` lists only probe history; the full suite passes.

Each slice updates the specification pages its behavior changes.

## What this amends

| ADR | Change |
| --- | --- |
| 0048 item 1 | Every record function takes the quoted form, a batch of one included. Equal keys are asked once per call |
| 0048 item 2 | The content cut goes. A request closes at a limit, the size, 4,096 inputs, a new state, the pause, the window or the end |
| 0048 item 5 | Replaced. The key is one question, and replay and cache work per question |
| 0048 items 6 and 9 | Partial replies store good answers. `meta.batch` goes and shares split by question |
| 0053 items 5 and 6 | No record is in another record's state. A cache stores the good answers of a partial reply |
| 0055 items 3 and 4 | A batch of one is quoted, and `choose`, `tag` and `score` take the fixed sentence |
| 0035, 0099 | Withdrawn. No folder binds to one address |
| 0092 | Withdrawn. `meta.batches` goes |
| 0100 | The alias refresh stays. Its lock and rename steps give way to one SQLite transaction |

## What Ian can overturn

Ian's rulings built here: the per-question key (2), accepting neighbour effects (3), the three question kinds as the cache unit (4), taken-at on each entry (5), one store for tests (6).

The design author's calls:

1. SQLite through bundled `rusqlite`, with the rollback journal.
2. No coalescing across processes, so a concurrent identical miss can be paid twice.
3. A replay of a question whose live answer failed is a miss, not a replayed exit 6.
4. `--record` replaces an entry and never reports a conflict.
5. `meta.batch` and `meta.batches` go, and `meta.requests` lists question keys.
6. `cache_answers` counts questions.
7. Probe replay leaves the `spec` gate.
8. No automatic conversion of a user's old cache.
