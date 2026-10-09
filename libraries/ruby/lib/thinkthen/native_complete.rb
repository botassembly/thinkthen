# frozen_string_literal: true
module ThinkThen
  module Complete
    class Functions
      def initialize(engine)
        @engine = engine
      end
      def inspect = '<CompleteFunctions>'
      %w[decide choose tag score filter rank find annotate recognize relate].each do |verb|
        define_method(verb) do |question, input, attempts: false, cancel: nil, deadline_ms: nil, context: nil|
          request = JSON.generate({verb: verb, question: Complete.to_json_value(question), input: Complete.to_json_value(input), attempts: attempts, context:context})
          begin
          call = @engine.__send__(:crossing, 'complete', nil, request, cancel, deadline_ms)
          rescue ThinkThen::Error => error
            snapshot=error.instance_variable_get(:@native_complete)
            error.instance_variable_set(:@complete,Complete.decode('CallError',JSON.parse(snapshot),preserve_unknown: true)) if snapshot
            raise
          end
          held = JSON.parse(call.value)
          rows = held['results'].is_a?(Array) ? held['results'] : [held['results']]
          Completed.new(results: rows.map { |row| Complete.decode(verb.capitalize + 'Result', row,preserve_unknown: true) }.freeze,
                        facts: Complete.decode('Facts', held['facts'],preserve_unknown: true), ordinals: held['ordinals'].freeze, inputs: held['inputs'].map { |v| Complete.decode('NativeInput',v,preserve_unknown: true) }.freeze).freeze
        end
      end
      %w[decide choose tag score filter annotate].each do |verb|
        define_method(verb+'_batch') do |question,input,attempts:false,cancel:nil,deadline_ms:nil,context:nil|
          native=@engine.instance_variable_get(:@native).complete_batch(JSON.generate(verb:verb,question:Complete.to_json_value(question),input:Complete.to_json_value(input),attempts:attempts,context:context),deadline_ms,cancel)
          Batch.new(native,verb.capitalize+'Result')
        end
      end

    end
    Completed = Struct.new(:results, :facts, :ordinals, :inputs, keyword_init: true) do
      def inspect = '<Completed: content withheld>'
    end
  end
  class Engine
    def complete = Complete::Functions.new(self)
  end
end

module ThinkThen
  class Error
    attr_reader :complete
  end
  module Complete
    BatchRow=Struct.new(:result,:ordinal,:input,keyword_init:true) do
      def inspect = '<BatchRow: content withheld>'
    end
    class Batch
      include Enumerable
      attr_reader :facts
      def initialize(native,kind)
        @native=native;@kind=kind;@ended=false;@facts=nil
      end
      def inspect = '<CompleteBatch>'
      def each
        return enum_for(:each) unless block_given?
        loop do
          row=self.next
          break if row.nil?
          yield row
        end
        self
      ensure
        close
      end
      def next
        return nil if @ended
        event=JSON.parse(@native.pull)
        if event['row']
          return BatchRow.new(result:Complete.decode(@kind,event['row'],preserve_unknown: true),ordinal:event['ordinal'],input:Complete.decode('NativeInput',event['input'],preserve_unknown: true)).freeze
        end
        close
        if event['error']
          error=event['error'];complete=Complete.decode('CallError',error,preserve_unknown: true)
          @facts=complete.facts unless complete.facts.equal?(ABSENT)
          raised=ThinkThen.const_get(error['kind'].capitalize+'Error').new(error['message'],error['kind'],error['retryable'])
          raised.instance_variable_set(:@complete,complete)
          raise raised
        end
        @facts=Complete.decode('Facts',event['facts'],preserve_unknown: true);nil
      end
      def close
        @ended=true;@native.close
      end
      def cancel = @native.cancel
    end
  end
end
