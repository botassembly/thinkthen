# Cache identity and response storage policy

Status: **Settled** for the 0.2 target by [ADR 0120](../sdlc/planning/adr/0120-sdk-result-and-cache-contract.md). Tickets 0443/0444/0450 implement it. The final section preserves the landed v1 question-store contract moved from [recording.md](recording.md); target clauses supersede it only upon adoption. Recording options, default-folder resolution, permissions, pruning, conversion commands and count-only usage retain that page's rules. This contract adds no command, host cache, routing policy or proof framework.

## Version 2 identity

`F(tag,parts)` is the ASCII tag followed by one NUL byte, then each part prefixed with its unsigned 64-bit big-endian byte length. Text parts are UTF-8. The v2 question key is lowercase SHA-256 of:

```text
F("thinkthen.question-key/2",
  adapter, canonical_final_URL,
  compact_JSON(requested_model), compact_JSON(answered_model),
  canonical_state_JSON, canonical_wire_question_JSON)
```

Models are typed compact JSON strings. State/question JSON uses the adapter's typed compact serializers and their field/option order, not arbitrary caller JSON formatting. Adapter is the admitted provider API type/dialect, today `systemone`. Canonicalize the final posting URL with the existing endpoint resolver's scheme/host normalization and base trailing-slash treatment, consistently for all inputs and migration. Preserve distinct paths, ports and escapes otherwise. A resolved suffix such as `decisions` is part of that address. Retain credential/control/userinfo/query/fragment refusals; no key is hashed, logged or persisted.

For the OpenAI target in ADR 0122, `canonical_wire_question_JSON` is the adapter's typed encoded per-question object with only its top-level request-local `name` omitted. Its canonical persistent answer likewise omits only the top-level correlation name after the raw reply was validated against its actual sent request. Preserve semantic instructions, descriptions, choice values, labels and their order. A saved canonical answer is validated against its name-free canonical question; cache/replay never requires a saved qN to equal a new request's qN. Raw recorded exchange bodies retain exact original names/order and are validated against their original request before projection. This is pure adapter normalization within existing per-question storage, not a new store, key version or migration rule. System One serializers and all original v1 key/recording validation remain unchanged.

With shared state and all other key constituents unchanged, B at q2 after A and B alone at q1 have one question key. The engine reuses held questions independently and sends only missing pieces. Correlation names/positions enter no persistent observation or answer identity: cache/replay keeps the saved observation ID, and unchanged semantic scope/reading keeps the answer ID despite transport repacking. Newly accepted live observations still receive fresh IDs under result/2. Semantic scope/order, changed evidence and readings retain their existing identity effects; exact raw request digests remain allowed to differ. No synthetic names or former envelope positions are added to saved per-question answers.

Both literal requested and reported models enter identity. No model name selects an SDK group, provider or target map. State identity includes ordered actual image media/immutable bytes and the admitted serialization version under 0447/0448; filenames, paths and timestamps are excluded. Image byte, media or order changes must change identity. Relocation alone must not. Preserve the state SHA-256 and stored usage/history.

V2 answer rows retain existing columns and add required `key_version:2`, `adapter` and `observation_id:ObservationId`. Existing `model` is the requested model, `answered_by` the reported model, and `url` the final posting URL. State, question, answer, model, adapter and address must suffice to rebuild and verify every key offline. Transient call/request IDs enter neither rows nor keys. Persistent observation identity is allowed answer metadata, independent of transport identity and retrieval origin.

## Lookup and freshness

Online reuse requires reported model equal to the literal requested model. A held mismatch is excluded; the call goes live with a safe held-answer warning that repeats no stored field, credential or address. Keep per-effective-request `jev-latest` refresh and explicit refresh. `Facts::held_model_mismatch()` reports this exclusion for the current native call; complete facts emit `held_model_mismatch:true` only when it occurred. It retains no historical model value. The CLI prints the fixed warning once per invocation. Offline replay preserves historical ambiguity rules and emits no online warning. The key does not establish the current target of a mutable alias that echoes itself: callers use no-cache/refresh, or providers prohibit storage. Add no discovery send or target index.

Offline replay returns one validated historical match and refuses multiple distinct historical reported-model matches locally with zero sends. It never asks where an alias points today. Future explicit proxy mode bypasses local reuse until an admitted policy-version contract proves it safe. A proxy hostname on ordinary model wire remains a direct route. Every stage, retry and split uses the engine's fixed endpoint/key/API type under ADR 0119.

