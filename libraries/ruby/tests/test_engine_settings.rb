# frozen_string_literal: true

# ThinkThen::Engine carries the ADR 0017 section 5 settings over the
# environment's (decisions 14 and 15). Each test runs in its own child with
# its own backend and cache folder.
require "minitest/autorun"
require_relative "backend"

class TestEngineSettings < Minitest::Test
  def entries(folder)
    Dir.glob("**/*", base: folder).reject { |name| File.directory?(File.join(folder, name)) }
  end

  # The environment seeds the engine. THINKTHEN_CACHE names the child's
  # cache folder, and a repeat answers from it with no second send.
  def test_the_environment_seeds_the_cache_folder
    TestBackend.with(<<~RUBY) do |backend, child, root|
      engine = T::Engine.new(throttle: 8, base_url: ENV.fetch("THINKTHEN_BASE_URL"))
      2.times { engine.decide("Is it urgent?", "the same text") }
      say engine.usage
    RUBY
      assert_equal({ "requests_sent" => 1, "retries" => 0, "cache_answers" => 1, "input_tokens" => 0, "output_tokens" => 0 }, child.hear)
      status, errors = child.finish
      assert status.success?, errors
      assert_equal 1, backend.count
      refute_empty entries(File.join(root, "cache")), "the answer is not in THINKTHEN_CACHE"
      assert_empty entries(File.join(root, "home")) + entries(File.join(root, "xdg-cache"))
    end
  end

  def test_the_throttle_holds_exactly_that_many_requests_in_flight
    TestBackend.with(<<~RUBY, arm: "arm/held") do |backend, child|
      say T::Engine.new(throttle: 8, batch: 2).decide_many("Is it urgent?", (1..20).map { |n| "record \#{n}" }).value.size
    RUBY
      assert_equal 8, backend.wait(8)
      sleep 0.3
      assert_equal 8, backend.count
      backend.release
      assert_equal 20, child.hear
      status, errors = child.finish
      assert status.success?, errors
    end
  end

  def test_bad_settings_refuse_before_any_request
    lines, count = TestBackend.run(<<~RUBY)
      [{ throttle: 33 }, { throttle: 0 }, { throttle: true }, { throttle: 1.5 }, { max_requests: "2" },
       { max_request_bytes: 0 }, { timeout: 0 }, { timeout: "30" }, { replay: 7 }, { cache: 3 }, { base_url: 7 }].each do |settings|
        T::Engine.new(**settings)
        say "built"
      rescue T::UsageError => e
        say e.message
      end
    RUBY
    assert_equal ["a throttle is a whole number from 1 through 32", "a throttle is a whole number from 1 through 32",
                  "throttle is a whole number or nil", "throttle is a whole number or nil",
                  "max_requests is a whole number or nil", "max_request_bytes is a whole number of at least 1", "a timeout is a time above zero",
                  "timeout is a whole number or nil", "replay is text or nil",
                  "cache is a folder path, false for none, or nil for the default", "base_url is text or nil"], lines
    assert_equal 0, count
  end

  # The token cap reaches Ruby through from_env; a regression if from_env stops reading the variable.
  def test_the_token_cap_variable_refuses_before_any_request
    TestBackend.with(<<~RUBY, extra: { "THINKTHEN_MAX_ESTIMATED_INPUT_TOKENS_TOTAL" => 10 }) do |backend, child|
      begin
        T::Engine.new(cache: false).decide("Is it urgent?", "text")
      rescue T::Error => e
        say [e.class.name, e.kind, e.message]
      end
    RUBY
      assert_equal ["ThinkThen::UsageError", "usage",
                    "max_estimated_input_tokens_total=10 (encoded-body-bytes-908-v1) would be exceeded before this call's first request"],
                   child.hear
      status, errors = child.finish
      assert status.success?, errors
      assert_equal 0, backend.count
    end
  end

  def test_cache_names_its_folder
    TestBackend.with(<<~RUBY) do |backend, child, root|
      named = File.join(ENV.fetch("HOME"), "named")
      T::Engine.new(cache: named).decide("Is it urgent?", "into the named folder")
      say named
    RUBY
      named = child.hear
      status, errors = child.finish
      assert status.success?, errors
      assert_equal 1, backend.count
      refute_empty entries(named)
      assert_empty entries(File.join(root, "cache")), "a named cache wrote the environment's folder"
    end
  end

  def test_the_base_url_setting_wins_over_the_environment
    other = TestBackend::Backend.new
    TestBackend.with(<<~RUBY) do |backend, child|
      T::Engine.new(base_url: "#{other.url}").decide("Is it urgent?", "text")
    RUBY
      status, errors = child.finish
      assert status.success?, errors
      assert_equal [0, 1], [backend.count, other.count]
    end
  ensure
    other&.close
  end

  # Ticket 0291's P1 through the public plan, with no key: the exact plan
  # object of the shared corpus. A batch of 0 refuses as usage with code 1,
  # and neither reaches the backend. ThinkThen.failed and ThinkThen.outcome
  # read an annotate answer and a decide answer.
  def test_plan_previews_p1_with_no_key_and_sends_nothing
    corpus = JSON.parse(File.read(File.expand_path("../../../specification/fixtures/types/corpus.json", __dir__)))
    p1 = corpus.fetch("cases").find { |one| one["name"] == "plan-p1" }
    given = p1.fetch("plan_input")
    assert_equal({}, given.fetch("settings"))
    TestBackend.with(<<~RUBY, extra: { "THINKTHEN_API_KEY" => nil }) do |backend, child|
      question, input = JSON.parse(hear)
      say T.plan(question, input)
      begin
        T.plan(question, input, batch: 0)
      rescue T::UsageError => e
        say [e.kind, e.code]
      end
      failure = { "failed" => { "kind" => "backend", "cause" => "missing_answer", "surprise" => 1 } }
      say [T.failed(failure), [nil, true, "billing", 1.2, %w[billing], {}, failure.merge("other" => 1)].map { |one| T.failed(one) }]
      say [true, false, nil].map { |one| T.outcome(one) }
    RUBY
      child.tell(JSON.generate([given.fetch("question"), given.fetch("input")]))
      assert_equal p1.fetch("response"), child.hear
      assert_equal ["usage", 1], child.hear
      assert_equal [{ "kind" => "backend", "cause" => "missing_answer", "surprise" => 1 }, [nil] * 7], child.hear
      assert_equal [1, 0, 2], child.hear
      status, errors = child.finish
      assert status.success?, errors
      assert_equal 0, backend.count
    end
  end
end
