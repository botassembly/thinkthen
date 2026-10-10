# frozen_string_literal: true
require "json"
require_relative "thinkthen/version"
module ThinkThen
  class Error < StandardError
    attr_reader :kind, :retryable, :facts, :complete, :results, :terminal
    def initialize(message = nil, kind = nil, retryable = false)
      super(message)
      @kind, @retryable = kind, retryable
    end
    def code = {"usage" => 1, "backend" => 2, "deadline" => 3, "local" => 4, "cancelled" => 5, "defect" => 6}.fetch(kind, 6)
  end
  class UsageError < Error; end
  class BackendError < Error; end
  class DeadlineError < Error; end
  class LocalError < Error; end
  class CancelledError < Error; end
  class DefectError < Error; end
end
require_relative "thinkthen/thinkthen"
require_relative "thinkthen/session"
module ThinkThen
  private_constant :Native
end
