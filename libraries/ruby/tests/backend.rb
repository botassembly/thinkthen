# frozen_string_literal: true

# One loopback backend and one scrubbed child process per test.
#
# The engine reads its address and cache from the environment once per
# process, so every call a test makes runs in a child `ruby` from the pinned
# prefix. Each child gets its own THINKTHEN_BASE_URL on 127.0.0.1, a fresh
# cache, HOME, XDG_CACHE_HOME, and XDG_CONFIG_HOME, and a fake key that rides
# only beside that loopback address. The real key never reaches a child.
require "json"
require "open3"
require "rbconfig"
require "timeout"
require "tmpdir"
require "uri"
require_relative "../../../conformance/children/children"

module TestBackend
  BIN = ENV.fetch("THINKTHEN_TEST_BACKEND")
  LIB = File.expand_path("../lib", __dir__)
  FAKE_KEY = "tt-ruby-test-not-a-key"

  # Helpers every child script starts with.
  PRELUDE = <<~RUBY
    require "json"
    require "thinkthen"
    T = ThinkThen
    def say(value) = (STDOUT.puts(JSON.generate(value)); STDOUT.flush)
    def hear = STDIN.gets&.strip
    def now = Process.clock_gettime(Process::CLOCK_MONOTONIC)
    def ms_since(start) = ((now - start) * 1000).round
    def threads = Dir.children("/proc/self/task").size
    def settled(before, within = 2.0)
      stop = now + within
      sleep 0.01 while threads > before && now < stop
      threads
    end
    def kind_of_raise
      yield
      "none"
    rescue T::Error => e
      e.class.name.split("::").last
    end
  RUBY

  # One backend process, driven over its standard input.
  class Backend
    attr_reader :port

    def initialize
      @in, @out, @thread = Open3.popen2(Children.env, BIN, unsetenv_others: true)
      @port = Integer(@out.gets)
    end

    def url(arm = "generic")
      "http://127.0.0.1:#{port}/#{arm}/v1"
    end

    def count
      order("count")
      Integer(@out.gets)
    end

    def capture
      order("capture")
      JSON.parse(@out.gets).fetch("bodies")
    end

    # The count once it reads at least n, or at 5 s.
    def wait(count)
      order("wait #{count}")
      line = @out.gets until line&.start_with?("wait ")
      Integer(line.split.last)
    end

    def round = order("round")

    def release = order("release")

    def close
      @in.close
      @thread.value
    end

    private

    def order(line)
      @in.puts(line)
      @in.flush
    end
  end

  # One child ruby running PRELUDE and a script, with a line channel both ways.
  class Child
    attr_reader :pid

    def initialize(env, script)
      @in, @out, @err, @thread = Open3.popen3(env, RbConfig.ruby, "-I", LIB, "-e", PRELUDE + script,
                                              unsetenv_others: true)
      @pid = @thread.pid
      @errors = Thread.new { @err.read }
    end

    def tell(line = "go")
      @in.puts(line)
      @in.flush
    end

    # The child's next JSON line, within the bound.
    def hear(within = 30)
      line = Timeout.timeout(within) { @out.gets }
      raise "the child ended early: #{@errors.value}" if line.nil?

      JSON.parse(line)
    end

    def finish(within = 60)
      @in.close
      status = Timeout.timeout(within) { @thread.value }
      [status, @errors.value]
    rescue Timeout::Error
      Process.kill("KILL", pid)
      raise "the child hung past #{within} s"
    end
  end

  # Each child's whole environment: PATH, the library path the pinned Ruby
  # needs, the locale its JSON reads, an installed gem's folder (ticket
  # 0128), and the fake key only beside a loopback address (ticket 0127).
  def self.env(base_url, root, extra = {})
    host = URI(base_url).host
    raise ArgumentError, "refusing a non-loopback backend address: #{host}" unless host == "127.0.0.1"

    %w[cache home xdg-cache xdg-config].each { |name| Dir.mkdir(File.join(root, name)) unless Dir.exist?(File.join(root, name)) }
    Children.env(
      keep: %w[LD_LIBRARY_PATH LANG GEM_PATH],
      "THINKTHEN_BASE_URL" => base_url,
      "THINKTHEN_API_KEY" => FAKE_KEY,
      "THINKTHEN_CACHE" => File.join(root, "cache"),
      "HOME" => File.join(root, "home"),
      "XDG_CACHE_HOME" => File.join(root, "xdg-cache"),
      "XDG_CONFIG_HOME" => File.join(root, "xdg-config"),
      **extra
    )
  end

  # Start a backend and a child on one arm, yield both, and clean up.
  def self.with(script, arm: "generic", extra: {})
    backend = Backend.new
    Dir.mktmpdir do |root|
      child = Child.new(env(backend.url(arm), root, extra.transform_values { |value| value.to_s.gsub("ROOT", root).gsub("PORT", backend.port.to_s) }), script)
      yield backend, child, root
    ensure
      begin
        Process.kill("KILL", child.pid) if child
      rescue Errno::ESRCH
        nil
      end
    end
  ensure
    backend&.close
  end

  # Run a script to its end and return its JSON lines, the final count, and
  # the child's standard error.
  def self.run(script, arm: "generic")
    backend = Backend.new
    Dir.mktmpdir do |root|
      out, errors, status = Open3.capture3(env(backend.url(arm), root), RbConfig.ruby, "-I", LIB, "-e", PRELUDE + script,
                                           unsetenv_others: true)
      raise "the child failed (#{status.exitstatus}): #{errors}" unless status.success?

      [out.lines.map { |line| JSON.parse(line) }, backend.count, errors]
    end
  ensure
    backend&.close
  end
end
