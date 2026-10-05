# 0405: Ten functions on every surface

Historical source audit at `ff047d8c50ce39ed9ab0acd22695c4960a963d38`; later search/backend work is not included. Source-confirmed capabilities below do not imply execution coverage. See [the report](0405-audit-report.md) for outcomes, proof limits and gap owners.

Named means a dedicated command/API; JSON call means a generic call without a dedicated named method; Base-host list requires extracting a column into the base host. WHERE decide is intentional SQL composition. The dimensions below explain restrictions hidden by these short cells.

| Function | CLI | Rust | C | Ada | C# | C++ | COBOL | Dart | Go | Java | JavaScript | Kotlin | Objective-C | PHP | Python | R | Ruby | Scala | Swift | TypeScript | Zig | pandas | Python Polars | Rust Polars eager/lazy | Flutter facade / exported Door | DuckDB | PostgreSQL | SQLite |
| --- | --- | --- | --- | --- | --- | --- | --- | --- | --- | --- | --- | --- | --- | --- | --- | --- | --- | --- | --- | --- | --- | --- | --- | --- | --- | --- | --- | --- |
| decide | Named | Named | Named | Named + JSON | Named + JSON | Named + JSON | Named + JSON | Named + JSON | Named + JSON | Named + JSON | Named | Named + JSON | Named + JSON | Named + JSON | Named | Named | Named | Named + JSON | Named + JSON | Named | Named + JSON | Column/frame | Column/frame | Column/frame | Facade decide + Door | Named | Named | Named |
| choose | Named | Named | Named | JSON call | JSON call | JSON call | JSON call | JSON call | JSON call | JSON call | Named | JSON call | JSON call | JSON call | Named | Named | Named | JSON call | JSON call | Named | JSON call | Column/frame | Column/frame | Column/frame | Door JSON | Named | Named | Named |
| tag | Named | Named | Named | JSON call | JSON call | JSON call | JSON call | JSON call | JSON call | JSON call | Named | JSON call | JSON call | JSON call | Named | Named | Named | JSON call | JSON call | Named | JSON call | Column/frame | Column/frame | Column/frame | Door JSON | Named | Named | Named |
| score | Named | Named | Named | JSON call | JSON call | JSON call | JSON call | JSON call | JSON call | JSON call | Named | JSON call | JSON call | JSON call | Named | Named | Named | JSON call | JSON call | Named | JSON call | Column/frame | Column/frame | Column/frame | Door JSON | Named | Named | Named |
| filter | Named | Named | Named | JSON call | JSON call | JSON call | JSON call | JSON call | JSON call | JSON call | Named | JSON call | JSON call | JSON call | Named | Named | Named | JSON call | JSON call | Named | JSON call | Base-host list | Base-host list | Base-host list | Door JSON | WHERE decide | WHERE decide | WHERE decide |
| rank | Named | Named | Named | JSON call | JSON call | JSON call | JSON call | JSON call | JSON call | JSON call | Named | JSON call | JSON call | JSON call | Named | Named | Named | JSON call | JSON call | Named | JSON call | Base-host list | Base-host list | Base-host list | Door JSON | Named | Named | Named |
| find | Named | Named | Named | JSON call | JSON call | JSON call | JSON call | JSON call | JSON call | JSON call | Named | JSON call | JSON call | JSON call | Named | Named | Named | JSON call | JSON call | Named | JSON call | Base-host list | Base-host list | Base-host list | Door JSON | Named | Named | Named |
| annotate | Named | Named | Named | JSON call | JSON call | JSON call | JSON call | JSON call | JSON call | JSON call | Named | JSON call | JSON call | JSON call | Named | Named | Named | JSON call | JSON call | Named | JSON call | Column/frame | Column/frame | Column/frame | Door JSON | Named | Named | Named |
| recognize | Named | Named | Named | Named + JSON | Named + JSON | Named + JSON | JSON call | Named + JSON | Named + JSON | Named + JSON | Named | JSON call | Named + JSON | Named + JSON | Named | Named | Named | JSON call | Named + JSON | Named | Named + JSON | Column/frame | Column/frame | Base-host list | Door JSON | Named | Named | Named |
| relate | Named | Named | Named | Named + JSON | Named + JSON | Named + JSON | JSON call | Named + JSON | Named + JSON | Named + JSON | Named | JSON call | Named + JSON | Named + JSON | Named | Named | Named | JSON call | Named + JSON | Named | Named + JSON | Base-host list | Base-host list | Base-host list | Door JSON | Named | Named | Named |

## Inputs

**C++, C#, Go, Java, Kotlin, Scala, Swift, Zig, PHP, Dart, Objective-C, Ada, COBOL, Flutter facade / exported Door (all functions)** — confirmed behavior. Host strings/ordered string arrays. No built-in text file, line, JSONL, CSV or TSV reader; callers parse at their edge. A JSON record can be supplied as serialized text. Scalar library questions refuse nonempty on pointers; annotate question sets can project per-group JSON pointers. Relation input uses entity JSON records.

**JavaScript, TypeScript, Ruby (all functions)** — confirmed behavior. Scalar text and ordered host string collections for four judgments; filter ordered records; rank ordered records; find complete units; annotate serialized JSON record strings with set on pointers; recognize text; relate entities. Caller owns text-file/lines/JSONL/CSV/TSV framing.

**Python (all functions)** — confirmed behavior. Scalar text and ordered host string collections for four judgments; filter ordered records; rank ordered records; find complete units; annotate serialized JSON record strings with set on pointers; recognize text; relate entities. Caller owns text-file/lines/JSONL/CSV/TSV framing. Python eager reiterable inputs and lazy iterators for four judgments/filter; column/frame variants inventoried separately.

**R (all functions)** — confirmed behavior. Scalar text and ordered host string collections for four judgments; filter ordered records; rank ordered records; find complete units; annotate serialized JSON record strings with set on pointers; recognize text; relate entities. Caller owns text-file/lines/JSONL/CSV/TSV framing. R coerces ordinary vectors to character; rank/find use one-based place, not zero-based index.

**pandas, Python Polars, Rust Polars eager/lazy (all functions)** — confirmed behavior. Four judgments read Series/text columns; annotate frame chooses one text column and set pointers operate within serialized cell. Python recognize frame with on=; Rust lazy expressions exist only for four judgments. Native filter/rank/find/relate absent. Files and row framing remain caller adapters.

**CLI (decide, choose, tag, score, filter, rank, annotate)** — confirmed behavior. Whole text, --lines, --jsonl, --csv, --tsv; --input one named file; --field/resolved on projects evidence. Filter/rank default lines or JSONL with a pointer; no named file list/windows.

