# ThinkThen for DuckDB

ThinkThen is ten functions on top of Jev, TypeSafe's decision model, or any model like it. It puts these decision models in your code, whatever model, language or database you use. Use Jev, OpenAI or a local model. Ask a typed question and get back `true`, `false`, a label or a number. Your code branches on the answer. A model can say not sure, and a failed call never looks like an answer. ThinkThen packs many records into few requests, saves each answer for reuse, and lets tests replay saved answers with no key and no network.

Use DuckDB 1.5.6 and 2.0 with the matching source host and archive from the [release downloads](https://github.com/botassembly/thinkthen/releases). Unpack `thinkthen-duckdb-0.2.0-x86_64-unknown-linux-gnu.tar.gz` and start `duckdb -unsigned`. Load the member for your host and platform; Linux x86_64 on 1.5.6 uses:

```sql
LOAD 'v1.5.6/linux_amd64/thinkthen.duckdb_extension';
```

For 2.0 on Linux x86_64, load `d4355d01c5/linux_amd64/thinkthen.duckdb_extension` with its matching [source host](https://github.com/duckdb/duckdb/tree/d4355d01c59362d57036ad377735dd5340276a5b). The [1.5.6 source host](https://github.com/duckdb/duckdb/tree/069cc9f9b5be802405797faecc284961b07c70ef) matches the first member.

Set your hosted backend key before starting DuckDB: `export THINKTHEN_API_KEY=...`. Backends include Jev, SystemOne, OpenAI and local models.

```sql
CREATE TABLE tickets AS FROM (VALUES
    (1, 'My parcel is a week late. Not okay.'),
    (2, 'Thanks for the quick help yesterday!'),
    (3, 'We are locked out of our account.'),
    (4, 'I was billed twice for one order.')
) t(id, body);
SELECT d.ordinal + 1 AS id, d.value AS is_complaint,
       d.probability
FROM (SELECT unnest(thinkthen_decide_details(
    'Is this a complaint?',
    (SELECT list(thinkthen_record(text := body)
                 ORDER BY id) FROM tickets))) AS d);
SELECT d.ordinal + 1 AS id, d.value AS team, d.probability
FROM (SELECT unnest(thinkthen_choose_details(
    {'choose': 'Which team owns this?',
     'options': ['billing', 'shipping', 'account']},
    (SELECT list(thinkthen_record(text := body)
                 ORDER BY id) FROM tickets))) AS d);
```

The extension is free and MIT licensed. Hosted models charge per token. Local models are free to use on your own machine.

[Backend setup](https://thinkthen.dev/models/) · [SQL reference](https://thinkthen.dev/databases/duckdb/)

## SQL values and details

The ordinary calls return only the plain value. `thinkthen_decide` returns a nullable boolean, `thinkthen_choose` a nullable option name, `thinkthen_tag` a list of labels, and `thinkthen_score` a number. SQL `NULL` means not sure for decide and choose. A failed plain call raises an error.

Each function has its own full-record route:

| Plain call | Full record |
| --- | --- |
| `thinkthen_decide` | `thinkthen_decide_details` |
| `thinkthen_choose` | `thinkthen_choose_details` |
| `thinkthen_tag` | `thinkthen_tag_details` |
| `thinkthen_score` | `thinkthen_score_details` |
| `thinkthen_filter` | `thinkthen_filter_details` |
| `thinkthen_rank` | `thinkthen_rank_details` |
| `thinkthen_find` | `thinkthen_find_details` |
| `thinkthen_annotate` | `thinkthen_annotate_details` |
| `thinkthen_recognize` | `thinkthen_recognize_details` |
| `thinkthen_relate` | `thinkthen_relate_details` |

A text input gives one typed record for decide, choose, tag, score, filter, annotate and recognize. A native record list gives a list of typed records. Expand that list with `unnest`, as in the example above. Rank and relate also return lists; find returns one selected record for its whole input set.

Every details record contains `ordinal`, `value`, `probability`, `result`, `facts`, `failure` and `observations`. `value` is the typed answer. `probability` is a number for decide, choose, filter, rank, find and relate, a span probability list for recognize, and SQL NULL for tag, score and annotate. `result` preserves the canonical answer, original input, question, provenance and answer identity. Authored JSON values remain JSON leaves inside typed structures.

`facts` belongs to the native call and repeats across its rows. Read it once; do not add repeated counts. Rank and relate put the call's observations only on the row whose `ordinal` is zero. Other rows carry an empty observations list. Use `ordinal`, not the displayed SQL row order, to identify that owner. Each relate row holds one typed edge in `value` and exactly that canonical edge in `result.value`. The canonical edge keeps entity names; the typed SQL edge uses the query's ids.

Rank, find and relate judge one whole set per call. Build rank input from the full table with `json_group_object`, build find input with `list(text ORDER BY id)`, and pass relate one query over all entities. Do not call them separately for each input row. Rank's plain table returns `key, rank, probability`; relate's plain table returns `relation, source, target, probability, either`.

## Native input

`thinkthen_record` accepts one `text`, `json` or `document` original. Optional members supply context, options, examples, seed spans, images and source locations. SQL NULL optional members mean absence; authored JSON null remains present. `thinkthen_location(file, first_line := NULL, last_line := NULL)` records source coordinates without sending the file name as model evidence.

`thinkthen_reading` supplies named reading controls such as `batch`, `attempts`, context and file selection. Pass it as the named `reading` argument to a details call. Native question objects, JSON question text and explicit `@file` questions share the same admission rules. A question set supplies the named questions for annotate. Use `thinkthen_source(paths, unit := 'line')` to read explicit files through DuckDB's own access rules.

Required SQL NULL operands return SQL NULL for scalar calls or no rows for set calls before inspecting other arguments. Details keep typed recoverable row failures separate from the value. Refusals before execution raise an error. A native failure carries its kind, message, retryable flag and any available stopping facts.

## Backend, recording and access

Configure one backend with `SET thinkthen_backend` and one model with `SET thinkthen_model`. A hosted backend reads `THINKTHEN_API_KEY` or that backend's own key variable. Set `thinkthen_record` to a recording folder to save answers. Set `thinkthen_replay` to that folder to answer offline without a key or a network request. `thinkthen_usage()` returns the process totals; `thinkthen_usage_status()` reports whether those totals were written.

DuckDB authorizes question files, source files, cache folders and recording folders through its own file system. Its `enable_external_access`, `allowed_directories`, `allowed_paths` and `disabled_filesystems` settings govern those reads and writes.

Relate runs one read-only SELECT on a separate connection to the caller's database. It sees committed tables. It cannot see temporary tables or uncommitted rows. The extension refuses nested relate calls and mutating statements. Query interruption cancels the running engine call.

## Explicit images

`thinkthen_image(bytes, mime)` and `thinkthen_image_file(path)` construct explicit image evidence. Pass an ordered list of these structures to `thinkthen_decide_images`, `thinkthen_choose_images` or `thinkthen_score_images`, or add them to a native record for its per-function details call. MIME is `image/png` or `image/jpeg`. Paths, URLs, arbitrary blobs and generic JSON never become images implicitly.

## Build and check

Run `tools/setup.sh --fetch` once to fetch the approved source hosts and dependency inputs. Run `tools/setup.sh` to check them offline. Run `check.sh PORT` to build the C++ extension and Rust bridge and run the focused local suites. With `THINKTHEN_ARTIFACT` set to a release archive, the check uses the installed extension instead of building it. The version authority is `tools/version.env`.
