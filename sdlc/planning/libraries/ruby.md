# The Ruby surface: goals and anti-goals

Shared rules live in [README.md](README.md). This page holds only what is particular to Ruby.

## What really good looks like

A Ruby programmer expects a gem to install in a second and disappear into the language. The names are the command's names, so `decide` stays `decide` and the `decide?` spelling is gone (ADR 0017 pick 1). A band answers `true`, `false`, or `nil`, with `nil` for unsure (pick 3), and the example leads with the `nil` check. The best such gem releases the global VM lock for the whole network wait, so eight Sidekiq threads take about the time of one. It adds nothing to the lockfile and patches no class the user owns.

```ruby
require "thinkthen"

delivery_date = ThinkThen.question("Names a delivery date.")
dated = ThinkThen.filter(delivery_date, notes, field: "/text", jobs: 8)
dated.each { |note| schedule(note) }
```

## Goals

- The lock is released for the whole wait. A bench of eight threads against a stub backend matches one call.
- Control-C lands while Rust blocks. The engine's poll runs on the calling thread, the closure cancels the token, and the call raises `Interrupt`. The 205 finding it retires: a batch waited out the whole run, 8.07 s deaf; the engine token cut that to 0.23 s in the stand-in and the poll carries it with no chunking (ADR 0017 section 2).
- `gem install thinkthen` runs no compiler where a prebuilt gem exists. The gemspec lists no runtime dependency.
- A record verb crosses the barrier once and yields the caller's own objects, lazily. Ruby spells `decide`'s bulk form `decide_many` (ADR 0017 pick 8).
- Strings come back as UTF-8. The no-GVL region must own its data, so the evidence text is copied into Rust-owned memory every call, frozen or not; the copy is microseconds-irrelevant and the page says so instead of promising a free ride (205).
- Threads are supported and tested. A `Fiber` under a fiber scheduler stays unsupported, now with evidence (205).
- The gem ships RBS signatures in `sig/` and no Sorbet RBI.
- The extension's init symbol derives from the file's basename under Ruby 3.4, so the `.so` sits at `lib/thinkthen/thinkthen.so` with `Init_thinkthen`. rake-compiler's convention is forced, not chosen (205).

## Anti-goals

- No monkey patching. A gem that edits `String` or `Array` breaks other gems.
- No HTTP, retry, JSON, or recording file in Ruby. Writing it here writes it once for every host language.
- No `method_missing` and no `instance_eval` block. Both hide the surface from `ri`, editors, and RBS.
- No Rails dependency and no railtie. The gem has to work in a plain script first.
- No process-global client at load. The engine owns the pool and rebuilds it after a fork on a process-ID mismatch, taking no inherited lock (ADR 0017 section 2). 205 confirmed no gem can fix the fork alone — rebuilding the question value in the child still landed on a dead process-global state — and 211 proved the engine-side fix. Until the real engine lands, prefork boots where the master never calls remain the safe shape for Puma and Sidekiq.

## Where this language wastes time

- **The global VM lock.** Held through a network wait, it serializes every thread. The shim reads a whole chunk under the lock, turns it into Rust-owned data, then calls `rb_thread_call_without_gvl` for that whole batch and touches no Ruby object inside. `magnus` does not wrap that call, and its `rb_sys` module is the door to it.
- **A call per record.** Each crossing pays a conversion and an allocation. A chunk goes down in one call and comes back in one.
- **String copies.** The widest containers Ruby has are an `Array`, any `Enumerable`, and an `Enumerator::Lazy`, and none of them reads many strings in one call. `RString::as_str` views Ruby's own memory with no copy. It is unsafe, it demands UTF-8 or US-ASCII, and it holds only while no other Ruby call runs. `RString::new_frozen` is the safe form and copies once. Whether `magnus` has a bulk string reader is unchecked.
- **Pulling from a lazy enumerator costs the lock.** `first(n)` and `next` are Ruby method calls. The shim takes one chunk, releases the lock for that whole batch, then takes the next chunk. Memory holds at one chunk plus the width, so an endless enumerator runs flat.
- **Work the shim must not do.** The engine asks an equal pair of question and evidence once inside a batch, and a cached answer costs nothing. No `uniq` and no hash of seen records in Ruby. `jobs` is one number for the process, and eight threads and two callers feed the one scheduler in Rust. A `decide` inside `select` is serial, and `ThinkThen.filter` and `ThinkThen.decide_many` are the bulk forms.
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
- An endless lazy enumerator through `filter` holds memory flat.
- A bench counts rows a second through `filter` over an array against the stub, beside the engine's own number from pure Rust. A gap is a defect in the shim.

## Open questions for the ADR

1. Answered by 205: `Outcome` is a plain symbol. It reads in `case` and costs nothing.
2. Are choices symbols or strings? `%i[billing shipping other]` reads better, and the homepage shows `%w[]`. The stand-in carried only the decide family, so `choose` never ran; the real engine's conformance cases settle it.
3. Answered by ADR 0017 section 2: the engine's process-ID check rebuilds the pool in the child, and 211 proved it on the wire and during live batches.
4. The fiber scheduler stays unsupported, now with evidence (205).
5. Does the shim borrow through the unsafe `as_str` or copy through `new_frozen`? Borrowing is free and demands UTF-8 and no Ruby call while the reference lives. One rule covers every verb.
