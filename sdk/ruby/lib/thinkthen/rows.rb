# frozen_string_literal: true

module ThinkThen
  class Rows < Enumerator

    def initialize(client, request, records, cancel, filter)
      @client, @filter, @items, @position = client, filter, {}, 0
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
      # Steep crashes on super with this native Enumerator constructor block.
      # Only the core initializer crosses this boundary; produce stays checked.
      # @type var initializer: untyped
      initializer = Enumerator.instance_method(:initialize)
      initializer.bind_call(self) { |yielder| produce(yielder) }
    rescue Exception
      close if @owner
      raise
    end

    def each
      return self unless block_given?
      raise ThreadError, "call already has an active driver" unless @traversal.try_lock
      begin
        super { |row| yield row }
      ensure
        close
        @traversal.unlock
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
      @driver&.synchronize do
        @reader&.close
        @reader = nil
        @initial = nil
        @items.clear
        @records = nil
      end
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
        Thread.handle_interrupt(Exception => :never) do
          begin
            drive
          rescue Interrupt => interruption
            @owner.cancel
            begin
              Native.checked { |error| Native.thinkthen_cancelled_error(error) }
            rescue Cancelled => error
              raise error, cause: interruption
            end
          end
        end
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
            row = Row.decode(native)
            if @filter
              item = @initial ? @initial[row.position - 1] : @items.delete(row.position)
              return FilterRow.new(item: item, row: row)
            end
            return row
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
      begin
        Thread.handle_interrupt(Exception => :immediate) { read.value }
      rescue Exception
        @owner.cancel
        raise
      ensure
        # Keep the row pinned until the native operation has stopped.
        read.value rescue nil
      end
    end

    def supply_input
      record = begin
        # Caller input may block in Ruby; native storage stays protected outside it.
        Thread.handle_interrupt(Exception => :immediate) { @records.next }
      rescue StopIteration
        @owner.use { |pointer| Native.thinkthen_call_end_input(pointer) }
        return
      end
      @position += 1
      @items[@position] = record if @filter
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
