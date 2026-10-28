# frozen_string_literal: true

require_relative "contract"
require_relative "native_allocator"

module ThinkThen
  module Native
    extend FFI::Library

    library = if FFI::Platform.windows?
                "thinkthen_ffi.dll"
              elsif FFI::Platform.mac?
                "libthinkthen_ffi.dylib"
              else
                "libthinkthen_ffi.so"
              end
    path = File.expand_path(library, __dir__ || ".")
    begin
      ffi_lib path
    rescue LoadError => error
      raise LocalError.new("cannot load native engine #{path}: #{error.message}", code: :"local.io")
    end
    begin
      attach_function :thinkthen_abi_version, [], :uint32
      attach_function :thinkthen_abi_compatible, [:uint32, :uint32], :uint8
      attach_function :thinkthen_code_name, [:int32], :string
      actual = thinkthen_abi_version
      required = NativeContract::ABI_VERSION
      unless thinkthen_abi_compatible(required, actual) != 0
        raise UsageError.new(NativeContract::ABI_MESSAGE, code: NativeContract::ABI_CODE)
      end
    rescue FFI::NotFoundError
      raise UsageError.new(NativeContract::ABI_MESSAGE, code: NativeContract::ABI_CODE)
    end

    class Buffer < FFI::Struct
      layout :data, :pointer, :len, :size_t
    end

    class RecordInput < FFI::Struct
      layout :struct_size, :uint32, :kind, :uint32, :data, :pointer, :len, :size_t
    end

    class NativeError < FFI::Struct
      layout :struct_size, :uint32, :code, :int32, :http_status, :uint16, :message, Buffer
      def initialize
        super
        self[:struct_size] = self.class.size
      end
    end

    class Row < FFI::Struct
      layout :struct_size, :uint32, :function, :uint32, :status, :uint32,
             :position, :uint64, :probability, :double, :error_code, :int32,
             :answer_bool, :uint8, :rank, :uint64, :number, :double,
             :id, Buffer, :answer, Buffer, :detail, Buffer, :source, Buffer, :error, Buffer
      def initialize
        super
        self[:struct_size] = self.class.size
      end
    end

    begin
      attach_function :thinkthen_toolbox_from_json, [:pointer, :size_t, :pointer, :pointer], :int32
      attach_function :thinkthen_toolbox_from_definitions_json, [:pointer, :size_t, :pointer, :pointer], :int32
      attach_function :thinkthen_toolbox_to_json, [:pointer, :pointer, :pointer], :int32
      attach_function :thinkthen_toolbox_definitions_json, [:pointer, :pointer, :pointer], :int32
      attach_function :thinkthen_toolbox_free, [:pointer], :void
      attach_function :thinkthen_buffer_free, [:pointer], :void
      attach_function :thinkthen_question_from_json, [:pointer, :size_t, :pointer, :pointer], :int32
      attach_function :thinkthen_question_new, [:uint32, :pointer, :size_t, :pointer, :pointer], :int32
      attach_function :thinkthen_question_set_cut, [:pointer, :double, :pointer], :int32
      attach_function :thinkthen_question_set_band, [:pointer, :double, :double, :pointer], :int32
      attach_function :thinkthen_question_free, [:pointer], :void
      attach_function :thinkthen_settings_new, [:pointer, :pointer], :int32
      attach_function :thinkthen_settings_set_api_key, [:pointer, :pointer, :size_t, :pointer], :int32
      attach_function :thinkthen_settings_from_json, [:pointer, :size_t, :pointer, :pointer], :int32
      attach_function :thinkthen_settings_copy_api_key, [:pointer, :pointer, :pointer], :int32
      attach_function :thinkthen_settings_free, [:pointer], :void
      attach_function :thinkthen_client_open, [:pointer, :pointer, :pointer], :int32, blocking: true
      attach_function :thinkthen_client_open_json, [:pointer, :size_t, :pointer, :pointer], :int32, blocking: true
      attach_function :thinkthen_settings_conversion_error, [:pointer], :int32
      attach_function :thinkthen_question_conversion_error, [:pointer], :int32
      attach_function :thinkthen_cancelled_error, [:pointer], :int32
      attach_function :thinkthen_call_send_invalid, [:pointer, :pointer], :int32, blocking: true
      attach_function :thinkthen_client_free, [:pointer], :void
      attach_function :thinkthen_call_open_records_json,
                      [:pointer, :pointer, :size_t, :pointer, :size_t, :size_t, :uint32, :pointer, :pointer], :int32, blocking: true
      attach_function :thinkthen_call_open_json,
                      [:pointer, :pointer, :size_t, :uint32, :pointer, :pointer], :int32, blocking: true
      attach_function :thinkthen_call_send_text, [:pointer, :pointer, :size_t, :pointer], :int32, blocking: true
      attach_function :thinkthen_call_send_json, [:pointer, :pointer, :size_t, :pointer], :int32, blocking: true
      attach_function :thinkthen_call_send_missing, [:pointer, :pointer], :int32, blocking: true
      attach_function :thinkthen_call_end_input, [:pointer], :void
      attach_function :thinkthen_call_next, [:pointer, :pointer, :pointer], :int32, blocking: true
      attach_function :thinkthen_call_cancel, [:pointer], :void
      attach_function :thinkthen_call_free, [:pointer], :void
      attach_function :thinkthen_error_free, [:pointer], :void
      attach_function :thinkthen_row_free, [:pointer], :void
    rescue FFI::NotFoundError
      raise UsageError.new(NativeContract::ABI_MESSAGE, code: NativeContract::ABI_CODE)
    end

    class SettingsPointer < FFI::AutoPointer
      def self.release(pointer) = Native.thinkthen_settings_free(pointer)
    end
    def self.settings
      output = FFI::MemoryPointer.new(:pointer)
      checked { |error| thinkthen_settings_new(output, error) }
      SettingsPointer.new(output.read_pointer)
    end
    class ClientPointer < FFI::AutoPointer
      def self.release(pointer)
        Native.thinkthen_client_free(pointer)
      end
    end

    class CallPointer < FFI::AutoPointer
      def self.release(pointer)
        Native.thinkthen_call_free(pointer)
        NativeAllocator.trim
      end
    end

    def self.validate_threshold(function, text, threshold)
      return if threshold.nil?
      output = FFI::MemoryPointer.new(:pointer)
      bytes(text || "") do |data, length|
        checked { |error| thinkthen_question_new(NativeContract::FUNCTION_NAMES.key(function.to_s), data, length, output, error) }
      end
      if threshold.is_a?(Hash)
        checked { |error| thinkthen_question_set_band(output.read_pointer, threshold.fetch("low"), threshold.fetch("high"), error) }
      else
        checked { |error| thinkthen_question_set_cut(output.read_pointer, threshold, error) }
      end
    ensure
      thinkthen_question_free(output.read_pointer) if output && !output.read_pointer.null?
    end

    def self.validate_question(value)
      question = FFI::MemoryPointer.new(:pointer)
      bytes(context_dump(value, :question)) do |data, length|
        checked { |error| thinkthen_question_from_json(data, length, question, error) }
      end
    ensure
      thinkthen_question_free(question.read_pointer) if question && !question.read_pointer.null?
    end

    def self.toolbox_json(json, saved: false, definitions: false)
      output = FFI::MemoryPointer.new(:pointer)
      result = Buffer.new
      bytes(json) do |data, length|
        checked do |error|
          if saved
            thinkthen_toolbox_from_json(data, length, output, error)
          else
            thinkthen_toolbox_from_definitions_json(data, length, output, error)
          end
        end
      end
      checked do |error|
        if definitions
          thinkthen_toolbox_definitions_json(output.read_pointer, result, error)
        else
          thinkthen_toolbox_to_json(output.read_pointer, result, error)
        end
      end
      copy(result)
    ensure
      thinkthen_buffer_free(result) if result
      thinkthen_toolbox_free(output.read_pointer) if output && !output.read_pointer.null?
    end

    def self.dump(value, symbols: false)
      JSON.generate(snapshot(value, {}, symbols), max_nesting: false)
    end

    def self.snapshot(value, seen = {}, symbols = false)
      case value
      when String then String.new(value)
      when Integer, Float, NilClass, TrueClass, FalseClass then value
      when Symbol
        raise TypeError, "expected a JSON value" unless symbols
        value.to_s
      when Array, Hash
        raise ArgumentError, "cyclic JSON value" if seen[value.object_id]
        seen[value.object_id] = true
        begin
          if value.is_a?(Array)
            Array.new(value).map { |entry| snapshot(entry, seen, symbols) }
          else
            value.to_a.to_h do |key, entry|
              raise TypeError, "JSON keys must be strings" unless key.is_a?(String) || (symbols && key.is_a?(Symbol))
              [key.to_s, snapshot(entry, seen, symbols)]
            end
          end
        ensure
          seen.delete(value.object_id)
        end
      else raise TypeError, "expected a JSON value"
      end
    end

    def self.context_dump(value, context)
      dump(value, symbols: true).encode(Encoding::UTF_8)
    rescue StandardError, SystemStackError
      checked { |error| public_send("thinkthen_#{context}_conversion_error", error) }
    end

    def self.records(values)
      values = Array.new(values)
      memory = FFI::MemoryPointer.new(RecordInput.size, [values.length, 1].max)
      # @type var buffers: Array[untyped]
      buffers = Array.new
      values.each_with_index do |value, index|
        record = RecordInput.new(memory + index * RecordInput.size)
        record[:struct_size] = RecordInput.size
        kind, bytes = record_value(value)
        record[:kind] = NativeContract::RECORD_NAMES.key(kind)
        buffer = bytes && FFI::MemoryPointer.from_string(bytes)
        record[:data] = buffer || FFI::Pointer::NULL
        record[:len] = bytes ? bytes.bytesize : 0
        buffers << buffer
      end
      yield memory, values.length
    ensure
      buffers&.each { |buffer| buffer&.free }
      memory&.free
    end

    def self.record_value(value)
      return ["missing", nil] if value.nil?
      bytes = value.is_a?(String) ? value.encode(Encoding::UTF_8) : dump(value).encode(Encoding::UTF_8)
      [value.is_a?(String) ? "text" : "json", bytes]
    rescue StandardError, SystemStackError
      ["invalid", nil]
    end

    def self.parse(value)
      JSON.parse(value, max_nesting: false)
    end

    def self.copy(buffer)
      buffer[:data].read_string_length(buffer[:len]).force_encoding(Encoding::UTF_8)
    end

    def self.error_symbol(value)
      unless value == "unknown" || NativeContract::CODE_NAMES.value?(value)
        raise UsageError.new(NativeContract::ABI_MESSAGE, code: NativeContract::ABI_CODE)
      end
      # The generated registry guard supplies the literal union at this boundary.
      # @type var result: untyped
      result = value.to_sym
      result
    end

    def self.discriminator(mapping, value)
      mapping.fetch(value) { raise UsageError.new(NativeContract::ABI_MESSAGE, code: NativeContract::ABI_CODE) }
    end

    def self.checked
      error = NativeError.new
      result = yield error
      if result == -1
        code = thinkthen_code_name(error[:code])
        unless code && NativeContract::CODE_NAMES.key?(error[:code])
          raise UnknownCodeError.new(copy(error[:message]), raw_code: error[:code], raw_name: code, http_status: error[:http_status].zero? ? nil : error[:http_status])
        end
        klass = case code
                when "local.cancelled" then Cancelled
                when /^usage\./ then UsageError
                when /^backend\./ then BackendError
                when /^internal\./ then InternalError
                else LocalError
                end
        raise klass.new(copy(error[:message]), code: error_symbol(code), http_status: error[:http_status].zero? ? nil : error[:http_status])
      end
      result
    ensure
      thinkthen_error_free(error) if error
    end

    def self.bytes(value)
      text = value.encode(Encoding::UTF_8)
      memory = FFI::MemoryPointer.from_string(text)
      yield memory, text.bytesize
    ensure
      memory&.free
    end
  end
end
