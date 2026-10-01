# frozen_string_literal: true

# A fork after the first call answers in the forked child, and the parent's
# counters do not move (0096, Q15). The parent reads under a 30 s bound, so
# a hang fails instead of stalling.
require "minitest/autorun"
require_relative "backend"

class TestFork < Minitest::Test
  def test_a_forked_child_answers_and_the_parents_counters_hold
    lines, count = TestBackend.run(<<~RUBY)
      require "timeout"
      T.decide("Is it urgent?", "before the fork")
      before = T.usage
      reader, writer = IO.pipe
      pid = Process.fork do
        reader.close
        writer.write(JSON.generate([T.decide("Is it urgent?", "in the forked child").value, T.usage[:requests_sent]]))
        writer.close
        exit!(0)
      end
      writer.close
      forked = begin
        Timeout.timeout(30) { reader.read }
      rescue Timeout::Error
        Process.kill("KILL", pid)
        "the forked child hung"
      end
      Process.wait(pid)
      say [forked, T.usage == before]
    RUBY
    # The child's counters start at zero (0096), so they read its one send.
    assert_equal [["[true,1]", true]], lines
    assert_equal 2, count
  end
end