**CLI, Rust, C, DuckDB, PostgreSQL, SQLite (find)** — confirmed behavior. Whole ordered candidate set: CLI lines/JSONL plus field projection; libraries ordered evidence units; SQL ordered arrays/JSON text arrays. 2–255 candidates (2–254 with none). CLI bounds total original bytes; no CSV/TSV CLI adapter.

**CLI, Rust, C, DuckDB, PostgreSQL, SQLite (recognize)** — confirmed behavior. One text; CLI explicit record framing and field projection also available; Rust/C text; SQL text and host tables. Span offsets follow each host contract.

**CLI, Rust, C, DuckDB, PostgreSQL, SQLite (relate)** — confirmed behavior. Complete entity set with names/kinds; CLI JSON array or explicit JSONL/pointers, Rust Entity list, C entity JSON records, SQL table/keyed entity query. Adding/removing an entity changes the item.

**Rust, C (decide, choose, tag, score, filter, rank, annotate)** — confirmed behavior. Caller supplies evidence text and ordered record collections; caller parses files/lines/JSONL/CSV/TSV. Typed Evidence can project record evidence; scalar saved on pointers are refused; annotate set on supported.

**DuckDB, PostgreSQL, SQLite (decide, choose, tag, score, filter, rank, annotate)** — confirmed behavior. SQL text scalar or keyed text object collections; SQL loads/project fields from JSON/table/file readers. Caller owns CSV/TSV/JSONL reading. Annotate question-set on projects its records. NULL and sorted key-order adapters are host contracts.


## Descriptions

**C++, C#, Go, Java, Kotlin, Scala, Swift, Zig, PHP, Dart, Objective-C, Ada, COBOL, Flutter facade / exported Door (decide, choose, tag, score, filter, annotate, recognize)** — confirmed behavior. Question JSON preserves structured descriptions and ordered description maps; annotate carries descriptions per question; recognize carries kind descriptions.

**C++, C#, Go, Java, Kotlin, Scala, Swift, Zig, PHP, Dart, Objective-C, Ada, COBOL, JavaScript, TypeScript, Python, Ruby, R, pandas, Python Polars, Rust Polars eager/lazy, Flutter facade / exported Door (rank)** — design gap. Rank accepts plain question wording, not separate yes/no criteria or saved score levels on this route.

**C++, C#, Go, Java, Kotlin, Scala, Swift, Zig, PHP, Dart, Objective-C, Ada, COBOL, JavaScript, TypeScript, Python, Ruby, R, pandas, Python Polars, Rust Polars eager/lazy, Flutter facade / exported Door (find)** — confirmed behavior. Find candidates are the item; no caller-authored label descriptions exist. Adding none changes the candidate contract.

**C++, C#, Go, Java, Kotlin, Scala, Swift, Zig, PHP, Dart, Objective-C, Ada, COBOL, JavaScript, TypeScript, Python, R, Flutter facade / exported Door (relate)** — confirmed behavior. Relation reads accepts independent relation wording in file/JSON plan, defaulting to name with underscores replaced by spaces. Generic JSON passes it through. Host inline relation convenience usually carries only name/endpoints; choose file/object plan for reads. JS recognize rule-list additionally accepts reads directly.

**JavaScript, TypeScript, Python, R (decide, choose, tag, score, filter, annotate, recognize); Ruby (decide, choose, tag, score, filter, annotate)** — confirmed behavior. Description maps and structured question values accepted; recognize supports kind descriptions; annotation set members carry descriptions.

**Ruby (recognize)** — confirmed behavior. Description maps and structured question values accepted; recognize supports kind descriptions; annotation set members carry descriptions. Ruby also accepts inline rule Hashes preserving reads and single.

**Ruby (relate)** — confirmed behavior. Relation reads accepts independent relation wording in file/JSON plan, defaulting to name with underscores replaced by spaces. Generic JSON passes it through. Host inline relation convenience usually carries only name/endpoints; choose file/object plan for reads. JS recognize rule-list additionally accepts reads directly. Ruby also accepts inline rule Hashes preserving reads and single.

**pandas, Python Polars (decide, choose, tag, score, annotate, recognize); Rust Polars eager/lazy (decide, choose, tag, score, annotate)** — confirmed behavior. Retains base saved/built question descriptions when supported; no independent per-row option set.

**pandas, Python Polars (filter, relate); Rust Polars eager/lazy (filter, recognize, relate)** — design gap. Retains base saved/built question descriptions when supported; no independent per-row option set.

**CLI, Rust, C, DuckDB, PostgreSQL, SQLite (decide)** — confirmed behavior. Independent true/false criteria; string or structured description in saved question, Rust builders and C/SQL grammar.

**CLI, Rust, C, DuckDB, PostgreSQL, SQLite (choose)** — confirmed behavior. Option names and ordered string/object descriptions; CLI inline --option and files.

**CLI, Rust, C, DuckDB, PostgreSQL, SQLite (tag)** — confirmed behavior. Ordered label names and descriptions; CLI --label and files.

**CLI, Rust, C, DuckDB, PostgreSQL, SQLite (score)** — confirmed behavior. All levels described or all bare; files/JSON maps and Rust ScoreBuilder::level(name, Option<Description>) support descriptions. CLI positional levels are bare (existing 0.2 idea owns --level).

**CLI, Rust, C, DuckDB, PostgreSQL, SQLite (filter)** — confirmed behavior. Same true/false criteria as decide.

**CLI (rank)** — confirmed behavior. CLI saved decide criteria or score descriptions; Rust/C/SQL named rank accepts plain yes/no wording only.

**CLI, Rust, C, DuckDB, PostgreSQL, SQLite (find)** — confirmed behavior. Candidate text supplies the item, no separate caller option descriptions.

**CLI, Rust, C, DuckDB, PostgreSQL, SQLite (annotate)** — confirmed behavior. Each member decide/choose/tag/score description follows its question grammar.

**CLI, Rust, C, DuckDB, PostgreSQL, SQLite (recognize)** — confirmed behavior. Kind descriptions reach kind step only; boundary step uses fixed wording. DuckDB convenience kind-list path lacks descriptions (existing 0.2 issue).

**CLI, Rust, C, DuckDB, PostgreSQL, SQLite (relate)** — confirmed behavior. Relation name and endpoint contract stay separate from textual reads; reads defaults to name with underscores replaced by spaces. Rich description objects unsupported.

**Rust, C, DuckDB, PostgreSQL, SQLite (rank)** — design gap. CLI saved decide criteria or score descriptions; Rust/C/SQL named rank accepts plain yes/no wording only.


