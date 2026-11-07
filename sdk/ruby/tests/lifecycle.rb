# frozen_string_literal: true

# Feature: abandoned Ruby streams release native calls before their client closes.
def ruby_abandoned_streams
  native = ThinkThen.send(:native)
  live = {}
  originals = %i[thinkthen_call_open_json thinkthen_call_open_records_json thinkthen_call_free].to_h do |name|
    [name, native.method(name)]
  end
  originals.each do |name, original|
    native.singleton_class.remove_method(name)
    native.define_singleton_method(name) do |*args|
      result = original.call(*args)
      if name == :thinkthen_call_free
        live.delete(args.first.address)
      elsif result == 0
        live[args[-2].read_pointer.address] = true
      end
      result
    end
  end
  ThinkThen::Client.open(backend: :stub) do |client|
    before = Thread.list
    token = ThinkThen::CancelToken.new
    100.times do |index|
      stream = client.decide_each("Ready?", %w[first second], cancel: token)
      stream.next if index.even?
    end
    10.times do
      GC.start
      Thread.pass
      break if live.empty?
    end
    equal(0, live.length, "GC frees all native calls while the client stays open")
    (Thread.list - before).each(&:join)
    active = client.decide_each("Ready?", ["record"], cancel: token)
    GC.start
    token.cancel
    raises(ThinkThen::CancelledError) { active.to_a }
    assert(active.closed?, "retained live stream remains cancellable after GC")
    assert(token.cancelled?, "retained token remains usable after stream collection")
    assert(!client.closed?, "GC does not close the caller's client")
    assert(client.decide("Ready?", "record").answered?, "client remains usable after stream collection")
  end
ensure
  originals&.each do |name, original|
    native.singleton_class.remove_method(name)
    native.define_singleton_method(name, original)
  end
end

# Feature: killing a reading thread cancels its native call and ends its reader.
def ruby_killed_readers
  {each: proc { |stream| stream.to_a }, next: proc { |stream| stream.next }}.each do |name, read|
    Backend.open("--held", "headers") do |backend|
      ThinkThen::Client.open(backend: :systemone, url: backend.url, api_key: "synthetic-loopback-key") do |client|
        before = Thread.list
        stream = client.decide_each("Ready?", ["record"])
        worker = Thread.new { read.call(stream) }
        equal("arrived", backend.command("arrived"), "#{name} read reaches the backend")
        worker.kill
        assert(worker.join(10), "#{name} read outlives Thread#kill")
        equal("closed", backend.command("closed"), "#{name} kill cancels the native call")
        assert(stream.closed?, "#{name} kill closes the stream")
        equal([], Thread.list - before, "#{name} kill ends the reader thread")
      end
    end
  end
end

# Feature: close returns while a reader waits on caller input, and that reader cleans up later.
def ruby_close_during_input
  [:stream, :client].each do |closing|
    ThinkThen::Client.open(backend: :stub) do |client|
      before = Thread.list
      waiting, release = Queue.new, Queue.new
      records = Enumerator.new do |output|
        waiting << true
        output << release.pop
      end
      stream = client.decide_each("Ready?", records)
      worker = Thread.new { raises(ThinkThen::UsageError) { stream.to_a } }
      begin
        waiting.pop
        closer = Thread.new { closing == :stream ? stream.close : client.close }
        assert(closer.join(10), "#{closing} close waits for caller input")
        assert(stream.closed?, "#{closing} close releases the native call")
      ensure
        release << "late"
        worker.join
      end
      equal(:"usage.closed", worker.value.code, "late input finds the call closed")
      equal([], Thread.list - before, "the reader leaves no thread behind")
    end
  end
end

# Feature: a killed reader helper answers its driver with an error, never a hang or a short stream.
def ruby_killed_helper
  reader = ThinkThen.const_get(:NativeReader)
  original = reader.method(:worker)
  ThinkThen::Client.open(backend: :stub) do |client|
    before = Thread.list
    stream = client.decide_each("Ready?", %w[first second])
    stream.next
    (Thread.list - before).each { |helper| helper.kill.join }
    raises(ThreadError) { stream.next }

    held, gate, queues = Queue.new, Queue.new, []
    reader.define_singleton_method(:worker) do |requests, reference|
      first = true
      requests.define_singleton_method(:pop) do |*arguments|
        if first
          first = false
          held << true
          gate.pop
        end
        super(*arguments)
      end
      queues << requests
      original.call(requests, reference)
    end
    before = Thread.list
    stream = client.decide_each("Ready?", ["record"])
    driver = Thread.new { raises(ThreadError) { stream.next } }
    held.pop
    Thread.pass until queues.first.size == 1
    (Thread.list - before - [driver]).each(&:kill)
    assert(driver.join(10), "a queued read outlives its killed helper")
    driver.value
  end
ensure
  reader&.define_singleton_method(:worker, original) if original
end

# Feature: each Ruby call opens one native call and never opens a temporary client.
def ruby_single_admission
  native = ThinkThen.send(:native)
  names = %i[thinkthen_client_open thinkthen_client_open_json thinkthen_call_open_json thinkthen_call_open_records_json]
  ThinkThen::Client.open(backend: :stub) do |client|
    originals = names.to_h { |name| [name, native.method(name)] }
    counts = Hash.new(0)
    originals.each do |name, original|
      native.singleton_class.remove_method(name)
      native.define_singleton_method(name) { |*args| counts[name] += 1; original.call(*args) }
    end
    assert(client.decide("Ready?", "record").answered?, "single call returns its row")
    equal(2, client.decide_each("Ready?", %w[first second].each).to_a.length, "lazy input returns each row")
    equal(0, counts[:thinkthen_client_open] + counts[:thinkthen_client_open_json], "calls reuse the admitted client")
    equal(1, counts[:thinkthen_call_open_records_json], "finite input is admitted once")
    equal(1, counts[:thinkthen_call_open_json], "lazy input is admitted once")
  ensure
    originals.each do |name, original|
      native.singleton_class.remove_method(name)
      native.define_singleton_method(name, original)
    end
  end
end

# Feature: forked children refuse inherited clients and can create their own.
def ruby_fork_ownership
  return unless Process.respond_to?(:fork)
  ThinkThen::Client.open(backend: :stub) do |client|
    stream = client.decide_each("Ready?", ["record"])
    input, output = IO.pipe
    pid = fork do
      input.close
      begin
        [proc { client.decide("Ready?", "record") }, proc { client.close }, proc { stream.next }, proc { stream.close }].each do |operation|
          error = raises(IOError, &operation)
          assert(error.message.include?("create a client after fork"), "fork error explains recovery")
        end
        ThinkThen::Client.open(backend: :stub) { |child| assert(child.decide("Ready?", "record").answered?, "child owns its new client") }
        output.write("ok")
      rescue Exception => error
        output.write(error.full_message)
      ensure
        output.close
        exit! 0
      end
    end
    output.close
    equal("ok", input.read, "child refuses parent ownership")
    Process.wait(pid)
    assert(stream.next.answered?, "parent keeps its call")
    stream.close
  ensure
    input&.close
    output&.close unless output&.closed?
  end
end
