# frozen_string_literal: true

require "rbconfig"
require_relative "cases"

# Feature: a program that ends with a stream open still exits, for any client.
def ruby_exit_with_open_stream
  %w[next peek shared].each do |mode|
    Open3.popen2e(RbConfig.ruby, "-w", __FILE__, mode) do |_input, output, process|
      equal("open", output.readline.strip, "#{mode} reads a row before exit")
      exited = process.join(30)
      Process.kill("KILL", process.pid) unless exited
      assert(exited, "#{mode} stream holds the process open at exit")
      result = output.read
      assert(process.value.success?, "#{mode} exit failed: #{result}")
      equal("", result, "#{mode} exits quietly")
    end
  end
end

if $PROGRAM_NAME == __FILE__
  isolated_ruby_environment do
    require "thinkthen"
    mode = ARGV.fetch(0)
    ENV["THINKTHEN_BACKEND"] = "stub"
    # A global keeps the stream open through exit; collection must not close it first.
    $stream = (mode == "shared" ? ThinkThen : ThinkThen::Client.new).decide_each("Ready?", %w[first second])
    mode == "peek" ? $stream.peek : $stream.next
    puts "open"
    $stdout.flush
  end
end