## Shared context

**C++, C#, Go, Java, Kotlin, Scala, Swift, Zig, PHP, Dart, Objective-C, Ada, COBOL, Flutter facade / exported Door (decide, choose, tag, score, filter, rank)** — confirmed behavior. call.context accepted on records

**C++, C#, Go, Java, Kotlin, Scala, Swift, Zig, PHP, Dart, Objective-C, Ada, COBOL, Flutter facade / exported Door (find, annotate, recognize, relate)** — confirmed behavior. No separate shared context control

**JavaScript, TypeScript, Python, Ruby, R (all functions)** — confirmed behavior. One shared string for eligible many-record four judgments/filter/rank; scalar, find, annotate, recognize, relate reject separate context.

**pandas, Python Polars, Rust Polars eager/lazy (all functions)** — confirmed behavior. One shared call context for four judgments; annotate rejects context; recognition has no separate context.

**CLI, Rust, C (decide, choose, tag, score, filter, rank); DuckDB, PostgreSQL, SQLite (filter, rank)** — confirmed behavior. One shared text on eligible record calls; CLI rejects single document or structured question context. Rust/C use many-record calls even for a contextual singleton.

**CLI, Rust, C, DuckDB, PostgreSQL, SQLite (find, annotate, recognize, relate)** — design gap. Separate shared context is refused; no dedicated state parameter.

**DuckDB, PostgreSQL, SQLite (decide, choose, tag, score)** — confirmed behavior. Scalar settings/context expression can vary by SQL row; adapter uses contextual singleton many path. Keyed many call has one shared context.


## Reading rules

**C++, C#, Go, Java, Kotlin, Scala, Swift, Zig, PHP, Dart, Objective-C, Ada, COBOL, JavaScript, TypeScript, Python, Ruby, R, pandas, Python Polars, Rust Polars eager/lazy, Flutter facade / exported Door (decide)** — confirmed behavior. Probability cut or band; true/false descriptions alter wording.

**C++, C#, Go, Java, Kotlin, Scala, Swift, Zig, PHP, Dart, Objective-C, Ada, COBOL, JavaScript, TypeScript, Python, Ruby, R, Flutter facade / exported Door (filter)** — confirmed behavior. One probability cut; rejects band; passing records retain input order.

**C++, C#, Go, Java, Kotlin, Scala, Swift, Zig, PHP, Dart, Objective-C, Ada, COBOL, JavaScript, TypeScript, Python, Ruby, R, Flutter facade / exported Door (rank)** — confirmed behavior. Yes-probability descending stable order; plain string question; no saved score-rank or description/model override on question. Current snapshot has no question-set rank. Planned CLI/Rust 0401D needs SQL parity in0417 (0.2) and C/binding parity in0418 (later).

**C++, C#, Go, Java, Kotlin, Scala, Swift, Zig, PHP, Dart, Objective-C, Ada, COBOL, JavaScript, TypeScript, Python, Ruby, R, Flutter facade / exported Door (find)** — confirmed behavior. One candidate choice plus optional none; no saved question tuning.

**C++, C#, Go, Java, Kotlin, Scala, Swift, Zig, PHP, Dart, Objective-C, Ada, COBOL, JavaScript, TypeScript, Python, Ruby, R, pandas, Python Polars, Rust Polars eager/lazy, Flutter facade / exported Door (choose)** — confirmed behavior. Unique leading option, optional winner cut; descriptions supported.

**C++, C#, Go, Java, Kotlin, Scala, Swift, Zig, PHP, Dart, Objective-C, Ada, COBOL, JavaScript, TypeScript, Python, Ruby, R, pandas, Python Polars, Rust Polars eager/lazy, Flutter facade / exported Door (score)** — confirmed behavior. Weighted ordinal positions; described levels supported; no probability cut.

**C++, C#, Go, Java, Kotlin, Scala, Swift, Zig, PHP, Dart, Objective-C, Ada, COBOL, JavaScript, TypeScript, Python, Ruby, R, pandas, Python Polars, Rust Polars eager/lazy, Flutter facade / exported Door (tag)** — confirmed behavior. Independent label cut; descriptions supported.

**C++, C#, Go, Java, Kotlin, Scala, Swift, Zig, PHP, Dart, Objective-C, Ada, COBOL, JavaScript, TypeScript, Python, Ruby, R, pandas, Python Polars, Rust Polars eager/lazy, Flutter facade / exported Door (annotate)** — confirmed behavior. Per-member decide/choose/tag reading cuts, score arithmetic; set pointers supported; shared context rejected.

**C++, C#, Go, Java, Kotlin, Scala, Swift, Zig, PHP, Dart, Objective-C, Ada, COBOL, JavaScript, TypeScript, Python, Ruby, R, pandas, Python Polars, Flutter facade / exported Door (recognize)** — confirmed behavior. Plan entity and relation thresholds; kind descriptions only influence kind stage, not span detection.

**C++, C#, Go, Java, Kotlin, Scala, Swift, Zig, PHP, Dart, Objective-C, Ada, COBOL, JavaScript, TypeScript, Python, Ruby, R, Flutter facade / exported Door (relate)** — confirmed behavior. Plan threshold, directed/either/single relation rules; reads changes relationship wording.

**pandas, Python Polars, Rust Polars eager/lazy (filter)** — design gap. One probability cut; rejects band; passing records retain input order.

**pandas, Python Polars, Rust Polars eager/lazy (rank)** — design gap. Yes-probability descending stable order; plain string question; no saved score-rank or description/model override on question. Current snapshot has no question-set rank. Planned CLI/Rust 0401D needs SQL parity in0417 (0.2) and C/binding parity in0418 (later).

**pandas, Python Polars, Rust Polars eager/lazy (find)** — design gap. One candidate choice plus optional none; no saved question tuning.

**pandas, Python Polars, Rust Polars eager/lazy (relate)** — design gap. Plan threshold, directed/either/single relation rules; reads changes relationship wording.

**Rust Polars eager/lazy (recognize)** — design gap. Plan entity and relation thresholds; kind descriptions only influence kind stage, not span detection.

**CLI, Rust, C, DuckDB, PostgreSQL, SQLite (decide)** — confirmed behavior. Cut or band; yes/no/unsure.

**CLI, Rust, C, DuckDB, PostgreSQL, SQLite (choose)** — confirmed behavior. Cut and declared-order tie handling; unresolved distinct from failure.

**CLI, Rust, C, DuckDB, PostgreSQL, SQLite (tag)** — confirmed behavior. Independent label cuts; declared label order.

