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
      assert_equal({ "requests_sent" => 1, "cache_answers" => 1, "input_tokens" => 0, "output_tokens" => 0 }, child.hear)
      status, errors = child.finish
      assert status.success?, errors
      assert_equal 1, backend.count
      refute_empty entries(File.join(root, "cache")), "the answer is not in THINKTHEN_CACHE"
      assert_empty entries(File.join(root, "home")) + entries(File.join(root, "xdg-cache")).grep_v(/\Athinkthen-usage\//)
    end
  end

  def test_the_throttle_holds_exactly_that_many_requests_in_flight
    TestBackend.with(<<~RUBY, arm: "arm/held") do |backend, child|
      say T::Engine.new(throttle: 8).decide_many("Is it urgent?", (1..20).map { |n| "record \#{n}" }).size
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
       { timeout: 0 }, { timeout: "30" }, { replay: 7 }, { cache: 3 }, { base_url: 7 }].each do |settings|
        T::Engine.new(**settings)
        say "built"
      rescue T::UsageError => e
        say e.message
      end
    RUBY
    assert_equal ["a throttle is a whole number from 1 through 32", "a throttle is a whole number from 1 through 32",
                  "throttle is a whole number or nil", "throttle is a whole number or nil",
                  "max_requests is a whole number or nil", "a timeout is a time above zero",
                  "timeout is a whole number or nil", "replay is text or nil",
                  "cache is a folder path, false for none, or nil for the default", "base_url is text or nil"], lines
    assert_equal 0, count
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
end
