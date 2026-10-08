# Rust examples

One runnable program for each `thinkthen` function, plus the deck's Rust slide and an offline recognition observer recipe. Each program depends on `thinkthen` the way an outside user does: by path, with default features off and `bundled-sqlite` on, through the public API alone. A build must name `bundled-sqlite` or `host-sqlite`, the SQLite the question cache runs on; only a SQLite extension names `host-sqlite`.

`choices!` keeps `Variant => "label"` for a bare label. Add `: "description"` for text, or a block returning `Result<Description, Error>` for a structured description built with `Description::builder()`. In a typed `choose` or `tag` builder, `None` takes that variant default and `Some(description)` overrides it for that label. A question loaded from JSON keeps its own complete ordered map, including explicit `null` entries; binding it to a `Choice` checks labels without replacing its descriptions.

```sh
export THINKTHEN_API_KEY=...
cargo run --example decide
```

Except for the explicit strict-replay observer recipe below, each program builds its engine with `Engine::from_env()`, so it reads `THINKTHEN_BASE_URL`, `THINKTHEN_API_KEY`, `THINKTHEN_CACHE`, and optional `THINKTHEN_CA_BUNDLE` as the command does. The CA file replaces bundled Mozilla trust for that engine; the explicit `EngineBuilder::ca_bundle(path)` setter overrides the environment value. The path must be absolute. The file is checked at `build`, even for replay-only engines. Certificate and hostname verification stay on.

`Engine::from_env()` also reads the configuration file's `cache: false` switch. A bare `Engine::builder()` starts with library defaults and does not read the configuration file, so that switch does not turn off its cache. Call `Engine::builder().no_cache()` to turn it off explicitly.

The answer cache is on by default. Each entry holds the complete request and reply, the judged text included, in plain text, with no expiry. Whoever can write the selected cache or recording folder controls the answers read from it; keep that folder private to people whose answers you trust. `cache prune` is the only thing that removes entries. Turn it off with `EngineBuilder::no_cache`.

The builder also sets `max_request_bytes`, `timeout`, `max_retries`, `profile`, `record`, and strict `replay`; replay alone sends nothing, even on a miss. The request-byte ceiling defaults to 96,000, and a smaller profile value wins.

## Run facts

Each eager engine call returns `Call<T>`. Read its answer with `value()` or take it with `into_value()`. `facts()` reports this call's finished records, sent attempts, cache answers, optional provider tokens and model, and wall seconds. A started eager call that fails exposes its final facts through `Error::facts()`. These are call facts, not a delta of the process totals. No call reports price, server time, or a provider request ID.

`Engine::details` returns `Call<Details>` for one text. Its `Details::to_json()` is the command's `--details` line, schema `thinkthen.result/1`, and its accessors read the same fields: `usage()`, `requests_sent()`, `cached()`, `requests()`, `url()`, `model()`, `probabilities()` and `confidence()`. The backend's reply supplies `meta.model`, `meta.usage` with its input and output tokens, and every probability, with `answer.confidence` when the backend sends one. The engine counts `meta.requests_sent` and sets `meta.cached` when a cache or recording answered. `meta.requests` holds the recording digest of each request, and `meta.url` names the address that answered. A field the backend did not report is absent.

`decide_many`, `filter`, `rank`, `choose_many`, `score_many`, `tag_many`, `details_many`, and `annotate` run on the command's question pipeline, by ADR 0111. Each question keeps its own cache key, so a longer call over the same records asks only the new records' questions. Rows list those keys in `meta.requests` and carry no `meta.batch`. `EngineBuilder::batch` selects the default; `CallOptions::batch` overrides it for one call. `EngineBuilder::from_env` reads `THINKTHEN_BATCH`; a bare builder does not. A parsed question file's batch is the lower tier. `CallOptions::context` borrows shared text for an eligible many-record call. Single-document calls, find, annotate, recognize and relate refuse that option before sending. Lazy `Batch<T>::facts()` becomes available after exhaustion or its terminal error. An observation callback can inspect borrowed question and row events in order, including a row omitted by `filter`; copying a field inside the callback retains it. A callback panic resumes on the caller after workers join.