**CLI, Rust, C, DuckDB, PostgreSQL, SQLite (score)** — confirmed behavior. Weighted numeric value; no threshold rule in ThinkThen; external cuts are caller work.

**CLI, Rust, C, DuckDB, PostgreSQL, SQLite (filter)** — confirmed behavior. One cut, passing input subsequence; no band, top or sorting. SQL WHERE over decide is equivalent; named SQL filter remains an existing idea.

**CLI, Rust, C, DuckDB, PostgreSQL, SQLite (rank)** — confirmed behavior. Stable descending yes probability; CLI top and saved score ranking. Rust/C/SQL top can be caller slicing/LIMIT; saved score rank missing. Current snapshot has no question-set rank. Planned CLI/Rust 0401D needs SQL parity in0417 (0.2) and C/binding parity in0418 (later).

**CLI, Rust, C, DuckDB, PostgreSQL, SQLite (find)** — confirmed behavior. Selected candidate or none; no free reading cut. none changes plan/candidate contract.

**CLI, Rust, C, DuckDB, PostgreSQL, SQLite (annotate)** — confirmed behavior. Each member rule; partial failures remain separate from valid null.

**CLI, Rust, C, DuckDB, PostgreSQL, SQLite (recognize)** — confirmed behavior. Strength cut and relation cut; lowering entity cut can introduce uncached downstream relation questions.

**CLI, Rust, C, DuckDB, PostgreSQL, SQLite (relate)** — confirmed behavior. Edge cut; either and single change question semantics/plan, not a free reading cut.


## Complete probabilities

**C++, C#, Go, Java, Kotlin, Scala, Swift, Zig, PHP, Dart, Objective-C, Ada, COBOL, Flutter facade / exported Door (decide, choose, tag, score)** — confirmed behavior. details:true on scalar/record JSON request returns full Rust answer probabilities; ordinary named decide returns yes probability. No single host probability control for score/tag.

**C++, C#, Go, Java, Kotlin, Scala, Swift, Zig, PHP, Dart, Objective-C, Ada, COBOL, Flutter facade / exported Door (filter)** — design gap. J1 returns kept input strings only; no probabilities for failed cut rows and no filter details key. Can ask decide with details:true then apply host cut.

**C++, C#, Go, Java, Kotlin, Scala, Swift, Zig, PHP, Dart, Objective-C, Ada, COBOL, Flutter facade / exported Door (rank)** — confirmed behavior. Each returned rank row has yes probability; J1 has no rank details key.

**C++, C#, Go, Java, Kotlin, Scala, Swift, Zig, PHP, Dart, Objective-C, Ada, COBOL, Flutter facade / exported Door (find)** — design gap. J1 returns selected candidate probability only, null for none; discarded full choice distribution has no generic details key. Caller-owned raw recording retains backend reply.

**C++, C#, Go, Java, Kotlin, Scala, Swift, Zig, PHP, Dart, Objective-C, Ada, COBOL, Flutter facade / exported Door (annotate)** — design gap. J1 returns per-member bare annotation values/failures only; no per-member full probabilities or details key.

**C++, C#, Go, Java, Kotlin, Scala, Swift, Zig, PHP, Dart, Objective-C, Ada, COBOL, Flutter facade / exported Door (recognize)** — design gap. Recognized entities carry strength; accepted relations carry probabilities; full internal span/kind distributions unavailable in generic value/facts.

**C++, C#, Go, Java, Kotlin, Scala, Swift, Zig, PHP, Dart, Objective-C, Ada, COBOL, Flutter facade / exported Door (relate)** — design gap. Value holds accepted edges and their probabilities; rejected pairs and complete single-menu distributions have no J1 details key.

**JavaScript, TypeScript, Python, Ruby, R (decide, choose, tag, score, filter, rank, find, annotate)** — confirmed behavior. Call.details question observations hold named distributions for judgment and find questions; Details scalar API exposes full probabilities. Selected found value is only a convenience view. Recognize/relate complete stage observation availability must be scoped to actual event shapes.

**JavaScript, TypeScript, Python, Ruby, R (recognize, relate)** — confirmed behavior. Call.details retains ordered internal question observations including per-stage named distributions and failed answers; ordinary value carries accepted entities/edges. This is richer than J1 generic value/facts.

**pandas, Python Polars (decide, choose, tag, score, annotate, recognize); Rust Polars eager/lazy (decide, choose, tag, score, annotate)** — confirmed behavior. Python calls retain ordered question observations; selected probability convenience only on decide/choose. Rust probability_frame and probability structs expose selected decide/choose probability, not every named probability. Raw Rust observer remains opt-in at call boundary; lazy output retains no facts without Tally.

**pandas, Python Polars (filter, rank, find, relate); Rust Polars eager/lazy (filter, rank, find, recognize, relate)** — design gap. Python calls retain ordered question observations; selected probability convenience only on decide/choose. Rust probability_frame and probability structs expose selected decide/choose probability, not every named probability. Raw Rust observer remains opt-in at call boundary; lazy output retains no facts without Tally.

**CLI, Rust, C (decide, choose, tag, score)** — confirmed behavior. Full yes/named probabilities in detailed judgment.

**CLI, Rust (filter)** — confirmed behavior. CLI details for passing records; native observer can report judged/rejected questions. C J1 filter and SQL WHERE convenience do not expose a full per-row probability carrier; SQL details can be called separately.

**CLI, Rust (rank)** — confirmed behavior. Every returned row has yes probability (or CLI score distribution under details). Native observer reports questions. C J1 and SQL rank lack full generic stage details.

**CLI, Rust, DuckDB, PostgreSQL, SQLite (find)** — confirmed behavior. CLI details and Rust Found candidates expose every candidate probability; SQL result candidates also expose all. C generic J1 value exposes winner only; J1 has no details key.

**CLI, Rust (annotate, recognize, relate)** — confirmed behavior. CLI details and native Rust question observation retain underlying probabilities/stages; C J1 returns bare value/facts only, SQL convenience output exposes named answers/entities/accepted edges rather than complete stage distributions. Plain output cannot recover rejected answers.

**C, DuckDB, PostgreSQL, SQLite (filter)** — design gap. CLI details for passing records; native observer can report judged/rejected questions. C J1 filter and SQL WHERE convenience do not expose a full per-row probability carrier; SQL details can be called separately.

**C, DuckDB, PostgreSQL, SQLite (rank)** — design gap. Every returned row has yes probability (or CLI score distribution under details). Native observer reports questions. C J1 and SQL rank lack full generic stage details.

