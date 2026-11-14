# frozen_string_literal: true

module ThinkThen
  # A single-pass Enumerable of native outcomes. Close it when external iteration stops early.
  class Rows < Enumerator

    def initialize(client, request, records, cancel, question)
      @client, @question, @items, @position = client, question, {}, 0
      initial = records.is_a?(Array) ? Array.new(records) : nil
      @initial = initial
      @owner = CallOwner.new(client.send(:admit, request, initial))
      @cancel = cancel
      @cancel&.attach(@owner)
      # @type var input: untyped
      input = initial ? [] : records
      @records = input.to_enum(:each)
      @reader = nil
      @driver = Mutex.new
      @traversal = Mutex.new
      @traversing = false
      # Steep crashes on super with this native Enumerator constructor block.
      # Only the core initializer crosses this boundary; produce stays checked.
      # @type var initializer: untyped
      initializer = Enumerator.instance_method(:initialize)
      initializer.bind_call(self, initial ? initial.size : records.respond_to?(:size) ? records.size : nil) { |yielder| produce(yielder) }
      # The block above captures this scope; drop the admitted copy it no longer needs.
      initial = nil
    rescue Exception
      close if @owner
      raise
    end

    def each
      return self unless block_given?
      entered = false
      begin
        @traversal.synchronize do
          raise ThreadError, "call already has an active driver" if @traversing
          @traversing = entered = true
        end
        super { |row| yield row }
      ensure
        if entered
          begin
            close
          ensure
            @traversal.synchronize { @traversing = false }
          end
        end
      end
    end

    def cancel
      @client.send(:check_process)
      @owner.cancel
    end
    def close
      @client.send(:check_process)
      @owner.close
      @cancel&.detach(@owner)
      release
      @client.send(:forget, self)
      nil
    end
    def closed? = @owner.closed?

    def rewind
      raise IOError, "calls cannot be rewound"
    end

    private

    def produce(yielder)
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

    def pull
      @client.send(:check_process)
      unless @driver.try_lock
        raise ThreadError, "call already has an active driver"
      end
      begin
        # Thread#kill is not an Exception, so only Object holds it back here.
        Thread.handle_interrupt(Object => :never) do
          begin
            drive
          rescue Interrupt => interruption
            @owner.cancel
            begin
              Native.checked { |error| Native.thinkthen_cancelled_error(error) }
            rescue CancelledError => error
              raise error, cause: interruption
            end
          end
        end
      ensure
        @driver.unlock
        release if @owner.closed?
      end
    end

    # A driver held in caller input must not hold up close, so it releases on its way out.
    def release
      return unless @driver&.try_lock
      begin
        @reader&.close
        @reader = nil
        @initial = nil
        @items.clear
        @records = nil
      ensure
        @driver.unlock
      end
    end

    def drive
      while true
        native = Native::Row.new
        begin
          status = blocking_next(native)
          if status == 0
            position = native[:position]
            item = @initial ? @initial[position - 1] : @items.delete(position)
            return Row.decode(native, question: @question, item: item)
          end
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
      @reader ||= NativeReader.new(@owner)
      read = @reader.start(native)
      finished = false
      begin
        status = Thread.handle_interrupt(Object => :immediate) { read.value }
        finished = true
        status
      ensure
        begin
          # Thread#kill runs ensure blocks but no rescue, so cancel here.
          @owner.cancel unless finished
        ensure
          # Keep the row pinned until the native operation has stopped.
          read.value rescue nil
        end
      end
    end

    def supply_input
      record = begin
        # Caller input may block in Ruby; native storage stays protected outside it.
        Thread.handle_interrupt(Object => :immediate) { @records.next }
      rescue StopIteration
        @owner.use { |pointer| Native.thinkthen_call_end_input(pointer) }
        return
      end
      @position += 1
      @items[@position] = record
      kind, value = Native.record_value(record)
      if value.nil?
        @owner.use do |pointer|
          Native.checked do |error|
            kind == "missing" ? Native.thinkthen_call_send_missing(pointer, error) : Native.thinkthen_call_send_invalid(pointer, error)
          end
        end
        return
      end
      text = kind == "text"
      Native.bytes(value) do |bytes, length|
        @owner.use do |pointer|
          Native.checked do |error|
            if text
              Native.thinkthen_call_send_text(pointer, bytes, length, error)
            else
              Native.thinkthen_call_send_json(pointer, bytes, length, error)
            end
          end
        end
      end
    end
  end
end
