# frozen_string_literal: true

# These cases prove the Ruby adapter's serialization, ownership and iteration.
def ruby_surface
  retained = nil
  ThinkThen::Client.open(adapter: "stub") do |client|
    equal([], client.decide("Ready?", []).to_a, "empty input")
    seen = []
    client.decide("Ready?", ["first", "second"]) do |value|
      seen << value
      break
    end
    equal(1, seen.length, "function block can stop early")
    external = client.decide("Ready?", ["first"])
    external.next
    raises(StopIteration) { external.next }
    assert(external.closed?, "external exhaustion releases call")
    id = {"a\u0000b" => 18_446_744_073_709_551_615}
    105.times { id = [id] }
    row = client.decide("Ready?", [{"id" => id, "body" => "é😀evidence\u0000"}], on: "/body").first
    equal(id, row.id, "full JSON identity beyond Ruby's default nesting limit")
    assert(row.frozen?, "row is immutable")
    assert(!row.respond_to?(:answer=), "row has no writers")
    assert(row.detail.is_a?(Hash), "detail decodes as JSON")
    row.detail.clear
    assert(!row.detail.empty?, "detail remains independently readable")
    retained = row

    stream = client.decide("Ready?", ["first", "second"])
    stream.each { break }
    assert(stream.closed?, "early each break closes")
    stream.close
    stream.cancel
    raises(IOError) { stream.rewind }
    raises(IOError) { stream.next }

    stream = client.decide("Ready?", ["first", "second"])
    begin
      assert(stream.peek.equal?(stream.next), "Enumerator peek retains its row")
    ensure
      stream.close
    end
    raises(IOError) { stream.next }

    conversion = Object.new
    def conversion.to_enum(*) = raise(TypeError, "cannot enumerate")
    raises(TypeError) { client.decide("Ready?", conversion) }

    reentrant = nil
    source = Enumerator.new { reentrant.next }
    reentrant = client.decide("Ready?", source)
    raises(ThreadError) { reentrant.to_a }
    assert(reentrant.closed?, "reentrant driver closes its call")

    original = RuntimeError.new("source failed")
    source = Enumerator.new do |yielder|
      yielder << "first"
      raise original
    end
    stream = client.decide("Ready?", source)
    equal(original, raises(RuntimeError) { stream.to_a }, "source exception preserved")
    assert(stream.closed?, "source exception closes")

    stream = client.decide("Ready?", ["first"])
    equal(original, raises(RuntimeError) { stream.each { raise original } }, "block exception preserved")
    assert(stream.closed?, "block exception closes")

    usage = raises(ThinkThen::Error) { client.decide("Ready?", [], threshold: "bad") }
    equal("usage.threshold", usage.code, "engine admission error")
    assert(!usage.message.empty?, "engine message preserved")
    equal("usage.batch", raises(ThinkThen::Error) { ThinkThen::Client.new(adapter: "stub", batch: 0) }.code,
          "client settings admission")
    equal("usage.reference", raises(ThinkThen::Error) { client.decide(ThinkThen.reference("missing"), []) }.code,
          "reference question reaches engine")

    toolbox = {"questions" => {"ready" => {"decide" => "Ready?"}}}
    wrapped = client.decide(ThinkThen.toolbox(toolbox, name: "ready"), ["first"]).first
    equal(client.decide("Ready?", ["first"]).first.to_h, wrapped.to_h, "toolbox wrapper")
  end
  assert(retained.detail.is_a?(Hash), "row outlives call and client")

  client = ThinkThen::Client.new(adapter: "stub")
  stream = client.decide("Ready?", ["first"])
  client.close
  client.close
  assert(client.closed?, "client closes idempotently")
  assert(!stream.to_a.empty?, "opened call survives client close")
  raises(IOError) { client.decide("Ready?", []) }

  Backend.open("--unavailable") do |backend|
    ThinkThen::Client.open(url: backend.url) do |remote|
      rows = []
      failure = raises(ThinkThen::Error) do
        remote.decide("Ready?", ["first"]).each { |row| rows << row }
      end
      equal("backend.transport", failure.code, "unreachable backend terminal code")
      assert(!rows.empty?, "unreachable backend returns stream failure")
      equal("failed", rows.first.status, "unreachable backend status")
      equal("backend.transport", rows.first.error.fetch("code"), "unreachable backend code")
      equal("0", backend.command("count"), "unavailable server counts no request")
    end
  end
  puts "installed Ruby gem: JSON fidelity, lazy detail, errors and resource ownership pass"
end

# The controller cannot cancel until the real loopback has received the request.
def held_cancellation
  %w[headers body refusal-body].each do |reply|
    Backend.open("--held", reply) do |backend|
      ThinkThen::Client.open(url: backend.url) do |client|
        cancel = ThinkThen::Cancel.new
        stream = client.decide("Ready?", ["evidence"], cancel: cancel)
        rows = []
        worker = Thread.new do
          begin
            stream.each { |row| rows << row }
            raise "cancelled call completed without its terminal error"
          rescue ThinkThen::Error => error
            equal("local.cancelled", error.code, "held terminal code")
          end
        end
        equal("arrived", backend.command("arrived"), "request arrived before Ruby cancellation")
        cancel.cancel
        worker.value
        closers = 2.times.map { Thread.new { stream.close } }
        closers.each(&:value)
        equal("closed", backend.command("closed"), "cancel aborts held socket")
        equal("1", backend.command("count"), "cancel sends no extra request")
        assert(stream.closed?, "cancelled driver releases call")
        rows.each do |row|
          equal("failed", row.status, "only a cancellation failure may precede the terminal error")
          equal("local.cancelled", row.error.fetch("code"), "cancellation row")
        end
      end
    end
  end
  puts "installed Ruby gem: Ruby threads cancel requests blocked at headers and bodies"
end

# Explicit close also cancels a foreign wait and coordinates concurrent closers.
def held_close
  Backend.open("--held", "headers") do |backend|
    ThinkThen::Client.open(url: backend.url) do |client|
      stream = client.decide("Ready?", ["evidence"])
      worker = Thread.new do
        begin
          row = stream.next
          equal("local.cancelled", row.error.fetch("code"), "close's retained cancellation row")
        rescue ThinkThen::Error => error
          equal("local.cancelled", error.code, "close's terminal code")
        end
      end
      equal("arrived", backend.command("arrived"), "request arrived before close")
      closers = 2.times.map { Thread.new { stream.close } }
      closers.each(&:value)
      worker.value
      equal("closed", backend.command("closed"), "close aborts held socket")
      assert(stream.closed?, "concurrent closers release the call")
    end
  end
  puts "installed Ruby gem: concurrent close waits for active host operations"
end


def held_interrupt
  Backend.open("--held", "headers") do |backend|
    ThinkThen::Client.open(url: backend.url) do |client|
      stream = client.decide("Ready?", ["evidence"])
      interrupted = Queue.new
      worker = Thread.new do
        begin
          stream.each { |_row| }
        rescue Interrupt
          interrupted << true
        end
      end
      equal("arrived", backend.command("arrived"), "request arrived before interrupt")
      worker.raise(Interrupt)
      begin
        require "timeout"
        Timeout.timeout(10) { interrupted.pop }
      ensure
        stream.cancel
        worker.join
      end
      assert(stream.closed?, "interrupt releases call")
      equal("closed", backend.command("closed"), "interrupt closes held connection")
    end
  end
end
