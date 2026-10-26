# Saved entries and runs

Run saved ThinkThen function requests and explicit local commands. This Rust module owns saved entry admission, local toolbox lookup, JSON-line pipelines and child process lifetime. It depends on `thinkthen` for function behavior and `thinkthen::files` for file-framed single-function entries.

```rust
use std::sync::mpsc;
use thinkthen::{Cancel, Settings};
use thinkthen::runs::{RunCompletion, RunEntry, execute};

let entry = RunEntry::from_bytes(
    br#"{"question":{"function":"decide","question":"Ready?"}}"#,
)?;
let client = Settings::from_json(r#"{"backend":"stub"}"#)?.engine(|_| None)?;
let (send, input) = mpsc::sync_channel(1);
send.send(Ok("\"alpha\"".to_owned()))?;
drop(send);
let mut rows = Vec::new();
let completion = execute(&entry, &client, Cancel::default(), input, |line| {
    rows.push(line.to_owned());
    Ok(())
});
assert!(matches!(completion, RunCompletion::Success));
assert_eq!(rows.len(), 1);
# Ok::<(), Box<dyn std::error::Error>>(())
```

Each input item contains one JSON value without its line ending. The caller owns input production and uses a bounded channel. A producer stops when its receiver closes or the caller's cancellation handle is set. The sink receives final JSON values. A run closes its pipes and reaps its declared child processes before returning. An early downstream reader stops the upstream work it no longer needs while the remaining downstream stages drain their records.

A toolbox file wraps named entries in `{"entries":{"ready":...}}`. A folder stores one entry per `.json` file. `load(directory, toolbox_or_name, name)` resolves an explicit toolbox and entry, a single-entry JSON file, or a name in `thinkthen.json` or `thinkthen/`. The command `thinkthen run NAME` reads `./thinkthen.json` or an entry under `./thinkthen/`. Inspect the saved entry before running it from an unfamiliar directory. Lookup does not change the process's working directory.

A run entry is `{"run":[...]}`. A function step uses the existing `SdkRequest` contract without `records`. A shell step is `{"shell":["program","argument"]}`. Arguments remain literal; shell syntax requires an explicit interpreter. Standard input and output carry JSON lines. Commands inherit the caller's environment, including backend API keys, and working directory. Run only toolbox entries and shell commands you trust with those keys. No function result becomes a command string or an implicit branch.

`RunCompletion` distinguishes success, an unchanged library error, a child exit status and caller cancellation. A failed function row never reaches another step. Records already written and effects of declared commands remain after failure. Each function call has its own cancellation handle; stopping one upstream stage does not discard valid downstream results.

For file framing, construct a `thinkthen::files::FileJob` and pass it to `RunEntry::execute_file`. That shared library boundary accepts exactly one function and otherwise returns `usage.file` before file or backend work. It delegates table parsing, row association, complete publication and failed-outcome reporting to `thinkthen::files`. Mixed-step file runs use JSON lines explicitly. Other language libraries expose the ten functions; this module adds no language or C ABI entry point.
