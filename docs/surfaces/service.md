# Where record text goes

By default, ThinkThen sends record text and questions to the TypeSafe System One service at `https://api.typesafe.ai/v1`, using `THINKTHEN_API_KEY`. `THINKTHEN_BASE_URL` changes only the System One address; an explicit client URL or command `--url` overrides the selected backend’s address. Selecting `openai` sends them to OpenAI and reads `OPENAI_API_KEY`; selecting `stub` sends nothing over the network and needs no key. `THINKTHEN_CA_FILE` supplies additional PEM certificate authorities alongside the operating system’s trust store. Proxy routing reads `ALL_PROXY`, `HTTPS_PROXY` and `HTTP_PROXY`, including their lowercase forms; SOCKS proxies are refused with a transport error naming the variable; `NO_PROXY` or `no_proxy` bypasses matching destinations. Local plain HTTP connections bypass proxies. Set these variables before creating a client, and select a service you trust with the records and questions.

A retried or manually resent POST can be processed and billed more than once, even if the first response was lost. ThinkThen does not provide an idempotency guarantee across attempts. Account for retries in request and cost budgets; disabling status retries does not undo a request already received by the service.

## Local response storage and recovery

Response storage is off by default, and no environment variable enables it. Explicit `cache` or `record` mode retains questions, selected record text and valid raw backend responses in a private directory on a local filesystem. These bodies can contain sensitive source data. Authentication headers and the configured provider credential are excluded from response storage. Namespace and model-generation labels are explicit caller choices; they are not derived from credentials or credential hashes.

Strict `replay` reads an existing private recording without a provider key or network request. A missing answer fails locally, with no live fallback. Cache mode can send misses; record mode sends requests and saves their valid responses. The [response-store guide](bash.md#record-cache-and-replay-responses) gives the explicit settings, retention limits and record-then-replay example.

File recovery is separate. Named CSV, TSV and XLSX command operations create a private recovery artifact beside the intended output by default; `--no-recovery` disables it. Recovery for JSONL enrichment or table input on stdin requires an explicit `--recovery PATH`. An artifact may contain questions, completed typed answers and source metadata. Successful output publication and completion remove the artifact; interruption or failure can retain it, with its path in the diagnostic. `thinkthen recover FILE` exports committed answers offline; it does not resume the operation. See [recovering completed file answers](bash.md#recover-completed-file-answers).

## Choose a service and get a key

For System One, get an API key from the [TypeSafe console](https://console.typesafe.ai), following the [official quickstart](https://docs.typesafe.ai/introduction/quickstart), and set `THINKTHEN_API_KEY` before running the command or creating a client. Choose `systemone` for the TypeSafe service and its typed question protocol. Choose `openai` when you intend to send records to your OpenAI account; create an API key in the [OpenAI platform](https://platform.openai.com/api-keys) and set `OPENAI_API_KEY`. Each service bills its own account. Check its current access and pricing before sending records. Choose `stub` for offline installation checks, with synthetic answers.

## Relation rule fields

The engine’s serialized rule uses `source`, `target`, `relation`, optional `reads` and optional `either`. `reads` supplies wording; `either` marks an undirected relation. Bindings use their own public names:

| Interface | Relation name | Wording | Undirected |
| --- | --- | --- | --- |
| Core JSON, Python | `relation` | `reads` | `either` |
| TypeScript | `name` | `phrase` | `symmetric` |
| Ruby | `relation` | `phrase` | `symmetric` |
| Go | `Name` | `Reads` | `Either` |
| Kotlin, Zig, SQLite, DuckDB, PostgreSQL | `name` | `reads` | `either` |
| C# | `Relation` | `Reads` | `Either` |
| Swift, Java builder, R constructor | `relation` | `reads` | `either` |

All rules also name source and target kinds. C and C++ expose typed rule constructors and setters in their [guide](c-cpp.md).
