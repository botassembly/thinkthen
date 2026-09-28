# A newer usage file breaks an older build's `status` and its usage writes

Status: Open. Filed 2026-09-28 from local experiment 345, finding 1. Severity 2: a working install breaks with no clear message after any newer binary touches the machine.

## What happens

`engine/usage/counts.rs` gained a `retries` field while the file schema stayed `thinkthen.usage/1`, and `Counts` reads with `deny_unknown_fields`. After one call on a current build writes `~/.cache/thinkthen-usage/2026-09.json`, every older build on the machine fails in two places:

```text
$ thinkthen status          # binary 02dc0b96, and 72d60786
thinkthen: status could not read the local cache or usage state; check its permissions and contents
```

```text
thinkthen: usage counters could not be updated; check the usage folder permissions and free space
```

The second line appears on every live call of an older build, so its spend stops being counted. The message names no file and blames permissions and disk space, neither of which is the cause. Reproduced on 2026-09-28 with the two named binaries against the real usage folder, and again in a clean `XDG_CACHE_HOME` where an empty regular file at the usage path or a newer month file triggers the same error. The current build reads the same file without complaint, so the failure is one-way: new writes break old readers.

This is the same defect class as review finding D19, the schema identifier that never versions. A user who keeps a released binary, or who runs the bench's pinned build as the Beatles Bench instructions direct, hits it as soon as any newer build runs once.

## Evidence

Local experiment 345, sections 1 and 6. The machine's usage file carries `{"schema":"thinkthen.usage/1","requests_sent":…,"retries":0,…}`. `crates/thinkthen/src/engine/usage/counts.rs` shows `deny_unknown_fields`, the `retries` field with `serde(default)`, and the unchanged `thinkthen.usage/1` rename. `cli/status.rs` maps both read failures to `Failure::StatusState`, whose text names no path.

## Direction

Either move the schema identifier when a field is added, or keep a tolerant reader for fields it does not know, and name the file in the failure message. The run-time cost of either is small.

## Done when

An older build on a machine whose usage file holds `retries` either reads it or says which file it cannot read, and a test pins the chosen rule.
