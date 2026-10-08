# Changelog

Every release of every surface shares one version number.

## Unreleased: 0.2.0

Version 0.2 is implemented on main and remains unreleased. The publication date is unknown. Public installation still names 0.1.2. The complete installed-package parity campaign passed. Final platform qualification, candidate release QA and rehearsal remain required. Publication requires Ian's explicit go.

### Functions, files and saved questions

Every SDK exposes the same ten functions: `decide`, `choose`, `tag`, `score`, `filter`, `rank`, `find`, `annotate`, `recognize` and `relate`. CLI, typed C, language SDKs, SQL and dataframe adapters use the native engine's admitted inputs, complete results, errors and cache/record/replay behavior. Existing family checks and the complete installed campaign passed across all 29 public consumers.

File readers retain physical source locations separately from selected evidence and original records. Folder descendants sort by relative path; explicit operands and duplicates keep their order. Paths never become model evidence or cache identity. PostgreSQL retains its client-reader/keyed-table workaround and privileged bounded question-file loader.

Saved questions add author names, wording versions, named lookup, restricted item/context declarations and authored yes/no readings. Existing 0.1 question files remain accepted. Local entries take precedence over named lookup. ThinkThen creates no question file or configuration. Declarations validate selected inputs without coercion; they do not implement arbitrary JSON Schema. Per-record context and replacement choose options remain associated with their records. Explicit empty context suppresses shared context. Effective context and options affect requests and cache identity.

### Images and MCP

`decide`, `choose` and `score` admit ordered JPEG/PNG images, including duplicates, with optional text. Separate whole-file image inputs retain their source provenance. The other seven functions refuse images before reading files or sending. Liquid `d1`, Perplexity `pplx-decider-v1-27b` and explicitly declared supported local setups have admitted image routes. TypeSafe, OpenRouter, Ollama, the named MLX route and OpenAI Decisions refuse images. Native decoded-input and vendor limits apply; oversized inputs refuse without hidden resizing. Admission makes no image accuracy claim.

The local stdio server starts with `thinkthen mcp` and exposes exactly the ten function tools through one native engine, route and cache. It accepts native question selectors, files, records, context/options and admitted images, and returns native complete results and errors. It adds no administrative, question-writing or business-routing tools. Incoming frames are limited to 16 MiB including newline; outgoing frames are limited to 192 MiB including newline and both equivalent result representations. Output overflow closes after any work already performed. Installed MCP parity passed; archive timing and final Windows qualification remain pending.

### Results, cache and compatibility

New complete results use `thinkthen.result/2` with stable answer IDs, observed source/model metadata, call/request identity, timing, attempts and usage facts. Rank details now return final positions starting at 1. Bare CLI, scalar SQL, convenience values and generic C compatibility projections remain. Requested and reported models stay separate; absent observations remain absent. Cached calls report zero current send cost without inventing token counts.

Cache/2 identity includes literal requested/reported models, normalized final endpoints and image media/bytes/order. Upgrade validates v1 records offline before transactional conversion; read-only replay changes no bytes. Damaged or ambiguous stores refuse before sends. Old/new concurrent writers and downgrade of migrated stores are unsupported; an old writer needs an unmigrated copy. Response no-store prohibits persistence; refresh sends no-cache and evicts old working answers after a good nonstorable reply. Explicit recording fails locally for such a reply. Reserved proxy types refuse activation in 0.2; override execution waits for an admitted 0.3 protocol.

`runs audit` and `runs diff` are the visible offline commands. Hidden top-level `audit` and `diff` aliases retain their options, outputs, diagnostics and exit codes. Reserved command nouns remain unimplemented; no eleventh judging function is added.

### Backends and known limits

Python, TypeScript, Ruby, R and the C JSON door can select a named backend in their constructor. Packages that forward C settings JSON inherit `"backend"`. DuckDB and PostgreSQL select a backend through their session settings. SQLite accepts `"backend"` in its process configuration before the first engine build. SQL accepts no address or key.

The default throttle is 8 simultaneous requests on every surface. Explicit throttles retain their process-wide precedence and range of 1 through 32.

Named backends add `perplexity` and `openrouter`. Configuration entries may set a relative posting `path`. OpenRouter preserves descriptions and fills a missing yes-or-no side with `{}`. Existing configured entries named `perplexity` or `openrouter` must be renamed or removed because those names now select built-ins; a built-in configuration entry accepts only `requests_per_minute`. OpenAI Decisions text support is implemented through the native adapter. Each engine resolves one endpoint, key and provider API type; business routing belongs to the proxy.

The release workflow covers the Windows x86-64 command, Rust crate, C DLL and Python wheel. Native qualification remains required. Windows Node, C# and JVM bindings are deferred to 0.3. Development command builds are unsigned. The fresh Linux installed-candidate documentation trial passed for text, images and MCP. Clean public-package checks follow actual publication.

## 0.1.2 (2026-10-03)

The macOS Ruby gems name no macOS version. `gem install thinkthen` on macOS 26 installed the 0.0.1 placeholder, because the 0.1.1 gems matched only darwin 24 (ticket 0394). The gems still need macOS 15.0.

The R package builds on R-universe again. The 0.1.1 source build stopped in configure with `cargo could not resolve thinkthen 0.1.1 from crates.io`, because the lock update named a package the lock no longer held (ticket 0395).