**C (find)** — design gap. CLI details and Rust Found candidates expose every candidate probability; SQL result candidates also expose all. C generic J1 value exposes winner only; J1 has no details key.

**C, DuckDB, PostgreSQL, SQLite (annotate, recognize, relate)** — design gap. CLI details and native Rust question observation retain underlying probabilities/stages; C J1 returns bare value/facts only, SQL convenience output exposes named answers/entities/accepted edges rather than complete stage distributions. Plain output cannot recover rejected answers.

**DuckDB, PostgreSQL, SQLite (decide, choose, tag, score)** — confirmed behavior. Scalar thinkthen_details exposes every probability. Keyed many score/tag rows expose values; scalar details is the richer route.


## Stores and backend selection

**C++, C#, Go, Java, Kotlin, Scala, Swift, Zig, PHP, Dart, Objective-C, Ada, COBOL, Flutter facade / exported Door (all functions)** — confirmed behavior. Native Engine settings pass record/replay/cache/base_url/model/profile through accepted C constructor JSON. Environment seeds defaults. No binding-specific store implementation. Reading-rule-only changes can reuse compatible wire answers. Named backend is supported through THINKTHEN_BACKEND/config; code-level named backend selector is deferred to existing ticket 0377. base_url is not equivalent to a named backend selector.

**JavaScript, TypeScript, Python, Ruby, R (all functions)** — confirmed behavior. Host Engine constructors expose native record/replay/cache/backend/model settings; uses same engine store. THINKTHEN_BACKEND/config supports named entries. Existing ticket 0377 owns missing code-level named backend selection.

**pandas, Python Polars (all functions)** — confirmed behavior. Native Engine settings pass record/replay/cache/base_url/model/profile through accepted C constructor JSON. Environment seeds defaults. No binding-specific store implementation. Reading-rule-only changes can reuse compatible wire answers. THINKTHEN_BACKEND/config supports named entries. Existing ticket 0377 owns missing code-level named backend selection.

**Rust Polars eager/lazy (all functions)** — confirmed behavior. Native Engine settings pass record/replay/cache/base_url/model/profile through accepted C constructor JSON. Environment seeds defaults. No binding-specific store implementation. Reading-rule-only changes can reuse compatible wire answers.

**CLI, Rust, C, DuckDB, PostgreSQL, SQLite (all functions)** — confirmed behavior. Record, strict replay and cache share Rust store. CLI and Rust code-level named backend supported; C/SQL environment/config only, explicit named setting belongs to ticket 0377. SQL folders follow host permission/config lifetime.


## Question files

**C++, C#, Go, Java, Kotlin, Scala, Swift, Zig, PHP, Dart, Objective-C, Ada, COBOL, Flutter facade / exported Door (all functions)** — confirmed behavior. No host named loader. Caller reads file then passes parsed/raw question JSON; annotation needs object, not path in J1. Named C question-file symbol is not exposed by these wrappers. Caller file reading is an edge adapter. Missing named convenience loader does not remove question equivalence for scalar four-function JSON questions. rank/find restriction remains a true question capability gap.

**JavaScript, TypeScript, Python, Ruby, R (decide, choose, tag, score, filter, annotate, recognize, relate)** — confirmed behavior. Named question loader (four judgments/filter), annotation set loader, and named recognize/relate plans.

**JavaScript, TypeScript, Python, Ruby, R (rank, find)** — design gap. Saved rich question refuses. JS/Ruby/R check extra keys; Python built question kind mismatches rank/find.

**pandas, Python Polars (decide, choose, tag, score, annotate, recognize); Rust Polars eager/lazy (decide, choose, tag, score, annotate)** — confirmed behavior. File-loaded base question/set accepted by supported adapter; Rust eager/lazy requires built Question/QuestionSet.

**pandas, Python Polars (filter, rank, find, relate); Rust Polars eager/lazy (filter, rank, find, recognize, relate)** — design gap. File-loaded base question/set accepted by supported adapter; Rust eager/lazy requires built Question/QuestionSet.

**CLI, Rust, C, DuckDB, PostgreSQL, SQLite (all functions)** — confirmed behavior. CLI @file and Rust loaders where grammar admits; C typed loader or raw parsed question; SQL @file/JSON except literal rank/find. Host file loading is an edge adapter.


## Options per record

**All surfaces (decide, tag, score, filter, rank, find, recognize, relate)** — confirmed behavior. Not applicable: this function does not accept independent per-record choice option lists. Its own label/candidate contract is described in label_descriptions and input_forms.

**C++, C#, Go, Java, Kotlin, Scala, Swift, Zig, PHP, Dart, Objective-C, Ada, COBOL, JavaScript, TypeScript, Python, Ruby, R, pandas, Python Polars, Rust Polars eager/lazy, Flutter facade / exported Door (choose, annotate)** — design gap. No separate dynamic label-options pointer or per-record options envelope. Choices and levels are captured in one question. Caller may construct and ask separate questions. This is a capability gap, not a CSV adapter gap.

**CLI, Rust, C, DuckDB, PostgreSQL, SQLite (choose, annotate)** — design gap. CLI choose --options pointer changes candidates per record; SQL scalar members expression can vary per row. Rust/C and integrated binding many-record choose capture one option set; no equivalent per-record option envelope.


## Context per record

**C++, C#, Go, Java, Kotlin, Scala, Swift, Zig, PHP, Dart, Objective-C, Ada, COBOL, JavaScript, TypeScript, Python, Ruby, R, pandas, Python Polars, Rust Polars eager/lazy, Flutter facade / exported Door (all functions)** — design gap. No independently supplied context per record. Eligible many-record calls accept one shared string; embedding context into evidence is a different request representation.

**CLI, Rust, C, DuckDB, PostgreSQL, SQLite (all functions)** — design gap. No record/context pair input in a batch. Eligible calls can be composed as contextual singleton many calls; SQL scalar context expression can vary per row. Embedding context into evidence changes the item representation.


## Layer controls

**C++, C#, Go, Java, Kotlin, Scala, Swift, Zig, PHP, Dart, Objective-C, Ada, COBOL, Flutter facade / exported Door (all functions)** — confirmed behavior. Function contract and item evidence separated; scalar four-function wording/label descriptions and constructor model independently configurable; eligible record calls have shared context. rank/find restrict wording to plain text and model to engine; no independent per-record context.

**JavaScript, TypeScript, Python, Ruby, R (all functions)** — confirmed behavior. Separate evidence, wording/description, read rules, model and eligible shared context for four judgments/filter; question set covers annotate wording and per-member rules. Rank/find omit saved tuning. No distinct per-record context.

