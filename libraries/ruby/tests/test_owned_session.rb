# frozen_string_literal: true
require "minitest/autorun"
require_relative "backend"

class TestOwnedSession < Minitest::Test
  def test_named_calls_return_owned_typed_results_after_block_cleanup
    script = <<~RUBY
      retained = T::Client.open(cache: false) do |client|
        asks = {
          "decide" => [{decide: "Is it urgent?"}, "text"],
          "choose" => [{choose: "Which team?", options: %w[billing shipping]}, "text"],
          "tag" => [{tag: "Which labels?", labels: %w[money shipping]}, "text"],
          "score" => [{score: "How urgent?", levels: %w[low mid high]}, "text"],
          "filter" => [{decide: "Is it urgent?"}, ["first", {note: "second"}]],
          "rank" => ["Is it urgent?", ["first", "second"]],
          "find" => ["Which line?", %w[first second]],
          "annotate" => [{version: 1, questions: {ok: {decide: "Is it urgent?"}}}, ["text"]],
          "recognize" => [{version: 1, recognize: {kinds: {person: nil}}}, "Ana Lima"],
          "relate" => [{version: 1, relate: {relations: [{name: "knows", source: "person", target: "person", either: false}]}}, [{name: "Ana", kind: "person"}, {name: "Bob", kind: "person"}]]
        }
        asks.map { |verb, (question, input)| [verb, client.public_send(verb, question, input)] }
      end
      retained.each do |verb, result|
        raise "untyped facts" unless result.facts.is_a?(T::Results::NativeFacts)
        raise "missing call ID" unless result.facts.call_id.is_a?(String)
        result.results.each do |row|
          raise "untyped row" unless row.class.name.start_with?("ThinkThen::Results::Native")
          raise "identity" unless row.answer_id.is_a?(String)
          raise "lost roundtrip" unless row.to_h.fetch("answer_id") == row.answer_id
          raise "text leak" if row.inspect.include?("text")
        end
      end
      say retained.map { |verb, result| [verb, result.results.length, result.facts.requests_sent] }
      say retained.first.last.value
      say retained[1].last.value
      say retained[3].last.value
      say retained[6].last.value
      say retained[9].last.value.map(&:relation)
    RUBY
    TestBackend.with(script) do |backend, child|
      results = child.hear
      assert_equal %w[decide choose tag score filter rank find annotate recognize relate], results.map(&:first)
      assert results.all? { |_, rows, sends| rows.positive? && sends.positive? }
      assert_equal true, child.hear
      assert_equal "billing", child.hear
      assert_equal 0.15, child.hear
      assert_equal "first", child.hear
      assert_equal %w[knows knows], child.hear
      status, errors = child.finish
      assert status.success?, errors
      assert_operator backend.count, :>=, 10
    end
  end

  def test_preview_and_retired_names_use_the_single_native_family
    TestBackend.with(<<~RUBY) do |backend, child|
      T::Client.open(cache: false) do |client|
        preview = client.plan("decide", "Question?", ["one", "two"], batch: 1)
        say [preview.fetch("records"), preview.fetch("requests")]
        say client.plan("decide", "Question?", {body: "one"}, batch: 1).fetch("records")
        say [T.const_defined?(:Engine, false), T.const_defined?(:Complete, false), T.respond_to?(:decide)]
        say client.inspect
      end
    RUBY
      assert_equal [2, 2], child.hear
      assert_equal 1, child.hear
      assert_equal [false, false, false], child.hear
      assert_equal "<ThinkThen::Client>", child.hear
      status, errors = child.finish
      assert status.success?, errors
      assert_equal 0, backend.count
    end
  end

  def test_a_child_after_fork_owns_its_engine_and_parent_counters_hold
    lines, count = TestBackend.run(<<~RUBY)
      require "timeout"
      client = T::Client.new(cache: false)
      client.decide("Is it urgent?", "before the fork")
      before = client.usage
      reader, writer = IO.pipe
      pid = Process.fork do
        reader.close
        T::Client.open(cache: false) do |owned|
          value = owned.decide("Is it urgent?", "in the forked child").value
          writer.write(JSON.generate([value, owned.usage[:requests_sent]]))
        end
        writer.close
        exit!(0)
      end
      writer.close
      begin
        forked = Timeout.timeout(5) { reader.read }
      rescue Timeout::Error
        Process.kill("KILL", pid)
        raise
      ensure
        Process.wait(pid)
      end
      say [forked, client.usage == before]
      client.close
    RUBY
    assert_equal [["[true,1]", true]], lines
    assert_equal 2, count
  end

  def test_saved_question_selectors_keep_native_lookup_and_literal_strings
    TestBackend.with(<<~RUBY) do |backend, child|
      require "fileutils"
      directory = File.join(ENV.fetch("XDG_CONFIG_HOME"), "thinkthen", "questions")
      FileUtils.mkdir_p(directory)
      File.write(File.join(directory, "refund.json"), JSON.generate(name: "refund", decide: "Saved wording?"))
      Dir.chdir(ENV.fetch("HOME")) do
        path = "@literal.json"
        File.write(path, JSON.generate(decide: "File wording?"))
        File.write("refund", JSON.generate(decide: "Local wording?"))
        selectors = [T::Client.question_file(path), T::Client.question_name("refund"), T::Client.question_reference("@refund")]
        say selectors.map { |selector| JSON.parse(T::Client.dump(selector.selector)) }
        raise "selector leak" unless selectors.all? { |selector| selector.inspect == "<ThinkThen::Client::Question: content withheld>" }
        T::Client.open(cache: false) do |client|
          results = selectors.map { |selector| client.decide(selector, "text") }
          File.unlink("refund")
          results << client.decide(T::Client.question_reference("@refund"), "text")
          results << client.decide("@refund", "text")
          say results.map { |result| result.results.first.question.text }
          say results.map { |result| result.facts.requests_sent }
        end
      end
    RUBY
      assert_equal [{"kind" => "file", "path" => "@literal.json"}, {"kind" => "name", "name" => "refund"}, {"kind" => "reference", "reference" => "@refund"}], child.hear
      assert_equal ["File wording?", "Saved wording?", "Local wording?", "Saved wording?", "@refund"], child.hear
      assert_equal [1, 1, 1, 1, 1], child.hear
      status, errors = child.finish
      assert status.success?, errors
      assert_equal 5, backend.count
    end
  end

  def test_saved_question_refusals_send_nothing
    TestBackend.with(<<~RUBY) do |backend, child|
      T::Client.open(cache: false) do |client|
        errors = [T::Client.question_name("../invalid"), T::Client.question_reference("refund"), T::Client.question_name("missing"), T::Client.question_reference("@missing")].map do |selector|
          begin
            client.decide(selector, "text")
            raise "selector accepted"
          rescue T::Error => error
            [error.kind, error.facts]
          end
        end
        say errors
        say client.usage[:requests_sent]
      end
    RUBY
      assert_equal [["usage", nil], ["usage", nil], ["local", nil], ["local", nil]], child.hear
      assert_equal 0, child.hear
      status, errors = child.finish
      assert status.success?, errors
      assert_equal 0, backend.count
    end
  end

  def test_false_null_and_physical_positions_survive_native_ownership
    TestBackend.with(<<~RUBY) do |backend, child|
      retained = T::Client.open(cache: false) do |client|
        missing = client.decide({decide: "Question?", threshold: 0.95}, "text")
        authored_null = client.decide({decide: "Question?", true: nil}, "text")
        unresolved = client.decide({decide: "Question?", threshold: "0.2:0.95"}, "text")
        path = File.join(ENV.fetch("HOME"), "input.txt")
        File.write(path, "first\\n\\nthird\\n")
        located = client.decide("Question?", T::Client.files([path], unit: "line"))
        File.write(path, JSON.generate(body: "framed text") + "\\n")
        framed = client.decide("Question?", T::Client.files([path], framing: "jsonl"), field: ["/body"])
        selector = T::Client.question_file(path)
        raise "path leak" if selector.inspect.include?(path)
        empty_edges = client.relate({version: 1, relate: {relations: [{name: "knows", source: "person", target: "person", either: false}]}, threshold: 0.95}, [{name: "Ana", kind: "person"}, {name: "Bob", kind: "person"}])
        [missing, authored_null, unresolved, located, framed, empty_edges]
      end
      missing, authored_null, unresolved, located, framed, empty_edges = retained
      say [missing.value, authored_null.value, unresolved.value]
      say [missing.results.first.question.key?("true"), authored_null.results.first.question.key?("true"), authored_null.results.first.question["true"]]
      say located.results.map { |row| [row.index, row.source.first_line, row.source.last_line, row.input] }
      say located.facts.records
      say [framed.results.first.input.to_h, framed.results.first.value]
      say empty_edges.value
    RUBY
      assert_equal [false, nil, nil], child.hear
      assert_equal [false, true, nil], child.hear
      assert_equal [[0, 1, 1, "first"], [1, 3, 3, "third"]], child.hear
      assert_equal 2, child.hear
      assert_equal [{"body" => "framed text"}, true], child.hear
      assert_equal [], child.hear
      status, errors = child.finish
      assert status.success?, errors
      assert_operator backend.count, :>=, 3
    end
  end

  def test_native_failure_retains_typed_settlement_and_has_no_value
    TestBackend.with(<<~RUBY) do |backend, child|
      client = T::Client.new(cache: false)
      begin
        client.decide("Question?", "text", deadline_ms: 0)
        raise "failure became a value"
      rescue T::DeadlineError => error
        client.close
        say [error.kind, error.complete.class.name, error.complete.key?("value"), error.terminal.kind, error.results, error.facts.requests_sent]
      end
    RUBY
      result = child.hear
      assert_equal "deadline", result[0]
      assert result[1].start_with?("ThinkThen::Results::NativeCallError")
      assert_equal [false, "terminal", [], 0], result.drop(2)
      status, errors = child.finish
      assert status.success?, errors
      assert_equal 0, backend.count
    end
  end

  def test_invalid_input_and_cancelled_token_send_nothing
    TestBackend.with(<<~RUBY) do |backend, child|
      client = T::Client.new(cache: false)
      errors = []
      begin
        client.decide("Question?", "text", field: ["bad pointer"])
      rescue T::Error => error
        errors << [error.kind, error.facts]
      end
      token = T::Cancel.new
      token.cancel
      begin
        client.decide("Question?", "text", cancel: token)
      rescue T::Error => error
        errors << [error.kind, error.facts]
      end
      client.close
      say errors
    RUBY
      assert_equal [["usage", nil], ["cancelled", nil]], child.hear
      status, errors = child.finish
      assert status.success?, errors
      assert_equal 0, backend.count
    end
  end

  def test_fiber_progress_and_cancel_cleanup_precede_held_provider_release
    TestBackend.with(<<~RUBY, arm: "arm/held") do |backend, child|
      token = T::Cancel.new
      client = T::Client.new(cache: false)
      ticks = 0
      running = true
      scheduler = Class.new do
        def initialize = (@waiting = [])
        def fiber(&block) = Fiber.new(blocking: false, &block).tap(&:resume)
        def kernel_sleep(duration = 0)
          @waiting << [Process.clock_gettime(Process::CLOCK_MONOTONIC) + duration, Fiber.current]
          Fiber.yield
          duration
        end
        def block(_blocker, timeout = nil) = kernel_sleep(timeout || 0)
        def unblock(_blocker, fiber) = (@waiting << [0, fiber])
        def io_wait(*) = raise("unexpected scheduler IO")
        def close
          until @waiting.empty?
            ready, @waiting = @waiting.partition { |at, _| at <= Process.clock_gettime(Process::CLOCK_MONOTONIC) }
            ready.each { |_, fiber| fiber.resume if fiber.alive? }
            sleep 0.001 unless @waiting.empty?
          end
        end
      end.new
      producer = ["first", "second"].lazy
      closed = false
      producer.define_singleton_method(:close) { closed = true }
      Thread.new do
        hear
        before = ticks
        sleep 0.01
        say ["progress", ticks > before]
        hear
        token.cancel
      end
      Fiber.set_scheduler(scheduler)
      Fiber.schedule { while running; ticks += 1; sleep 0.001; end }
      Fiber.schedule do
        begin
          client.decide("Question?", producer, cancel: token, batch: 1)
        rescue T::CancelledError => error
          client.close
          say [error.kind, ticks.positive?, error.facts, closed]
        ensure
          running = false
        end
      end
      Fiber.set_scheduler(nil)
    RUBY
      assert_equal 1, backend.wait(1)
      child.tell
      assert_equal ["progress", true], child.hear
      child.tell
      assert_equal ["cancelled", true, nil, true], child.hear
      status, errors = child.finish
      assert status.success?, errors
      assert_equal 1, backend.count
      backend.release
    end
  end
end
