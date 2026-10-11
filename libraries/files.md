# Explicit files in libraries

The [files and folders page](https://thinkthen.dev/learn/files/) runs all ten CLI and Python examples over the shared `specification/fixtures/files` corpus.

Select paths explicitly. A text argument is always evidence; it never becomes a filename. All readers use the native engine's file parser. Reader options are `unit: "line"` (the default), `unit: "window", window: N` for a positive nonoverlapping line window, or `unit: "file"` for complete documents. Files follow operand order; real folder contents follow sorted relative paths. Unrepresentable UTF-8 source paths fail before content admission. Physical lines are one-based and inclusive, including skipped blank lines. Windows preserve internal LF/CRLF bytes; file units preserve the whole file.

Python uses `read_files(path_or_paths, *, unit="line", window=None)`. Iteration yields immutable `SourceRecord` values with `record`, `file`, `first_line`, and `last_line`. Passing the selection to any of the ten existing engine functions returns a `Result`. Its `.value` presents the ordinary answer; `.details` retains typed result rows with provenance. Each row's `to_dict()` converts it to JSON-compatible data. Find retains original input and physical sources on its candidates; relate returns typed edges whose source and target retain their coordinates. Recognize keeps local Unicode scalar offsets and adds physical line coordinates to its spans. `on=` frame recognition is refused for a source selection because decoded fields have no physical source map.

Use `Item`, `Image`, `Files` and `QuestionSource` with Python's named `Engine` calls. The [Python package guide](python/README.md) owns current examples and argument order.

The C JSON door accepts a `source` envelope in place of `evidence`, `records`, or `units`. Mixtures and invalid options fail before reading. Keep the existing question grammar:

```json
{"filter":"Does this contain a support contract?","source":{"paths":["documents"],"unit":"file"}}
```

The reply remains `{"value":...,"facts":...}`. Mapped values are arrays of `{"record": ORIGINAL, "file": FILE, "first_line": FIRST, "last_line": LAST, "value": ANSWER}`. Rank retains ordering and index/probability members. Find returns a located selection or null. Recognize returns located record results with located spans. Relate returns `{"edges":[...]}` with located source and target endpoints. Equal source entity name/kind pairs share one judgment entity; returned edges expand back to occurrence pairs. Existing record calls retain their duplicate-rejection behavior. A whole-call failure returns no rows.

Source metadata stays outside caller records, annotated values, model payload and cache identity. Relate judges the complete supplied record content as name and `*` as kind; it never runs recognize implicitly. Find retains its complete-set size/count limits and relate retains its distinct-entity limit. No model quality is promised by these examples.

The planning door also accepts `source` in place of `input`, for its four existing judgment verbs. It reads selected evidence to estimate the native plan, without a key or request:

```json
{"verb":"decide","question":"Does this require attention?","source":{"paths":["documents"],"unit":"file"}}
```

Each host supplies generated file input selectors to its named function. The [upgrade guide](UPGRADING-0.2.md#replace-removed-calls) links every package README and its current calls, including JVM `Engine`, Dart `Engine`, COBOL `TT_SESSION_*` and Apple Foundation `TTFoundationClient`. Generated request records replace the removed JSON/file helper APIs. Rust callers use `thinkthen::read_files` and retain source records when judging selected evidence.

COBOL request text and authored JSON scalars have a per-scalar representation limit; the [COBOL guide](cobol/README.md) describes overflow and borrowed native views. C callers retain `thinkthen_call_opts` and the frozen 0.1 source envelope described above.
