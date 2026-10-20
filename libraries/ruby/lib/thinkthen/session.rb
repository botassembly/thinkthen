# frozen_string_literal: true
require_relative "native_result"
require_relative "results_generated"
module ThinkThen
  Plan = Results::NativePlan
  UsageStatus = Struct.new(:state, :advice) do
    def initialize(state, advice)
      super(state.freeze, advice&.freeze)
      freeze
    end
  end
  module Results
    # A native failure never becomes a value. Read .value to branch on decisions:
    # Ruby treats every object as true, including a result whose value is false.
    class Completed
      attr_reader :results, :terminal, :observations
      def initialize(tree, verb)
        @results = Results.field(Results::CALL_TYPES.fetch(verb).fetch("results"), tree.fetch("results"))
        @results = [@results].freeze unless @results.is_a?(Array)
        @facts = Results.convert("completeFacts", tree.fetch("facts"))
        @value = Results.field(Results::CALL_TYPES.fetch(verb).fetch("value"), tree["value"])
        @probability = tree["probability"]
        @observations = (tree["observations"] || []).map { |packet| Results.convert("completesessionPacket", packet) }.freeze
        @terminal = Results.convert("completesessionPacket", {"kind" => "terminal", "facts" => tree.fetch("facts")}.freeze)
        freeze
      end
      attr_reader :value, :probability, :facts
      def inspect = "<ThinkThen::Results::Completed: content withheld>"
    end
  end

  # One family of named calls over the owned native engine.
  class Client
    FUNCTIONS = %w[decide choose tag score filter rank find annotate recognize relate].freeze
    class Carrier
      private attr_reader :payload
      def initialize(payload)
        @payload = payload
        freeze
      end
      private_class_method :new, :allocate
      def inspect = "<#{self.class.name}: content withheld>"
    end
    private_constant :Carrier
    Source = Class.new(Carrier) { alias_method :source, :payload }
    Question = Class.new(Carrier) { alias_method :selector, :payload }
    Item = Class.new(Carrier) { alias_method :descriptor, :payload }
    def self.open(**settings)
      client = new(**settings)
      return client unless block_given?
      begin
        yield client
      ensure
        cleanup(client)
      end
    end
    private_class_method def self.cleanup(owner)
      active = $!
      owner.close
    rescue Exception
      raise unless active
    end
    def initialize(**settings)
      @native = Native.request_engine(Client.send(:dump, settings))
      @operations = []
      @closed = false
    rescue JSON::GeneratorError, TypeError
      raise UsageError.new("settings cannot be converted to native JSON", "usage")
    end
    %w[file name reference].each do |kind|
      field = kind == "file" ? :path : kind.to_sym
      define_singleton_method("question_#{kind}") do |value|
        Question.send(:new, {kind: kind, field => value})
      end
    end
    def self.files(paths, framing: nil, media: "text", **reading)
      source = {paths: Array(paths), reading: reading, media: media}
      source[:framing] = framing unless framing.nil?
      Source.send(:new, source)
    end
    ITEM_UNSET = Object.new.freeze
    private_constant :ITEM_UNSET
    def self.item(value = ITEM_UNSET, **fields)
      Item.send(:new, value.equal?(ITEM_UNSET) ? fields : {original: original(value), **fields})
    end
    def self.dump(value)
      JSON.generate(value)
    rescue JSON::GeneratorError, TypeError, EncodingError
      raise UsageError.new("input cannot be converted to native JSON", "usage")
    end
    private_class_method :dump
    private_class_method def self.original(value)
      value.is_a?(String) ? {kind: "text", text: value} : {kind: "json", value: value}
    end
    private_class_method def self.descriptor(value)
      value.is_a?(Item) ? value.send(:descriptor) : {original: original(value)}
    end
    private_class_method def self.producer?(input)
      input.is_a?(Enumerable) && !input.is_a?(Array) && !input.is_a?(Hash) && !input.is_a?(Source) && !input.is_a?(Item)
    end
    private_class_method def self.components(question, input, options)
      selector = if question.is_a?(Question)
        question.send(:selector)
      elsif question.is_a?(String)
        {kind: "text", text: question}
      else
        {kind: "definition", value: question}
      end
      evidence = if producer?(input)
        {kind: "feed", name: "ruby"}
      elsif input.is_a?(Source)
        {kind: "source", source: input.send(:source)}
      else
        {kind: "records", items: (input.is_a?(Array) ? input : [input]).map { |value| descriptor(value) }}
      end
      [dump(selector), dump(evidence), dump(options)]
    end
    def plan(verb, question, input, **options)
      raise UsageError.new("client is closed", "usage") if @closed
      @native.plan(verb, question, input, [options, false])
    end
    FUNCTIONS.each do |verb|
      define_method(verb) do |question, input, cancel: nil, **options|
        operation = start(verb, question, input, cancel: cancel, **options)
        begin
          operation.result
        ensure
          Client.send(:cleanup, operation)
        end
      end
    end
    def start(verb, question, input, cancel: nil, **options)
      raise UsageError.new("client is closed", "usage") if @closed
      operation = Operation.new(@native, verb, question, input, cancel, options) { |o| @operations.delete(o) }
      @operations << operation
      return operation unless block_given?
      begin
        yield operation
      ensure
        Client.send(:cleanup, operation)
      end
    end
    def close
      @closed = true
      errors = []
      @operations.dup.each do |operation|
        begin
          operation.close
        rescue Exception => error
          errors << error
        end
      end
      raise errors.first unless errors.empty?
      nil
    end
    def inspect = "<ThinkThen::Client>"
    def usage = @native.usage.transform_keys(&:to_sym)
    def usage_persistence
      raise UsageError.new("client is closed", "usage") if @closed
      UsageStatus.new(*@native.usage_persistence)
    end
    def finish_usage_status
      raise UsageError.new("client is closed", "usage") if @closed
      UsageStatus.new(*@native.finish_usage_status)
    end

    class Operation
      attr_reader :terminal
      def initialize(engine, verb, question, input, token, options, &remove)
        @engine, @verb, @question, @input, @options = engine, verb, question, input, options
        @remove, @token = remove, token || Cancel.new
      end
      def result
        work = -> { Results::Completed.new(@engine.call(@verb, [@question, @input, @options], @token, producer? ? self : nil), @verb) }
        answer = Fiber.scheduler && !Fiber.current.blocking? ? Thread.new(&work).value : work.call
        @terminal = answer.terminal
        answer
      ensure
        Client.send(:cleanup, self)
      end
      def cancel = @token.cancel
      def close
        cancel
        @input.close if producer? && @input.respond_to?(:close)
      ensure
        @remove&.call(self)
        @remove = nil
      end
      def native_next
        @enumerator ||= @input.to_enum
        Client.send(:dump, Client.send(:descriptor, @enumerator.next))
      rescue StopIteration
        nil
      end
      private def producer? = Client.send(:producer?, @input)
      def inspect = "<ThinkThen::Client::Operation>"
    end
  end
end
