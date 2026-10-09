# A usage write that fails after a good start is silent on the libraries

Status: closed.
Resolution: 0468

Owner: ticket 0468.

Kind: debt

Debt: 032

Severity: medium

Pay when: 0468 implements explicit persistence reporting before 0.2.

Milestone: 0.2

Keeping it risks a usage count that stops partway through a long library or SQL session without a word. A status read afterwards then shows less spend than was sent.

## What happens

Ticket 0360 reads the usage totals once before a process's first send and refuses when the folder cannot be read. A write can still fail later:

- The disk fills.
- The folder's permissions change mid-run.
- Another program writes a bad month file.
- A cache answer arrives before any send in the process. Cache answers never pass the check.

The writer then stops for the rest of the process (`crates/thinkthen/src/engine/usage.rs`, `Queue.failed`). The command prints its fixed warning at exit (`crates/thinkthen/src/cli/mod.rs`). `Engine::finish_usage` discards the result (`crates/thinkthen/src/engine/facade/finish.rs`), so the libraries, the C door and the SQL extensions say nothing.

## Site pages for the builder

Several `site/` pages and example outputs still show the old `thinkthen-usage` folder under the cache home:

- `site/src/pages/install/configuration.astro`
- `site/examples/install/configuration/1-default.out`
- `site/examples/install/configuration/2-xdg.out`

The usage totals now live at `$XDG_STATE_HOME/thinkthen` (`~/.local/state/thinkthen`) on Linux and `~/Library/Application Support/thinkthen/usage` on macOS (`specification/recording.md`).

## Reconciliation, 2026-10-08

The earlier warning-channel trigger is superseded by 0468. Successful answers must retain explicit pending/written/failed/disabled persistence facts. This remains a runtime defect, not later debt. The old site paths below describe historical observations; current configuration uses platform state paths.
