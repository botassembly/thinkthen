# A usage write that fails after a good start is silent on the libraries

Status: open. Filed by ticket 0360.

Owner: the ticket that gives the bindings a warning channel, or the first monthly spend limit.

Kind: debt

Debt: 032

Severity: low

Pay when: a binding gains a warning channel, or a monthly spend limit reads the usage totals.

Milestone: later

Keeping it risks a usage count that stops partway through a long library or SQL session without a word. A status read afterwards then shows less spend than was sent.

## What happens

Ticket 0360 reads the usage totals once before a process's first send and refuses when the folder cannot be read. A write can still fail later:

- The disk fills.
- The folder's permissions change mid-run.
- Another program writes a bad month file.
- A cache answer arrives before any send in the process. Cache answers never pass the check.

The writer then stops for the rest of the process (`crates/thinkthen/src/engine/usage.rs`, `Queue.failed`). The command prints its fixed warning at exit (`crates/thinkthen/src/cli/mod.rs`). `Engine::finish_usage` discards the result (`crates/thinkthen/src/engine/facade/finish.rs`), so the libraries, the C door and the SQL extensions say nothing.

## For marketing

Several `site/` pages and example outputs still show the old `thinkthen-usage` folder under the cache home:

- `site/src/pages/install/configuration.astro`
- `site/examples/install/configuration/1-default.out`
- `site/examples/install/configuration/2-xdg.out`

The usage totals now live at `$XDG_STATE_HOME/thinkthen` (`~/.local/state/thinkthen`) on Linux and `~/Library/Application Support/thinkthen/usage` on macOS (`specification/recording.md`).
