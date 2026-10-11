# 0420: Read files and folders through all ten functions

Status: COMPLETE.

Opened as: 2026-10-11. The whole text-file outcome passed source review, correction of six behavior defects and focused fix review. Full tests and lint run on the landing commit.
Milestone: 0.2

## Outcome

Every judging function accepts explicit files and folders through one reader on each surface. A caller selects a physical line, a nonoverlapping window of lines, or a whole file. Whole-file mode makes a folder a set of documents. CLI and Python run all ten approved examples over one shared fixture folder. DuckDB runs the same ten examples, and SQLite also supplies its folder table reader. Every result identifies its original record and file. Find identifies its selected physical line range, recognize identifies each span's physical line range, and relate identifies both endpoints' sources. Existing text, record and column calls work unchanged.

Build after the current Windows corrections. The final release rehearsal covers this change.

## Evidence

- Starts from: Ian's approved 2026-10-05 files outcome. Ticket 0401 and `specification/records.md` already establish seven-function multi-file input, bounded line windows, physical positions and metadata excluded from provider requests and cache identity. Ticket 0401 retains the window finding from experiment 434 (asked 2026-10-03); this ticket needs no new accuracy experiment. `cli/intake.rs` already implements this reader shape. `cli/find.rs` retains selected original bytes and positions but reads one source. `cli/recognize.rs` still has its own source path. `cli/relate/input.rs` and `libraries/c/src/door.rs` use the shared record parser. `libraries/c/src/call.rs` supplies all ten functions through the closed JSON envelope. DuckDB and SQLite already return relation endpoints through caller ids and preserve the complete-set relation meaning.
- Keeps: the ten judging verbs; current scalar/list/column results, question files and question sets; physical-line and CRLF rules; original records; saved `on` and text handling; all current bounds and failure exits; relation meaning and the 255 distinct-entity limit. Source metadata never becomes evidence, a question member or a cache key. A string never becomes a path by inference. Core opens no file and reads no environment, socket, clock or process.
- Changes: an explicit located reader, additive source inputs and source-aware results across CLI, Python and the C JSON door; folder table readers in DuckDB and SQLite; shared fixture examples and the small specification/API amendment needed for these inputs.
- Proof: run the ten approved examples over the shared fixture folder in CLI, Python and DuckDB using saved exchanges. The same fixture checks ordering, physical positions, CRLF, Unicode, skipped blank lines, final short windows, whole-file records, original bytes, duplicate occurrences, missing/unreadable/special files, invalid UTF-8 and existing size/count boundaries. Count zero requests for invalid reader options, missing operands and enumeration failures before first admission, and for replay/cache runs. Later streamed content/read failures may follow requests. Streaming CLI functions retain their already-emitted prefix; rank withholds partial output, find and relate require complete sets, and whole-call APIs retain their existing whole-call error behavior. Compare request bodies and keys for the same evidence at a different path. Check recognize spans and both relate endpoints. SQLite verifies its table reader and a joined judgment. Retain existing call compatibility checks. Run focused checks while building, then full tests, lint, specification and affected surface checks on the landing commit. No paid calls.
- Native intake amendment: additive `try_details_many_with`, `try_filter_with`, `try_annotate_with`, `try_rank_with` and `try_find_with` accept `Result<T, Error>` inputs. They share the original ordered scheduler and packing. Successful inputs preceding a reader failure retain their ordered rows; the terminal error and batch carry final drained counts. A reader failure on the first over-limit item takes precedence over the record-limit refusal. Whole-call bindings discard the partial array. `Error::with_facts` replaces facts without adding counts or changing the error identity, retryability or denial reasons; recognize attaches its tally after counting the failed call exactly once. Rank and find enforce an engine cap during admission; find also enforces its original count and 16 MiB complete-set limits before retaining an excess unit. Generic rank remains uncapped without an engine cap. Source-only rank uses a 16 MiB cumulative original-evidence admission bound to avoid unbounded folder contents. Details serialize original text alone; coordinates stay in the outer located row.
- Defers: PostgreSQL server paths; paragraph/overlap windows, discovery filters, watch mode, archives, PDF/binary extraction, implicit paths, and relation sets beyond 255 distinct entities. Other SDKs follow through the C door in this ticket without extra tickets unless a binding breaks. This ticket introduces neither a grep alias nor an eleventh judging function.

## API decisions to review

### Reader and source contract

Use `SourceRecord<T>` with original `record: T`, `file`, `first_line` and `last_line`. Lines are one-based and inclusive. Keep selection and metadata separate. Rust implements `AsRef<str>` for text source records so existing generic filter/rank/find and record pipelines can retain them. A source occurrence has an internal ordinal for joins and duplicate records; it never enters model state.

Keep existing `position: {file,first,last}` on existing CLI details. New explicit source-mode outputs carry `file`, `first_line` and `last_line` beside original `input`/`record` and answer `value`. Do not add metadata fields inside an input JSON object or annotated value: caller keys and named answers could collide.

