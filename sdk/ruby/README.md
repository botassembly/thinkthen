# ThinkThen for Ruby

Ask typed questions about text from ordinary Ruby code. The gem calls the shared engine through its C ABI.

Install the platform gem with `gem install thinkthen --version 0.2.0`. Require the library and open a client with a block:

```ruby
require "thinkthen"

ThinkThen::Client.open(adapter: "stub") do |client|
  client.decide("Does the message request a refund?", ["Please refund my order."]) do |row|
    puts row.answer if row.status == "answered"
  end
end
```

The stub gives deterministic test answers. Choose a backend and supply its configured environment key for real judgments. Client construction accepts keyword settings from the [client contract](../../schemas/client.schema.json), including `mode`, `adapter`, `url`, `key_source`, `model` and `batch`. Rust owns their defaults and admission. The same client and function API applies in either mode. Ruby implements no transport.

## Ten functions

Each function takes an enumerable of records and returns `ThinkThen::Rows`, an Enumerator of immutable rows. Supplying a block iterates those rows and closes the call in `ensure`. One text record is `[text]`; an array inside that enumerable is one JSON record. Strings are text, including strings that look like JSON. Other records use Ruby JSON values. Hash keys may be strings or symbols when Ruby serializes input. Decoded objects always have string keys.

```ruby
ThinkThen::Client.open(adapter: "stub") do |client|
  text = "Ada works for Acme.\nThe delivery arrives Friday."
  records = [text]

  client.decide("Does this mention a delivery?", records) { |row| p row.answer }
  client.choose("What is this about?", records,
                options: ["employment", "delivery", "other"]) { |row| p row.answer }
  client.tag("Which topics occur?", records,
             labels: ["employment", "delivery"]) { |row| p row.answer }
  client.score("How urgent is this?", records,
               levels: ["routine", "urgent"]) { |row| p row.answer }
  client.filter("Does this mention a delivery?", records) { |row| p row.id if row.kept? }
  client.rank("Does this need immediate attention?", records, top: 1) { |row| p row.id }
  client.find("When does the delivery arrive?", records) { |row| p row.answer }
  client.annotate({"questions" => {
                    "delivery" => {"decide" => "Does this mention a delivery?"}
                  }}, records) { |row| p row.answer }
  client.recognize(["person", "company"], records) { |row| p row.answer }
  client.relate([{"name" => "works_for", "source" => "person", "target" => "company"}],
                records) { |row| p row.answer }
end
```

Every function uses `(question, records, **options, &block)`. Text questions become tagged inline questions. Annotate takes its inline toolbox Hash; recognize takes its kinds Array; relate takes its rules Array. `ThinkThen.toolbox(value, name: nil)` and `ThinkThen.reference(name)` wrap the corresponding tagged question forms without admitting them. Options pass to the [request contract](../../schemas/request.schema.json); unknown keys and invalid combinations reach Rust unchanged. Ruby checks no threshold, name, evidence pointer, backend, question or selection rule.

The enumerable is consumed on demand after the C ABI requests input. Ruby sends one text or JSON record at a time. It neither collects the enumerable nor creates a producer thread. A source exception propagates unchanged after cancellation and cleanup. The source remains caller-owned; Ruby does not close a file supplied as a source. Full ranking follows the engine's retention rule. Filter exposes the shared stream rows and their `kept?` flag, including failed rows; callers select kept records with that flag. Rank follows engine rank order. Other functions follow input order.

## Rows and errors

`ThinkThen::Row` is a hand-written frozen value with readers `id`, `status`, `answer`, `p`, `error`, `detail`, `position` and `kept?`. `detail` decodes its copied raw JSON only when read. Its reader does not mutate the frozen row or cache a mutable object inside it. Returned nested JSON values use ordinary Hash, Array, String, Integer, Float, true, false and nil. Those nested values remain mutable. Row fields have no writers. No generated result hierarchy is shipped.

The parser uses Ruby JSON with string keys, without class additions or symbolization. JSON conversion disables Ruby JSON’s separate nesting limit, so Rust owns request depth admission and Ruby can decode every admitted result. It preserves escaped NUL in strings and object keys, JSON null as nil, and the full unsigned 64-bit integer range as Integer. No C string conversion reads a payload: every native buffer is copied using its explicit length. Absence and explicit null remain distinct in the retained row fields; `row.to_h` includes only fields present in the contract and adds detail only when present. Convenience readers return nil for either absence or null. Position and kept are native metadata, outside `to_h`. Text offsets use Unicode code points to match Ruby UTF-8 String indexing.

`ThinkThen::Error < StandardError` has a `code` reader and the engine's unchanged message. Client and call admission failures and terminal native errors raise this exception. A stream row with `status == "failed"` remains a row; its `error` is a plain JSON Hash. Cancellation raises the shared `local.cancelled` error when the driver pulls its terminal result. Ruby encoding or enumeration exceptions remain Ruby exceptions. A missing packaged native asset raises `LoadError` and never triggers a download or a build.

## Iteration and ownership

Each function also accepts `cancel: ThinkThen::Cancel.new`. A caller may share that handle across calls and invoke `cancel` from another thread. Cancelling the handle stops its attached calls and future calls. Closing one call leaves the handle usable.

