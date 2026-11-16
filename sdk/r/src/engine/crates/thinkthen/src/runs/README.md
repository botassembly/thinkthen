# Saved entries and runs

Run saved ThinkThen function requests and explicit local commands. This Rust module owns saved entry admission, local toolbox lookup, JSON-line pipelines and child process lifetime. It depends on `thinkthen` for function behavior and `thinkthen::files` for file-framed single-function entries.

A reusable saved question entry uses the command's `entries` envelope. Save this as `question-entries.json`:

```json
{"entries":{"ready":{"question":{"function":"filter","question":"Does the customer request a refund?","field":"/text"}},"topic":{"question":{"function":"choose","question":"What is the message about?","choices":["billing","shipping","other"],"field":"/text"}}}}
```

Run one named entry over the whole JSON Lines collection:

```bash
thinkthen run question-entries.json --entry ready records.jsonl --backend stub --json
```


Each input item contains one JSON value without its line ending. The caller owns input production and uses a bounded channel. A producer stops when its receiver closes or the caller's cancellation handle is set. The sink receives final JSON values. A run closes its pipes and reaps its declared child processes before returning. An early downstream reader stops the upstream work it no longer needs while the remaining downstream stages drain their records.

A toolbox file wraps named entries in `{"entries":{"ready":...}}`. A folder stores one entry per `.json` file. `load(directory, toolbox_or_name, name)` resolves an explicit toolbox and entry, a single-entry JSON file, or a name in `thinkthen.json` or `thinkthen/`. The command `thinkthen run NAME` reads `./thinkthen.json` or an entry under `./thinkthen/`, and `thinkthen run TOOLBOX NAME FILE...` names the toolbox first; `--entry NAME` is the same as NAME after TOOLBOX. A toolbox mistake is a usage error (exit 2) whose message names the toolbox file. Inspect the saved entry before running it from an unfamiliar directory. Lookup does not change the process's working directory.

A run entry is `{"run":[...]}`. A function step uses the existing `SdkRequest` contract without `records`. A shell step is `{"shell":["program","argument"]}`. Arguments remain literal; shell syntax requires an explicit interpreter. Standard input and output carry JSON lines, one JSON value per line between steps. The last step's output reaches the caller unchanged, so it may print plain text. Commands inherit the caller's working directory and environment, except backend API keys are removed unless the shell step lists their variable names in its `env` array. For example, `{"shell":["program"],"env":["THINKTHEN_API_KEY"]}` explicitly passes that key. Run only commands you trust with any explicitly passed keys. No function result becomes a command string or an implicit branch.

`RunCompletion` distinguishes success, an unchanged library error, a child exit status and caller cancellation. A failed function row never reaches another step. Records already written and effects of declared commands remain after failure. Each function call has its own cancellation handle; stopping one upstream stage does not discard valid downstream results.

Only single-function entries accept spreadsheet framing.