## 0.1.1 (2026-10-02)

This release fixes the PyPI, npm and NuGet publish steps. 0.1.0 reached only Maven Central, pub.dev and the Homebrew tap. Every surface carries the same code as 0.1.0.

## 0.1.0 (2026-10-02)

The first public release. ThinkThen answers typed questions about text. It returns `true`, `false`, a label, a number, or a ranked list. A failed call never looks like an answer.

### The functions

- `decide` answers yes, no, or not sure. On one text, the exit code carries the answer.
- `choose` picks the one option that fits best.
- `tag` names every label that applies.
- `score` places the text on a scale you name and prints a number.
- `filter` keeps the records where the answer is yes.
- `rank` orders records by how likely the answer is yes.
- `find` picks the one unit of text that best answers a question, or nothing.
- `annotate` asks a saved set of named questions about each record.
- `recognize` finds the names in a text and gives each one a kind.
- `relate` finds the relations between named things.

The [specification](specification/README.md) gives each function's contract, its exit codes and its failures.

### The command

- `thinkthen` reads the text on standard input and prints the answer on standard output.
- The exit code tells the outcome apart: 0 to 7, 70 for a defect, and the signal codes. [channels.md](specification/channels.md) lists them.
- `--details` prints the full result object. `--plan` prints the requests a run would send, needs no key and sends nothing.
- Record mode reads JSON Lines, text lines, CSV or TSV. It keeps input order and reports where a run stopped.
- `audit` grades saved answers against an answer key and suggests a threshold.
- `diff` shows the saved answers that changed between two runs or two cuts.
- `check` sends four fixed requests to a backend you name and reports whether it works with ThinkThen.
- `status` reports the resolved settings, the cache size and the local usage counts.
- `cache prune`, `cache unused` and `cache convert` maintain a cache folder without sending a request.
- `transform list` and `transform show` print the built-in `jq` transforms.

### Libraries and extensions

Every surface runs the same Rust engine.

- Rust: the `thinkthen` crate. Its `polars` feature adds the Polars door.
- C: `thinkthen.h` with a shared and a static library.
- Python, with pandas and Polars helpers.
- TypeScript, Ruby and R.
- C++, C#, Go, Java, Kotlin, Scala, Swift, Zig, PHP, Dart, Ada, Objective-C and COBOL, each over the C library.
- SQLite, DuckDB 1.5.5 and PostgreSQL 16 extensions.

### Platforms

- Linux on x86-64 and ARM64. The command is a static binary.
- macOS on Apple silicon and Intel.
- The command, the C library, the three SQL extensions and the Python, TypeScript and Ruby packages ship for all four.
- The packages over the C library are built and checked on Linux x86-64.
- Windows is not supported yet.

### Where to get it

- The command: `curl -fsSL https://thinkthen.dev/install.sh | sh`, `brew install botassembly/thinkthen/thinkthen`, or `cargo install thinkthen`.
- crates.io: `thinkthen`.
- PyPI: `thinkthen`, as four abi3 wheels.
- npm: `thinkthen`, one package with all four native addons.
- RubyGems: `thinkthen`, as four platform gems.
- R-universe: `thinkthen`, under `botassembly`.
- NuGet: `Botassembly.ThinkThen`.
- Maven Central: `io.github.botassembly:thinkthen-jvm`.
- pub.dev: `thinkthen_dart`.
- Packagist: `botassembly/thinkthen`.
- Go: `github.com/botassembly/thinkthen/libraries/go`.
- The release page holds the command, C library, SQL extension and language archives, each with a `.sha256` file. It also holds `thinkthen-first-run.tar.gz`, a sample that runs with no key.

The C++, C#, Go, Java, Kotlin, Scala, Swift, Zig, PHP, Dart, Ada, Objective-C and COBOL packages call the C library. Take `thinkthen-c-0.1.0-TARGET.tar.gz` from the same release.

### Backends

- A run that names no backend asks TypeSafe's Jev, pinned to model `jev-1.13.0`. It reads the key from `THINKTHEN_API_KEY`.
- Three backends are built in, and each reads only its own key variables. `typesafe` reads `TYPESAFE_API_KEY`. `liquid` reads `LIQUIDAI_API_KEY`, then `LIQUID_API_KEY`. `ollama` reaches Ollama on the local machine and sends no key there.
- `--backend NAME`, `THINKTHEN_BACKEND` or the configuration file selects a backend. The configuration file's `backends` adds more servers by address, key variable and model.
- Each backend sends only its own key.

### Settings and local files

- A typed value wins, then the environment, then the question file, then the configuration file, then the default. [settings.md](specification/settings.md) lists every setting, its default and its spelling on each surface.
- ThinkThen reads the configuration file and never writes it.
- `--record` saves each exchange in a folder. `--replay` answers from that folder with no key and no network.
- The command keeps one answer for each question in a cache folder by default. A longer run over the same records sends only the new questions. `--no-cache` turns the cache off.
- ThinkThen keeps count-only usage totals in the platform state folder. It never stores the key.
- Nothing is paced by default. `requests_per_minute` limits the request rate within one process.
- `THINKTHEN_MAX_ESTIMATED_INPUT_TOKENS_TOTAL` refuses a request before it is sent when its estimated input would pass the total.

### Breaking changes

None. This is the first release.