Read explicitly named paths in argument order. Sort a folder's regular files by relative path before reading them; recurse into real subdirectories, skip descendant symlinks, and include hidden regular files. No inferred extensions or ignore rules. Explicit symlink file operands keep the existing file-open policy. An empty folder yields no records and no requests. Validate options and operands, enumerate folders and reject unsupported enumerated file kinds before first admission; these failures send nothing. Open and read admitted files incrementally. Later permission changes, content errors and read failures may follow requests. Streaming CLI functions retain emitted prefixes; aggregate functions and whole-call APIs retain their existing failure behavior. Do not pre-read folder contents or read files twice to promise zero sends. Do not retain an unbounded set of opened handles or whole-folder contents. Bound a sorted folder manifest using the existing 16 MiB byte bound; refuse an oversized manifest before requests rather than adding a configurable discovery subsystem.

Reuse bounded `Chunks`, `Reading` and `Intake` behavior. Extract reusable I/O and source metadata outside core instead of adding another record parser. SDKs use the shared native reader. DuckDB supplies authorized handles through its filesystem; do not bypass `enable_external_access`, `allowed_paths`, `allowed_directories` or disabled filesystems with host Rust reads. SQLite uses caller filesystem permissions. The pinned DuckDB 1.5.4/1.5.5 listing API silently omits special files and failed listings; strict local no-follow metadata enumeration uses a minimal C++ host classifier only after executing DuckDB filesystem authorization for each operand, directory and descendant (including disabled filesystem dispatch). Content opens and reads always use DuckDB handles, and Rust never opens DuckDB paths. Refuse remote/custom filesystem routes rather than interpreting them as local paths.

A line/window retains internal LF/CRLF content and drops only the final record ending under the existing rules. A whole file retains its whole evidence. Apply the existing 16 MiB per-record bound, find's complete-set 16 MiB and 2–255 (254 with none) bounds, recognize's default 600,000-byte text bound, and relate's 255 distinct-entity bound. Source relate on CLI, SDK and SQL admits at most 255 source records before deduplication. CLI and SDK also bound cumulative retained original evidence and serialized expanded edges separately at 16 MiB. Count JSON escaping and both endpoint records before cloning an expanded SDK edge or retaining CLI endpoint references; refuse excess with the complete-set failure behavior and no partial output. Duplicate occurrence expansion stays complete within these bounds. No silent truncation or chunked replacement for aggregate find/relate. Source wrappers retain the same question-set parsing and plain-text-versus-JSON behavior that annotate already has.

### Surface spellings

CLI: allow repeated `--input FILE_OR_FOLDER` for all ten. Add `--unit line|file`; existing `--window N` selects N-line windows. Refuse contradictory unit/framing choices before sending. Existing single-file invocations keep their current behavior and output. A folder or an explicit unit selects located output. Every explicit reader uses line units by default on every surface. Whole-document examples select `--unit file`; windows select `--window N`. Existing CLI file calls without a reader option keep their current framing defaults. `filter --files-only` prints each matching file once in first-match order. It changes output only. Existing positional labels never become file operands.

Python: `read_files(path_or_paths, *, unit="line", window=None)` returns a typed reusable source selection whose iteration yields located records and whose native reader can be consumed without discarding provenance. `engine.filter(question, read_files("documents"))` and the other nine existing function names recognize this explicit type. Located results retain original source records with values; old inputs retain old result types. Decide/choose/tag/score and annotate map over these records. Recognize maps over source records and returns located spans. Find returns its selected source record. Relate consumes one complete source set. Update stubs and export the reader and source types; paths remain explicit only in the reader.

C JSON: the explicit reader defaults to line units. Add envelope `source: {"paths":["documents"],"unit":"line"}` or `{"paths":["documents"],"unit":"window","window":4}` or `{"paths":["documents"],"unit":"file"}`. It replaces evidence/records/units and refuses mixtures before reading. Keep each existing question grammar and the outer reply with value and facts. Seven mapped functions return located rows; find returns a located selection; recognize returns located record results and spans; relate returns edges with located endpoints. This lets every SDK using the existing JSON door reach files without changing ten separate typed ABIs. Add source support to the existing planning door rather than teaching another planner.

SQL: both extensions expose `thinkthen_read_files(path[, reader_options])` as a table with `ordinal, record, file, first_line, last_line`. Reader options use the same explicit unit/window contract and default to line units. All seven row judgments keep these source columns in the SELECT. Rank orders these rows using its existing probability/rank API. Find aggregates `record ORDER BY ordinal`, then maps its selected index back to the row. Recognize joins its returned span rows to the source row and computes physical lines using the shared native offset mapper. Relate's read-only query emits `ordinal AS id`, complete record content AS name and `'*' AS kind`; join both returned endpoint ids back to reader rows. Use a committed fixture view/table where DuckDB's separate relate connection requires it. Do not add ten filesystem-aware SQL judging functions.

### Span and endpoint rules

For text units, recognize keeps its existing host offset units. Map start and exclusive end to inclusive physical lines by counting LF before the span in the original record and adding `first_line`; use the last included character for end-line calculation. CRLF counts once. Spans never cross source-record or file boundaries. Preserve local start/end offsets and add file/physical lines; do not reinterpret old offsets as whole-folder offsets.