**pandas, Python Polars (decide, choose, tag, score, annotate, recognize); Rust Polars eager/lazy (decide, choose, tag, score, annotate)** — confirmed behavior. Supported adapters retain the base question layers; no distinct row-context column. Missing functions require composition with base library.

**pandas, Python Polars (filter, rank, find, relate); Rust Polars eager/lazy (filter, rank, find, recognize, relate)** — design gap. Supported adapters retain the base question layers; no distinct row-context column. Missing functions require composition with base library.

**CLI, Rust, C, DuckDB, PostgreSQL, SQLite (all functions)** — confirmed behavior. See function layer table and digest/cache distinction in audit report. Changing request wording/item/model/context changes request identity; reading cut stays out of QuestionKey.


## Rereading stored answers

**C++, C#, Go, Java, Kotlin, Scala, Swift, Zig, PHP, Dart, Objective-C, Ada, COBOL, JavaScript, TypeScript, Python, Ruby, R, pandas, Python Polars, Rust Polars eager/lazy, Flutter facade / exported Door (all functions)** — confirmed behavior. Engine stores wire answers and reads them with current thresholds. A repeated compatible request under strict replay can re-read without a send. This is not a standalone host saved-result re-reader. Changed wording/model/context changes prepared bytes and can miss. This audit did not count fresh sends. Evidence limit: Use settings replay/cache cases plus core stored-answer tests; per-function/per-binding changed-reading no-send proof is unproved unless separately cited.

**CLI, Rust, C, DuckDB, PostgreSQL, SQLite (all functions)** — unproved. Compatible strict replay reads wire answers under current rules; see counted CLI probes. Per-function SQL/C changed-rule no-send route is unproved. Recognition downstream relation coverage must be complete.

## Source map and available checks

Paths refer to the historical snapshot; line numbers are omitted because later code moved. A runner lists available checks, not evidence that every capability ran. For canonical runners, the function selector table below describes membership before host exclusions; the report lists the C/SQL exclusions. Python/R/Ruby/JS hosts also omit internal defect injection and legacy separate-group annotation. Generic wrappers use their separate J1 shape/value/error corpus. Variant checks establish only their stated adapter scope.

