# frozen_string_literal: true

# Stops during one held decide (R1-20 single half, the raising tick, the
# caller's token) and a token fired before the call. Each stop must arrive
# within 150 ms while the one send stays held, and nothing more is sent.
require "minitest/autorun"
require_relative "backend"

class TestInterruptSingle < Minitest::Test
  # Hold one decide, trigger the stop, and return what the child reports.
  def held_stop(script)
    TestBackend.with(script, arm: "arm/held") do |backend, child|
      assert_equal 1, backend.wait(1)
      child.tell
      reported = child.hear
      sleep 0.3
      backend.release
      sleep 0.2
      assert_equal 1, backend.count, "the stopped call sent again"
      status, errors = child.finish
      assert status.success?, errors
      reported
    end
  end

  # R1-20 single half: Ctrl-C raises CancelledError, caused by the Interrupt.
  def test_ctrl_c_cancels_a_held_decide_at_once
    name, cause, elapsed = held_stop(<<~RUBY)
      sent_at = nil
      Thread.new { hear; sent_at = now; Process.kill("INT", Process.pid) }
      begin
        T.decide("Is it urgent?", "text")
      rescue T::CancelledError => e
        say [e.class.name, e.cause.class.name, ms_since(sent_at)]
      end
    RUBY
    assert_equal ["ThinkThen::CancelledError", "Interrupt"], [name, cause]
    assert_operator elapsed, :<, 150
  end

  # A tick that raises stops the call through the call's own token, and the
  # tick's own error surfaces within 150 ms of its raise.
  def test_a_raising_tick_stops_a_held_decide_at_once
    message, elapsed = held_stop(<<~RUBY)
      runs = 0
      raised_at = nil
      Thread.new { hear }
      T.with_tick do
        runs += 1
        if runs == 2
          raised_at = now
          raise ArgumentError, "the tick stops the call"
        end
      end
      begin
        T.decide("Is it urgent?", "text")
      rescue ArgumentError => e
        say [e.message, ms_since(raised_at)]
      end
    RUBY
    assert_equal "the tick stops the call", message
    assert_operator elapsed, :<, 150
  end

  def test_the_callers_token_cancels_a_held_decide_at_once
    name, elapsed = held_stop(<<~RUBY)
      token = T::Cancel.new
      fired_at = nil
      Thread.new { hear; fired_at = now; token.cancel }
      begin
        T.decide("Is it urgent?", "text", cancel: token)
      rescue T::CancelledError => e
        say [e.class.name, ms_since(fired_at)]
      end
    RUBY
    assert_equal "ThinkThen::CancelledError", name
    assert_operator elapsed, :<, 150
  end

  # Thread#kill ends a thread held in a call, and the process goes on.
  def test_thread_kill_ends_a_held_call
    TestBackend.with(<<~RUBY, arm: "arm/held") do |backend, child|
      call = Thread.new { T.decide("Is it urgent?", "text") }
      hear
      call.kill
      say [call.join(1).nil? ? "still alive" : call.status.inspect]
    RUBY
      assert_equal 1, backend.wait(1)
      child.tell
      assert_equal ["false"], child.hear
      status, errors = child.finish
      assert status.success?, errors
    end
  end

  def test_a_token_fired_before_the_call_sends_nothing
    lines, count = TestBackend.run(<<~RUBY)
      token = T::Cancel.new
      token.cancel
      say kind_of_raise { T.decide("Is it urgent?", "text", cancel: token) }
      say kind_of_raise { T.decide_many("Is it urgent?", %w[one two], cancel: token) }
      say token.cancelled?
    RUBY
    assert_equal ["CancelledError", "CancelledError", true], lines
    assert_equal 0, count
  end
end
