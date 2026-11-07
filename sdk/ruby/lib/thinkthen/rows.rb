# frozen_string_literal: true

module ThinkThen
  # A single-pass Enumerable of native outcomes. Close it when external iteration stops early.
  class Rows < Enumerator

    def initialize(client, request, records, cancel, filter)
      @client, @filter, @items, @position = client, filter, {}, 0
      initial = records.is_a?(Array) ? Array.new(records) : nil
      # Filter returns the caller's own objects. Rank keeps only the engine's
      # snapshots and returns a copy of each record read back from JSON, with String keys.
      @initial = filter == :filter ? initial : nil
      @invalid = Hash.new #: Hash[Integer, String]
      @owner = CallOwner.new(client.send(:admit, request, initial, @invalid))
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
        @invalid.clear
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
            row = explain(Row.decode(native))
            if @filter == :rank
              return RankRow.new(item: Native.original(native), row: row)
            elsif @filter
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

    # The engine reports only record.invalid; name the record's class and reason.
    def explain(row)
      reason = @invalid.delete(row.position)
      return row unless reason && row.failed? && row.error.code == :"record.invalid"
      error = row.error
      row.with(error: RowError.new(code: error.code, message: "record #{row.position}: #{reason}", http_status: error.http_status,
                                   raw_code: error.raw_code, raw_name: error.raw_name))
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
      @items[@position] = record if @filter == :filter
      kind, value, reason = Native.record_value(record)
      @invalid[@position] = reason if reason
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