## Response Cache-Control

Read only response `Cache-Control`. Accept at most eight fields, 8,192 combined field-value bytes and 64 directives. Parse ASCII HTTP token directive names case-insensitively; separate commas outside quoted strings; accept HTTP optional whitespace (space/tab), token or quoted-string values and quoted escapes. Unclosed quotes, invalid tokens/escapes, control bytes or other malformed syntax conservatively prohibit storage. Match the exact directive name `no-store`; text inside another directive's quoted value is not a match.

Valid repeated fields combine. Any `no-store` prohibits storage for the entire packed reply, including every good answer in a partial reply. Malformed or over-bound Cache-Control also prohibits storage; it does not invalidate an otherwise valid answer. An absent header permits existing storage behavior. Unknown valid directives and unrelated headers have no effect.

| Header values | Storage |
| --- | --- |
| `max-age=30` | permitted |
| `private="no-store"` | permitted; quoted text is not a directive |
| `No-StOrE` | prohibited |
| Two fields: `max-age=30` and `no-store` | prohibited |
| Unclosed quoted value or a parser bound exceeded | prohibited |

Refresh bypasses held answers and sends request `Cache-Control: no-cache` on every actual request, including retries/split children. A successful nonstorable refresh transactionally evicts prior working-cache answers for those questions. An ordinary cache-mode call may return accepted answers without persisting them. A failed refresh leaves held answers intact but never serves them during that invocation. Failure to commit required eviction is a local storage failure, not a successful refresh that leaves stale state reusable.

Explicit recording refuses a nonstorable reply after the send with final started-call facts and the exact message:

```text
the reply forbids storage, so the recording cannot be written
```

This is a local failure (CLI exit 5). Write none of that reply's answers and never retry for this failure. Other already committed replies are not retroactively removed. Explicit replay reads no headers, changes no store bytes and sends nothing. Never infer cache instructions from unknown reply body fields.

## Validation and migration

The landed v1 key is SHA-256 of `adapter`, original stored URL bytes, compact requested-model JSON, state JSON and wire question JSON, separated by single LF bytes, with no trailing LF. Validate that original key **before** any URL normalization. Then verify the state digest and typed model/state/question/answer, reported usage and admitted adapter. Derive legacy observation identity before rewriting. Only then normalize/rekey to v2. A v1 row's adapter is known from its existing store schema; do not infer an unknown adapter or missing address.

An absent key version is v1 only for a validated existing v1 store/fixture shape. Unknown versions/adapters, missing addresses, damaged entries or incomplete key identity and distinct saved snapshots that collapse onto one normalized v2 key refuse locally before any send. Identical validated duplicate snapshots can share an entry; differing answer, observation identity or preserved history must not be chosen silently. This target replaces v1 cache behavior that treated an undecodable held answer as a miss: no paid send repairs a damaged store.

Writable SQLite conversion is one idempotent transaction before lookup. Keep original rows until the new index commits; any failure rolls back completely. Read-only replay builds the validated v2 index in memory and changes no file bytes or metadata. Preserve strict offline replay of committed fixtures. Keep the legacy `thinkthen.recording/1` converter and its original-request byte-for-byte validation and quoting rules; no address is invented. Its good answers receive identity from original validated parts before conversion supplies synthetic metadata.

No new migration command, discovery request, automatic backup mechanism or supported downgrade guarantee is added. Old/new concurrent writers are unsupported. A user who needs an old writer must use an unmigrated copy made before upgrade. Existing `cache convert` remains the explicit fixture conversion command. Usage stays count-only and is never migrated through answer records. `meta.requests` uses v2 question keys after adoption, requiring one controlled example/fixture update by 0444.

## Legacy observation identity

Preserve a validated existing ObservationId. Otherwise derive lowercase SHA-256 of:

```text
F("thinkthen.legacy-observation/1", original_verified_question_key,
  canonical_answer_JSON, canonical_existing_metadata_JSON)
```

The original key part is its 64-character lowercase hexadecimal text. The metadata object includes **only validated fields already present in the saved source**, in this order: `answered_by`, `input_tokens`, `output_tokens`, `taken_at`, `origin`. Omit absent fields; preserve an existing valid null token count as null. Use the original validated values, never the current clock, retrieval origin, filename, fabricated timestamp or converter-generated provenance. Canonical answer JSON uses the saved question's typed decoder/serializer. An empty metadata object is `{}`.

