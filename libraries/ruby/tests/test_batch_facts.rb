# frozen_string_literal: true

# A captured default request and final account through the actual Ruby host.
require "digest"
require "minitest/autorun"
require "socket"
require_relative "backend"

class TestBatchFacts < Minitest::Test
  def test_account_starts_at_rust_accounting_not_worker_creation
    lines, count = TestBackend.run(<<~RUBY)
      begin
        T::Engine.new(max_requests: 1).rank("Question?", %w[one two])
      rescue T::Error => error
        say [error.kind, error.facts, error.details]
      end
      begin
        T.decide("Question?", "text", deadline: 0)
      rescue T::Error => error
        say [error.kind, error.facts, error.details]
      end
    RUBY
    assert_equal ["usage", nil, nil], lines.first
    kind, facts, details = lines.last
    assert_equal "deadline", kind
    assert_equal [0, 0], facts.values_at("records", "requests_sent")
    assert_equal [], details
    assert_equal 0, count
  end

  def test_runtime_labels_module_delegates_and_keyword_boundaries
    lines, count = TestBackend.run(<<~RUBY)
      options = { billing: "For billing problems", shipping: "For deliveries" }
      calls = [T.choose_many("Which team?", ["one", "two"], options: options, batch: 2, context: "shared reference"),
               T.score_many("How urgent?", ["one", "two"], levels: %w[low high], batch: 2),
               T.tag_many("Which labels?", ["one", "two"], labels: { money: "Payment", shipping: "Delivery" }, batch: 2)]
      say calls.map { |call| [call.value, call.facts[:records], call.facts[:requests_sent], call.details.map { |row| row[:index] }] }
      scalar = [->(control) { T.decide("Question?", "text", **control) },
                ->(control) { T.choose("Which?", "text", options: %w[a b], **control) },
                ->(control) { T.score("How?", "text", levels: %w[a b], **control) },
                ->(control) { T.score_with_level("How?", "text", levels: %w[a b], **control) },
                ->(control) { T.tag("Which?", "text", labels: %w[a b], **control) },
                ->(control) { T.details("Question?", "text", **control) },
                ->(control) { T.find("Question?", %w[one two], **control) },
                ->(control) { T.recognize("text", kinds: %w[person], **control) },
                ->(control) { T.relate([%w[one person], %w[two person]], relations: %w[knows], **control) }]
      scalar.each do |call|
        [{ batch: 1 }, { context: "reference" }].each do |control|
          begin
            call.call(control)
          rescue T::UsageError => error
            say [error.kind, error.facts]
          end
        end
      end
      begin
        T.annotate(T.set(a: { decide: "Question?" }), %w[one], context: "reference")
      rescue T::UsageError => error
        say [error.kind, error.facts]
      end
      begin
        T.decide_many("Question?", %w[one two], context: "\\xff".b)
      rescue T::UsageError => error
        say [error.kind, error.message, error.facts]
      end
    RUBY
    assert_equal [[%w[billing billing], 2, 1, [0, 1]], [[0.1, 0.1], 2, 1, [0, 1]],
                  [[%w[money shipping], %w[money shipping]], 2, 1, [0, 1]]], lines.first
    assert_equal Array.new(19) { ["usage", nil] }, lines[1, 19]
    assert_equal ["usage", "context is not valid UTF-8", nil], lines.last
    assert_equal 3, count
  end

  def test_conversion_error_retains_completed_account
    lines, count = TestBackend.run(<<~RUBY)
      begin
        T.rank("Is it urgent?", %w[one two], top: "invalid")
      rescue TypeError => error
        say [error.facts[:records], error.facts[:requests_sent], error.details.map { |row| row[:index] },
             error.facts.frozen?, error.details.frozen?]
      end
    RUBY
    assert_equal [[2, 1, [0, 1], true, true]], lines
    assert_equal 1, count
  end

  def test_filtered_and_top_dropped_rows_keep_the_whole_account_and_original_values
    lines, count = TestBackend.run(<<~RUBY)
      records = ["one", { id: 2 }]
      kept = T.filter("Is it urgent?", records, batch: 2)
      dropped = T.filter(T.question(decide: "Is it urgent?", threshold: 0.95), records, batch: 2)
      ranked = T.rank("Is it urgent?", records, top: 1, batch: 2)
      say [kept.value.last.equal?(records.last), kept.facts[:records], kept.details.map { |row| row[:index] },
           dropped.value, dropped.facts[:records], dropped.details.map { |row| row[:index] },
           ranked.value.size, ranked.facts[:records], ranked.details.map { |row| row[:index] }]
    RUBY
    assert_equal [[true, 2, [0, 1], [], 2, [0, 1], 1, 2, [0, 1]]], lines
    assert_equal 1, count
  end

  def test_context_changes_request_identity_without_changing_question_identity
    lines, count = TestBackend.run(<<~RUBY)
      plain = T.decide_many("Is it urgent?", %w[one two], batch: 2)
      shared = T.decide_many("Is it urgent?", %w[one two], batch: 2, context: "reference text")
      say [plain.details.first[:question_sha256] == shared.details.first[:question_sha256],
           plain.details.first[:requests] != shared.details.first[:requests],
           plain.facts[:requests_sent], shared.facts[:requests_sent]]
    RUBY
    assert_equal [[true, true, 1, 1]], lines
    assert_equal 2, count
  end

  def test_prompt_stop_receipt_waits_for_final_worker_account
    script = <<~RUBY
      token = T::Cancel.new
      engine = T::Engine.new(throttle: 2, batch: 2)
      records = (1..20).map { |index| "record \#{index}" }
      Thread.new { hear; token.cancel }
      begin
        engine.decide_many("Is it urgent?", records, cancel: token)
      rescue T::CancelledError => error
        pending = begin
          error.completion.result(timeout: 0)
        rescue Timeout::Error
          "pending"
        end
        say [error.kind, error.facts, error.details, error.completion.done?, pending]
        hear
        say error.completion.result(timeout: 2)
      end
    RUBY
    TestBackend.with(script, arm: "arm/held") do |backend, child|
      assert_equal 2, backend.wait(2)
      child.tell
      kind, facts, details, done, pending = child.hear
      assert_equal ["cancelled", nil, nil, false, "pending"], [kind, facts, details, done, pending]
      assert_equal 2, backend.count
      backend.release
      child.tell
      final = child.hear
      assert_equal "failed", final.fetch("outcome")
      assert_equal "cancelled", final.fetch("kind")
      assert_equal 2, final.fetch("facts").fetch("requests_sent")
      assert_equal 4, final.fetch("facts").fetch("records")
      assert_equal [0, 1, 2, 3], final.fetch("details").map { |row| row.fetch("index") }
      assert_equal [1, 0, 1, 0], final.fetch("details").map { |row| row.fetch("requests_sent") }
      status, errors = child.finish
      assert status.success?, errors
      assert_equal 2, backend.count
    end
  end

  def test_default_batch_packs_and_reuses_repeated_text_with_one_account
    listener = TCPServer.new("127.0.0.1", 0)
    port = listener.addr[1]
    received = Queue.new
    serving = Thread.new do
      2.times do
        socket = listener.accept
        headers = []
        headers << socket.gets until headers.last == "\r\n"
        length = Integer(headers.grep(/\Acontent-length:/i).first.split(":", 2).last)
        body = socket.read(length)
        received << [headers.first.strip, body]
        questions = JSON.parse(body).fetch("questions")
        answers = questions.to_h do |id, question|
          answer = if question.fetch("type") == "choice"
                     { type: "choice", probabilities: { billing: 0.9, shipping: 0.1 } }
                   else
                     { type: "noul", noul: 0.9 }
                   end
          [id, answer]
        end
        response = JSON.generate(model: "jev-1.13.0", answers: answers, usage: { input_tokens: 6, output_tokens: 3 })
        socket.write("HTTP/1.1 200 OK\r\nContent-Type: application/json\r\nContent-Length: #{response.bytesize}\r\nConnection: close\r\n\r\n#{response}")
        socket.close
      end
    end
    Dir.mktmpdir("thinkthen-ruby-packed-") do |root|
      url = "http://127.0.0.1:#{port}/generic/v1"
      script = <<~RUBY
        result = T.decide_many("Is it urgent?", ["alpha", "beta", "alpha"])
        say [result.value, result.facts, result.details, result.inspect,
             result.facts.frozen? && result.details.frozen? && result.details.all?(&:frozen?) &&
             result.details.all? { |row| row[:requests].frozen? }]
        options = { billing: { route: { desk: "Refunds", channels: ["mail", "phone"] } }, shipping: "Delivery" }
        chosen = T.choose_many("Which team?", ["first", "second"], options: options)
        say [chosen.value, chosen.facts, chosen.details]
      RUBY
      out, errors, status = Open3.capture3(TestBackend.env(url, root), RbConfig.ruby, "-I", TestBackend::LIB,
                                           "-e", TestBackend::PRELUDE + script, unsetenv_others: true)
      assert status.success?, errors
      serving.value
      line, body = received.pop
      choice_line, choice_body = received.pop
      assert_equal "POST /generic/v1/systemone HTTP/1.1", line
      expected = %q({"state":"Each question quotes the text it asks about.","model":"jev-1.13.0","questions":{"q1":{"type":"noul","instructions":"The text is \"alpha\". Is it urgent?"},"q2":{"type":"noul","instructions":"The text is \"beta\". Is it urgent?"}}})
      assert_equal expected, body
      values, facts, details, inspection, frozen = JSON.parse(out.lines.first)
      assert_equal [true, true, true], values
      assert_equal [3, 1, 0, 6, 3], facts.values_at("records", "requests_sent", "cache_answers", "input_tokens", "output_tokens")
      assert_equal [0, 1, 2], details.map { |row| row.fetch("index") }
      assert_equal 1, details.map { |row| row.fetch("requests").first }.uniq.size
      digest = Digest::SHA256.hexdigest("systemone\n#{url}/systemone\n#{body}")
      assert_equal [digest], details.first.fetch("requests")
      assert_equal inspection.include?("alpha"), false
      assert_equal inspection.include?("beta"), false
      assert_equal 2, JSON.parse(body).fetch("questions").size
      assert frozen
      assert_equal "POST /generic/v1/systemone HTTP/1.1", choice_line
      expected_choice = %q({"state":{"records":["first","second"]},"model":"jev-1.13.0","questions":{"q1":{"type":"choice","instructions":"The text is \"first\". Which team?","criteria":{"billing":{"route":{"desk":"Refunds","channels":["mail","phone"]}},"shipping":"Delivery"}},"q2":{"type":"choice","instructions":"The text is \"second\". Which team?","criteria":{"billing":{"route":{"desk":"Refunds","channels":["mail","phone"]}},"shipping":"Delivery"}}}})
      assert_equal expected_choice, choice_body
      chosen, chosen_facts, chosen_details = JSON.parse(out.lines.last)
      assert_equal %w[billing billing], chosen
      assert_equal [2, 1], chosen_facts.values_at("records", "requests_sent")
      assert_equal [0, 1], chosen_details.map { |row| row.fetch("index") }
      choice_digest = Digest::SHA256.hexdigest("systemone\n#{url}/systemone\n#{expected_choice}")
      assert_equal [[choice_digest], [choice_digest]], chosen_details.map { |row| row.fetch("requests") }
    end
  ensure
    listener&.close
    serving&.kill
  end
end
