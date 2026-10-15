# Changelog

Every release of every surface shares one version number.

## 0.2.0

### Functions, files and saved questions

ThinkThen exposes ten functions: `decide`, `choose`, `tag`, `score`, `filter`, `rank`, `find`, `annotate`, `recognize` and `relate`. The command, libraries and database extensions use the same engine.

File readers keep source locations beside the selected text and original records. Folder contents sort by relative path. Explicit files and duplicates keep their order. Paths never become model evidence.

`find` accepts one source unit when the question offers `none`. DuckDB file recognition retains every returned row.

Saved questions add author names, wording versions, input declarations and authored yes/no readings. Existing 0.1 question files work unchanged. Declarations check values without changing them. ThinkThen creates no question file or configuration.

### Images and MCP

`decide`, `choose` and `score` accept ordered JPEG and PNG images with optional text. Other functions refuse images before reading files or sending. Images need a supported backend and model. ThinkThen does not resize images to fit. The [known limits](release-notes/0.2.0-known-gaps.md) name the supported routes.

`thinkthen mcp` exposes the ten functions over local stdio. It uses the same engine, backend and answer cache as the command. It adds no question-writing, administration or business-routing tools. Frame and attachment limits apply. An oversized output closes the connection after any work already performed.

### Results and compatibility

Each library function except C returns one result with `value`, `probability`, `results` and `facts`. C returns typed session packets. SQL plain calls return values. SQL `thinkthen_<function>_details` calls return typed rows. The command keeps `--details` for complete results.

Complete results use `thinkthen.result/2`. They keep source locations, answer identities, timing, attempts, usage and observed model facts. Rank positions start at 1. Missing facts stay missing. Requested and reported models stay separate. Cached calls report no current send cost.

ThinkThen refuses 0.1 cache and recording files before sending. The message names the file and says to delete it. The database bytes and modification time stay unchanged. ThinkThen does not convert old stores. Save any answers you need before deleting those files.

`runs audit` and `runs diff` compare saved answers offline. The old top-level `audit` and `diff` spellings still work.

### Backends and platforms

Named backends add `perplexity`, `openrouter`, `llamacpp`, `mlx` and `openai`. Configuration entries may set a relative posting path. Existing custom entries named `perplexity` or `openrouter` must use another name. Each engine uses one backend route. Your application or proxy supplies routing and fallback policy.

Windows command builds are unsigned. JVM requires JDK 22 or later. Objective-C requires Apple Foundation and ARC.

## 0.1.2 (2026-10-03)

The macOS Ruby gems name no macOS version. `gem install thinkthen` on macOS 26 installed the 0.0.1 placeholder, because the 0.1.1 gems matched only darwin 24. The gems still need macOS 15.0.

The R package builds on R-universe again. The 0.1.1 source build stopped in configure with `cargo could not resolve thinkthen 0.1.1 from crates.io`, because the lock update named a package the lock no longer held.

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
