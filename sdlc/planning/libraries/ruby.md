# The Ruby surface: goals and anti-goals

Shared rules live in [README.md](README.md). This page holds only what is particular to Ruby. Ticket 0112 ported the surface onto the public `thinkthen` API, and this page follows that build.

## What really good looks like

A Ruby programmer expects a gem to install in a second and disappear into the language. The names are the command's names, so `decide` stays `decide` and the `decide?` spelling is gone (ADR 0017 pick 1). A band answers `true`, `false`, or `nil`, with `nil` for unsure (pick 3), and the example leads with the `nil` check. The gem releases the global VM lock for the whole network wait, so eight threads take about the time of one. It adds nothing to the lockfile and patches no class the user owns.

```ruby
require "thinkthen"

dated = ThinkThen.filter("Does this note name a delivery date?", notes)
dated.each { |note| schedule(note) }

engine = ThinkThen::Engine.new(throttle: 8, cache: "/var/cache/notes")
engine.decide_many("Is this urgent?", notes, deadline: 30)
```

## Goals

- The lock is released for the whole wait. Each call runs on its own worker thread, and the Ruby thread waits in 50 ms slices with the lock released (ADR 0047, Ruby rulings).
- Control-C lands at once. The call raises `ThinkThen::CancelledError` with the `Interrupt` as its cause, within one slice. `Thread#raise`, `Thread#kill`, a raising trap, and a raising tick stop the call just as fast. Tests hold each stop to 150 ms on the loopback backend's held arm.
- A stopped call sends nothing new. It fires its own token, and the engine sends no request after that. Requests already sent finish on the detached worker, and their answers are dropped.
- `ThinkThen::Engine.new` takes `base_url:`, `model:`, `throttle:`, `max_requests:`, `cache:`, and `cache_bytes:`. Each omitted setting comes from the environment, as the module functions' engine does. The throttle holds per loaded copy (ADR 0047 item 5).
- A record verb crosses once for the whole list and maps places back to the caller's own objects. `filter` returns the records it was handed.
- A record that is not a `String` crosses as its JSON text. `nil`, invalid UTF-8, and a NUL byte refuse with `UsageError` naming the index, before any request.
- The gemspec lists no runtime dependency, carries the MIT license, and names the host platform.
- The extension sits at `lib/thinkthen/thinkthen.<DLEXT>` with `Init_thinkthen`. `build.sh` derives the file name from Cargo's output and Ruby's `DLEXT`.

## Anti-goals

- No monkey patching. A gem that edits `String` or `Array` breaks other gems.
- No HTTP, retry, recording file, or question grammar in Ruby. The keyword builders write the question file's JSON, and the engine reads it.
- No `method_missing` and no `instance_eval` block. Both hide the surface from `ri`, editors, and RBS.
- No Rails dependency and no railtie.
- No process-global client at load. The default engine is built on first use. The engine rebuilds its state after a fork (0096), and a forked child's counters start at zero.

## How the build runs

- The binding is `magnus` 0.7.1 over `rb-sys` 0.9.130, in its own Cargo workspace (ADR 0047). `src/ffi.rs` holds every `unsafe` line. `src/lib.rs` holds the guard, the error table, the handoff, and the settings. `src/call.rs` maps each verb to the public API.
- Ruby 3.4.11 and libyaml 0.2.5 build from source once per machine through `setup-ruby.sh`, pinned by `toolchain.env`. `check.sh` uses only that prefix and prints "not run" when it is missing or its stamp differs.
- No Docker, sudo, or network runs in the check.

## Tests only this surface needs

- Ctrl-C, `Thread#raise`, a raising tick, and a caller's token each stop a held call within 150 ms, single and batch, and the backend's count does not grow after the stop.
- A spurious `Thread#wakeup` and a harmless trapped signal leave a held batch running. A raise on one thread leaves a sibling sharing its token running.
- A running tick survives `GC.start` from another thread. A tick runs only for its own thread's calls.
- A raising `USR1` flood through four held 2,000-record batches, and 3,000 random `Thread#raise` calls, neither crash nor leak a watchdog row.
- A fork after the first call answers in the child.

## Open questions

1. Choices are strings. The conformance cases and the question file use strings, and `choose` returns the chosen string.
2. A band is a Ruby `Range` or the file's `"lo:hi"` text.
3. The fiber scheduler stays unsupported. Nothing tests it.
4. The Mac build waits for the release ticket (`sdlc/issues/2026-09-25-release-and-install-for-0-1.md`).
