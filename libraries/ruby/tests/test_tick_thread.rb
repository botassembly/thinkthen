# frozen_string_literal: true

# A tick belongs to the thread that set it (R3-16). The collector rows
# (R2-8, R7-9, R4-18) closed by design: Rust holds no Ruby object, and
# check.sh guards that in src.
require "minitest/autorun"
require_relative "backend"

class TestTickThread < Minitest::Test
  # A second thread sets a tick and ends. A held call on a thread with no
  # tick of its own must not run it. A tick kept on the module would run.
  def test_a_tick_runs_only_for_its_own_threads_calls
    TestBackend.with(<<~RUBY, arm: "arm/held") do |backend, child|
      runs = 0
      engine = T::Engine.new(throttle: 4)
      Thread.new { engine.with_tick { runs += 1 } }.join
      busy = Thread.new { engine.decide("Is it urgent?", "the busy thread's text") }
      hear
      say runs
      hear
      busy.join
    RUBY
      assert_equal 1, backend.wait(1)
      sleep 0.5
      child.tell
      assert_equal 0, child.hear
      backend.release
      child.tell
      status, errors = child.finish
      assert status.success?, errors
    end
  end
end
