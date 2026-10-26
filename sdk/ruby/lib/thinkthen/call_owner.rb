# frozen_string_literal: true

module ThinkThen
  # The owner pins a call across every FFI operation. Only cancellation may
  # enter concurrently; close waits for an active host operation before free.
  class CallOwner
    def initialize(pointer)
      @pointer = Native::CallPointer.new(pointer)
      @mutex = Mutex.new
      @idle = ConditionVariable.new
      @active = false
      @closing = false
    end

    def use
      @mutex.synchronize do
        raise IOError, "call is closed" if @closing || !@pointer
        raise ThreadError, "call already has an active driver" if @active
        @active = true
      end
      begin
        yield @pointer
      ensure
        @mutex.synchronize do
          @active = false
          @idle.broadcast
        end
      end
    end

    def cancel
      @mutex.synchronize { Native.thinkthen_call_cancel(@pointer) if @pointer }
      nil
    end

    def close
      Thread.handle_interrupt(Exception => :never) do
        @mutex.synchronize do
          return if !@pointer
          @closing = true
          Native.thinkthen_call_cancel(@pointer)
          @idle.wait(@mutex) while @active
          return unless @pointer
          @pointer.free
          @pointer = nil
        end
      end
      nil
    end

    def closed?
      @mutex.synchronize { !@pointer }
    end
  end
  private_constant :CallOwner
end
