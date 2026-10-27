# frozen_string_literal: true

module ThinkThen
  # A read keeps its outcome available when interruption follows completion.
  class NativeRead
    def initialize(native)
      @native = native
      @ready = Queue.new
      @result = @error = nil
    end

    def run(owner)
      @result = owner.use do |pointer|
        Native.checked { |error| Native.thinkthen_call_next(pointer, @native, error) }
      end
    rescue Exception => error
      @error = error
    ensure
      @ready.close
    end

    def value
      @ready.pop
      error = @error
      raise error if error
      @result
    end
  end

  # One helper owns blocking reads for a stream and exits when that stream closes.
  class NativeReader
    def initialize(owner)
      @requests = Queue.new
      @thread = self.class.worker(@requests, WeakRef.new(owner))
      ObjectSpace.define_finalizer(self, self.class.finalizer(@requests))
    end

    def self.finalizer(requests)
      proc { requests.close }
    end

    def self.worker(requests, reference)
      Thread.new do
        Thread.handle_interrupt(Exception => :never) do
          while (read = requests.pop)
            read.run(reference.__getobj__)
            read = nil
          end
        end
      end
    end

    def start(native)
      read = NativeRead.new(native)
      @requests << read
      read
    end

    def close
      @requests.close
      @thread.join
      nil
    end
  end
  private_constant :NativeRead, :NativeReader
end