The pipeline runs on a background thread while the calling thread pulls each record, so records need not be `Send`. `Max` sends the open request after a size or profile limit, 4,096 records, a full window, exhaustion, a local refusal, or 50 ms with no new record, including while the iterator blocks in `next()`. An interactive caller should select `BatchSetting::Records(1)`; each answer then returns before the library asks for the next input.

`Engine::usage()` returns this engine's running totals of requests sent, retries, cache answers, input tokens and output tokens. Retries are a subset of requests sent.

## Width is per loaded copy

The engine's width is the limit on requests in flight at once. It holds per loaded copy of the engine. A process that loads two copies, such as a Rust program and a native extension built from another surface, can run up to twice the width (ADR 0047 item 5).

## Checks

A bare `cargo test` builds every example, then runs `tests/examples.rs`. `cargo test --test examples` alone would run stale programs. It starts a loopback backend for each program, runs the program in its own process, and compares its output with the `.txt` file beside it. The loopback backend's generic arm answers every question by a fixed rule: the first option, level, label, or yes gets 0.9. The pinned text shows that rule's answers. A model would answer differently.

`check.sh` is this surface's entry in the surface rung, `sdlc/scripts/surfaces`. It runs the formatter, Clippy, and the tests. It then runs the slide against the rung's own loopback backend. That run proves the port the rung hands every surface.

## Rank a saved question set

`RankSet::from_json` and `RankSet::load` admit ordered decide sets without authored thresholds or member pointers. `Engine::rank_set(&set, records)` and `rank_set_with` judge every member over every input, sort each member stably, then take turns at each depth in saved member order. Duplicates consume visits and each original input position appears once. Rust returns the full order; take its prefix for a top selection.

`SetRanked<T>` owns the original and provides `index()`, `input()`, `into_input()`, `probability()` and `question_name()`. The probability belongs to the selecting member. Originals need no Clone, Send or Serialize implementation. One CallOptions and one facts report cover the whole call; records count inputs, not member answers. Question observations retain every named member answer and receipt. Any member failure fails the whole call. Existing `rank`, `Ranked` and `RankedRow` stay unchanged. SQL sets belong to ticket 0417 for 0.2; C and language sets belong to 0418 later.

```rust
let set = thinkthen::RankSet::from_json(
    r#"{"version":1,"questions":{"billing":{"decide":"Billing issue?"},"recovery":{"decide":"Recovery steps?"}}}"#,
)?;
let ranked = engine.rank_set(&set, ["First passage", "Second passage"])?;
for row in ranked.value().iter().take(5) {
    let _ = (row.index(), row.input(), row.question_name(), row.probability());
}
```

The CLI contract and executable replay examples live in [rank](../../specification/rank.md#rank-question-sets) and [rank sets](../../spec/rank-sets.md).

Explicit files and folders use the [library reader contract](../files.md), with line, window or whole-file units and located results. Existing text, record and column methods retain their arguments.

## Inspect recognition questions without a key

[recognize_observe.rs](examples/recognize_observe.rs) is a complete native recipe. From this folder, run:

```sh
env -u THINKTHEN_API_KEY cargo run --locked --offline --example recognize_observe
```

It builds an explicit engine with no cache and strict replay of the saved generic receipt fixture. It uses `CallOptions::new().observe(&callback)`, owns borrowed question details inside the callback, and groups them by original record index, stage and question position. It prints the actual normalized question text, declared options and descriptions, typed answer or failure, probabilities, request references and source metadata. `Row` events mark completion, and the returned records retain their originals. The fixture runs twice to show separate zero-based original occurrences.

The [recognition observation contract](../../specification/recognize.md#read-native-stage-observations) explains stage conditions, failures, privacy and the CLI/serialized-event limits. The existing example gate pins the output in [recognize_observe.txt](examples/recognize_observe.txt), removes the key and counts zero connections at the saved loopback endpoint. This recipe is the exception to the generic live loopback rule above; the saved endpoint must be available for the gate to bind its counter.
