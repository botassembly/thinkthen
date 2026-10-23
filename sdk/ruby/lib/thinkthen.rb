# frozen_string_literal: true

require "json"
require "ffi"
require_relative "thinkthen/native"
require_relative "thinkthen/row"
require_relative "thinkthen/rows"
require_relative "thinkthen/client"

module ThinkThen
  class Error < StandardError
    attr_reader :code

    def initialize(code, message)
      @code = code
      super(message)
    end
  end

  class Cancel
    def initialize
      @mutex = Mutex.new
      @cancelled = false
      @calls = []
    end

    def cancel
      @mutex.synchronize do
        @cancelled = true
        @calls.each(&:cancel)
      end
      nil
    end

    def attach(owner)
      @mutex.synchronize do
        @calls << owner
        owner.cancel if @cancelled
      end
    end

    def detach(owner)
      @mutex.synchronize { @calls.delete(owner) }
    end
  end

  class Question
    attr_reader :tagged

    def initialize(tagged)
      @tagged = tagged
      freeze
    end
  end

  def self.toolbox(value, name: nil)
    Question.new({type: "toolbox", value: value, name: name})
  end

  def self.reference(name)
    Question.new({type: "reference", name: name})
  end

  private_constant :Native
end
