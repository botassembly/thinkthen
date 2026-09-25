# frozen_string_literal: true

# Interrupts during a held batch at a throttle of 8 (R1-20, R5-9, R2-15).
# The held arm keeps every reply until the test releases it, so the count
# reads exactly the sends in flight. A stop must arrive within 150 ms while
# the eight sends stay held, and the detached worker must send nothing more.
require "minitest/autorun"
require_relative "backend"

class TestInterruptBatch < Minitest::Test
  BATCH = <<~RUBY
    engine = T::Engine.new(throttle: 8)
    before = threads
    records = (1..200).map { |n| "record \#{n}" }
  RUBY

  # After release: the worker has ended, nothing more was sent, and the
  # next call answers at once.
  def after_release(backend, child)
    sleep 0.3
    assert_equal 8, backend.count, "the stopped batch sent past its eight held requests"
    backend.release
    child.tell
    now_threads, before = child.hear
    assert_equal before, now_threads, "the detached worker did not end within 2 s"
    assert_equal 8, backend.count, "the released worker sent again"
    child.tell
    assert_operator child.hear, :<, 1000, "the following decide waited on held throttle slots"
    status, errors = child.finish
    assert status.success?, errors
  end

  def follow
    <<~RUBY
      hear
      say [settled(before), before]
      hear
      start = now
      engine.decide("Is it urgent?", "after the batch")
      say ms_since(start)
    RUBY
  end

  # R1-20: Thread#raise surfaces unchanged and at once.
  def test_thread_raise_stops_a_held_batch_at_once
    script = BATCH + <<~RUBY + follow
      main = Thread.current
      raised_at = nil
      raiser = Thread.new { hear; raised_at = now; main.raise(RuntimeError, "a test raise") }
      begin
        engine.decide_many("Is it urgent?", records)
        say ["answered"]
      rescue RuntimeError => e
        say [e.message, ms_since(raised_at)]
      end
      raiser.join
    RUBY
    TestBackend.with(script, arm: "arm/held") do |backend, child|
      assert_equal 8, backend.wait(8)
      child.tell
      message, elapsed = child.hear
      assert_equal "a test raise", message
      assert_operator elapsed, :<, 150
      after_release(backend, child)
    end
  end

  # R5-9: Ctrl-C raises CancelledError with the Interrupt as its cause.
  def test_ctrl_c_cancels_a_held_batch_at_once
    script = BATCH + <<~RUBY + follow
      sent_at = nil
      signaller = Thread.new { hear; sent_at = now; Process.kill("INT", Process.pid) }
      begin
        engine.decide_many("Is it urgent?", records)
        say ["answered"]
      rescue T::CancelledError => e
        say [e.class.name, e.cause.class.name, ms_since(sent_at)]
      end
      signaller.join
    RUBY
    TestBackend.with(script, arm: "arm/held") do |backend, child|
      assert_equal 8, backend.wait(8)
      child.tell
      name, cause, elapsed = child.hear
      assert_equal ["ThinkThen::CancelledError", "Interrupt"], [name, cause]
      assert_operator elapsed, :<, 150
      after_release(backend, child)
    end
  end

  # R2-15: a spurious wakeup and a trapped signal that raises nothing leave
  # the call alone. A raise on one thread leaves a sibling that shares its
  # caller token answering, and the shared token stays unfired.
  def test_harmless_wakeups_and_a_sibling_raise_leave_calls_running
    script = <<~RUBY
      engine = T::Engine.new(throttle: 16)
      Signal.trap("USR1") {}
      shared = T::Cancel.new
      records = (1..8).map { |n| "record \#{n}" }
      calls = Array.new(2) do |n|
        Thread.new do
          engine.decide_many("Is it urgent?", records.map { |one| "\#{one} of \#{n}" }, cancel: shared).size
        rescue RuntimeError => e
          e.message
        end
      end
      hear
      calls[0].wakeup
      Process.kill("USR1", Process.pid)
      sleep 0.2
      calls[1].raise(RuntimeError, "a raise on one call")
      say [calls[1].value, calls[0].alive?]
      hear
      say [calls[0].value, shared.cancelled?]
    RUBY
    TestBackend.with(script, arm: "arm/held") do |backend, child|
      assert_equal 16, backend.wait(16)
      child.tell
      assert_equal ["a raise on one call", true], child.hear
      backend.release
      child.tell
      assert_equal [8, false], child.hear
      status, errors = child.finish
      assert status.success?, errors
    end
  end
end
