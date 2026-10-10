# frozen_string_literal: true
require_relative "results_generated"
module ThinkThen
  module Results
    # A native failure never becomes a value. Read .value to branch on decisions:
    # Ruby treats every object as true, including a result whose value is false.
    class Completed
      attr_reader :results, :terminal
      def initialize(results, terminal, scalar, verb)
        @results, @terminal, @scalar, @verb = results.freeze, terminal, scalar, verb
        freeze
      end
      def facts = terminal.facts
      def value
        return results.map(&:input).freeze if @verb == "filter"
        return results if @verb == "rank"
        values = results.map { |row| row.key?("value") ? row.value : row }
        @scalar && values.size == 1 ? values.first : values.freeze
      end
      def inspect = "<ThinkThen::Results::Completed: content withheld>"
    end
  end

  # One canonical family of named calls. The compatibility Engine remains
  # available until installed migration parity permits its removal.
  class Client
    FUNCTIONS = %w[decide choose tag score filter rank find annotate recognize relate].freeze
    Source = Struct.new(:source) do
      def inspect = "<ThinkThen::Client::Source: content withheld>"
    end
    Item = Struct.new(:descriptor) do
      def inspect = "<ThinkThen::Client::Item: content withheld>"
    end
    def self.open(**settings)
      client = new(**settings)
      return client unless block_given?
      begin
        yield client
      ensure
        cleanup(client)
      end
    end
    def self.cleanup(owner)
      active = $!
      owner.close
    rescue Exception
      raise unless active
    end
    def initialize(**settings)
      @native = Native.request_engine(Client.dump(settings))
      @operations = []
      @closed = false
    rescue JSON::GeneratorError, TypeError
      raise UsageError.new("settings cannot be converted to native JSON", "usage")
    end
    def self.files(paths, **reading)
      Source.new({paths: Array(paths), reading: reading, media: "text"}).freeze
    end
    def self.item(value, **fields)
      Item.new({original: original(value), **fields}).freeze
    end
    def self.dump(value)
      JSON.generate(value)
    rescue JSON::GeneratorError, TypeError, EncodingError
      raise UsageError.new("input cannot be converted to native JSON", "usage")
    end
    def self.original(value)
      value.is_a?(String) ? {kind: "text", text: value} : {kind: "json", value: value}
    end
    def self.descriptor(value)
      value.is_a?(Item) ? value.descriptor : {original: original(value)}
    end
    FUNCTIONS.each do |verb|
      define_method(verb) do |question, input, cancel: nil, **options|
        operation = start(verb, question, input, cancel: cancel, **options)
        begin
          operation.result
        ensure
          Client.cleanup(operation)
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
        Client.cleanup(operation)
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

    class Operation
      attr_reader :terminal
      def initialize(engine, verb, question, input, token, options, &remove)
        @remove, @token, @verb = remove, token, verb
        @rows = []
        @closed = false
        @scalar = !input.is_a?(Array) && !input.is_a?(Source) && !input.is_a?(Enumerable)
        source = if input.is_a?(Source)
          {kind: "source", source: input.source}
        elsif input.is_a?(Array)
          {kind: verb == "find" ? "units" : verb == "relate" ? "entities" : "records", items: input.map { |v| Client.descriptor(v) }}
        elsif input.is_a?(Enumerable) && !input.is_a?(Hash)
          @producer = input.to_enum
          @owner = input
          {kind: "feed", name: "ruby"}
        else
          @scalar = true
          {kind: "records", items: [Client.descriptor(input)]}
        end
        asked = question.is_a?(String) ? {kind: "text", text: question} : {kind: "definition", value: question}
        request = {schema: REQUEST_VERSION, call: {function: verb, question: asked, input: source, options: options}}
        if token&.cancelled?
          raise CancelledError.new("the call was cancelled", "cancelled")
        end
        @session = engine.request_session(Client.dump(request))
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
          raise CancelledError.new("the call was cancelled", "cancelled") if @closed
          raise CancelledError.new("the call was cancelled", "cancelled") if @token&.cancelled?
          packet = @session.poll
          if packet
            return Results::Completed.new(@rows, @terminal, @scalar, @verb) if packet == "end"
            case packet.kind
            when "row" then @rows << packet.value
            when "aggregate" then @rows = packet.value.is_a?(Array) ? packet.value : [packet.value]
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
                raise error
              end
              return Results::Completed.new(@rows, packet, @scalar, @verb)
            end
          end
          feed if @producer
          sleep 0.001
        end
      ensure
        Client.cleanup(self)
      end
      def feed
        unless @pending
          begin
            @pending = Client.dump(item: Client.descriptor(@producer.next))
          rescue StopIteration
            @session.finish(nil)
            close_producer
            return
          rescue StandardError
            @session.finish(JSON.generate(kind: "invalid_input"))
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
      def close_producer
        owner, @owner, @producer = @owner, nil, nil
        owner.close if owner.respond_to?(:close)
      end
      def inspect = "<ThinkThen::Client::Operation>"
    end
  end
  class Error
    attr_reader :results, :terminal
  end
end