New located recognize input uses text units. Existing structured/field recognition remains available unchanged. Do not claim an exact physical line inside a decoded multiline JSON field without a parser source map; reject that new combination with a useful local message. JSONL source entities for relate use the existing explicit JSONL framing and shared parser, so their endpoint source is the containing physical record range.

For plain text source records, relate uses the full record content as the entity name and kind `*`, as its existing lines mode does. Whole-file mode supplies the full document, not the filename, so the model sees the content it judges. Structured entity records retain caller-selected names/kinds. Relate still asks about supplied names and never silently runs recognize. Equal source name/kind pairs share one admitted entity and fan each accepted edge back to source occurrence pairs, as SQL already does. Existing non-source duplicate rejection stays unchanged. An endpoint carries its original source record and range. A document endpoint names its complete document range, not an invented supporting sentence.

These are the additive Rust reader and fallible-intake signatures implemented and reviewed in this ticket.

### Added public declarations

```text
ReaderOptions::unit: SourceUnit
ReaderOptions::window: Option<usize>
SourceRecord::file: String
SourceRecord::first_line: usize
SourceRecord::last_line: usize
SourceRecord::record: T
SourceUnit::File
SourceUnit::Line
SourceUnit::Window
enum SourceUnit
fn CallOptions::started(self) -> Result<CallOptions<'a>, Error>
fn Engine::check_record_limit(&self, usize) -> Result<(), Error>
fn Engine::try_annotate_with<'a, I, T>(&'a self, &'a QuestionSet, I, CallOptions<'a>) -> Batch<'a, AnnotatedRecord<T>> where I: IntoIterator<Item = Result<T, Error>> + 'a, T: Evidence + 'a
fn Engine::try_details_many_with<'a, I, T, Q: DetailQuestion + ?Sized>(&'a self, &'a Q, I, CallOptions<'a>) -> Batch<'a, Row<T, Details>> where I: IntoIterator<Item = Result<T, Error>> + 'a, T: Evidence + Serialize + 'a
fn Engine::try_filter_with<'a, I, T>(&'a self, &'a Question, I, CallOptions<'a>) -> Batch<'a, T> where I: IntoIterator<Item = Result<T, Error>> + 'a, T: Evidence + 'a
fn Engine::try_find_with<I, T>(&self, &Question, I, CallOptions<'_>) -> Result<Call<Found<T>>, Error> where I: IntoIterator<Item = Result<T, Error>>, T: Evidence
fn Engine::try_plan_with<Q, I, T>(&self, &Q, I, CallOptions<'_>) -> Result<PlanEstimate, Error> where Q: DetailQuestion + ?Sized, I: IntoIterator<Item = Result<T, Error>>, T: Evidence
fn Engine::try_rank_with<I, T>(&self, &Question, I, CallOptions<'_>) -> Result<Call<Vec<Ranked<T>>>, Error> where I: IntoIterator<Item = Result<T, Error>>, T: Evidence
fn Error::with_facts(self, Facts) -> Error
fn FileReader::new(impl Into<String>, R, ReaderOptions) -> Result<FileReader<R>, Error>
fn ReaderOptions::validate(self) -> Result<ReaderOptions, Error>
fn SourceRecord::span_lines(&self, usize, usize) -> Result<(usize, usize), Error>
fn enumerate_files(impl IntoIterator<Item = impl AsRef<Path>>) -> Result<Vec<PathBuf>, Error>
fn read_files(impl IntoIterator<Item = impl AsRef<Path>>, ReaderOptions) -> Result<SourceRecords, Error>
impl AsRef<str> for SourceRecord
impl Default for ReaderOptions
impl Default for SourceUnit
impl Deserialize<'de> for ReaderOptions
impl Deserialize<'de> for SourceRecord
impl Deserialize<'de> for SourceUnit
impl Evidence for SourceRecord
impl Iterator for FileReader
impl Iterator for SourceRecords
impl Serialize for ReaderOptions
impl Serialize for SourceRecord
impl Serialize for SourceUnit
struct FileReader<R>
struct ReaderOptions
struct SourceRecord<T>
struct SourceRecords
type FileReader::Item = Result<SourceRecord<String>, Error>
type SourceRecords::Item = Result<SourceRecord<String>, Error>
```

## Build slices

1. Record the additive contract and build the shared bounded reader outside core. Add CLI folder/unit support and source locations through all ten, reusing intake and current parsers.
2. Extend the C JSON envelope and Python explicit reader dispatch. Preserve old-call contracts and original inputs. Update the other JSON-door SDK examples in the same ticket as needed.
3. Add DuckDB and SQLite table readers through each host's filesystem rules. Publish and replay the ten CLI/Python/DuckDB examples from one fixture folder. Run the affected gates and land the complete outcome.

## Choices the owner can overturn

Recursive real-directory traversal with descendant symlinks skipped; a 16 MiB folder-manifest bound; `--unit`/`read_files`/`source` spellings; new source-mode carriers alongside unchanged old carriers; content-as-name relate with duplicate occurrence expansion. These are additive choices within the approved outcome. The exact JSON-field span map remains deferred; ordinary text spans and JSONL endpoint ranges fulfill the approved examples without new model semantics.