`Rows < Enumerator` owns one call and supports Ruby's usual `each`, `map`, `to_a`, `next` and `peek`. It is a one-shot stream. `rewind` raises `IOError`; it cannot repeat backend calls. Its extra methods are `cancel`, `close` and `closed?`. `cancel` requests native cancellation and leaves the owner available for the driver to observe the terminal error. `close` cancels and releases ownership. Both are idempotent. No result reader holds native memory after a row is yielded.

```ruby
ThinkThen::Client.open(adapter: "stub") do |client|
  client.decide("Does this need review?", ["Please check my order."]) do |row|
    puts row.status
    break
  end

  rows = client.decide("Does this need review?", ["Please check my order."])
  begin
    p rows.next.answer
  ensure
    rows.close
  end
end
```

Function blocks and `rows.each` use an outer `ensure` to close on exhaustion, break or a raised exception. External enumeration through `next` or `peek` suspends an Enumerator fiber. Its suspension does not promise an ensure has run. Use the explicit `begin`/`ensure` form when abandoning such an iterator. Explicit close invalidates the shared call owner before any suspended iterator can resume a native operation. Completion closes automatically. Garbage collection is only a fallback, never the documented cleanup mechanism.

One Ruby thread drives a call. Independent calls may use the same client concurrently. `Rows#cancel` may run on another Ruby thread during a blocked pull. The wrapper pins the call while an operation is active, and never holds its ownership mutex across a blocking native call. `close` cancels first, then waits only for an active host operation to leave C before freeing its pointer. Cancel cannot race with free. Thread interruption is deferred across the native ownership transition and cleanup, so an asynchronous Ruby exception cannot discard a returned row or error buffer. It is delivered after those owned buffers are copied or freed. A blocking pull runs on a host worker. The driver waits in Ruby so `Interrupt` can cancel the native call, wait for that host operation, and release its buffers before propagating unchanged.

```ruby
rows = client.decide("Does this need review?", records)
worker = Thread.new do
  begin
    rows.each { |row| p row.answer }
  rescue ThinkThen::Error => error
    warn error.code
  end
end
# A controller can do this while worker is blocked waiting for the backend:
rows.cancel
worker.join
rows.close
```

The cancellation test waits for a loopback backend's request-arrived event before another Ruby thread calls cancel, then checks disconnect and the engine cancellation result. A pre-cancelled call cannot satisfy it.

`Client.open(**settings)` yields a client and closes it in `ensure`. `Client.new(**settings)` pairs with `close`; `closed?` reports ownership. Client close prevents new calls and releases its native client owner. Existing calls retain their engine. Each call must still be closed. The native loader stays rooted for the process lifetime because native workers can finish after call cleanup.

## Native dependency and packaging

The package uses the `ffi` gem directly against the shipped `thinkthen_ffi` library. Its Struct layout handles pointer, size_t and row alignment; its counted memory access preserves payload bytes. Blocking engine entries, including `thinkthen_next`, use `blocking: true`. Ruby then releases its global lock and another Ruby thread can call the existing C cancellation entry. Fast ownership operations need no host worker thread. No Ruby callback enters the engine.

The platform gem contains Ruby source and its matching optimized engine. Its version follows the repository package version. It depends on `ffi ~> 1.17` and uses Ruby's JSON library. The runtime floor is CRuby 3.3. Platform and operating-system requirements follow the release package and the shared [runtime floor contract](../../scripts/runtime-floors.json).

The supported installation path includes the matching precompiled ffi gem. It needs Ruby, RubyGems and the ordinary platform runtime libraries, but no Rust toolchain, C compiler, standalone ThinkThen executable or separately installed engine. If a caller chooses ffi's source gem, ffi itself requires a C compiler and Ruby development headers. An offline installation prefetches the platform ThinkThen gem and the matching ffi gem and installs them with `gem install --local`; no installation hook fetches assets. The package loader resolves only its own asset path and keeps the handle alive. It exposes no library-path override or downloader.

The existing C header remains the source for the small hand-written ABI declarations. Outside-in proof uses installed platform gems in clean offline jobs, the shared conformance driver, length-sensitive JSON cases, lazy detail, error codes, early break and the held-request Ruby-thread cancellation case. The dedicated gate builds no engine asset and installs the gem with a pre-fetched ffi dependency before running these checks.

## Design sources

- [C ABI and ownership](../../crates/thinkthen-ffi/README.md), [public header](../../crates/thinkthen-ffi/include/thinkthen.h), and the linked shared schemas define engine behavior.
- [ffi blocking calls](https://github.com/ffi/ffi/wiki/Callbacks#gil) document releasing the Ruby lock for Ruby-to-native calls.
- [ffi library declarations](https://github.com/ffi/ffi/blob/master/lib/ffi/library.rb) define attachment options and native types. [ffi installation](https://github.com/ffi/ffi#installation) describes source build prerequisites.
- [Ruby Enumerator](https://docs.ruby-lang.org/en/3.4/Enumerator.html#class-Enumerator-label-External+Iteration) describes the separate fiber used for external iteration.
- [Ruby Fiddle::Function](https://docs.ruby-lang.org/en/3.4/Fiddle/Function.html) exposes the `need_gvl` option.
- [RubyGems platforms](https://guides.rubygems.org/platforms/) describes platform gem selection.
