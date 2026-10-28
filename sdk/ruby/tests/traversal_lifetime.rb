# frozen_string_literal: true

require "rbconfig"
require_relative "cases"

# Feature: closing a suspended traversal leaves no lock owned by an abandoned fiber.
def ruby_suspended_traversal
  output, status = Open3.capture2e(RbConfig.ruby, "-w", __FILE__)
  assert(status.success?, "suspended traversal cleanup: #{output}")
  equal("closed and collected", output.strip, "abandoned traversal permits normal shutdown")
end

if $PROGRAM_NAME == __FILE__
  isolated_ruby_environment do
    require "thinkthen"
    20.times do
      client = ThinkThen::Client.new(backend: :stub)
      token = ThinkThen::CancelToken.new
      stream = client.decide_each("Ready?", %w[first second], cancel: token)
      fiber = Fiber.new { stream.each { |row| Fiber.yield(row.position) } }
      equal(1, fiber.resume, "caller suspends after its first row")
      raises(ThreadError) { stream.each { |_row| } }
      start = Queue.new
      workers = [proc { token.cancel }, proc { stream.close }, proc { stream.close }].map do |operation|
        Thread.new { start.pop; operation.call }
      end
      workers.length.times { start << true }
      workers.each(&:value)
      assert(stream.closed?, "concurrent close retires the suspended call")
      client.close
      fiber = stream = client = token = workers = nil
      GC.start
    end
    puts "closed and collected"
  end
end