For existing v1 question-store rows, these metadata columns exist: retain their validated values, including a stored `taken_at:0` or `origin:"converted"`. Original recording/1 envelopes lack timestamp and storage origin. Their reply's validated reported model/usage can contribute existing metadata (original reported token counts, not conversion-generated question shares), but **do not require or invent `taken_at` or `origin`** for identity. Derive each original logical question key from its validated original request parts before quoting/rekeying; it is not the whole exchange digest. Read-only conversion uses exactly the same derivation without writing anything.

Reason: `engine/store/fixture.rs` requires timestamp/origin on question-store rows, while `core/recording.rs::Entry` has neither, and its converter assigns synthetic history. Requiring those values on original exchanges would break valid offline replay; hashing freshly invented values would make identity depend on conversion or retrieval. Indistinguishable legacy snapshots cannot recover distinct historical occurrences. Identity is correlation, not tamper attestation.

## Landed v1 storage

The following clauses are moved unchanged from recording.md. They describe behavior already built, not completion of the v2 target above.

## The question store

Every command keeps one answer for each question, by ADR 0111.

A question's key is the SHA-256 of the adapter, the address, the model, the shared state and the wire question as sent. A live store is `thinkthen.sqlite` in the folder that `--cache`, `--record`, `THINKTHEN_CACHE` or the default cache names. A committed fixture is `thinkthen.jsonl` in the same folder, with the same rows. The file uses SQLite's rollback journal with full sync and secure delete, and it is created with mode `0600`. Network filesystems are unsupported, because SQLite's locks are not reliable there.

The default cache and an explicit recording use this same key. For one-document text, `hello\n` and `hello\r\n` put different line endings in the quoted evidence and make different keys. The literal text `{"a":1}` and `{"a":1.0}` likewise differs when sent as text. A replay of one spelling misses the other, and a cache sends a new request. The rule applies to the encoded question bytes, not every original file byte: `--lines` strips a record terminator, and JSONL or a selected JSON value can be parsed and re-encoded before the request is built. Use recordings made with the same question, backend, request settings, and encoded evidence when transferring a run; do not expect a cosmetic text change to replay.

| Mode | Look up | Send | Write |
| --- | --- | --- | --- |
| `--replay DIR` | yes | nothing; a miss exits 5 | no |
| `--record DIR` | no | every question | yes, replacing |
| `--cache DIR`, both options on one folder, `THINKTHEN_CACHE`, the default cache | yes | misses | yes |
| A cache with `--refresh-cache`, or the model `jev-latest` | no | every question | yes, replacing |
| `--no-cache` | no | every question | no |

A run packs only the questions no stored answer covers. A longer run over the same records therefore sends only the new records' questions. Within one run, a question already on its way is not sent again, and every record that needs it waits for that answer. Two processes may write one store at once. A write waits for the other's commit, and every wait is limited to 30 seconds in all.

`--replay DIR` loads `thinkthen.jsonl` when it exists, and otherwise opens `thinkthen.sqlite` read-only. A question the folder lacks exits 5 with ``the replay folder holds no answer for question `KEY`; the key is the SHA-256 of the adapter, address, model, shared state and question as sent``. A folder that holds both files exits 5 with ``the replay folder holds both thinkthen.jsonl and thinkthen.sqlite; run `thinkthen cache convert DIR` to merge them into thinkthen.jsonl``. A read-only replay that meets a write that did not finish exits 5 and says to open the folder once with write access. A stored answer that no longer decodes is a miss under a cache and a local failure under `--replay`.

A partial reply stores its good answers. Its failed question fails its record, as before, and is never stored, so the next run under the same folder asks only that question. A replay of that question is a miss at exit 5. A too-large request that halves stores both halves' answers. The refused whole request stores nothing.

Each stored answer keeps its share of its request's reported usage, split evenly over the request's questions with the remainder to the earliest. A row's usage sums its questions' shares, live or stored. A row whose answers all came from the store reports `meta.cached: true` and zero requests sent. `meta.requests` lists the row's question keys in answer order. `--facts` counts `cache_answers` as questions answered from the answer cache: the default cache, `THINKTHEN_CACHE` or `--cache`. An answer from a `--replay` or `--record` folder never counts, even with both options on one folder, so a run answered wholly from a replay folder prints `"cache_answers":0` beside `meta.cached: true`.

