# frozen_string_literal: true

# Feature: Ruby keyword arguments and typed questions retain required domains.
def ruby_questions
  ThinkThen::Client.open(backend: :stub) do |client|
    %i[choose tag score rank].each { |name| raises(ArgumentError) { client.public_send(name, "Which?", "record") } }
    %i[recognize relate].each { |name| raises(ArgumentError) { client.public_send(name, "record") } }
    choices = ["billing".dup, "shipping".dup]
    question = ThinkThen::ChooseQuestion.new("Which team?", choices: choices)
    toolbox = ThinkThen::Toolbox.new(questions: {"topic" => question})
    choices.first.replace("changed")
    choices << "extra"
    equal(client.choose("Which team?", "refund", choices: %w[billing shipping]).to_h,
          client.choose(toolbox["topic"], "refund").to_h, "question snapshots its required domain")
    assert(question.frozen? && toolbox.frozen?, "typed questions are immutable")
    raises(KeyError) { toolbox["absent"] }
    raises(ArgumentError) { client.choose(question, "refund", choices: ["changed", "extra"]) }
    annotation = client.annotate(ThinkThen::AnnotationQuestions.new(questions: {"topic" => toolbox["topic"]}), "refund")
    assert(annotation.answered?, "toolbox annotation returns an outer answered row")
    assert(annotation.answer.fetch("topic").is_a?(Data), "annotation entry is a Data value")
    assert(!ThinkThen.respond_to?(:reference), "reference questions are removed")
  end
end

# Feature: Ruby Range bands and evidence paths translate without changing meaning.
def ruby_threshold_and_evidence
  ThinkThen::Client.open(backend: :stub) do |client|
    record = {"a/b~c" => [{"body" => "é😀evidence"}]}
    equal(client.decide("Ready?", record, threshold: 0.2..0.8, evidence: "/a~1b~0c/0/body").to_h,
          client.decide("Ready?", record, threshold: 0.2..0.8, evidence: ["a/b~c", 0, :body]).to_h,
          "path components escape to the same JSON pointer")
    equal(client.decide("Ready?", record, evidence: "").to_h,
          client.decide("Ready?", record, evidence: []).to_h, "empty path selects root")
    [0.2...0.8, (..0.8), (0.2..), "0.2:0.8", {low: 0.2, high: 0.8}].each do |threshold|
      begin
        client.decide("Ready?", "record", threshold: threshold)
      rescue TypeError, ArgumentError
        next
      end
      raise "invalid host threshold shape was accepted"
    end
    error = raises(ThinkThen::UsageError) { client.decide("Ready?", "record", threshold: 0.0) }
    equal(:"usage.threshold", error.code, "numeric threshold uses engine admission")
    assert(!error.message.empty?, "engine message survives translation")
    raises(ArgumentError) { client.score("Scale?", "record", levels: %w[low high], threshold: 0.5) }
    raises(ArgumentError) { client.rank("Scale?", "record", levels: %w[low high], threshold: 0.5) }
  end
end

# Feature: Ruby identity preserves signed/unsigned limits and rejects other IDs as rows.
def ruby_record_identity
  Backend.open("--unavailable") do |backend|
    ThinkThen::Client.open(backend: :systemone, url: backend.url, api_key: "synthetic-loopback-key") do |client|
      invalid = [nil, true, false, 1.5, [], {}, -(2**63) - 1, 2**64]
      records = invalid.map { |id| {"id" => id, "body" => "record"} }
      rows = client.decide_each("Ready?", [nil, *records]).to_a
      equal(records.length + 1, rows.length, "every invalid or missing record survives")
      equal((1..rows.length).to_a, rows.map(&:position), "failed records consume positions")
      equal([:"record.missing", *Array.new(invalid.length, :"record.invalid")], rows.map { |row| row.error.code }, "typed admission codes")
      assert(rows.all? { |row| row.is_a?(ThinkThen::Failed) }, "record errors are failed variants")
      equal("0", backend.command("count"), "invalid identities make no backend call")
    end
  end
  ThinkThen::Client.open(backend: :stub) do |client|
    ids = ["001", -(2**63), 2**63 - 1, 2**64 - 1]
    rows = client.decide_each("Ready?", ids.map { |id| {"id" => id, "body" => "record"} }).to_a
    equal(ids, rows.map(&:id), "identity value and type survive")
    assert(rows.all?(&:id_present?), "valid identity is present")
    absent = client.decide("Ready?", {"body" => "record"})
    assert(!absent.id_present? && !absent.to_h.key?("id"), "missing id is omitted")
  end
