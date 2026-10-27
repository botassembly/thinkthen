# frozen_string_literal: true

require "timeout"
require "rbconfig"
require_relative "cases"

# Feature: Ruby interrupts stop blocked caller input and release the native call.
def ruby_input_interruptions
  %w[interrupt timeout raise].each do |action|
    Open3.popen2e(RbConfig.ruby, "-w", __FILE__, action) do |input, output, process|
      equal("waiting", output.readline.strip, "#{action} reaches caller input")
      if action == "interrupt"
        Process.kill("INT", process.pid)
      elsif action == "raise"
        input.puts("raise")
        input.flush
      end
      result = output.read
      assert(process.value.success?, "#{action}: #{result}")
      equal("closed and freed", result.strip, "#{action} cleans up before returning")
    end
  end
end

if $PROGRAM_NAME == __FILE__
  isolated_ruby_environment do
    require "thinkthen"
    action = ARGV.fetch(0)
    ThinkThen::Client.open(backend: :stub) do |client|
      queue = Queue.new
      records = Enumerator.new do |out|
        $stdout.puts("waiting")
        $stdout.flush
        out << (action == "timeout" ? Timeout.timeout(0.01) { queue.pop } : queue.pop)
      end
      stream = client.decide_each("Ready?", records)
      main = Thread.current
      # An open controller pipe keeps Ruby from treating the blocked Queue as
      # a process deadlock before the parent delivers the interrupt.
      controller = Thread.new do
        $stdin.gets
        main.raise(RuntimeError, "caller interruption") if action == "raise"
        $stdin.read
      end
      freed = false
      trace = TracePoint.new(:return) do |event|
        freed = true if event.method_id == :release && event.defined_class.to_s == "#<Class:ThinkThen::Native::CallPointer>"
      end
      begin
        error = trace.enable do
          raises(action == "interrupt" ? ThinkThen::Cancelled : action == "timeout" ? Timeout::Error : RuntimeError) do
            stream.to_a
          end
        end
        assert(error.cause.is_a?(Interrupt), "Ctrl-C keeps its cause") if action == "interrupt"
        equal("caller interruption", error.message, "Thread#raise keeps its error") if action == "raise"
        assert(stream.closed? && freed, "input interruption frees the native call")
        puts "closed and freed"
      ensure
        trace.disable
        controller.kill.join
        stream.close
      end
    end
  end
end
