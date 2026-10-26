# frozen_string_literal: true

require "json"
require "ffi"
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
      @mutex.synchronize { @cancelled = true; @calls.each(&:cancel) }
      nil
    end
    def cancelled? = @mutex.synchronize { @cancelled }
    def attach(owner)
      @mutex.synchronize { @calls << owner; owner.cancel if @cancelled }
    end
    def detach(owner) = @mutex.synchronize { @calls.delete(owner) }
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