end

# Feature: filters retain failed records and the original Ruby item.
def ruby_filter_rows
  ThinkThen::Client.open(backend: :stub) do |client|
    items = [{"id" => "first", "body" => "refund"}, nil, {"id" => false, "body" => "invalid"}]
    rows = client.filter_each("Ready?", items).to_a
    equal(items.length, rows.length, "filter never drops a failure")
    rows.zip(items).each do |value, item|
      assert(value.is_a?(ThinkThen::FilterRow) && value.is_a?(Data), "typed filter wrapper")
      assert(value.item.equal?(item), "original item identity")
      equal(value.row.answered? && value.row.answer == true, value.kept?, "kept projection")
    end
    assert(!items.first.frozen?, "binding does not freeze caller input")
    equal([:"record.missing", :"record.invalid"], rows.drop(1).map { |value| value.row.error.code }, "filter failures stay visible")
    assert(client.filter("Ready?", items.first).is_a?(ThinkThen::FilterRow), "single filter has same shape")
  end
end

# Feature: Ruby rows remain usable after client cleanup and freeze nested values.
def ruby_row_values
  retained = nil
  ThinkThen::Client.open(backend: :stub) do |client|
    retained = client.choose("Which team?", "record", choices: %w[billing shipping])
    case retained
    in ThinkThen::Answered[answer:, probability:]
      assert(answer.is_a?(String), "Data pattern exposes typed answer")
      assert(probability.nil? || probability.is_a?(Float), "Data pattern exposes Float probability")
    else
      raise "stub choice must answer"
    end
    raises(FrozenError) { retained.answer.replace("changed") }
    assert(retained.detail.is_a?(ThinkThen::LabelDetail), "detail is a typed value")
    raises(FrozenError) { retained.detail.labels.clear }
    raises(FrozenError) { retained.source.model.replace("changed") }
  end
  assert(retained.answer.is_a?(String), "answer outlives client")
  assert(retained.detail.labels.frozen?, "detail outlives native row")
end

# Feature: lazy Ruby streams clean up at traversal and scope boundaries.
def ruby_enumeration
  scoped = nil
  ThinkThen::Client.open(backend: :stub) do |client|
    pulled = 0
    source = Enumerator.new { |out| 100.times { pulled += 1; out << "record" } }
    stream = client.decide_each("Ready?", source)
    assert(stream.is_a?(Enumerator), "Rows follows Ruby Enumerator")
    equal(0, pulled, "constructing Rows does not consume input")
    equal(1, stream.lazy.first(1).length, "lazy bounded traversal yields requested rows")
    assert(stream.closed?, "lazy first closes traversal")
    assert(pulled < 100, "bounded traversal does not materialize input")
    stream.close
    stream.close
    raises(IOError) { stream.rewind }

    stream = client.decide_each("Ready?", %w[first second])
    stream.each { break }
    assert(stream.closed?, "each break closes")
    external = client.decide_each("Ready?", ["first"])
    external.next
    raises(StopIteration) { external.next }
    assert(external.closed?, "external exhaustion closes")

    original = RuntimeError.new("caller block failed")
    stream = client.decide_each("Ready?", ["first"])
    equal(original, raises(RuntimeError) { stream.each { raise original } }, "caller exception preserved")
    assert(stream.closed?, "caller exception closes call")

    source = Enumerator.new { |out| out << "first"; raise original }
    stream = client.decide_each("Ready?", source)
    equal(original, raises(RuntimeError) { stream.to_a }, "input exception preserved")
    assert(stream.closed?, "input exception closes call")
    scoped = client.decide_each("Ready?", %w[first second])
    scoped.next
  end
  assert(scoped.closed?, "Client.open closes a suspended external enumerator")
end

# Feature: caller-owned IO remains open after the stream closes.
def ruby_input_ownership
  require "stringio"
  input = StringIO.new("first\nsecond\n")
  ThinkThen::Client.open(backend: :stub) do |client|
    client.decide_each("Ready?", input.each_line) { break }
    assert(!input.closed?, "Ruby caller retains IO ownership")
  end
