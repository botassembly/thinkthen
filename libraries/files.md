# Explicit files in libraries

Select paths explicitly. A text argument is always evidence; it never becomes a filename. All readers use the native engine's file parser. Reader options are `unit: "line"` (the default), `unit: "window", window: N` for a positive nonoverlapping line window, or `unit: "file"` for complete documents. Files follow operand order; real folder contents follow sorted relative paths. Physical lines are one-based and inclusive, including skipped blank lines. Windows preserve internal LF/CRLF bytes; file units preserve the whole file.

Python uses `read_files(path_or_paths, *, unit="line", window=None)`. Iteration yields immutable `SourceRecord` values with `record`, `file`, `first_line`, and `last_line`. Passing the selection to any of the ten existing engine functions returns `Call` values with `Located(source, value)` rows. Find returns one located selection or `None`; relate returns edges whose source and target are located occurrences. Recognize keeps local Unicode scalar offsets and adds physical line coordinates to its spans. `on=` frame recognition is refused for a source selection because decoded fields have no physical source map.

```python
import thinkthen as tt
engine = tt.Engine()
documents = tt.read_files("documents", unit="file")
engine.decide("Does this require attention today?", documents)
engine.choose("Which document is this?", documents, options=["policy", "contract"])
engine.tag("Which topics appear?", documents, labels=["refund", "support"])
engine.score("How urgent?", documents, levels=["low", "high"])
engine.filter("Does this contain a support contract?", documents)
engine.rank("Which needs attention first?", documents)
engine.find("Which contains a refund policy?", documents)
engine.annotate("questions.json", documents)
engine.recognize(documents, kinds=["person"])
engine.relate(documents, relations={"supports": ("*", "*")})
```

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

Other SDKs expose an explicit envelope helper; their existing typed text/record methods keep their original arguments. The helper takes the same ten-function question object and explicit paths/reader options, then uses the native source dispatch. It returns located JSON values and facts rather than the old scalar result type.

| SDK | Explicit entry point |
| --- | --- |
| TypeScript | `engine.files(questionObject, paths, readerOptions, callOptions)` |
| Ruby | `engine.files(questionHash, paths, unit: "file")` |
| R | `tt_files(questionList, paths, unit = "file")` |
| C++ | `tt::files(engine, questionJson, paths, "file")` |
| C# | `engine.Files(questionDictionary, paths, unit: "file")` |
| Go | `engine.Files(ctx, questionMap, paths, ReaderOptions{Unit: "file"})` |
| Java | `door.files(questionJson, paths, "file", null, -1, null)` |
| Kotlin / Scala | Their caller adapters expose `files(questionJson, paths)`; Java's door supplies reader options. |
| Swift | `engine.files(questionDictionary, paths: paths, unit: "file")` |
| PHP | `$engine->files($question, $paths, 'file')` |
| Dart | `door.files(engine, questionMap, paths, unit: 'file')` |
| Zig | `engine.files(questionJson, paths, .{ .unit = "file" }, .{})` |
| Ada | `Files(Client, Question_JSON, Source_JSON, Result, Error)` |
| Objective-C | `[client files:questionJSON source:sourceJSON deadline:-1 token:nil failure:&failure]` |
| COBOL | `TT-FILES`, with question/source JSON buffers and byte lengths, then the existing deadline/result/failure arguments |

C callers can use `thinkthen_call_opts` with the complete source envelope. Ada, Objective-C and COBOL accept the reader selection as a JSON object containing `paths`, `unit`, and optional `window`. Their adapters compose JSON only; they do not open or parse source files. COBOL's existing 8192-byte request/result limits still apply. Rust callers use `thinkthen::read_files` and retain source records when judging their selected evidence.
