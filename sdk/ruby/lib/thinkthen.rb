# frozen_string_literal: true

require "json"
require "ffi"
require "weakref"
require_relative "thinkthen/version"
require_relative "thinkthen/contract"

module ThinkThen
  UNSET = Object.new.freeze
  private_constant :UNSET
  class Error < StandardError
    attr_reader :code, :http_status
    def initialize(message = nil, code:, http_status: nil)
      @code, @http_status = code.to_sym, http_status
      super(message)
    end
  end
  class UnknownCodeError < Error
    attr_reader :raw_code, :raw_name
    def initialize(message = nil, raw_code:, raw_name: nil, http_status: nil)
      @raw_code, @raw_name = raw_code, raw_name&.dup&.freeze
      super(message, code: :unknown, http_status: http_status)
    end
  end
  class UsageError < Error; end
  class BackendError < Error; end
  class Cancelled < Error; end
  class LocalError < Error; end
  class InternalError < Error; end

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
