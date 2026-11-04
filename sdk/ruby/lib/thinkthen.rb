# frozen_string_literal: true

require "json"
require "ffi"
require "weakref"
require_relative "thinkthen/version"
require_relative "thinkthen/contract"

module ThinkThen
  UNSET = Object.new.freeze
  private_constant :UNSET
  # Rescue every engine and binding error through this shared family.
  module Error
    attr_reader :code, :http_status
    def initialize(message = nil, code:, http_status: nil)
      @code, @http_status = code.to_sym, http_status
      super(message)
    end
  end
  class LibraryError < StandardError
    include Error
  end
  private_constant :LibraryError
  class UnknownCodeError < LibraryError
    attr_reader :raw_code, :raw_name
    def initialize(message = nil, raw_code:, raw_name: nil, http_status: nil)
      @raw_code, @raw_name = raw_code, raw_name&.dup&.freeze
      super(message, code: :unknown, http_status: http_status)
    end
  end
  # An invalid call; also an ArgumentError for normal Ruby argument handling.
  class UsageError < ArgumentError
    include Error
  end
  # The selected backend could not complete the call.
  class BackendError < LibraryError; end
  # A cancelled native call, including a Ruby interrupt during native work.
  class CancelledError < LibraryError; end
  # A local runtime or IO failure.
  class LocalError < LibraryError; end
  # An unexpected native engine failure.
  class InternalError < LibraryError; end

  # A probability band did not select a definite answer.
  class UnsureError < StandardError
    attr_reader :probability
    def initialize(probability)
      @probability = probability
      super("the answer is not sure")
    end
  end
  # Failed input rows, retained in input order after the kept records.
  class RecordError < LibraryError
    attr_reader :rows
    def initialize(message = nil, code:, http_status: nil, rows: [])
      @rows = rows.freeze
      super(message, code: code, http_status: http_status)
    end
  end

  def self.native
    raise @native_error if @native_error
    require_relative "thinkthen/native"
    private_constant :Native
    Native
  rescue UsageError => error
    @native_error = error
    raise
  end
  private_class_method :native

  # Cancel any attached calls without closing their client.
  class CancelToken
    def initialize
      @mutex, @cancelled, @calls = Mutex.new, false, []
    end
    def cancel
      @mutex.synchronize do
        @cancelled = true
        @calls.each { |reference| reference.__getobj__.cancel if reference.weakref_alive? }
      end
      nil
    end
    def cancelled? = @mutex.synchronize { @cancelled }
    def attach(owner)
      @mutex.synchronize do
        @calls.reject! { |reference| !reference.weakref_alive? }
        @calls << WeakRef.new(owner)
        owner.cancel if @cancelled
      end
    end
    def detach(owner) = @mutex.synchronize { @calls.reject! { |reference| !reference.weakref_alive? || reference.__getobj__.equal?(owner) } }
  end
end
require_relative "thinkthen/values"
require_relative "thinkthen/questions"
require_relative "thinkthen/row"
require_relative "thinkthen/call_owner"
require_relative "thinkthen/native_reader"
require_relative "thinkthen/rows"
require_relative "thinkthen/client"
require_relative "thinkthen/configuration"
