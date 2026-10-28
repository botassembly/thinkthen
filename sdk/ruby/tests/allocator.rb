# frozen_string_literal: true

require "rbconfig"
require_relative "cases"

# Feature: a released call trims only the active glibc allocator, once after cleanup.
def ruby_native_allocator
  %w[cleanup platform missing-version missing-trim load-error malloc calloc realloc free trim-error].each do |mode|
    output, status = Open3.capture2e(RbConfig.ruby, "-w", __FILE__, mode)
    assert(status.success?, "allocator #{mode}: #{output}")
    equal("allocator cleanup passed", output.strip, "allocator #{mode} completes")
  end
end

if $PROGRAM_NAME == __FILE__
  require "ffi"
  mode = ARGV.fetch(0, "cleanup")
  libc = FFI::DynamicLibrary.open(FFI::Library::LIBC, FFI::DynamicLibrary::RTLD_LAZY)
  trim = libc.find_function("malloc_trim")
  glibc = FFI::Platform::IS_LINUX && !!libc.find_function("gnu_get_libc_version")
  expected = mode == "cleanup" && glibc ? 1 : 0
  if mode == "platform"
    FFI::Platform.send(:remove_const, :IS_LINUX)
    FFI::Platform.const_set(:IS_LINUX, false)
  elsif mode != "cleanup" && mode != "noop"
    process_library = nil
    FFI::DynamicLibrary.singleton_class.prepend(Module.new do
      define_method(:open) do |name, flags|
        raise LoadError, "unavailable allocator" if mode == "load-error" && name == FFI::Library::LIBC
        library = super(name, flags)
        process_library = library if name.nil?
        library
      end
    end)
    FFI::DynamicLibrary.prepend(Module.new do
      define_method(:find_function) do |name|
        return nil if mode == "missing-version" && name == "gnu_get_libc_version"
        return nil if mode == "missing-trim" && name == "malloc_trim"
        return FFI::Pointer.new(1) if equal?(process_library) && mode == name
        super(name)
      end
    end)
  end
  isolated_ruby_environment do
    require "thinkthen"
    if mode == "trim-error"
      ThinkThen.send(:native)
      failed = Object.new
      failed.define_singleton_method(:call) { |_pad| raise "allocator maintenance failed" }
      ThinkThen.const_get(:NativeAllocator).instance_variable_set(:@trim, failed)
    end
    events = []
    active = 0
    trace = TracePoint.new(:c_call, :c_return) do |event|
      if event.method_id == :thinkthen_call_next
        active += event.event == :c_call ? 1 : -1
      elsif event.method_id == :thinkthen_call_free && event.event == :c_return
        events << :free
      elsif event.event == :c_call && event.method_id == :call && event.self.is_a?(FFI::Function) && trim && event.self.address == trim.address
        equal(0, active, "trim follows the active native read")
        events << :trim
      end
    end
    verify = proc do |label|
      equal([:free, *Array.new(expected, :trim)], events, "#{label} releases and trims once")
      events.clear
    end
    trace.enable(target_thread: nil) do
      ThinkThen::Client.open(backend: :stub) do |client|
        client.decide_each("Ready?", Array.new(16, "record")) do |_row|
          equal([], events, "rows do not trim")
        end
        verify.call("exhaustion")

        stream = client.decide_each("Ready?", %w[first second])
        stream.next
        equal([], events, "a suspended stream does not trim")
        start = Queue.new
        closers = 3.times.map { Thread.new { start.pop; stream.close } }
        closers.length.times { start << true }
        closers.each(&:value)
        stream.close
        verify.call("concurrent close")

        # Client retains streams weakly; this call is collected while it stays open.
        client.decide_each("Ready?", ["record"])
        10.times do
          GC.start
          Thread.pass
          break unless events.empty?
        end
        verify.call("finalization")
      end

      Backend.open("--held", "headers") do |backend|
        ThinkThen::Client.open(backend: :systemone, url: backend.url, api_key: "synthetic-loopback-key") do |client|
          token = ThinkThen::CancelToken.new
          stream = client.decide_each("Ready?", ["record"], cancel: token)
          worker = Thread.new { raises(ThinkThen::Cancelled) { stream.to_a } }
          equal("arrived", backend.command("arrived"), "native read precedes cancellation")
          token.cancel
          worker.value
          stream.close
          verify.call("cancellation")
        end
      end
    end
    puts "allocator cleanup passed"
  ensure
    trace&.disable
  end
end