| Surface | Product sources | Existing runner and scope |
| --- | --- | --- |
| CLI | [crates/thinkthen/src/cli/annotate.rs](../../crates/thinkthen/src/cli/annotate.rs), [crates/thinkthen/src/cli/args.rs](../../crates/thinkthen/src/cli/args.rs), [crates/thinkthen/src/cli/asking.rs](../../crates/thinkthen/src/cli/asking.rs), [crates/thinkthen/src/cli/find.rs](../../crates/thinkthen/src/cli/find.rs), [crates/thinkthen/src/cli/judge.rs](../../crates/thinkthen/src/cli/judge.rs), [crates/thinkthen/src/cli/recognize.rs](../../crates/thinkthen/src/cli/recognize.rs), [crates/thinkthen/src/cli/relate.rs](../../crates/thinkthen/src/cli/relate.rs) | [crates/thinkthen/tests/backend/loopback_cases.rs](../../crates/thinkthen/tests/backend/loopback_cases.rs): Existing conformance case definition/runner evidence, not an independent execution receipt for each capability. Prior 21-surface routine checkpoint proves only selected runner scope; audit focused CLI/public receipts are separate. |
| Rust | [crates/thinkthen/src/public/builders.rs](../../crates/thinkthen/src/public/builders.rs), [crates/thinkthen/src/public/bulk.rs](../../crates/thinkthen/src/public/bulk.rs), [crates/thinkthen/src/public/bulk/annotation.rs](../../crates/thinkthen/src/public/bulk/annotation.rs), [crates/thinkthen/src/public/options.rs](../../crates/thinkthen/src/public/options.rs), [crates/thinkthen/src/public/question.rs](../../crates/thinkthen/src/public/question.rs), [crates/thinkthen/src/public/recognize.rs](../../crates/thinkthen/src/public/recognize.rs), [crates/thinkthen/src/public/relate.rs](../../crates/thinkthen/src/public/relate.rs) | [conformance/consumer/consumer/tests/public/cases.rs](../../conformance/consumer/consumer/tests/public/cases.rs): Existing conformance case definition/runner evidence, not an independent execution receipt for each capability. Prior 21-surface routine checkpoint proves only selected runner scope; audit focused CLI/public receipts are separate. |
| C | [libraries/c/src/call.rs](../../libraries/c/src/call.rs), [libraries/c/src/door.rs](../../libraries/c/src/door.rs), [libraries/c/src/settings.rs](../../libraries/c/src/settings.rs) | [libraries/c/tests/door/cases.rs](../../libraries/c/tests/door/cases.rs): Corpus IDs and routine/extended IDs describe corpus and selector membership only. Executable IDs remove the runner exclusions explicitly listed here. Excluded IDs did not run and are not covered by the passing checkpoint. No independent execution receipt for every capability is claimed. Separate internal-defect/cancellation tests do not make the omitted corpus case executed. |
| Ada | [libraries/ada/src/thinkthen.ads](../../libraries/ada/src/thinkthen.ads), [libraries/c/src/call.rs](../../libraries/c/src/call.rs) | [libraries/ada/checks/public_types.py](../../libraries/ada/checks/public_types.py): J1 shape/value/error expectations only; full 55 behavior cases are not executed by this type corpus. No new runs. Source-derived feature support is not a per-feature execution receipt. |
| C# | [libraries/c/src/call.rs](../../libraries/c/src/call.rs), [libraries/csharp/src/ThinkThen.cs](../../libraries/csharp/src/ThinkThen.cs) | [libraries/csharp/tests/public_types.py](../../libraries/csharp/tests/public_types.py): J1 shape/value/error expectations only; full 55 behavior cases are not executed by this type corpus. No new runs. Source-derived feature support is not a per-feature execution receipt. |
| C++ | [libraries/c/src/call.rs](../../libraries/c/src/call.rs), [libraries/cpp/include/thinkthen/door.hpp](../../libraries/cpp/include/thinkthen/door.hpp) | [libraries/cpp/fixtures/type_cases.py](../../libraries/cpp/fixtures/type_cases.py): J1 shape/value/error expectations only; full 55 behavior cases are not executed by this type corpus. No new runs. Source-derived feature support is not a per-feature execution receipt. |
| COBOL | [libraries/c/src/call.rs](../../libraries/c/src/call.rs), [libraries/cobol/src/tt_call.cob](../../libraries/cobol/src/tt_call.cob) | [libraries/cobol/checks/public_types.py](../../libraries/cobol/checks/public_types.py): J1 shape/value/error expectations only; full 55 behavior cases are not executed by this type corpus. No new runs. Source-derived feature support is not a per-feature execution receipt. |
| Dart | [libraries/c/src/call.rs](../../libraries/c/src/call.rs), [libraries/dart/lib/src/door.dart](../../libraries/dart/lib/src/door.dart) | [libraries/dart/checks/public_types.py](../../libraries/dart/checks/public_types.py): J1 shape/value/error expectations only; full 55 behavior cases are not executed by this type corpus. No new runs. Source-derived feature support is not a per-feature execution receipt. |
| Go | [libraries/c/src/call.rs](../../libraries/c/src/call.rs), [libraries/go/thinkthen.go](../../libraries/go/thinkthen.go) | [libraries/go/fixtures/type_cases.py](../../libraries/go/fixtures/type_cases.py): J1 shape/value/error expectations only; full 55 behavior cases are not executed by this type corpus. No new runs. Source-derived feature support is not a per-feature execution receipt. |
| Java | [libraries/c/src/call.rs](../../libraries/c/src/call.rs), [libraries/jvm/door/thinkthen/Door.java](../../libraries/jvm/door/thinkthen/Door.java) | [libraries/jvm/tests/public_types.py](../../libraries/jvm/tests/public_types.py): J1 shape/value/error expectations only; full 55 behavior cases are not executed by this type corpus. No new runs. Source-derived feature support is not a per-feature execution receipt. |
| JavaScript | [libraries/typescript/index.js](../../libraries/typescript/index.js), [libraries/typescript/src/door/result.rs](../../libraries/typescript/src/door/result.rs) | [libraries/typescript/tests/cases.mjs](../../libraries/typescript/tests/cases.mjs): Runner maps canonical cases to actual host APIs. Routine selector only includes selected IDs; baseline aggregate pass is not a new per-cell receipt. |
| Kotlin | [libraries/c/src/call.rs](../../libraries/c/src/call.rs), [libraries/jvm/kotlin/KotlinCaller.kt](../../libraries/jvm/kotlin/KotlinCaller.kt) | [libraries/jvm/tests/public_types.py](../../libraries/jvm/tests/public_types.py): J1 shape/value/error expectations only; full 55 behavior cases are not executed by this type corpus. No new runs. Source-derived feature support is not a per-feature execution receipt. |
| Objective-C | [libraries/c/src/call.rs](../../libraries/c/src/call.rs), [libraries/objective-c/Sources/ThinkThen.h](../../libraries/objective-c/Sources/ThinkThen.h) | [libraries/objective-c/checks/public_types.py](../../libraries/objective-c/checks/public_types.py): J1 shape/value/error expectations only; full 55 behavior cases are not executed by this type corpus. No new runs. Source-derived feature support is not a per-feature execution receipt. |
| PHP | [libraries/c/src/call.rs](../../libraries/c/src/call.rs), [libraries/php/src/ThinkThen.php](../../libraries/php/src/ThinkThen.php) | [libraries/php/fixtures/type_cases.py](../../libraries/php/fixtures/type_cases.py): J1 shape/value/error expectations only; full 55 behavior cases are not executed by this type corpus. No new runs. Source-derived feature support is not a per-feature execution receipt. |
| Python | [libraries/python/src/engine.rs](../../libraries/python/src/engine.rs), [libraries/python/thinkthen/__init__.py](../../libraries/python/thinkthen/__init__.py) | [libraries/python/tests/conformance.py](../../libraries/python/tests/conformance.py): Runner maps canonical cases to actual host APIs. Routine selector only includes selected IDs; baseline aggregate pass is not a new per-cell receipt. |
| R | [libraries/r/thinkthen/R/thinkthen.R](../../libraries/r/thinkthen/R/thinkthen.R), [libraries/r/thinkthen/src/rust/src/calls.rs](../../libraries/r/thinkthen/src/rust/src/calls.rs) | [libraries/r/tests/conformance.R](../../libraries/r/tests/conformance.R): Runner maps canonical cases to actual host APIs. Routine selector only includes selected IDs; baseline aggregate pass is not a new per-cell receipt. |
| Ruby | [libraries/ruby/lib/thinkthen.rb](../../libraries/ruby/lib/thinkthen.rb), [libraries/ruby/src/call.rs](../../libraries/ruby/src/call.rs) | [libraries/ruby/tests/conformance.rb](../../libraries/ruby/tests/conformance.rb): Runner maps canonical cases to actual host APIs. Routine selector only includes selected IDs; baseline aggregate pass is not a new per-cell receipt. |
| Scala | [libraries/c/src/call.rs](../../libraries/c/src/call.rs), [libraries/jvm/scala/ScalaCaller.scala](../../libraries/jvm/scala/ScalaCaller.scala) | [libraries/jvm/tests/public_types.py](../../libraries/jvm/tests/public_types.py): J1 shape/value/error expectations only; full 55 behavior cases are not executed by this type corpus. No new runs. Source-derived feature support is not a per-feature execution receipt. |
| Swift | [libraries/c/src/call.rs](../../libraries/c/src/call.rs), [libraries/swift/Sources/ThinkThen/ThinkThen.swift](../../libraries/swift/Sources/ThinkThen/ThinkThen.swift) | [libraries/swift/Tests/fixtures/public_types.py](../../libraries/swift/Tests/fixtures/public_types.py): J1 shape/value/error expectations only; full 55 behavior cases are not executed by this type corpus. No new runs. Source-derived feature support is not a per-feature execution receipt. |
| TypeScript | [libraries/typescript/index.d.ts](../../libraries/typescript/index.d.ts), [libraries/typescript/index.js](../../libraries/typescript/index.js), [libraries/typescript/src/door/result.rs](../../libraries/typescript/src/door/result.rs) | [libraries/typescript/tests/cases.mjs](../../libraries/typescript/tests/cases.mjs): Runner maps canonical cases to actual host APIs. Routine selector only includes selected IDs; baseline aggregate pass is not a new per-cell receipt. |
| Zig | [libraries/c/src/call.rs](../../libraries/c/src/call.rs), [libraries/zig/src/thinkthen.zig](../../libraries/zig/src/thinkthen.zig) | [libraries/zig/Tests/public_types.py](../../libraries/zig/Tests/public_types.py): J1 shape/value/error expectations only; full 55 behavior cases are not executed by this type corpus. No new runs. Source-derived feature support is not a per-feature execution receipt. |
| pandas | [libraries/python/src/input.rs](../../libraries/python/src/input.rs), [libraries/python/thinkthen/judge.py](../../libraries/python/thinkthen/judge.py) | [libraries/python/tests/test_pandas.py](../../libraries/python/tests/test_pandas.py): pandas test_each_verb_answers_a_series_as_its_list_does; test_a_frame_gains_answer_columns_and_keeps_its_own; test_every_pandas_refusal_sends_nothing; Python conformance additionally exercises Polars four judgments/annotate. Rust cases.rs explicitly RUN/NOT_RUN lists only four judgments/annotate, not ten functions. |
| Python Polars | [libraries/python/src/input.rs](../../libraries/python/src/input.rs), [libraries/python/thinkthen/judge.py](../../libraries/python/thinkthen/judge.py) | [libraries/python/tests/conformance.py](../../libraries/python/tests/conformance.py): pandas test_each_verb_answers_a_series_as_its_list_does; test_a_frame_gains_answer_columns_and_keeps_its_own; test_every_pandas_refusal_sends_nothing; Python conformance additionally exercises Polars four judgments/annotate. Rust cases.rs explicitly RUN/NOT_RUN lists only four judgments/annotate, not ten functions. |
| Rust Polars eager/lazy | [crates/thinkthen/src/public/frame.rs](../../crates/thinkthen/src/public/frame.rs) | [crates/thinkthen/tests/polars/cases.rs](../../crates/thinkthen/tests/polars/cases.rs): pandas test_each_verb_answers_a_series_as_its_list_does; test_a_frame_gains_answer_columns_and_keeps_its_own; test_every_pandas_refusal_sends_nothing; Python conformance additionally exercises Polars four judgments/annotate. Rust cases.rs explicitly RUN/NOT_RUN lists only four judgments/annotate, not ten functions. |
| Flutter facade / exported Door | [libraries/c/src/call.rs](../../libraries/c/src/call.rs), [libraries/dart/flutter/lib/thinkthen_flutter.dart](../../libraries/dart/flutter/lib/thinkthen_flutter.dart), [libraries/dart/lib/src/door.dart](../../libraries/dart/lib/src/door.dart) | [libraries/dart/check.sh](../../libraries/dart/check.sh): Dart Door J1 corpus does not prove Flutter facade each function; Flutter example/host checks prove its decide flow only. |
| DuckDB | [databases/duckdb/README.md](../../databases/duckdb/README.md), [databases/duckdb/bridge/src/ffi/portable_many/ffi.rs](../../databases/duckdb/bridge/src/ffi/portable_many/ffi.rs), [databases/duckdb/src/engines.rs](../../databases/duckdb/src/engines.rs) | [databases/duckdb/tools/conformance.py](../../databases/duckdb/tools/conformance.py): Corpus IDs and routine/extended IDs describe corpus and selector membership only. Executable IDs remove the runner exclusions explicitly listed here. Excluded IDs did not run and are not covered by the passing checkpoint. No independent execution receipt for every capability is claimed. Separate internal-defect/cancellation tests do not make the omitted corpus case executed. |
| PostgreSQL | [databases/postgresql/src/find.rs](../../databases/postgresql/src/find.rs), [databases/postgresql/src/keyed.rs](../../databases/postgresql/src/keyed.rs), [databases/postgresql/src/lib.rs](../../databases/postgresql/src/lib.rs), [databases/postgresql/src/scalar.rs](../../databases/postgresql/src/scalar.rs) | [databases/postgresql/tests/runner.py](../../databases/postgresql/tests/runner.py): Corpus IDs and routine/extended IDs describe corpus and selector membership only. Executable IDs remove the runner exclusions explicitly listed here. Excluded IDs did not run and are not covered by the passing checkpoint. No independent execution receipt for every capability is claimed. Separate internal-defect/cancellation tests do not make the omitted corpus case executed. |
| SQLite | [databases/sqlite/src/many.rs](../../databases/sqlite/src/many.rs), [databases/sqlite/src/scalars.rs](../../databases/sqlite/src/scalars.rs), [databases/sqlite/src/scalars/find.rs](../../databases/sqlite/src/scalars/find.rs), [databases/sqlite/src/tables.rs](../../databases/sqlite/src/tables.rs) | [databases/sqlite/tests/conformance.py](../../databases/sqlite/tests/conformance.py): Corpus IDs and routine/extended IDs describe corpus and selector membership only. Executable IDs remove the runner exclusions explicitly listed here. Excluded IDs did not run and are not covered by the passing checkpoint. No independent execution receipt for every capability is claimed. Separate internal-defect/cancellation tests do not make the omitted corpus case executed. |

