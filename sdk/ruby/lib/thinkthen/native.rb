# frozen_string_literal: true

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
    ffi_lib File.expand_path(library, __dir__)

    class Buffer < FFI::Struct
      layout :data, :pointer, :len, :size_t
    end

    class Row < FFI::Struct
      layout :fields, Buffer, :detail, Buffer, :position, :size_t, :kept, :uint8
    end

    attach_function :thinkthen_client_open, [:pointer, :size_t, :pointer, :pointer], :int32, blocking: true
    attach_function :thinkthen_client_free, [:pointer], :void
    attach_function :thinkthen_call_open,
                    [:pointer, :pointer, :size_t, :uint32, :pointer, :pointer], :int32, blocking: true
    attach_function :thinkthen_send_text, [:pointer, :pointer, :size_t, :pointer], :int32, blocking: true
    attach_function :thinkthen_send_json, [:pointer, :pointer, :size_t, :pointer], :int32, blocking: true
    attach_function :thinkthen_end_input, [:pointer], :void
    attach_function :thinkthen_next, [:pointer, :pointer, :pointer], :int32, blocking: true
    attach_function :thinkthen_cancel, [:pointer], :void
    attach_function :thinkthen_free, [:pointer], :void
    attach_function :thinkthen_buffer_free, [:pointer], :void
    attach_function :thinkthen_row_free, [:pointer], :void

    class ClientPointer < FFI::AutoPointer
      def self.release(pointer)
        Native.thinkthen_client_free(pointer)
      end
    end

    class CallPointer < FFI::AutoPointer
      def self.release(pointer)
        Native.thinkthen_free(pointer)
      end
    end

    def self.dump(value)
      JSON.generate(value, max_nesting: false)
    end

    def self.parse(value)
      JSON.parse(value, max_nesting: false, create_additions: false)
    end

    def self.copy(buffer)
      buffer[:data].read_string_length(buffer[:len]).force_encoding(Encoding::UTF_8)
    end

    def self.checked
      error = Buffer.new
      result = yield error
      if result == -1
        value = parse(copy(error))
        raise Error.new(value.fetch("code"), value.fetch("message"))
      end
      result
    ensure
      thinkthen_buffer_free(error) if error
    end

    def self.bytes(value)
      text = value.encode(Encoding::UTF_8)
      memory = FFI::MemoryPointer.from_string(text)
      yield memory, text.bytesize
    end
  end
end