The repository commits no `thinkthen.sqlite`. A new live recording goes to a scratch folder with `--record`, and `thinkthen cache convert` merges it into the fixture. An old cache of digest entries is ignored by these commands until `cache convert` runs on it. The repository's committed folders hold only their fixtures; the finished probes keep their old entries as history.

- A stored answer holds the question and the reply's answer, never a header. No key can reach a recording.
- Live attempt timings and screened response request IDs stay in opt-in memory or current detailed output. Recording and replay keep no header or timing history; replay emits no current attempt event.
- A cache or recording answer is trusted for its answer. Whoever can write a folder that `--cache`, `--record`, `--replay` or `THINKTHEN_CACHE` names decides the answers read from it, and a changed answer replays with no sign. Keep such a folder private to the people whose answers it holds. Do not restore a shared cache across a trust boundary.
- On Unix, a command warns once before using an existing named cache or recording folder when another user owns it or its other write bit is set. A group write bit alone stays quiet, as it does for the configuration file, because the usual 002 umask sets it. A group-writable folder is trusted to its whole group, so a cache shared within a group that should not trust every member uses mode 0700. The warning names no path or answer and does not authenticate an answer. Newly created owner-private folders stay quiet. On non-Unix systems this permission check is unavailable; keep named folders private to trusted writers.
- The SQL extensions for PostgreSQL, DuckDB and SQLite apply the same ownership and other-write rule as a refusal, because a SQL call has no warning line. A named cache, record or replay folder that fails it when the extension builds its engine is a usage failure before any request, and the sentence names no path. They use no platform default cache, so an answer cache runs only in a folder that `THINKTHEN_CACHE` or the extension's cache setting names. In PostgreSQL every role runs as the server's one operating-system user, so every role whose calls reach a named folder shares its answers and bypasses row-level security. Ticket 0318.
- No refusal repeats a stored field. A fixture line that does not parse is refused with its line number and never its text, because every field under `DIR` is untrusted text: it is unbounded, it can carry a control byte, and it can quote the evidence back.
- A stored answer keeps the address the request went to, its path included, so a token must never sit in the path of a base. A base carrying user information, a query, or a fragment is refused for the same reason.
- A `DIR` the tool creates is readable by its owner alone, and so is `thinkthen.sqlite`, because a recording holds the evidence. On Unix that is mode `0700` for the folder and `0600` for the file, whatever the umask says. A `DIR` that already exists keeps the mode it has.

Every recording storage failure exits 5 and prints `thinkthen: the recording folder could not be read or written; check its permissions and free space`. The platform default cache is the folder no option or `THINKTHEN_CACHE` names. When it fails, the command prints `thinkthen: the default cache folder could not be read or written; check its permissions and free space, use --no-cache, or set THINKTHEN_CACHE to another folder` instead. The message carries no path, entry bytes, evidence, credential, or operating-system error. On Unix the process safely handles `SIGXFSZ`, so a file-size limit reaches this failure and its normal rollback instead of terminating the process. The command installs that handler before it reads input. The engine library installs no signal handler, so an embedding host keeps the disposition it chose. A later disk-full or sync failure can still discard an answer the backend already returned.

A successful `--record` run can replay the answers it printed. A refreshed answer cache is working state: if one question refreshes twice to different answers during one run, later replay reads the last replacement rather than an earlier printed answer. A repeated trial that wants another backend answer uses a fresh folder.

The same request can return another probability. Experiment 212 sent 100 messages twice on `jev-1.13.0`. Where an answer moved, the mean move was 0.02. Answers away from the middle moved by at most 0.03, and borderline answers, 0.33 to 0.67, moved by up to 0.08. That sample was one question on one day, and half of it was picked as borderline. The offline [0163 drift record](../sdlc/records/0163-answer-drift.md) found 4,075 differing digests among 5,111 repeats in the pinned benchmark recordings; 310 had a gap above 0.1, 430 crossed 0.5, and the largest gap was 0.45. The repository probes had no repeats, and its other recordings had three repeats with no probability gap. A borderline answer can cross the cut from one call to the next. The not-sure band in [threshold.md](threshold.md) marks where a second look pays. ADR 0010's amendment of 2026-09-25 withdraws the earlier claim that repeats match.