## Canonical function selectors

These IDs describe the 55-case corpus and the routine/extended selector, not executed cases. Remove each host’s exclusions listed in the report before inferring eligible execution. Noncanonical wrapper and variant checks above do not inherit these IDs.

| Function | Routine IDs | Extended IDs |
| --- | --- | --- |
| decide | 01-decide-yes-captured, 02-decide-no, 03-decide-band-unsure, 20-usage-fault, 21-backend-fault, 22-local-fault, 23-cancelled-fault, 24-deadline-fault, 25-defect-fault, 40-decide-counters | 04-decide-band-yes, 05-decide-meanings, 27-decide-many, 28-decide-many-repeated-texts, 29-usage-json-text, 30-local-question-file |
| choose | 06-choose-billing, 07-choose-unsure, 08-choose-tie |  |
| tag | 09-tag-two, 10-tag-none | 33-tag-threshold-excludes |
| score | 11-score-middle, 32-score-equal-distribution | 12-score-upper |
| filter | 13-filter-records, 26-filter-empty-list | 14-filter-none |
| rank | 15-rank-records, 16-rank-stable-tie, 31-usage-rank-blank-question |  |
| find | 18-find-second, 19-find-none |  |
| annotate | 17-annotate-mixed, 17-annotate-partial, 36-annotate-two-columns | 18-annotate-two-groups, 34-annotate-repeated-texts, 35-annotate-score-repeated-texts, 37-annotate-choose-one, 38-annotate-score-one, 39-annotate-tag-one |
| recognize | 41-offsets-past-an-accent-and-an-emoji, 42-recognize-C01-relations, 53-recognize-either | 43-recognize-C02-relations, 44-recognize-C12-relations, 45-recognize-C13-relations, 46-recognize-C14-relations, 47-recognize-C18-relations, 48-recognize-C24-relations, 49-recognize-C33-relations, 50-recognize-C40-relations |
| relate | 51-same-kind-alerts, 52-cross-kind-staff |  |
