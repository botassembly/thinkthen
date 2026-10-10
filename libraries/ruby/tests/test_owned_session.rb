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
    RUBY
    TestBackend.with(script) do |backend, child|
      results = child.hear
      assert_equal %w[decide choose tag score filter rank find annotate recognize relate], results.map(&:first)
      assert results.all? { |_, rows, sends| rows.positive? && sends.positive? }
      assert_equal true, child.hear
      assert_equal "billing", child.hear
      assert_equal 0.15, child.hear
      status, errors = child.finish
      assert status.success?, errors
      assert_operator backend.count, :>=, 10
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
        [missing, authored_null, unresolved, located]
      end
      missing, authored_null, unresolved, located = retained
      say [missing.value, authored_null.value, unresolved.value]
      say [missing.results.first.question.key?("true"), authored_null.results.first.question.key?("true"), authored_null.results.first.question["true"]]
      say located.results.map { |row| [row.index, row.source.first_line, row.source.last_line, row.input] }
      say located.facts.records
    RUBY
      assert_equal [false, nil, nil], child.hear
      assert_equal [false, true, nil], child.hear
      assert_equal [[0, 1, 1, "first"], [1, 3, 3, "third"]], child.hear
      assert_equal 2, child.hear
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

  def test_thread_progress_and_cancel_cleanup_precede_held_provider_release
    TestBackend.with(<<~RUBY, arm: "arm/held") do |backend, child|
      token = T::Cancel.new
      client = T::Client.new(cache: false)
      ticks = 0
      running = true
      ticker = Thread.new { while running; ticks += 1; sleep 0.001; end }
      Thread.new do
        hear
        before = ticks
        sleep 0.01
        say ["progress", ticks > before]
        hear
        token.cancel
      end
      begin
        client.decide("Question?", ["first", "second"].lazy, cancel: token, batch: 1)
      rescue T::CancelledError => error
        client.close
        say [error.kind, ticks.positive?, error.facts]
      ensure
        running = false
        ticker.join
      end
    RUBY
      assert_equal 1, backend.wait(1)
      child.tell
      assert_equal ["progress", true], child.hear
      child.tell
      assert_equal ["cancelled", true, nil], child.hear
      status, errors = child.finish
      assert status.success?, errors
      assert_equal 1, backend.count
      backend.release
    end
  end
end