ensure
  input&.close
end

# Feature: cancellation raises Cancelled and releases a held backend request.
def ruby_held_calls
  [:token, :interrupt, :close, :client_close].each do |action|
    Backend.open("--held", "headers") do |backend|
      ThinkThen::Client.open(backend: :systemone, url: backend.url, api_key: "synthetic-loopback-key") do |client|
        token = ThinkThen::CancelToken.new
        stream = client.decide_each("Ready?", ["record"], cancel: token)
        worker = Thread.new do
          begin
            stream.to_a
            raise "cancelled traversal completed normally" unless [:close, :client_close].include?(action)
          rescue ThinkThen::Cancelled => error
            equal(:"local.cancelled", error.code, "typed cancellation")
            assert(error.cause.is_a?(Interrupt), "Interrupt becomes cancellation cause") if action == :interrupt
          end
        end
        worker.report_on_exception = false
        equal("arrived", backend.command("arrived"), "request event precedes cancellation")
        raises(ThreadError) { stream.each { |_row| } }
        case action
        when :token
          token.cancel
          token.cancel
          assert(token.cancelled?, "token state persists")
        when :interrupt then worker.raise(Interrupt)
        when :close
          closers = 2.times.map { Thread.new { stream.close } }
          closers.each(&:value)
        when :client_close then client.close
        end
        worker.value
        stream.close
        assert(stream.closed?, "held call releases native owner")
        equal("closed", backend.command("closed"), "held socket closes")
      ensure
        stream&.cancel
        worker&.join
      end
    end
  end
  ThinkThen::Client.open(backend: :stub) do |client|
    token = ThinkThen::CancelToken.new
    token.cancel
    raises(ThinkThen::Cancelled) { client.decide("Ready?", "record", cancel: token) }
  end
end

# Feature: streaming rows reuse one native reader without per-row thread allocation.
def ruby_reader_lifetime
  ThinkThen::Client.open(backend: :stub) do |client|
    created = 0
    trace = TracePoint.new(:thread_begin) { created += 1 }
    begin
      rows = trace.enable(target_thread: nil) { client.decide_each("Ready?", Array.new(128, "record")).to_a }
      equal(128, rows.length, "reader returns every row")
      equal((1..128).to_a, rows.map(&:position), "reader preserves positions")
      assert(rows.all?(&:answered?), "reader preserves answers")
      equal(1, created, "one reader serves the whole stream")
    ensure
      trace.disable
    end
  end
end

# Feature: interruption at native completion still has the cancellation type.
def ruby_completion_interruption
  ThinkThen::Client.open(backend: :stub) do |client|
    stream = client.decide_each("Ready?", ["record"])
    interrupted = false
    trace = TracePoint.new(:return) do |event|
      if !interrupted && event.method_id == :value && event.defined_class.name == "ThinkThen::NativeRead"
        interrupted = true
        raise Interrupt
      end
    end
    begin
      error = raises(ThinkThen::Cancelled) { trace.enable { stream.to_a } }
      assert(interrupted, "interruption follows completed native worker")
      equal(:"local.cancelled", error.code, "completed worker cancellation code")
      assert(error.cause.is_a?(Interrupt), "completion interruption remains the cause")
      assert(stream.closed?, "completion interruption releases the call")
    ensure
      trace.disable
      stream.close
    end
  end
end

