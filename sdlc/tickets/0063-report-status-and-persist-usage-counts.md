---
flow: build
priority: 36
opens: crates/thinkthen specification README.md AGENTS.md sdlc/issues sdlc/planning sdlc/ratchet.json
---

# 0063: Report status and persist numeric usage counts

Status: landed

## Outcome

`thinkthen status` reports resolved local settings, answer-cache size, and numeric usage for the current UTC month and all recorded months. `status --json` emits the same facts as one closed object. Live sends, provider-reported tokens, and answer-cache hits update count-only monthly totals beside the platform cache. Status never outputs, formats, or persists a key value, sends a request, or changes a file. Usage bookkeeping never changes a judgment result.

## Current facts and decisions

Ian's later build-queue ruling controls: status shows requests sent, input and output tokens, and cache answers for this month and in total. Plain count rows contain no judged text and enforce no budget. This supersedes ADR 0017's older process-only sentence and the status issue's older no-ledger recommendation. Ticket 0062 controls configuration and answer-cache paths, precedence, privacy, allocated-byte measurement, and the rule that `--no-cache` does not disable numeric bookkeeping.

Human output is one stable `name value` line in this order:

```text
version 0.0.1
configuration_path /home/u/.config/thinkthen/config.json
configuration_present false
url https://api.typesafe.ai/v1/systemone
url_source built_in
model jev-latest
model_source built_in
api_key_set false
cache_enabled true
cache_enabled_source built_in
cache_path /home/u/.cache/thinkthen
cache_path_source platform
cache_entries 0
cache_bytes 0
cache_target_bytes 100000000
cache_target_source built_in
usage_path /home/u/.cache/thinkthen-usage
usage_month 2026-09
month_requests_sent 0
month_input_tokens 0
month_output_tokens 0
month_cache_answers 0
total_requests_sent 0
total_input_tokens 0
total_output_tokens 0
total_cache_answers 0
```

`status --json` prints one compact line with this closed shape:

```json
{"schema":"thinkthen.status/1","version":"0.0.1","configuration":{"path":"/home/u/.config/thinkthen/config.json","present":false},"backend":{"url":"https://api.typesafe.ai/v1/systemone","url_source":"built_in","model":"jev-latest","model_source":"built_in","api_key_set":false},"cache":{"enabled":true,"enabled_source":"built_in","path":"/home/u/.cache/thinkthen","path_source":"platform","entries":0,"bytes":0,"target_bytes":100000000,"target_source":"built_in"},"usage":{"path":"/home/u/.cache/thinkthen-usage","month":"2026-09","this_month":{"requests_sent":0,"input_tokens":0,"output_tokens":0,"cache_answers":0},"total":{"requests_sent":0,"input_tokens":0,"output_tokens":0,"cache_answers":0}}}
```

URL sources are `environment`, `configuration`, and `built_in`. Model sources are `configuration` and `built_in`. Cache path sources are `environment` and `platform`; enabled sources are `environment`, `configuration`, and `built_in`; target sources are `configuration` and `built_in`. `THINKTHEN_CACHE` is an environment choice and enables the answer cache. Status takes only `--json`, so verb-local overrides do not apply.

When no absolute configuration home exists, `configuration_path` is `unavailable`; JSON uses `path:null`. When no effective answer-cache path exists, its path, entries, and bytes are `unavailable`; JSON uses nulls. When no platform usage path exists, human output prints `usage_path unavailable` and `unavailable` for all eight counts; JSON uses `path:null`, `this_month:null`, and `total:null`. `configuration_present` and `usage_month` remain concrete. Missing cache and usage directories otherwise mean zero. This preserves the accepted no-home behavior of `--no-cache`, explicit record/replay, dry-run, and prune.

Usage lives in the platform sibling `thinkthen-usage`, never in `THINKTHEN_CACHE`, an explicit recording, or the configuration folder. That keeps one history and lets `--no-cache` avoid creating the answer-cache folder. The usage directory contains a permanent `.lock`, at most one ignored `.update.tmp`, and exact UTC `YYYY-MM.json` monthly files. Each monthly file is one closed aggregate row:

```json
{"schema":"thinkthen.usage/1","requests_sent":1,"input_tokens":0,"output_tokens":0,"cache_answers":0}
```

The writer creates the usage directory as `0700` and lock and data files as `0600`. It verifies the same modes on existing objects, refuses symlinks and non-regular objects, opens handles and verifies their identity, and never silently changes permissions. First setup creates and syncs the stable lock before publishing state. Each update takes the exclusive lock, reads only the current month, adds with checked arithmetic, writes and syncs `.update.tmp`, atomically replaces the monthly file, then syncs the directory. A crash may leave only the ignored temporary file; the last complete monthly total remains readable. The update cost stays constant as the month grows.

Status never creates the lock. An absent usage directory is zero. An existing usage directory without a valid private lock is a local failure. Status takes a shared lock, validates every recognized month, and sums with checked arithmetic. Recognized names have a real year and month; unknown names are ignored. A malformed row, non-regular object, identity change, bad mode, unreadable state, or overflow fails with no partial output or repair. Cache inspection reuses ticket 0062's validity, allocated-byte, privacy, and folder-gate rules.

