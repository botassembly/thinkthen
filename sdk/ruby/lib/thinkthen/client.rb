# frozen_string_literal: true

module ThinkThen
  class Client
    # Open a client. A block closes it reliably and returns the block result.
    def self.open(**settings)
      client = new(**settings)
      return client unless block_given?
      begin
        yield client
      ensure
        client.close
      end
    end
    # Open the native engine with optional backend settings and a RetryPolicy policy.
    def initialize(backend: nil, model: nil, url: nil, api_key: nil, batch: nil, max_requests: nil, retry: nil)
      @mutex, @streams = Mutex.new, []
      @pid = Process.pid
      ThinkThen.send(:native)
      key = api_key.nil? || api_key.is_a?(RedactedKey) ? api_key : RedactedKey.new(api_key)
      @api_key = key
      policy = binding.local_variable_get(:retry)
      raise UsageError.new("retry must be a RetryPolicy", code: :"usage.settings") unless policy.nil? || policy.is_a?(RetryPolicy)
      raise UsageError.new("backend must be a Symbol", code: :"usage.backend") unless backend.nil? || backend.is_a?(Symbol)
      raw_settings = {backend: backend, model: model, url: url, batch: batch, max_requests: max_requests, retry: policy&.to_h}.reject { |_, value| value.nil? }
      output = FFI::MemoryPointer.new(:pointer)
      Native.bytes(Native.context_dump(raw_settings, :settings)) do |bytes, length|
        Native.checked { |error| Native.thinkthen_settings_from_json(bytes, length, output, error) }
      end
      settings = Native::SettingsPointer.new(output.read_pointer)
      key&.send(:copy_to, settings)
      Thread.handle_interrupt(Object => :never) do
        output = FFI::MemoryPointer.new(:pointer)
        Native.checked { |error| Native.thinkthen_client_open(settings, output, error) }
        @pointer = Native::ClientPointer.new(output.read_pointer)
      end
    ensure
      settings&.free
    end
    # The redacted key wrapper; its display never reveals the key.
    attr_reader :api_key
    def inspect = "#<ThinkThen::Client closed=#{closed?}>"
    alias to_s inspect
    # Close active calls and release this client. Repeated close calls are safe.
    def close
      check_process
      streams = @mutex.synchronize do
        @closed = true
        @streams.filter_map { |reference| reference.__getobj__ if reference.weakref_alive? }
      end
      streams.each(&:close)
      Thread.handle_interrupt(Object => :never) do
        @mutex.synchronize do
          if @pointer
            @usage = Native.usage(@pointer, settled: true)
            @pointer.free
            @pointer = nil
          end
        end
      end
      nil
    end
    # Cumulative usage; after close returns the copied settled snapshot.
    def usage
      check_process
      @mutex.synchronize { @pointer ? Native.usage(@pointer) : @usage }
    end
    # Whether this client has released its native engine.
    def closed? = @mutex.synchronize { @closed || !@pointer }
    # Return a boolean answer row with its probability. A collection belongs in decide_each.
    def decide(question, record, threshold: UNSET, field: UNSET, name: nil, cancel: nil, yes_means: UNSET, no_means: UNSET)
      question = build_question(:decide, question, {yes_means: yes_means, no_means: no_means, threshold: threshold, field: field})
      stream = open_rows(question, single_record(record, :decide), name, cancel)
      single_result(stream)
    end
    # Yield one decide row per input. Without a block, return a closable Rows enumerator.
    def decide_each(question, records, threshold: UNSET, field: UNSET, name: nil, cancel: nil, yes_means: UNSET, no_means: UNSET, &block)
      question = build_question(:decide, question, {yes_means: yes_means, no_means: no_means, threshold: threshold, field: field})
      stream = open_rows(question, records, name, cancel)
      return stream unless block
      stream.each(&block)
      nil
    end
    # Return the selected label and its distribution. A collection belongs in choose_each.
    def choose(question, record, choices: UNSET, threshold: UNSET, field: UNSET, name: nil, cancel: nil, descriptions: UNSET)
      question = build_question(:choose, question, {descriptions: descriptions, choices: choices, threshold: threshold, field: field})
      stream = open_rows(question, single_record(record, :choose), name, cancel)
      single_result(stream)
    end
    # Yield one choose row per input. Without a block, return a closable Rows enumerator.
    def choose_each(question, records, choices: UNSET, threshold: UNSET, field: UNSET, name: nil, cancel: nil, descriptions: UNSET, &block)
      question = build_question(:choose, question, {descriptions: descriptions, choices: choices, threshold: threshold, field: field})
      stream = open_rows(question, records, name, cancel)
      return stream unless block
      stream.each(&block)
      nil
    end
    # Return the matching labels and their probabilities. A collection belongs in tag_each.
    def tag(question, record, labels: UNSET, threshold: UNSET, field: UNSET, name: nil, cancel: nil, descriptions: UNSET)
      question = build_question(:tag, question, {descriptions: descriptions, labels: labels, threshold: threshold, field: field})
      stream = open_rows(question, single_record(record, :tag), name, cancel)
      single_result(stream)
    end
    # Yield one tag row per input. Without a block, return a closable Rows enumerator.
    def tag_each(question, records, labels: UNSET, threshold: UNSET, field: UNSET, name: nil, cancel: nil, descriptions: UNSET, &block)
      question = build_question(:tag, question, {descriptions: descriptions, labels: labels, threshold: threshold, field: field})
      stream = open_rows(question, records, name, cancel)
      return stream unless block
      stream.each(&block)
      nil
    end
    # Return a normalized score and its level probabilities. A collection belongs in score_each.
    def score(question, record, levels: UNSET, field: UNSET, name: nil, cancel: nil, descriptions: UNSET)
      question = build_question(:score, question, {descriptions: descriptions, levels: levels, field: field})
      stream = open_rows(question, single_record(record, :score), name, cancel)
      single_result(stream)
    end
    # Yield one score row per input. Without a block, return a closable Rows enumerator.
    def score_each(question, records, levels: UNSET, field: UNSET, name: nil, cancel: nil, descriptions: UNSET, &block)
      question = build_question(:score, question, {descriptions: descriptions, levels: levels, field: field})
      stream = open_rows(question, records, name, cancel)
      return stream unless block
      stream.each(&block)
      nil
    end
    # Yield only the original kept records, then raise once with failed results.
    def filter(question, records, threshold: UNSET, field: UNSET, name: nil, cancel: nil, yes_means: UNSET, no_means: UNSET, &block)
      question = build_question(:filter, question, {yes_means: yes_means, no_means: no_means, threshold: threshold, field: field})
      stream = open_rows(question, records, name, cancel)
      delivered = Enumerator.new do |output|
        # @type var failed: Array[Result]
        failed = []
        begin
          stream.each do |row|
            if row.failed?
              failed << row
            elsif row.answer
              output << row.item
            end
          end
          unless failed.empty?
            # Collected native failed outcomes always carry their error.
            # @type var error: untyped
            error = failed.first.error
            raise FailedError.new(error.message, code: error.code, http_status: error.http_status, rows: failed)
          end
        ensure
          stream.close
        end
      end
      return delivered unless block
      delivered.each(&block)
      nil
    end
    # Yield ranked results, failed records last, retaining each caller-owned item.
    def rank(question, records, levels: UNSET, top: UNSET, field: UNSET, name: nil, cancel: nil, descriptions: UNSET, &block)
      question = build_question(:rank, question, {descriptions: descriptions, levels: levels, top: top, field: field})
      stream = open_rows(question, records, name, cancel)
      return stream unless block
      stream.each(&block)
      nil
    end
    # Return the matched span, or an answered nil when no span matches. A collection belongs in find_each.
    def find(question, record, threshold: UNSET, field: UNSET, name: nil, cancel: nil)
      question = build_question(:find, question, {threshold: threshold, field: field})
      stream = open_rows(question, single_record(record, :find), name, cancel)
      single_result(stream)
    end
    # Yield one find row per input. Without a block, return a closable Rows enumerator.
    def find_each(question, records, threshold: UNSET, field: UNSET, name: nil, cancel: nil, &block)
      question = build_question(:find, question, {threshold: threshold, field: field})
      stream = open_rows(question, records, name, cancel)
      return stream unless block
      stream.each(&block)
      nil
    end
    # Return recognized names with character offsets and probabilities. A collection belongs in recognize_each.
    def recognize(record, kinds: UNSET, threshold: UNSET, field: UNSET, name: nil, cancel: nil)
      question = build_question(:recognize, nil, {kinds: kinds, threshold: threshold, field: field})
      stream = open_rows(question, single_record(record, :recognize), name, cancel)
      single_result(stream)
    end
    def recognize_each(records, kinds: UNSET, threshold: UNSET, field: UNSET, name: nil, cancel: nil, &block)
      question = build_question(:recognize, nil, {kinds: kinds, threshold: threshold, field: field})
      stream = open_rows(question, records, name, cancel)
      return stream unless block
      stream.each(&block)
      nil
    end
    def relate(record, rules: UNSET, threshold: UNSET, field: UNSET, name: nil, cancel: nil)
      question = build_question(:relate, nil, {rules: rules, threshold: threshold, field: field})
      stream = open_rows(question, single_record(record, :relate), name, cancel)
      single_result(stream)
    end
    def relate_each(records, rules: UNSET, threshold: UNSET, field: UNSET, name: nil, cancel: nil, &block)
      question = build_question(:relate, nil, {rules: rules, threshold: threshold, field: field})
      stream = open_rows(question, records, name, cancel)
      return stream unless block
      stream.each(&block)
      nil
    end
    # Return named child answer rows for the supplied questions. A collection belongs in annotate_each.
    def annotate(question, record, field: UNSET, name: nil, cancel: nil)
      question = build_question(:annotate, question, {field: field})
      stream = open_rows(question, single_record(record, :annotate), name, cancel)
      single_result(stream)
    end
    # Yield one annotate row per input. Without a block, return a closable Rows enumerator.
    def annotate_each(question, records, field: UNSET, name: nil, cancel: nil, &block)
      question = build_question(:annotate, question, {field: field})
      stream = open_rows(question, records, name, cancel)
      return stream unless block
      stream.each(&block)
      nil
    end
    private
    def single_result(stream)
      row = stream.next
      if row.failed?
        error = row.error
        raise FailedError.new(error.message, code: error.code, http_status: error.http_status, rows: [row])
      end
      row
    ensure
      stream.close
    end
    def single_record(record, function)
      if record.is_a?(Array) || record.respond_to?(:each) && !record.is_a?(Hash) && !record.is_a?(String) && !record.respond_to?(:to_h)
        raise UsageError.new("a single call does not accept a collection; use #{function}_each", code: :"usage.argument")
      end
      [record]
    end
    def check_process
      raise IOError, "client belongs to another process; create a client after fork" unless @pid == Process.pid
    end
    def build_question(function, question, options)
      check_process
      # @type var klass: untyped
      klass = ThinkThen.const_get("#{function.to_s.capitalize}Question")
      # @type var options: Hash[Symbol, untyped]
      options = options.reject { |_, value| value.equal?(UNSET) }
      if question.is_a?(Question)
        raise UsageError.new("question does not match function", code: :"usage.argument") unless question.is_a?(klass)
        raise UsageError.new("saved question owns its options", code: :"usage.argument") unless options.empty?
        question
      elsif function == :annotate
        klass.new(question, **options)
      elsif [:recognize, :relate].include?(function)
        klass.new(**options)
      else
        klass.new(question, **options)
      end
    end
    def open_rows(question, records, name, cancel)
      raise UsageError.new("records must be an Enumerable; use the single-record method for text", code: :"usage.argument") unless records.respond_to?(:each) && !records.is_a?(Hash)
      raise UsageError.new("name must be a String", code: :"usage.name") unless name.nil? || name.is_a?(String)
      raise UsageError.new("cancel must be a Cancel", code: :"usage.argument") unless cancel.nil? || cancel.is_a?(Cancel)
      request = Native.context_dump({question: question.value, **(name.nil? ? {} : {name: name})}, :question)
      stream = Rows.new(self, request, records, cancel, question)
      @mutex.synchronize do
        raise UsageError.new("client is closed", code: :"usage.closed") if @closed || !@pointer
        @streams.reject! { |reference| !reference.weakref_alive? }
        @streams << WeakRef.new(stream)
      end
      stream
    rescue Exception
      stream&.close
      raise
    end
    def admit(request, records = nil)
      @mutex.synchronize do
        raise UsageError.new("client is closed", code: :"usage.closed") if @closed || !@pointer
        Native.bytes(request) do |bytes, length|
          output = FFI::MemoryPointer.new(:pointer)
          if records
            Native.records(records) do |input, count|
              Native.checked do |error|
                Native.thinkthen_call_open_records_json(@pointer, bytes, length, input, count, Native::RecordInput.size,
                                                        NativeContract::OFFSET_NAMES.key("code_points"), output, error)
              end
            end
          else
            Native.checked do |error|
              Native.thinkthen_call_open_json(@pointer, bytes, length, NativeContract::OFFSET_NAMES.key("code_points"), output, error)
            end
          end
          output.read_pointer
        end
      end
    end
    def forget(stream) = @mutex.synchronize { @streams.reject! { |reference| !reference.weakref_alive? || reference.__getobj__.equal?(stream) } }
  end
end
