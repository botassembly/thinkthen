<picture>
  <source media="(prefers-color-scheme: dark)" srcset="https://raw.githubusercontent.com/botassembly/thinkthen/main/site/public/brand/thinkthen-mark-dark.svg">
  <source media="(prefers-color-scheme: light)" srcset="https://raw.githubusercontent.com/botassembly/thinkthen/main/site/public/brand/thinkthen-mark-light.svg">
  <img src="https://raw.githubusercontent.com/botassembly/thinkthen/main/site/public/brand/thinkthen-mark-light.svg" alt="ThinkThen" width="64" height="64">
</picture>

# thinkthen

ThinkThen answers typed questions about text. Development 0.2 also admits JPEG and PNG images for `decide`, `choose` and `score`. Ask from a shell script or from your own program, and get back `true`, `false`, a label, or a number. A failed call never looks like an answer. Use it to gate a script, label records, or grade answers in an eval.

```sh
thinkthen decide 'Does the customer ask for money back?' < message.txt
```

```text
true
```

The exit code is 0 for yes, 1 for no, and 3 for not sure. A shell `if` can branch on it. [How-to 01](https://github.com/botassembly/thinkthen/tree/main/demos/01-refund-gate) runs this question against a recorded answer.

## Install

The public release remains 0.1.2. The 0.2 examples below require a reviewed development command or candidate SDK; they do not describe a published package.

```sh
curl -fsSL https://thinkthen.dev/install.sh | sh
```

On a Mac, Homebrew works too:

```sh
brew install botassembly/thinkthen/thinkthen
```

With Rust installed, `cargo install thinkthen` builds the command from crates.io.

Windows x86-64 support starts with 0.2. Development builds are unsigned. Public signing and distribution remain open; a public 0.2 archive may not exist yet. From a reviewed development checkout, use a supplied development mirror in Windows PowerShell 5.1 or PowerShell 7:

```powershell
$env:THINKTHEN_INSTALL_BASE = 'https://your-development-mirror.example'
.\install.ps1 -Version 0.2.0
```

Review `install.ps1` before running it. If your host execution policy refuses the script, follow your organization's policy; the installer changes no execution policy. It installs `thinkthen.exe` and a receipt under `%LOCALAPPDATA%\Programs\thinkthen`, or the absolute local NTFS directory named by `THINKTHEN_INSTALL_DIR`. It verifies the archive checksum and executable version. Checksums do not establish publisher identity.

The installer prints the full command path and a PATH command for the current session. For future sessions, add its directory through Windows environment settings. It changes no profile, registry or runtime configuration. To remove it, delete `thinkthen.exe`, `thinkthen.install.json` and `.thinkthen-install.lock` from the install directory. If interrupted recovery artifacts remain, inspect them before removing them or retrying. The script's Windows runner proof remains pending.

Windows reads `%APPDATA%\thinkthen\config.json`, stores cached answers in `%LOCALAPPDATA%\thinkthen\cache`, and stores count-only usage totals in `%LOCALAPPDATA%\thinkthen\usage`.

A live run needs a backend. A hosted backend needs a key. [Backends](https://thinkthen.dev/install/backends/) shows how to get a TypeSafe key or point ThinkThen at another server, such as Ollama on your own machine.

### Try it with no key

Download the release's sample, then replay its recorded answer with no key and no network:

```sh
curl -fsSL https://github.com/botassembly/thinkthen/releases/latest/download/thinkthen-first-run.tar.gz | tar -xz && cd thinkthen-first-run
thinkthen decide 'Does this report say what the person did before the problem appeared?' \
  --replay recording < report.txt
```

```text
true
```

[How-to 27](https://github.com/botassembly/thinkthen/tree/main/demos/27-test-with-no-network) shows how a test uses a recording.

## The ten functions

| Function | It answers |
| --- | --- |
| `decide` | yes, no, or not sure |
| `choose` | the one option that fits best |
| `tag` | every label that applies |
| `score` | a number on a scale |
| `filter` | the records that pass a yes/no question |
| `rank` | the records, best first |
| `find` | the one unit of text that answers the question, or nothing |
| `annotate` | a saved set of named questions for each record |
| `recognize` | the names in a text and what kind each is |
| `relate` | the relations between named things |

The [specification](https://github.com/botassembly/thinkthen/blob/main/specification/README.md) gives each function's contract, its exit codes, and its failures.

For agents, the [official ThinkThen skill](skills/thinkthen/SKILL.md) explains function selection, evidence framing, abstention and bounded requests.

## Files and provenance in development 0.2

`decide`, `filter`, `rank` and `annotate` accept document files as positional operands. For other functions, repeat `--input FILE`; positional operands in `choose`, `tag` and `score` name options, labels and levels. The two file input forms cannot mix. With several document files, decide/choose/tag/score print one JSONL row per file with `input_file` and `value`. A completed multi-document run exits 0 even for false or not sure answers. One document retains its scalar output and answer exit code.

Use `--input folder --unit line`, `--unit file` or `--window N` for located readers. Folder descendants sort by relative path; explicit operands and duplicates keep their order. Details retain physical file/line positions separately from selected evidence. Blank lines still count toward physical positions. Paths never become model evidence or cache identity. `--field` selects evidence from JSON records; the original stays in the answer. The [file guide](https://thinkthen.dev/learn/files/) shows each function's located output and limits.

## Images in development 0.2

Select an admitted backend and model explicitly. For example, with the Liquid backend key configured:

```sh
thinkthen decide 'Does the package have visible damage?' \
  --backend liquid --model d1 --image package.jpg
thinkthen choose 'Which package is damaged?' first second neither \
  --backend liquid --model d1 --image first.jpg --image second.jpg
thinkthen score 'How severe is the damage?' none minor severe \
  --backend liquid --model d1 --image package.jpg
```

Repeated `--image` attachments form one ordered input. Duplicates remain present. Optional stdin text or framed caption records can accompany attachments. `--image-media image/png` or `image/jpeg` declares the attachment format; omission detects it from bytes. Attachments cannot accompany `--media`, `--unit` or `--window`. To judge separate whole-file images, use `--input photos --unit file --media image` instead. Seven other functions refuse images before reading files or sending.

Liquid `d1` and Perplexity `pplx-decider-v1-27b` have admitted image routes. Local images require an explicit profile declaring an exact supported setup and model alias. TypeSafe, OpenRouter, Ollama, the named MLX route and OpenAI Decisions refuse images. A provider name alone establishes no image capability. The [image contract](specification/files.md#explicit-image-files-04470448) and [local setup declarations](specification/backends.md#local-image-declarations) give limits and profiles. No image accuracy claim follows from admission.

## Saved questions and records in development 0.2

Write `refund.json` yourself:

```json
{
  "decide": "Does the writer request a refund?",
  "true": "The writer asks for money back.",
  "false": "The writer asks for something else.",
  "threshold": "0.2:0.8",
  "name": "refund",
  "wording_version": 1,
  "item_schema": {"type": "string"},
  "context_schema": {"type": "string"}
}
```

```sh
thinkthen decide @refund.json --jsonl --field /body \
  --context-field /policy < tickets.jsonl
```

Each JSON record supplies its own `body` and separate `policy` context. The answer retains the whole original record. Per-record context changes request/cache identity and stays associated with its record. An explicit empty context suppresses shared context from `--context FILE`. Item selection precedes declaration validation; extra properties remain present and values are never coerced. Declarations support root strings or root objects with string, finite number, boolean and string-list properties. They are a restricted grammar, not arbitrary JSON Schema.

`true` and `false` are authored readings of yes and no. Saved readings can also be JSON objects, lists or null. A present null remains distinct from an absent reading. CLI `--true` and `--false` supply text overrides.

For named lookup, place the same file at `questions/refund.json` under the platform configuration directory. On Linux this defaults to `~/.config/thinkthen/questions/refund.json`; macOS uses `~/Library/Application Support/thinkthen/questions/refund.json`; Windows uses `%APPDATA%\thinkthen\questions\refund.json`. Then use `@refund`. An existing local `refund` entry takes precedence over named lookup. References containing path punctuation keep path behavior. ThinkThen creates no question file or configuration. A present authored name must match the requested name. Author names, wording versions and declarations describe the caller's question; they do not change its digest or answer identity. Existing 0.1 question files remain accepted. The [question-file contract](specification/question-file.md) gives the full grammar and safe refusals.

## MCP in development 0.2

Install or unpack the reviewed candidate command, put it on PATH, and configure its backend environment as for the CLI. Configure your MCP client to launch:

```json
{"command":"thinkthen","args":["mcp"]}
```

The Rust server uses local stdio, one native engine and the same cache. It exposes exactly the ten functions above. It starts no shell and no process per call. A `decide` tool call accepts `{"question":"Does the writer request a refund?","evidence":"Please refund my order."}`. Use `question_file`, `question_name` or `question_reference` for explicit file, named or @ lookup; those selectors are exclusive with literal `question`. Literal @ text is never treated as a path. Ordered images are admitted only for decide/choose/score. The [MCP guide](libraries/mcp/README.md) gives source inputs, per-record context, complete results and a no-key recorded call.

MCP accepts newline-delimited JSON-RPC frames. Its incoming frame limit also bounds the aggregate original compressed attachment bytes in a call, counting ordered duplicates. Use explicit native text-file inputs for captions too large for a frame; native input and provider body limits still apply. See the [framing and attachment contract](specification/mcp.md). Final platform and release qualification remain required.

## Languages

Every language uses the same Rust engine. Each page below gives the published install line and a first call. The [0.2 upgrade guide](libraries/UPGRADING-0.2.md) maps changed calls and links the package documentation for development APIs. The [binding guide](libraries/BINDING-AUTHOR.md) defines the one typed function family per language; it links the generated request and result contracts.

The development JVM session uses stable JDK 22 or later without preview features. Its packaged native loader needs no manual library path. The released JVM API still uses JDK 21 preview features and a separate C archive. The development Objective-C API uses Apple Foundation, ARC and generated results. GNU support has ended. Matching installed Apple Foundation execution remains unrun; see the [Foundation guide](libraries/objective-c/README.md). The [package design](sdlc/decisions/2026-10-09-native-package-design.md) defines the intended targets and runtime floors. Installed and platform qualification remain separate requirements.

The Rust crate supports Linux, macOS and Windows x86-64. Add it to a Rust project with `cargo add thinkthen`. The Windows C archive contains the header, `thinkthen.dll` and its MSVC import library. The Windows command archive contains `thinkthen.exe` alone. Native C runner proof remains pending.

| Kind | Install pages |
| --- | --- |
| Command line | [Bash](https://thinkthen.dev/install/shell/) |
| Libraries | [Python](https://thinkthen.dev/install/python/), [pandas](https://thinkthen.dev/install/pandas/), [Polars](https://thinkthen.dev/install/polars/), [TypeScript](https://thinkthen.dev/install/typescript/), [Ruby](https://thinkthen.dev/install/ruby/), [R](https://thinkthen.dev/install/r/), [Rust](https://thinkthen.dev/install/rust/), [C](https://thinkthen.dev/install/c/), [C++](https://thinkthen.dev/install/cpp/), [C#](https://thinkthen.dev/install/csharp/), [Go](https://thinkthen.dev/install/go/), [Java](https://thinkthen.dev/install/java/), [Kotlin](https://thinkthen.dev/install/kotlin/), [Scala](https://thinkthen.dev/install/scala/), [Swift](https://thinkthen.dev/install/swift/), [Zig](https://thinkthen.dev/install/zig/), [PHP](https://thinkthen.dev/install/php/), [Dart](https://thinkthen.dev/install/dart/), [Ada](https://thinkthen.dev/install/ada/), [Objective-C](https://thinkthen.dev/install/objective-c/), [COBOL](https://thinkthen.dev/install/cobol/) |
| Databases | [DuckDB](https://thinkthen.dev/install/duckdb/), [SQLite](https://thinkthen.dev/install/sqlite/), [PostgreSQL](https://thinkthen.dev/install/postgresql/) |

Each folder under [libraries/](https://github.com/botassembly/thinkthen/tree/main/libraries) and [databases/](https://github.com/botassembly/thinkthen/tree/main/databases) has a README that builds that binding from source.

On 2026-10-01 a call to Jev took a median of 138 ms. ThinkThen's own work took about 2 ms of it. Each binding added a median of 3 ms or less, and the slowest single function added 6.7 ms. [Overhead](https://thinkthen.dev/learn/overhead/) gives every measurement and its spread.

## How-tos

- [Build a triage pipeline that drafts, blocks, or asks a person](https://github.com/botassembly/thinkthen/tree/main/demos/16-triage-pipeline)
- [Gate a script step on a yes/no answer](https://github.com/botassembly/thinkthen/tree/main/demos/01-refund-gate)
- [Branch on a label with `choose` and `case`](https://github.com/botassembly/thinkthen/tree/main/demos/02-route-a-ticket)
- [Lint a change by meaning and fail the build](https://github.com/botassembly/thinkthen/tree/main/demos/43-lint-a-change)
- [Grade an assistant's answers with a rubric](https://github.com/botassembly/thinkthen/tree/main/demos/14-grade-a-batch)

[All how-tos](https://github.com/botassembly/thinkthen/blob/main/demos/README.md) lists every page. Each green one is a real shell job that the gate runs.

## Your data

By default ThinkThen sends your question and text to TypeSafe. [Backends](https://thinkthen.dev/install/backends/) links TypeSafe's terms. Read them before you send sensitive text.

## Contributing

[CONTRIBUTING.md](https://github.com/botassembly/thinkthen/blob/main/CONTRIBUTING.md) explains how to build from a checkout, run the gate, and record work. It also defines the four names this repository uses. No gate touches the network.

## License

MIT. [LICENSE](https://github.com/botassembly/thinkthen/blob/main/LICENSE) holds the text.
