# frozen_string_literal: true
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
      def initialize(results, terminal, scalar, verb, observations)
        @results, @terminal, @scalar, @verb, @observations = results.freeze, terminal, scalar, verb, observations.freeze
        freeze
      end
      def facts = terminal.facts
      def value
        spec = CALL_RESULTS.fetch(@verb)
        values = results.map { |row| row.public_send(spec.fetch("value_projection")) }
        project(values, spec)
      end
      def probability
        spec = CALL_RESULTS.fetch(@verb)
        mode = spec.fetch("probability_mode")
        return nil if mode == "null"
        values = results.map do |row|
          case mode
          when "yes" then read_path(row, spec.fetch("probability_path"))
          when "picked"
            read_path(row, spec.fetch("probability_path"))[read_path(row, spec.fetch("pick_path"))]
          when "items" then item_probabilities(row)
          end
        end
        project(values, spec)
      end
      private def project(values, spec)
        return values.freeze if spec.fetch("collection")
        @scalar ? values.first : values.freeze
      end
      private def read_path(row, path)
        path.reduce(row) { |value, member| value.public_send(member) }
      end
      private def item_probabilities(row)
        return row.answer.probability if row.answer.respond_to?(:probability)
        return row.value.map(&:probability).freeze if row.value.is_a?(Array)
        return row.value.proposals.map(&:probability).freeze if row.value.key?("proposals")
        proposals = row.answer.proposals.select(&:kept).to_h do |proposal|
          [[proposal.selected.start, proposal.selected.end, proposal.kind], proposal.span_probability]
        end
        row.value.entities.map { |entity| proposals.fetch([entity.start, entity.end, entity.kind]) }.freeze
      end
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
    def self.item(value, **fields)
      Item.send(:new, {original: original(value), **fields})
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
    def plan(verb, question, input, **options)
      raise UsageError.new("client is closed", "usage") if @closed
      reject_details(options)
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
      reject_details(options)
      operation = Operation.new(@native, verb, question, input, cancel, options) { |o| @operations.delete(o) }
      @operations << operation
      return operation unless block_given?
      begin
        yield operation
      ensure
        Client.send(:cleanup, operation)
      end
    end
    private def reject_details(options)
      raise UsageError.new("details is not a call option", "usage") if options.key?(:details)
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
        @remove, @token, @verb = remove, token, verb
        @rows = []
        @observations = []
        @closed = false
        if input.is_a?(Enumerable) && !input.is_a?(Array) && !input.is_a?(Hash) && !input.is_a?(Source) && !input.is_a?(Item)
          @producer = input.to_enum
          @owner = input
        end
        @scalar = !Results::CALL_RESULTS.fetch(verb).fetch("collection") && (verb == "find" || verb == "relate" || !(input.is_a?(Array) || input.is_a?(Source) || @producer))
        if token&.cancelled?
          raise CancelledError.new("the call was cancelled", "cancelled")
        end
        @session = engine.request_session(verb, question, input, [options, !@producer.nil?])
      rescue Exception => error
        begin
          close_producer
        rescue Exception
          # Cleanup cannot replace the failed admission.
        end
        raise error
      end
      def result
        loop do
          raise cancelled if @closed
          raise cancelled if @token&.cancelled?
          packet = @session.poll
          if packet
            return Results::Completed.new(@rows, @terminal, @scalar, @verb, @observations) if packet == "end"
            case packet.kind
            when "row" then @rows << packet.value
            when "observation" then @observations << packet
            when "aggregate"
              values = packet.value.is_a?(Array) ? packet.value : [packet.value]
              @rows.concat(values)
            when "terminal"
              @terminal = packet
              if packet.key?("failure")
                failure = packet.failure
                detail = failure.error
                error = ThinkThen.const_get(detail.kind.capitalize + "Error").new(detail.message, detail.kind, detail.retryable)
                error.instance_variable_set(:@facts, failure.key?("facts") ? failure.facts : nil)
                error.instance_variable_set(:@complete, failure)
                error.instance_variable_set(:@results, @rows.freeze)
                error.instance_variable_set(:@terminal, packet)
                error.instance_variable_set(:@observations, @observations.freeze)
                raise error, cause: @producer_error
              end
              return Results::Completed.new(@rows, packet, @scalar, @verb, @observations)
            end
          end
          feed if @producer
          input = !@producer.nil?
          if Fiber.scheduler && !Fiber.current.blocking?
            # Thread#value lets the fiber scheduler run while native wait releases the GVL.
            Thread.new { @session.wait(input) }.value
          else
            @session.wait(input)
          end
        end
      ensure
        Client.send(:cleanup, self)
      end
      private def cancelled
        error = CancelledError.new("the call was cancelled", "cancelled")
        error.instance_variable_set(:@results, @rows.dup.freeze)
        error.instance_variable_set(:@terminal, @terminal)
        error.instance_variable_set(:@facts, @terminal&.facts)
        error
      end
      private def feed
        unless @pending
          begin
            @pending = @session.item(Client.send(:descriptor, @producer.next))
          rescue StopIteration
            @session.finish(nil)
            close_producer
            return
          rescue StandardError => error
            @producer_error = error
            @session.finish(true)
            close_producer
            return
          end
        end
        case @session.push(@pending)
        when "accepted" then @pending = nil
        when "closed" then @pending = nil; close_producer
        end
      end
      def cancel
        @session&.cancel
        close
      end
      def close
        return if @closed
        @closed = true
        begin
          close_producer
        ensure
          @session&.close
          @remove&.call(self)
        end
      end
      private def close_producer
        owner, @owner, @producer = @owner, nil, nil
        owner.close if owner.respond_to?(:close)
      rescue StandardError
        raise unless @producer_error
      end
      def inspect = "<ThinkThen::Client::Operation>"
    end
  end
end