# Feature: repeated interruptions keep row storage until the native read stops.
def ruby_repeated_interruption
  ThinkThen::Client.open(backend: :stub) do |client|
    native = ThinkThen.send(:native)
    read_class = ThinkThen.const_get(:NativeRead)
    original_new = read_class.method(:new)
    original_next = native.method(:thinkthen_call_next)
    original_free = native.method(:thinkthen_row_free)
    waiting, cleanup, resume, release = 4.times.map { Queue.new }
    completed = freed_early = false
    read_class.define_singleton_method(:new) do |buffer|
      read = original_new.call(buffer)
      ready = read.instance_variable_get(:@ready)
      attempts = 0
      ready.define_singleton_method(:pop) do
        attempts += 1
        if attempts == 1
          waiting << true
        elsif attempts == 2
          cleanup << true
          resume.pop
          release << true
        end
        super()
      end
      read
    end
    native.singleton_class.remove_method(:thinkthen_call_next)
    native.define_singleton_method(:thinkthen_call_next) do |pointer, row, error|
      release.pop
      result = freed_early ? 1 : original_next.call(pointer, row, error)
      completed = true
      result
    end
    native.singleton_class.remove_method(:thinkthen_row_free)
    native.define_singleton_method(:thinkthen_row_free) do |row|
      freed_early ||= !completed
      release << true if freed_early
      original_free.call(row)
    end
    worker = Thread.new do
      begin
        client.decide_each("Ready?", ["record"]).to_a
      rescue Exception
        # Several asynchronous interrupts can remain pending after cancellation.
      end
    end
    waiting.pop
    Thread.pass until worker.status == "sleep"
    worker.raise(Interrupt)
    cleanup.pop
    Thread.pass until worker.status == "sleep"
    worker.raise(Interrupt)
    worker.raise(Interrupt)
    resume << true
    worker.join
    assert(completed, "native read stops")
    assert(!freed_early, "native completion precedes row storage release")
  ensure
    resume << true if resume
    release << true if release
    worker&.join
    read_class&.singleton_class&.remove_method(:new)
    if original_next
      native.singleton_class.remove_method(:thinkthen_call_next)
      native.define_singleton_method(:thinkthen_call_next, original_next)
    end
    if original_free
      native.singleton_class.remove_method(:thinkthen_row_free)
      native.define_singleton_method(:thinkthen_row_free, original_free)
    end
  end
end

# Feature: configuration freezes when the shared client opens and secrets stay redacted.
def ruby_configuration
  require "pp"
  secret = "synthetic-key-for-redaction-only"
  retained = nil
  ThinkThen.configure do |config|
    config.backend = :stub
    config.api_key = secret
    config.batch = 1
    config.retry = ThinkThen::Retry.new(max_retries: 0)
    retained = config
  end
  shared = 4.times.map { Thread.new { ThinkThen.client } }.map(&:value)
  assert(shared.all? { |client| client.equal?(shared.first) }, "shared client creation is synchronized")
  assert(ThinkThen.decide("Ready?", "record").is_a?(Data), "module function delegates")
  retained.backend = :openai
  equal(:stub, ThinkThen.decide("Ready?", "record").source.backend, "configuration publishes a snapshot")
  begin
    ThinkThen.configure { |config| config.backend = :openai }
  rescue RuntimeError, IOError, ThinkThen::Error
    rejected = true
  end
  assert(rejected, "cannot configure after shared-client creation")
  [retained, shared.first, retained.api_key].each do |value|
    [value.inspect, value.to_s, PP.pp(value, +"")].each do |display|
      assert(!display.include?(secret), "display never exposes key")
    end
  end
  error = raises(ThinkThen::UsageError) do
    ThinkThen::Client.new(backend: :openai, api_key: secret, url: "http://example.com")
  end
  assert(!error.message.include?(secret) && !error.inspect.include?(secret), "setup errors redact keys")
  ThinkThen.close
  assert(shared.first.closed?, "module close closes shared client")
  raises(IOError) { ThinkThen.decide("Ready?", "record") }
ensure
  ThinkThen.close
end

# Feature: Ruby scheduler tasks can cancel a native pull without an Interrupt.
def ruby_scheduler_cancellation
  require "async"
  [:token, :interrupt].each do |action|
    Backend.open("--held", "headers") do |backend|
      ThinkThen::Client.open(backend: :systemone, url: backend.url, api_key: "synthetic-loopback-key") do |client|
        token = ThinkThen::CancelToken.new
        stream = client.decide_each("Ready?", ["record"], cancel: token)
        Async do |task|
          fiber = nil
          reader = task.async do
            fiber = Fiber.current
            error = raises(ThinkThen::Cancelled) { stream.to_a }
            equal(:"local.cancelled", error.code, "scheduler cancellation code")
            assert(error.cause.is_a?(Interrupt), "scheduler interrupt remains the cancellation cause") if action == :interrupt
          end
          task.async do
            equal("arrived", backend.command("arrived"), "scheduler observes request event")
            action == :token ? token.cancel : Fiber.scheduler.raise(fiber, Interrupt.new)
          end.wait
          reader.wait
        end.wait
        assert(stream.closed?, "scheduler call releases ownership")
      ensure
        stream&.close
      end
    end
  end
end
