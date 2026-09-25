# frozen_string_literal: true

# The crash shapes. R4-2: a raising USR1 trap every 0.2 ms through four held
# 2,000-record batches. The old crossing took the lock back inside the
# released region and crashed or hung. R3-5: 3,000 calls, each under a
# Thread#raise at a random offset, which must neither crash the process nor
# leave a watchdog row behind.
require "minitest/autorun"
require_relative "backend"

class TestFlood < Minitest::Test
  def test_a_raising_trap_flood_ends_each_held_round_with_the_traps_raise
    TestBackend.with(<<~RUBY, arm: "arm/held") do |backend, child|
      engine = T::Engine.new(throttle: 8)
      $flooding = false
      Signal.trap("USR1") { raise "the trap's raise" if $flooding }
      4.times do |round|
        before = threads
        outcome = nil
        begin
          $flooding = true
          flood = Thread.new { loop { Process.kill("USR1", Process.pid); sleep 0.0002 } }
          engine.decide_many("Is it urgent?", (1..2000).map { |n| "round \#{round} record \#{n}" })
          outcome = "answered"
        rescue RuntimeError => e
          outcome ||= e.message
        ensure
          $flooding = false
          flood&.kill
          flood&.join
        end
        say outcome
        hear
        say settled(before, 5) == before
      end
    RUBY
      4.times do
        assert_equal "the trap's raise", child.hear
        backend.round
        child.tell
        assert_equal true, child.hear, "the round's worker did not end after its replies went"
      end
      status, errors = child.finish
      assert status.success?, errors
    end
  end

  # A leaked watchdog row keeps running its tick with no call in flight, so
  # the tick's run count keeps growing after the storm.
  def test_a_raise_storm_leaves_no_crash_and_no_watchdog_row
    lines, = TestBackend.run(<<~RUBY)
      runs = 0
      T.with_tick { runs += 1 }
      main = Thread.current
      3000.times do |n|
        raiser = Thread.new { sleep(rand * 0.005); main.raise(RuntimeError, "the storm") }
        begin
          T.decide("Is it urgent?", "storm \#{n}")
        rescue RuntimeError
          nil
        end
        raiser.join
      rescue RuntimeError
        nil
      end
      GC.start
      sleep 0.2
      before = runs
      sleep 0.3
      say runs - before
    RUBY
    assert_equal [0], lines
  end
end
