# frozen_string_literal: true
require "minitest/autorun"
require_relative "backend"

class TestOwnedSession < Minitest::Test
  def test_authored_decide_values_match_the_installed_return_signature
    require "rbs"
    require "rbs/test"
    loader = RBS::EnvironmentLoader.new
    loader.add(path: Pathname.new(File.expand_path("../sig", TestBackend::LIB)))
    environment = RBS::Environment.from_loader(loader).resolve_type_names
    builder = RBS::DefinitionBuilder.new(env: environment)
    client = builder.build_instance(RBS::TypeName.parse("::ThinkThen::Client"))
    value_type = client.methods.fetch(:decide).method_types.first.type.return_type.args.first
    checker = RBS::Test::TypeCheck.new(self_class: Object, builder: builder,
      sample_size: 100, unchecked_classes: [])
    TestBackend.with(<<~RUBY) do |backend, child|
      T::Client.open(cache: false) do |client|
        meanings = ["approved", {status: "approved"}, ["approved", {code: 7}]]
        answers = meanings.flat_map do |meaning|
          ["text", ["text"]].map do |input|
            result = client.decide({decide: "Approve?", true: meaning}, input)
            [result.value, result.probability]
          end
        end
        say answers
      end
    RUBY
      answers = child.hear
      assert_equal [["approved", 0.9], [["approved"], [0.9]],
        [{"status" => "approved"}, 0.9], [[{"status" => "approved"}], [0.9]],
        [["approved", {"code" => 7}], 0.9], [[["approved", {"code" => 7}]], [0.9]]], answers
      answers.each { |value, _| assert checker.value(value, value_type), "authored value rejected by installed RBS" }
      refute checker.value(7, value_type), "decide signature accepts an arbitrary scalar"
      status, errors = child.finish
      assert status.success?, errors
      assert_equal 6, backend.count
    end
  end

  def test_producer_exception_and_native_admission_message_survive
    script = <<~RUBY
      marker = RuntimeError.new("private producer marker")
      input = Enumerator.new { raise marker }
      client = T::Client.new(cache: false)
      begin
        client.decide("Is it?", input)
        raise "producer answered"
      rescue T::LocalError => error
        raise "cause lost" unless error.cause.equal?(marker)
        raise "native message changed" unless error.message == "session producer raised an exception"
        raise "producer sent" unless error.facts.requests_sent == 0
      end
      begin
        client.decide({decide: "Is it?", item_schema: {type: "array"}}, "unreachable")
        raise "unsupported declaration admitted"
      rescue T::UsageError => error
        raise "native admission changed" unless error.message == "the question declaration uses an unsupported feature"
      end
      %i[decide plan].each do |method|
        begin
          method == :plan ? client.plan("decide", "Is it?", "unreachable", details: true) : client.decide("Is it?", "unreachable", details: true)
          raise "details admitted"
        rescue T::UsageError
        end
      end
      begin
        client.start("unknown", "Is it?", "unreachable")
        raise "unknown function admitted"
      rescue T::UsageError
      end
      client.close
      say true
    RUBY
    TestBackend.with(script) do |backend, child|
      assert_equal true, child.hear
      assert_equal 0, backend.count
    end
  end

  def test_named_calls_return_owned_typed_results_after_block_cleanup
    script = <<~RUBY
      retained = T::Client.open(cache: false) do |client|
        asks = {
          "decide" => [{decide: "Is it urgent?"}, "text"],
          "choose" => [T::Client.choose_question("Which team?", options: {billing: "Money", shipping: "Delivery"}), "text"],
          "tag" => [{tag: "Which labels?", labels: %w[money shipping]}, "text"],
          "score" => [{score: "How urgent?", levels: %w[low mid high]}, "text"],
          "filter" => [{decide: "Is it urgent?"}, ["first", {note: "second"}]],
          "rank" => ["Is it urgent?", ["first", "second"]],
          "find" => ["Which line?", %w[first second]],
          "annotate" => [{version: 1, questions: {ok: {decide: "Is it urgent?"}}}, ["text"]],
          "recognize" => [T::Client.recognize_question(kinds: {person: nil}), "Ana Lima"],
          "relate" => [T::Client.relate_question(relations: [T::Client.relation("knows", source: "person", target: "person", either: false)]), [{name: "Ana", kind: "person"}, {name: "Bob", kind: "person"}]]
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
      say retained[5].last.value
      say retained[6].last.value
      say retained[9].last.value.map(&:relation)
      say retained.map { |_, result| result.probability }
      raise "recognition span probability" unless retained[8].last.probability == retained[8].last.results.first.answer.proposals.select(&:kept).map(&:span_probability)
    RUBY
    TestBackend.with(script) do |backend, child|
      results = child.hear
      assert_equal %w[decide choose tag score filter rank find annotate recognize relate], results.map(&:first)
      assert results.all? { |_, rows, sends| rows.positive? && sends.positive? }
      assert_equal true, child.hear
      assert_equal "billing", child.hear
      assert_equal 0.15, child.hear
      assert_equal [0, 1], child.hear
      assert_equal "first", child.hear
      assert_equal %w[knows knows], child.hear
      probabilities = child.hear
      assert_equal 0.9, probabilities[0]
      assert_equal 0.9, probabilities[1]
      assert_nil probabilities[2]
      assert_nil probabilities[3]
      assert_equal [0.9, 0.9], probabilities[4]
      assert_equal [0.9, 0.9], probabilities[5]
      assert_nil probabilities[7]
      assert_equal 2, probabilities[9].length
      status, errors = child.finish
      assert status.success?, errors
      assert_operator backend.count, :>=, 10
    end
  end

  def test_one_record_collections_keep_probability_arrays
    TestBackend.with(<<~RUBY) do |backend, child|
      T::Client.open(cache: false) do |client|
        path = File.join(ENV.fetch("HOME"), "records.jsonl")
        File.write(path, JSON.generate(note: "first") + 10.chr)
        inputs = ["first", ["first"], ["first"].each, T::Client.files([path], framing: "jsonl")]
        say inputs.map { |input| result = client.decide("Is it urgent?", input); [result.value, result.probability] }
      end
    RUBY
      assert_equal [[true, 0.9], [[true], [0.9]], [[true], [0.9]], [[true], [0.9]]], child.hear
      status, errors = child.finish
      assert status.success?, errors
      assert_equal 4, backend.count
    end
  end

  def test_rank_keeps_a_plain_position_list_for_every_input_shape
    TestBackend.with(<<~RUBY) do |backend, child|
      T::Client.open(cache: false) do |client|
        answers = ["first", %w[first second], []].map { |input| client.rank("Which record?", input) }
        say answers.map(&:value)
        say answers.map { |answer| answer.value.is_a?(Array) && answer.value.frozen? }
        say answers.map { |answer| answer.facts.requests_sent }
      end
    RUBY
      assert_equal [[0], [0, 1], []], child.hear
      assert_equal [true, true, true], child.hear
      assert_equal [1, 1, 0], child.hear
      status, errors = child.finish
      assert status.success?, errors
      assert_equal 2, backend.count
    end
  end

  def test_rank_returns_original_indexes_for_reordered_ties_and_duplicate_values
    TestBackend.with(<<~RUBY, arm: "case/15-rank-records") do |backend, child|
      originals = [{body: "item 0", id: 81}, {body: "item 1", id: 37},
        {body: "item 2", id: 92}, {body: "item 1", id: 37}]
      retained = T::Client.open(cache: false) do |client|
        client.rank("How relevant is this?", originals, field: ["/body"], batch: 1)
      end
      say retained.value
      say retained.results.map(&:input)
      say retained.results.map(&:value)
      say [retained.value.frozen?, retained.value.all? { |index| index.is_a?(Integer) }]
      say retained.facts.requests_sent
    RUBY
      assert_equal [1, 3, 2, 0], child.hear
      assert_equal [{"body" => "item 1", "id" => 37}, {"body" => "item 1", "id" => 37},
        {"body" => "item 2", "id" => 92}, {"body" => "item 0", "id" => 81}], child.hear
      assert_equal [1, 2, 3, 4], child.hear
      assert_equal [true, true], child.hear
      assert_equal 3, child.hear
      status, errors = child.finish
      assert status.success?, errors
      assert_equal 3, backend.count
    end
  end

  def test_filter_retains_original_values_in_its_plain_answer
    TestBackend.with(<<~RUBY) do |backend, child|
      retained = T::Client.open(cache: false) do |client|
        client.filter("Keep this?", [{body: "duplicate", id: 81}, {body: "duplicate", id: 81}, "other"])
      end
      say retained.value
      say retained.value.frozen?
    RUBY
      assert_equal [{"body" => "duplicate", "id" => 81}, {"body" => "duplicate", "id" => 81}, "other"], child.hear
      assert_equal true, child.hear
      status, errors = child.finish
      assert status.success?, errors
      assert_equal 1, backend.count
    end
  end

  def test_find_keeps_the_selected_original_hash_and_wide_integer
    TestBackend.with(<<~RUBY) do |backend, child|
      retained = T::Client.open(cache: false) do |client|
        client.find("Which record?", [T::Client.item({body: "first", extra: 18446744073709551615}), {body: "second"}], field: ["/body"])
      end
      say retained.value
      say [retained.value.is_a?(Hash), retained.value.frozen?, retained.value["extra"].is_a?(Integer)]
      say retained.facts.requests_sent
    RUBY
      assert_equal({"body" => "first", "extra" => 18446744073709551615}, child.hear)
      assert_equal [true, true, true], child.hear
      assert_equal 1, child.hear
      status, errors = child.finish
      assert status.success?, errors
      assert_equal 1, backend.count
    end
  end

  def test_input_helpers_hide_native_envelopes_and_constructors
    TestBackend.with(<<~RUBY) do |backend, child|
      helpers = [[T::Client.item({body: "private marker"}, context: {tag: "private context"}), :descriptor],
        [T::Client.question_file("private path"), :selector],
        [T::Client.files(["private path"]), :source]]
      helpers.each do |helper, accessor|
        [accessor, :to_h, :[], :[]=].each do |method|
          raise "public envelope \#{method}" if helper.respond_to?(method)
          begin
            helper.public_send(method, *([:[], :[]=].include?(method) ? [0] : []))
            raise "public envelope invoked"
          rescue NoMethodError => error
            raise "wrong missing method" unless error.name == method
          end
        end
        %i[new allocate].each do |method|
          raise "public constructor" if helper.class.respond_to?(method)
          begin
            helper.class.public_send(method, * (method == :new ? [{original: {kind: "text", text: "private marker"}}] : []))
            raise "public constructor invoked"
          rescue NoMethodError => error
            raise "wrong missing constructor" unless error.name == method
          end
        end
        raise "inspect leak" if helper.inspect.include?("private")
        raise "mutable carrier" unless helper.frozen?
      end
      say true
    RUBY
      assert_equal true, child.hear
      status, errors = child.finish
      assert status.success?, errors
      assert_equal 0, backend.count
    end
  end

  def test_preview_and_retired_names_use_the_single_native_family
    TestBackend.with(<<~RUBY) do |backend, child|
      T::Client.open(cache: false) do |client|
        preview = client.plan("decide", "Question?", ["one", "two"], batch: 1)
        raise "untyped preview" unless preview.is_a?(T::Plan) && T::Plan::FIELDS.length == 9
        %i[dump original descriptor cleanup].each do |method|
          raise "public helper" if T::Client.respond_to?(method)
          begin
            T::Client.public_send(method, "caller input")
            raise "public helper invoked"
          rescue NoMethodError => error
            raise "wrong missing method" unless error.name == method
          end
        end
        say [preview.records, preview.requests]
        say client.plan("decide", "Question?", {body: "one"}, batch: 1).records
        say [T.const_defined?(:Engine, false), T.const_defined?(:Complete, false), T.respond_to?(:decide)]
        say client.inspect
        questions = {decide: "Question?", filter: "Question?", rank: "Question?",
          choose: T::Client.choose_question("Question?", options: %w[first second]),
          tag: {tag: "Question?", labels: %w[first second]}, score: {score: "Question?", levels: %w[low high]}}
        questions.each do |verb, question|
          plan = client.plan(verb.to_s, question, ["one", "two"], batch: 1)
          raise "missing preview field" unless T::Plan::FIELDS.all? { |field| plan.key?(field) }
          raise "wrong preview" unless plan.records == 2 && plan.requests == (verb == :tag ? 4 : 2)
        end
        unsupported = {find: "Question?", annotate: {version: 1, questions: {ok: {decide: "Question?"}}},
          recognize: T::Client.recognize_question(kinds: {person: nil}),
          relate: T::Client.relate_question(relations: [T::Client.relation("knows")])}
        unsupported.each do |verb, question|
          begin
            client.plan(verb.to_s, question, ["one", "two"])
            raise "unsupported preview admitted"
          rescue T::UsageError
          end
        end
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
        say selectors.map { |selector| [selector.is_a?(T::Client::Question), selector.respond_to?(:selector)] }
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
      assert_equal [[true, false], [true, false], [true, false]], child.hear
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
        authored_null = client.decide({decide: "Question?", true: nil}, T::Client.item("text"))
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
      [["Question?", "text", {field: ["bad pointer"]}], ["\\xFF".force_encoding("UTF-8"), "text", {}], ["Question?", T::Client.item("text", context: 1), {}]].each_with_index do |(question, input, options), index|
        begin
          client.decide(question, input, **options)
          raise "invalid input accepted: " + index.to_s
        rescue T::UsageError => error
          errors << [error.kind, error.message, error.facts]
        end
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
      assert_equal [["usage", "invalid field pointer", nil], ["usage", "input cannot be converted to native JSON", nil], ["usage", "the per-item context does not match context_schema", nil], ["cancelled", nil]], child.hear
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
