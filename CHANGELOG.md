# Changelog

Every release of every surface shares one version number.

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
