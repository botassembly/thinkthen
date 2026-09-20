# The Ruby surface: goals and anti-goals

Shared rules live in [README.md](README.md). This page holds only what is particular to Ruby.

## What really good looks like

A Ruby programmer expects a gem to install in a second and disappear into the language. The question mark already means a predicate, so `ThinkThen.decide?` needs no explanation. A band drops the question mark, so `ThinkThen.decide` with a Range returns an `Outcome`. The best such gem releases the global VM lock for the whole network wait, so eight Sidekiq threads take about the time of one. It adds nothing to the lockfile and patches no class the user owns.

```ruby
require "thinkthen"

delivery_date = ThinkThen.question("Names a delivery date.")
dated = ThinkThen.filter(delivery_date, notes, field: "/text", jobs: 8)
dated.each { |note| schedule(note) }
```

## Goals

- The lock is released for the whole wait. A bench of eight threads against a stub backend matches one call.
- Control-C lands while Rust blocks. The unblocking function cancels, and the call raises `Interrupt`.
- `gem install thinkthen` runs no compiler where a prebuilt gem exists. The gemspec lists no runtime dependency.
- A record verb crosses the barrier once per chunk. `filter` yields the caller's own objects, lazily.
- Strings come back as UTF-8. A frozen one goes down without a copy where `magnus` allows it.
- Threads are supported and tested. A `Fiber` under a fiber scheduler is unchecked.
- The gem ships RBS signatures in `sig/` and no Sorbet RBI.

## Anti-goals

- No monkey patching. A gem that edits `String` or `Array` breaks other gems.
- No HTTP, retry, JSON, or recording file in Ruby. Writing it here writes it once for every host language.
- No `method_missing` and no `instance_eval` block. Both hide the surface from `ri`, editors, and RBS.
- No Rails dependency and no railtie. The gem has to work in a plain script first.
- No process-global client at load. A pool made before `fork` is dead in the child.

## Where this language wastes time

- **The global VM lock.** Held through a network wait, it serializes every thread. The shim wraps the engine call in `rb_thread_call_without_gvl` and touches no Ruby object inside.
- **A call per record.** Each crossing pays a conversion and an allocation. A chunk goes down in one call and comes back in one.
- **String copies.** A copy each way per record beats the shim's own cost. Frozen strings skip the copy in.
- **Load time.** A `require` that pulls a chain of Ruby files spends the budget first. This one loads a compiled file and a small Ruby file.

## How little code

The binding is `magnus` over `rb-sys`, because `rb-sys` is what RubyGems and Bundler build Rust extensions with today. The build is `rb_sys` with `rake-compiler`.

The shim holds argument and keyword conversion, the lock release and its unblocking function, chunking, answer conversion, and the mapping of a failure to a `ThinkThen::Error` subclass.

It never holds threshold math, band rules, JSON, retries, the rate limit wait, recording names, or the question grammar.

`rake-compiler-dock` cross-builds the platform gems: Linux x86_64 and arm64 against glibc, macOS x86_64 and arm64. Linux musl and Windows are unchecked. A source gem ships beside them.

## Tests only this surface needs

- Control-C during a call raises `Interrupt` and leaves the pool usable.
- A `fork` after the pool exists gives a working child or a named error, never a hang.
- `filter` yields the same `object_id` it was handed, lazily.
- `gem install` on an image with no Rust toolchain, then a call under replay.

## Open questions for the ADR

1. Is `Outcome` a symbol (`:yes`, `:no`, `:unresolved`) or a value object equal to one? A symbol reads best in `case`.
2. Are choices symbols or strings? `%i[billing shipping other]` reads better, and the homepage shows `%w[]`.
3. How does the pool survive `fork` under Puma and Sidekiq? A pid check per call, a fork hook, or a rule that the child builds it.
4. Does the fiber scheduler move from unchecked to supported before release?
