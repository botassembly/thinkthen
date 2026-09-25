# frozen_string_literal: true

# The tick lives in the watchdog's Ruby row, an ordinary root the collector
# marks (R2-8, R7-9, R4-18). The first run clears the caller's thread-local
# by name, the only other hold, and checks the drop took. A second thread
# runs GC.start through a batch held for 20 ticks. A finalizer on the tick
# must not run while the call still runs it. Ticks belong to their own
# thread (R3-16).
require "minitest/autorun"
require_relative "backend"

class TestTickGc < Minitest::Test
  def test_the_collector_leaves_a_running_tick_alone
    TestBackend.with(<<~RUBY, arm: "arm/held") do |backend, child|
      caller = Thread.current
      collected = false
      dropped = nil
      ticks = 0
      tick = proc do
        ticks += 1
        if ticks == 1
          caller[:thinkthen_tick] = nil
          dropped = caller[:thinkthen_tick].nil?
        end
        say "twenty" if ticks == 20
      end
      ObjectSpace.define_finalizer(tick, proc { collected = true })
      T.with_tick(&tick)
      tick = nil
      stop = false
      pressure = Thread.new { until stop; GC.start; sleep 0.01; end }
      answers = T::Engine.new(throttle: 4).decide_many("Is it urgent?", %w[one two three four])
      stop = true
      pressure.join
      say [dropped, collected, ticks >= 20, answers.size]
    RUBY
      assert_equal "twenty", child.hear(10)
      assert_equal 4, backend.count
      backend.release
      assert_equal [true, false, true, 4], child.hear
      status, errors = child.finish
      assert status.success?, errors
    end
  end

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
