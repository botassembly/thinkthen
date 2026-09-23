# A signal during a call never resends a paid request, and Ctrl-C stops a
# call within one in-flight round: sent requests finish, none starts after
# (surfaces-review-5). An in-process loopback stub counts
# every request body that arrives - an abandoned request counts too, the
# blind spot of a counter that only counts completed replies.
#
#   ruby -I lib tests/test_signal_no_resend.rb
#
# Needs no network beyond loopback; the gate runs it offline.
ENV.delete("ENGINE_NULL")
ENV.delete("THINKTHEN_NULL")
require "json"
require "socket"

# The stub: one thread per connection, a fixed delay per request, the
# vendor's reply shape, and a count of every body read.
class CountingStub
  REPLY = JSON.generate(
    "model" => "jev-latest",
    "answers" => { "q1" => { "type" => "noul", "noul" => 0.9 } },
    "usage" => { "input_tokens" => 10, "output_tokens" => 2 },
  )

  attr_reader :port
  attr_accessor :delay

  def initialize
    @server = TCPServer.new("127.0.0.1", 0)
    @port = @server.addr[1]
    @bodies = 0
    @lock = Mutex.new
    @delay = 1.0
    Thread.new { loop { client = @server.accept; Thread.new(client) { |one| serve(one) } } }
  end

  def bodies
    @lock.synchronize { @bodies }
  end

  def reset
    @lock.synchronize { @bodies = 0 }
  end

  private

  def serve(client)
    loop do
      length = nil
      while (line = client.gets)
        break if line == "\r\n"
        length = line.split(":", 2)[1].to_i if line.downcase.start_with?("content-length:")
      end
      break unless line && length
      client.read(length)
      @lock.synchronize { @bodies += 1 }
      sleep @delay
      client.write("HTTP/1.1 200 OK\r\ncontent-type: application/json\r\n" \
                   "content-length: #{REPLY.bytesize}\r\n\r\n#{REPLY}")
    end
  rescue IOError, SystemCallError
    nil
  ensure
    client.close rescue nil
  end
end

STUB = CountingStub.new
ENV["THINKTHEN_BASE_URL"] = "http://127.0.0.1:#{STUB.port}/v1"
$LOAD_PATH.unshift File.expand_path("../lib", __dir__)
require "thinkthen"

QUESTION = ThinkThen.question(decide: "Is this a complaint?")

def clock
  Process.clock_gettime(Process::CLOCK_MONOTONIC)
end

# Run the block with a signal sent at `at` seconds; return its outcome,
# the elapsed seconds, and the bodies the stub saw once every sent
# request had time to land.
def measured(signal, at)
  STUB.reset
  Thread.new { sleep at; Process.kill(signal, Process.pid) }
  started = clock
  outcome = begin
    yield
  rescue Exception => e # rubocop:disable Lint/RescueException
    e.class
  end
  elapsed = clock - started
  sleep STUB.delay + 1.2
  [outcome, elapsed, STUB.bodies]
end

def check(name, held, detail)
  raise "#{name}: #{detail}" unless held
  puts "ok  #{name}: #{detail}"
end

trap("USR1") {}
STUB.delay = 1.5
outcome, elapsed, bodies = measured("USR1", 0.3) { ThinkThen.decide(QUESTION, "usr1 #{rand}") }
check("single-usr1-no-resend", outcome == true && bodies == 1,
      "a trapped USR1 answered #{outcome.inspect} with #{bodies} request bodies")

outcome, elapsed, bodies = measured("INT", 0.3) { ThinkThen.decide(QUESTION, "int #{rand}") }
check("single-int-prompt", outcome == Interrupt && elapsed < STUB.delay + 0.5 && bodies == 1,
      "Ctrl-C raised #{outcome} after #{format('%.1f', elapsed)} s with #{bodies} request bodies")

STUB.delay = 1.0
records = Array.new(40) { |i| "bulk #{i} #{rand}" }
outcome, elapsed, bodies = measured("INT", 0.5) { ThinkThen.decide_many(QUESTION, records) }
check("bulk-int-prompt", outcome == Interrupt && elapsed < 1.9 && bodies <= 16,
      "Ctrl-C raised #{outcome} after #{format('%.1f', elapsed)} s with #{bodies} of 40 request bodies")

# A token cancelled before the call sends nothing, single or bulk: the
# crossing fires the call's own token before it starts, not on the
# watchdog's next poll (surfaces-review-5).
STUB.delay = 0.2
fired = ThinkThen::Cancel.new
fired.cancel
[["single", -> { ThinkThen.decide(QUESTION, "pre #{rand}", cancel: fired) }],
 ["bulk", -> { ThinkThen.decide_many(QUESTION, Array.new(8) { |i| "pre #{i} #{rand}" }, cancel: fired) }]].each do |shape, call|
  STUB.reset
  outcome = begin
    call.call
  rescue ThinkThen::CancelledError => e
    e.class
  end
  sleep 0.5
  check("pre-cancelled-#{shape}-sends-nothing", outcome == ThinkThen::CancelledError && STUB.bodies.zero?,
        "a cancelled token answered #{outcome.inspect} with #{STUB.bodies} request bodies")
end
