# Explicit files in libraries

Select paths explicitly through the host's file input selector. A text argument is always evidence; it never becomes a filename. Each package README describes its named calls and file input types. The [file contract](../specification/files.md) owns the shared reader behavior, and the [canonical Request grammar](../specification/request.schema.json) defines source selectors.

All readers use the native engine's file parser. Reader options select lines by default, positive nonoverlapping line windows, or complete files. Files follow operand order; real folder contents follow sorted relative paths. Unrepresentable UTF-8 source paths fail before content admission. Physical lines are one-based and inclusive, including skipped blank lines. Windows preserve internal LF/CRLF bytes; file units preserve the whole file.

Located input records retain the original record, file and first and last physical lines. Complete typed results preserve source metadata on rows, find candidates and relation endpoints. Recognition keeps local Unicode scalar offsets and adds physical line coordinates to its spans. Field-based recognition with `on` is refused for a source selection because decoded fields have no physical source map.

Source metadata stays outside caller records, annotated values, model payload and cache identity. For plain-text source records, relate uses the complete record content as the name and `*` as the kind; it never runs recognize implicitly. Find and relate retain their native complete-set and distinct-entity limits. The examples make no promise about model quality.

Request preview reads selected evidence to estimate supported fixed atomic questions without a key or request. Preview support and ownership follow the native planning contract; preview support does not limit the functions that accept file inputs.

The [files and folders page](https://thinkthen.dev/start/files/) runs the ten CLI and Python examples over the shared [file corpus](../specification/fixtures/files/).