The counters mean:

- The engine process counter increments `requests_sent` immediately before every HTTP attempt, including retries and failures. Persistence makes a best-effort atomic precharge before the send.
- Provider-reported input and output tokens increment once for each live successful response whose usage object the adapter validates, including a response with a later refused answer. Replayed bytes never add tokens.
- `cache_answers` increments after a valid default, `--cache`, or `THINKTHEN_CACHE` hit decodes. Explicit replay and an explicit record/replay pair do not count. A repaired entry is a live send.
- Packed annotate is one exchange. Output failure does not undo counts.

The engine owns the four process counters. The later Rust library exposes their snapshot; this ticket keeps it private and proves it. Persistent statistics are observational. A missing platform path or the first lock, validation, write, rename, or sync failure disables persistence for the rest of the process. The command prints exactly one warning after ordered results: `thinkthen: usage counters could not be updated; check the usage folder permissions and free space`. The judgment output and exit meaning stay unchanged. Totals can undercount after that warning or a crash before a later token update, and can overcount one request after a crash between precharge and socket write. The manual calls them local conservative statistics, never an account bill.

Status is strict, read-only, and offline. It checks only whether `THINKTHEN_API_KEY` is nonblank and never outputs, formats, or persists its value. ADR 0017 is amended for persistent command counts. ADR 0034 records the public status shape, provenance, atomic monthly totals, counter meanings, unavailable state, and warning. Ian can overturn the field names, UTC period, sibling path, warning policy, and explicit-replay exclusion. The count categories, periods, count-only storage, no budget, and offline status are ruled.

## Scope

Add private engine process counters, atomic count-only monthly totals, `status` and `status --json`, read-only cache inspection, configuration provenance, the fixed warning, ADR 0034, the ADR 0017 amendment, and only the help, specification, roadmap, queue, and issue lines whose claims change. Preserve answer-cache entries and request bytes.

Excluded: budgets, prices, provider totals, reachability or model listing, usage reset/prune, daily/model/address breakdowns, configuration writes, automatic cache prune, `meta.replayed` naming, retry timing, Ctrl-C, Windows release support, public library APIs, and paid calls.

## Acceptance

- Golden human and JSON tests freeze both public shapes, provenance, absent and no-home forms, key presence without its value, cache counts and target, and strict no-partial-output failures. Status sends zero requests and creates or changes no filesystem object on success and failure.
- Pure Linux/macOS path and UTC-month tests cover relative and blank homes, the usage sibling, and config/cache/usage independence. Environment and configuration precedence is table-tested. `THINKTHEN_CACHE` moves cache status but not usage.
- Cache and usage readers refuse malformed recognized files, symlinks, non-regular objects, unsafe modes, identity changes, unreadable state, and overflow. Missing state is zero. Status holds the shared usage lock and shared cache folder gate. Tests pin first-creation ordering and a crash residue that leaves the last monthly total readable.
- Retry, failure, cache-hit, explicit-replay, and packed-annotate fixtures prove the four process and persistent counts. Two processes update one month without a lost total. Month rollover and all checked additions are deterministic. Ledger, warning, error, and `Debug` marker sweeps find no key or judged text.
- Injected setup, lock, validation, write, file-sync, rename, and directory-sync failures warn once, preserve judgment output and exit meaning, and prevent later persistent attempts in that process. Missing platform usage path proceeds silently and status reports unavailable.
- Existing results, requests, recordings, cache entries, order, exits, configuration, prune, and no-home behavior stay compatible. The affected ADR, specification, help, roadmap, queue, and two open issues agree. The four offline gates and `git diff --check` pass.

## Dependencies

Ticket 0062, accepted ADR 0017, ADR 0033, Lane A4 of the build queue, and the open status and cache-surface issues.

## Complexity

- Contract: 2
- State and timing: 2
- Reach: 1
- Proof: 2
- Cost of error: 1
- Total: 8
- Minimum level floor: shared durable state
- Final level: 3
- Reasons: one public status surface reads cross-process atomic counts at HTTP, adapter, cache, command, and filesystem boundaries. A failure loses local statistics but never a judgment, credential, or judged text.
- Selected model: `gpt-5.6-sol` with medium reasoning.

Re-score if the work adds limits, prices, provider calls, public engine methods, or another storage format.

## Review

Independent design review rejected the first draft because an interrupted append could poison every future run, validation made append cost grow quadratically, read-only locking and existing-object privacy were incomplete, no-home and cache provenance shapes were incomplete, durability wording exceeded best-effort behavior, and the complexity score was too high. The rewrite uses one atomically replaced monthly aggregate, validates opened private objects, closes every public unavailable and provenance field, states undercount and overcount honestly, narrows proof, and routes the level-3 work to Sol Medium.

Independent design re-review accepted the rewritten contract, its level-3 Sol Medium route, and keeping status with the counters it reports. It requested three wording and schema simplifications, which remove a false parentheses claim, describe key-presence inspection accurately, and drop a redundant `usage.available` field.
