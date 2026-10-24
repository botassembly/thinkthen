# frozen_string_literal: true

module ThinkThen
  class Rows < Enumerator
    # The owner pins a call across every FFI operation. Only cancellation may
    # enter concurrently; close waits for an active host operation before free.
    class Owner
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
        @mutex.synchronize { Native.thinkthen_cancel(@pointer) if @pointer }
        nil
      end

      def close
        Thread.handle_interrupt(Exception => :never) do
          @mutex.synchronize do
            return if !@pointer
            @closing = true
            Native.thinkthen_cancel(@pointer)
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
    private_constant :Owner

    def initialize(pointer, records, cancel)
      @owner = Owner.new(pointer)
      @cancel = cancel
      @cancel&.attach(@owner)
      @records = records.to_enum(:each)
      @driver = Mutex.new
      super() do |yielder|
        begin
          while true
            row = begin
              pull
            rescue StopIteration
              break
            end
            yielder << row
          end
        ensure
          close
        end
      end
    rescue Exception
      close if @owner
      raise
    end

    def each
      return self unless block_given?
      begin
        super { |row| yield row }
      ensure
        close
      end
    end

    def cancel = @owner.cancel
    def close
      @owner.close
      @cancel&.detach(@owner)
      nil
    end
    def closed? = @owner.closed?

    def rewind
      raise IOError, "calls cannot be rewound"
    end

    private

    def pull
      unless @driver.try_lock
        raise ThreadError, "call already has an active driver"
      end
      begin
        Thread.handle_interrupt(Exception => :never) { drive }
      ensure
        @driver.unlock
      end
    end

    def drive
      while true
        native = Native::Row.new
        begin
          status = blocking_next(native)
          return Row.new(native) if status == 0
          raise StopIteration if status == 1
        ensure
          Native.thinkthen_row_free(native)
        end
        supply_input
      end
    end

    # Ruby can interrupt a thread wait; FFI cannot interrupt a foreign call.
    # Cancel before joining, then release row memory after the foreign call ends.
    def blocking_next(native)
      worker = Thread.new do
        Thread.handle_interrupt(Exception => :never) do
          @owner.use do |pointer|
            Native.checked { |error| Native.thinkthen_next(pointer, native, error) }
          end
        end
      end
      worker.report_on_exception = false
      begin
        Thread.handle_interrupt(Exception => :immediate) { worker.value }
      rescue Exception
        @owner.cancel
        raise
      ensure
        worker.join rescue nil # Preserve the caller interruption or original native error.
      end
    end

    def supply_input
      record = begin
        @records.next
      rescue StopIteration
        @owner.use { |pointer| Native.thinkthen_end_input(pointer) }
        return
      end
      text = record.is_a?(String)
      value = text ? record : Native.dump(record)
      Native.bytes(value) do |bytes, length|
        @owner.use do |pointer|
          Native.checked do |error|
            if text
              Native.thinkthen_send_text(pointer, bytes, length, error)
            else
              Native.thinkthen_send_json(pointer, bytes, length, error)
            end
          end
        end
      end
    end
  end
end
