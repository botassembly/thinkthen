# frozen_string_literal: true

module ThinkThen
  class Client
    def self.open(**settings)
      client = new(**settings)
      begin
        yield client
      ensure
        client.close
      end
    end
    def initialize(backend: nil, model: nil, url: nil, api_key: nil, batch: nil, retry: nil)
      @mutex, @streams = Mutex.new, []
      ThinkThen.send(:native)
      key = api_key.nil? || api_key.is_a?(RedactedKey) ? api_key : RedactedKey.new(api_key)
      @api_key = key
      policy = binding.local_variable_get(:retry)
      raise TypeError, "retry must be a Retry" unless policy.nil? || policy.is_a?(Retry)
      raise TypeError, "backend must be a Symbol" unless backend.nil? || backend.is_a?(Symbol)
      raw_settings = {backend: backend, model: model, url: url, batch: batch, retry: policy&.to_h}.reject { |_, value| value.nil? }
      output = FFI::MemoryPointer.new(:pointer)
      Native.bytes(Native.context_dump(raw_settings, :settings)) do |bytes, length|
        Native.checked { |error| Native.thinkthen_settings_from_json(bytes, length, output, error) }
      end
      settings = Native::SettingsPointer.new(output.read_pointer)
      key&.send(:copy_to, settings)
      Thread.handle_interrupt(Exception => :never) do
        output = FFI::MemoryPointer.new(:pointer)
        Native.checked { |error| Native.thinkthen_client_open(settings, output, error) }
        @pointer = Native::ClientPointer.new(output.read_pointer)
      end
    ensure
      settings&.free
    end
    attr_reader :api_key
    def inspect = "#<ThinkThen::Client closed=#{closed?}>"
    alias to_s inspect
    def close
      streams = @mutex.synchronize { @closed = true; @streams.dup }
      streams.each(&:close)
      Thread.handle_interrupt(Exception => :never) do
        @mutex.synchronize { @pointer&.free; @pointer = nil }
      end
      nil
    end
    def closed? = @mutex.synchronize { @closed || !@pointer }
    def decide(question, record, threshold: UNSET, evidence: UNSET, name: nil, cancel: nil)
      question = build_question(:decide, question, {threshold: threshold, evidence: evidence})
      stream = open_rows(question, [record], name, cancel)
      begin
        stream.next
      ensure
        stream.close
      end
    end
    def decide_each(question, records, threshold: UNSET, evidence: UNSET, name: nil, cancel: nil, &block)
      question = build_question(:decide, question, {threshold: threshold, evidence: evidence})
      stream = open_rows(question, records, name, cancel)
      return stream unless block
      stream.each(&block)
      nil
    end
    def choose(question, record, choices: UNSET, threshold: UNSET, evidence: UNSET, name: nil, cancel: nil)
      question = build_question(:choose, question, {choices: choices, threshold: threshold, evidence: evidence})
      stream = open_rows(question, [record], name, cancel)
      begin
        stream.next
      ensure
        stream.close
      end
    end
    def choose_each(question, records, choices: UNSET, threshold: UNSET, evidence: UNSET, name: nil, cancel: nil, &block)
      question = build_question(:choose, question, {choices: choices, threshold: threshold, evidence: evidence})
      stream = open_rows(question, records, name, cancel)
      return stream unless block
      stream.each(&block)
      nil
    end
    def tag(question, record, labels: UNSET, threshold: UNSET, evidence: UNSET, name: nil, cancel: nil)
      question = build_question(:tag, question, {labels: labels, threshold: threshold, evidence: evidence})
      stream = open_rows(question, [record], name, cancel)
      begin
        stream.next
      ensure
        stream.close
      end
    end
    def tag_each(question, records, labels: UNSET, threshold: UNSET, evidence: UNSET, name: nil, cancel: nil, &block)
      question = build_question(:tag, question, {labels: labels, threshold: threshold, evidence: evidence})
      stream = open_rows(question, records, name, cancel)
      return stream unless block
      stream.each(&block)
      nil
    end
    def score(question, record, levels: UNSET, evidence: UNSET, name: nil, cancel: nil)
      question = build_question(:score, question, {levels: levels, evidence: evidence})
      stream = open_rows(question, [record], name, cancel)
      begin
        stream.next
      ensure
        stream.close
      end
    end
    def score_each(question, records, levels: UNSET, evidence: UNSET, name: nil, cancel: nil, &block)
      question = build_question(:score, question, {levels: levels, evidence: evidence})
      stream = open_rows(question, records, name, cancel)
      return stream unless block
      stream.each(&block)
      nil
    end
    def filter(question, record, threshold: UNSET, evidence: UNSET, name: nil, cancel: nil)
      question = build_question(:filter, question, {threshold: threshold, evidence: evidence})
      stream = open_rows(question, [record], name, cancel)
      begin
        stream.next
      ensure
        stream.close
      end
    end
    def filter_each(question, records, threshold: UNSET, evidence: UNSET, name: nil, cancel: nil, &block)
      question = build_question(:filter, question, {threshold: threshold, evidence: evidence})
      stream = open_rows(question, records, name, cancel)
      return stream unless block
      stream.each(&block)
      nil
    end
    def rank(question, record, levels: UNSET, top: UNSET, evidence: UNSET, name: nil, cancel: nil)
      question = build_question(:rank, question, {levels: levels, top: top, evidence: evidence})
      stream = open_rows(question, [record], name, cancel)
      begin
        stream.next
      ensure
        stream.close
      end
    end
    def rank_each(question, records, levels: UNSET, top: UNSET, evidence: UNSET, name: nil, cancel: nil, &block)
      question = build_question(:rank, question, {levels: levels, top: top, evidence: evidence})
      stream = open_rows(question, records, name, cancel)
      return stream unless block
      stream.each(&block)
      nil
    end
    def find(question, record, threshold: UNSET, evidence: UNSET, name: nil, cancel: nil)
      question = build_question(:find, question, {threshold: threshold, evidence: evidence})
      stream = open_rows(question, [record], name, cancel)
      begin
        stream.next
      ensure
        stream.close
      end
    end
    def find_each(question, records, threshold: UNSET, evidence: UNSET, name: nil, cancel: nil, &block)
      question = build_question(:find, question, {threshold: threshold, evidence: evidence})
      stream = open_rows(question, records, name, cancel)
      return stream unless block
      stream.each(&block)
      nil
    end
    def recognize(question_or_record, record = UNSET, kinds: UNSET, threshold: UNSET, evidence: UNSET, name: nil, cancel: nil)
      input = record.equal?(UNSET) ? [question_or_record] : [record]
      question = record.equal?(UNSET) ? nil : question_or_record
      question = build_question(:recognize, question, {kinds: kinds, threshold: threshold, evidence: evidence})
      stream = open_rows(question, input, name, cancel)
      begin
        stream.next
      ensure
        stream.close
      end
    end
    def recognize_each(question_or_records, records = UNSET, kinds: UNSET, threshold: UNSET, evidence: UNSET, name: nil, cancel: nil, &block)
      question = UNSET.equal?(records) ? nil : question_or_records
      records = question_or_records if UNSET.equal?(records)
      question = build_question(:recognize, question, {kinds: kinds, threshold: threshold, evidence: evidence})
      stream = open_rows(question, records, name, cancel)
      return stream unless block
      stream.each(&block)
      nil
    end
    def relate(question_or_record, record = UNSET, rules: UNSET, threshold: UNSET, evidence: UNSET, name: nil, cancel: nil)
      input = record.equal?(UNSET) ? [question_or_record] : [record]
      question = record.equal?(UNSET) ? nil : question_or_record
      question = build_question(:relate, question, {rules: rules, threshold: threshold, evidence: evidence})
      stream = open_rows(question, input, name, cancel)
      begin
        stream.next
      ensure
        stream.close
      end
    end
    def relate_each(question_or_records, records = UNSET, rules: UNSET, threshold: UNSET, evidence: UNSET, name: nil, cancel: nil, &block)
      question = UNSET.equal?(records) ? nil : question_or_records
      records = question_or_records if UNSET.equal?(records)
      question = build_question(:relate, question, {rules: rules, threshold: threshold, evidence: evidence})
      stream = open_rows(question, records, name, cancel)
      return stream unless block
      stream.each(&block)
      nil
    end
    def annotate(question, record, evidence: UNSET, name: nil, cancel: nil)
      question = build_question(:annotate, question, {evidence: evidence})
      stream = open_rows(question, [record], name, cancel)
      begin
        stream.next
      ensure
        stream.close
      end
    end
    def annotate_each(question, records, evidence: UNSET, name: nil, cancel: nil, &block)
      question = build_question(:annotate, question, {evidence: evidence})
      stream = open_rows(question, records, name, cancel)
      return stream unless block
      stream.each(&block)
      nil
    end
    private
    def build_question(function, question, options)
      # @type var klass: untyped
      klass = ThinkThen.const_get("#{function.to_s.capitalize}Question")
      # @type var options: Hash[Symbol, untyped]
      options = options.reject { |_, value| value.equal?(UNSET) }
      if question.is_a?(Question)
        raise TypeError, "question does not match function" unless question.is_a?(klass)
        raise ArgumentError, "saved question owns its options" unless options.empty?
        question
      elsif [:recognize, :relate].include?(function)
        klass.new(**options)
      else
        klass.new(question, **options)
      end
    end
    def open_rows(question, records, name, cancel)
      raise TypeError, "name must be a String" unless name.nil? || name.is_a?(String)
      raise TypeError, "cancel must be a CancelToken" unless cancel.nil? || cancel.is_a?(CancelToken)
      request = Native.context_dump({question: question.value, **(name.nil? ? {} : {name: name})}, :question)
      stream = Rows.new(self, request, records, cancel, question.value.fetch("function") == "filter")
      @mutex.synchronize do
        raise IOError, "client is closed" if @closed || !@pointer
        @streams << stream
      end
      stream
    rescue Exception
      stream&.close
      raise
    end
    def admit(request, records = nil)
      @mutex.synchronize do
        raise IOError, "client is closed" if @closed || !@pointer
        Native.bytes(request) do |bytes, length|
          output = FFI::MemoryPointer.new(:pointer)
          Native.checked do |error|
            Native.thinkthen_call_open_json(@pointer, bytes, length, NativeContract::OFFSET_NAMES.key("code_points"), output, error)
          end
          if records
            Native.thinkthen_call_free(output.read_pointer)
            output.write_pointer(FFI::Pointer::NULL)
            Native.records(records) do |input, count|
              Native.checked do |error|
                Native.thinkthen_call_open_records_json(@pointer, bytes, length, input, count, Native::RecordInput.size,
                                                        NativeContract::OFFSET_NAMES.key("code_points"), output, error)
              end
            end
          end
          output.read_pointer
        end
      end
    end
    def forget(stream) = @mutex.synchronize { @streams.delete(stream) }
  end
end
