# Explicit files and folders

Status: **Settled** for version 0.2, by ticket 0420's reviewed additive contract.

All ten judging functions accept explicitly selected files and folders. A text string never becomes a path by inference. The shared reader lives outside core. File names, physical positions and source occurrence identities never enter model evidence, question digests or cache keys.

## Reader

An explicit reader defaults to physical lines. A line drops its final LF and an immediately preceding CR. A window joins a positive number of physical lines without overlap, retains internal endings and drops only the final ending. A whole file retains all original content and endings. Blank line units and all-whitespace windows are skipped after size validation. Physical positions include skipped lines. Whole-file mode supplies full content rather than the file name.

Paths retain operand order. Each folder recursively includes hidden regular files and sorts its regular descendants by relative path. Descendant symlinks are skipped. Explicit file symlinks keep the existing open policy. Missing or unsupported operands and enumeration failures fail before admission. Discovered paths that cannot be represented exactly as UTF-8 fail before content admission; the native source reader also refuses such explicit paths. A sorted manifest has at most 16 MiB of encoded path storage. Content opens and reads incrementally through one active handle. Later read and content failures can follow completed requests. A reader stops at the first content failure and never admits its unread tail.

Each record retains the 16 MiB bound. Source-only rank admits at most 16 MiB of cumulative original evidence through its existing scheduler; it refuses an excess record without retaining it or reading the tail. Earlier admitted CLI requests may complete before a later excess; stdout stays withheld. Generic rank keeps its existing unlimited aggregate behavior when no engine count cap is set. Find retains its complete-set 16 MiB bound and 2–255 candidates, or at most 254 candidates when offering none. Recognize retains its 600,000-byte default text bound. Relate retains 255 distinct entities. Source relate on CLI, SDK and SQL admits at most 255 source records before deduplication. CLI and SDK source relate retain at most 16 MiB of cumulative original evidence and at most 16 MiB of serialized expanded edges, counting both endpoints and JSON escaping before retaining the expansion or emitting output. An excess fails the complete source call; no endpoint pairs are truncated. No aggregate silently truncates or replaces a complete set with chunked semantics.

Streaming command functions retain already emitted results after a later read failure. Rank withholds partial results. Find and relate read their complete sets before asking. Whole-call library functions keep their whole-call error behavior.

## Source records

`SourceRecord<T>` keeps `record: T`, `file`, `first_line` and `last_line`. Lines are one-based and inclusive. Sources are separate carriers; no metadata is inserted into caller objects or annotated values. Equal original records remain separate source occurrences.

Existing calls keep their result types. Explicit source-mode rows keep original `input` or `record`, answer `value`, and flat source coordinates. Existing command details retain `position: {file,first,last}`. Find retains the selected source record and physical range. Recognition retains local host offsets and adds the file and physical span lines. The span's exclusive end maps through its last included character. LF counts once, including CRLF. Spans never cross source records.

Located recognition accepts text units. It rejects decoded JSON fields because they have no parser source map. Existing field recognition remains available. Relate accepts explicit JSONL entity records using its existing parser. Their endpoints identify the containing physical record. Plain-text relate uses complete record content as the entity name and `*` as kind. It never runs recognize implicitly. Equal source name/kind pairs share one admitted entity; accepted edges expand back to source occurrence pairs. Existing non-source duplicate rejection remains unchanged. Document endpoints identify complete documents, without inventing supporting sentences.

## Surface spellings

Every command accepts repeated `--input FILE_OR_FOLDER`. A folder, `--unit line|file`, or `--window N` selects located output. Existing file calls without reader options retain their framing defaults. Contradictory unit and framing choices fail before admission. `filter --files-only` prints each matching file once, in first-match order.

Rust exports `read_files(paths, ReaderOptions)` and the `SourceRecords` iterator. Each item is `Result<SourceRecord<String>, Error>`. `SourceRecord<T>` implements text evidence selection when `T` implements `AsRef<str>`. `FileReader::new(file, authorized_handle, options)` supplies identical framing to hosts that own filesystem access. `SourceRecord::span_lines(start,end)` maps native Unicode scalar offsets to inclusive physical lines.

Python exports `read_files(path_or_paths, *, unit="line", window=None)` as a reusable explicit selection. Existing engine function names recognize this type. Old inputs retain old result types. Source results retain original records and coordinates.

The C JSON request envelope accepts `source: {"paths":["documents"],"unit":"line"}`, `source: {"paths":["documents"],"unit":"window","window":4}` or `source: {"paths":["documents"],"unit":"file"}`. Unit defaults to line. Source replaces evidence, records or units and rejects mixtures before reading. Existing question grammar and the outer value/facts reply remain unchanged. The existing planning door accepts the same source selection.

DuckDB and SQLite expose `thinkthen_read_files(path[, reader_options])` with `ordinal, record, file, first_line, last_line`. Existing SQL judging functions consume these rows. Find maps its selected index back to ordinal; recognize maps spans through the native mapper; relate joins both endpoint ids to source rows. DuckDB obtains authorized handles through its filesystem and respects external-access, path restrictions and disabled filesystems. For pinned DuckDB versions whose listing API omits failures and unsupported entries, strict local metadata enumeration occurs in C++ only after the executing filesystem authorizes each operand, directory and descendant. All content remains on DuckDB handles; Rust opens no DuckDB path. Remote/custom filesystem routes are refused rather than reinterpreted as local metadata. SQLite uses caller filesystem permissions. `thinkthen_span_lines(record, first_line, start, end)` calls the shared native Unicode scalar mapper, returning physical line coordinates as a DuckDB struct or SQLite JSON object. PostgreSQL server paths remain deferred.

Paragraphs, overlapping windows, discovery filters, watch mode, archives and binary extraction remain out of scope. The ten functions remain the only judging functions.

## Explicit image files (0447/0448)

Add `ReaderMedia::{Text,Image}` and `InputReaderOptions {reading:ReaderOptions,media:ReaderMedia}`; default media is text. `read_inputs` and handle-based `InputFileReader` use the shared reader. Image mode requires file units with no window. `SourceItem::Text(SourceRecord<String>)` retains the existing required text coordinates. `SourceItem::Image(ImageSourceRecord {record:ImageInput,file:String})` omits line fields entirely. Original media is decoded from content, never extension. Each folder file is a separate input, preserving enumeration order and duplicates. Filenames never enter image state or identity.

CLI `--image FILE` may repeat to attach originals to one question, with optional stdin text. It cannot mix with --input/media/unit/pointers/record framing. Detailed attachments retain ordered names in position.images; image file details retain file and position.file with no first/last coordinates. Seven text-only functions refuse the explicit image input before transport. [ADR0121](../sdlc/planning/adr/0121-native-image-input-and-route-admission.md) specifies native types, limits and exact routes. C/host/SQL image doors remain owned by their separate tickets.

Native complete source relations expose `CompleteRelated::source_edges()` beside semantic `value()` edges. Each typed source endpoint exposes its original input ordinal, selected entity, complete `RawRecord` and actual `SourceLocation`. The complete source result serializes expanded occurrence endpoints with `name`, `kind`, `record`, `file` and the supplied optional line range. The ordered arbitrary caller originals remain in the existing `CompleteRecord` input carrier. Mixing located and unlocated records in a source relation call refuses before lookup/send. No location is inferred and no host result parser is required.
