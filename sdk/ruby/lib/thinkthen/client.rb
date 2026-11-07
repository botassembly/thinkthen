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
    # Open the native engine with optional backend settings and a Retry policy.
    def initialize(backend: nil, model: nil, url: nil, api_key: nil, batch: nil, retry: nil)
      @mutex, @streams = Mutex.new, []
      @pid = Process.pid
      ThinkThen.send(:native)
      key = api_key.nil? || api_key.is_a?(RedactedKey) ? api_key : RedactedKey.new(api_key)
      @api_key = key
      policy = binding.local_variable_get(:retry)
      raise UsageError.new("retry must be a Retry", code: :"usage.settings") unless policy.nil? || policy.is_a?(Retry)
      raise UsageError.new("backend must be a Symbol", code: :"usage.backend") unless backend.nil? || backend.is_a?(Symbol)
      raw_settings = {backend: backend, model: model, url: url, batch: batch, retry: policy&.to_h}.reject { |_, value| value.nil? }
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
        @mutex.synchronize { @pointer&.free; @pointer = nil }
      end
      nil
    end
    # Whether this client has released its native engine.
    def closed? = @mutex.synchronize { @closed || !@pointer }
    # Return a boolean answer row with its probability. A collection belongs in decide_each.
    def decide(question, record, threshold: UNSET, field: UNSET, name: nil, cancel: nil, yes_means: UNSET, no_means: UNSET)
      question = build_question(:decide, question, {yes_means: yes_means, no_means: no_means, threshold: threshold, field: field})
      stream = open_rows(question, single_record(record), name, cancel)
      begin
        single_result(stream.next)
      ensure
        stream.close
      end
    end
    # Return a boolean, raising on not sure or failure.
    def decide?(question, record, **options) = public_send(:decide, question, record, **options).yes?
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
      stream = open_rows(question, single_record(record), name, cancel)
      begin
        single_result(stream.next)
      ensure
        stream.close
      end
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
      stream = open_rows(question, single_record(record), name, cancel)
      begin
        single_result(stream.next)
      ensure
        stream.close
      end
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
      stream = open_rows(question, single_record(record), name, cancel)
      begin
        single_result(stream.next)
      ensure
        stream.close
      end
    end
    # Yield one score row per input. Without a block, return a closable Rows enumerator.
    def score_each(question, records, levels: UNSET, field: UNSET, name: nil, cancel: nil, descriptions: UNSET, &block)
      question = build_question(:score, question, {descriptions: descriptions, levels: levels, field: field})
      stream = open_rows(question, records, name, cancel)
      return stream unless block
      stream.each(&block)
      nil
    end
    # Return an Enumerator of the kept records, the caller's own objects, in input order.
    # Not-sure records are left out. After the kept records it raises one RecordError
    # listing every failed row; failures: :row yields those FilterRows instead.
    # With a block, yield each kept record and return nil.
    def filter(question, records, failures: :raise, **options, &block)
      check_failures(failures)
      delivered = deliver(public_send(:filter_each, question, records, **options), failures) do |entry, output|
        output << entry.item if entry.kept?
      end
      return delivered unless block
      delivered.each(&block)
      nil
    end
    # Yield every decision with the original item, including not-sure and failed rows.
    def filter_each(question, records, threshold: UNSET, field: UNSET, name: nil, cancel: nil, yes_means: UNSET, no_means: UNSET, &block)
      question = build_question(:filter, question, {yes_means: yes_means, no_means: no_means, threshold: threshold, field: field})
      stream = open_rows(question, records, name, cancel)
      return stream unless block
      stream.each(&block)
      nil
    end
    # Return an Enumerator of RankRows in the engine's rank order. Each row's +rank+ is
    # the engine's rank number and its +item+ an equal copy of the record. A not-sure
    # row stays in its place. After the ranked rows it raises one RecordError listing
    # every failed row; failures: :row yields them instead. With a block, yield each row.
    def rank(question, records, failures: :raise, **options, &block)
      check_failures(failures)
      delivered = deliver(public_send(:rank_each, question, records, **options), failures) { |entry, output| output << entry }
      return delivered unless block
      delivered.each(&block)
      nil
    end
    # Yield every rank row in rank order, failed rows included, each with a copy of its record.
    def rank_each(question, records, levels: UNSET, top: UNSET, field: UNSET, name: nil, cancel: nil, descriptions: UNSET, &block)
      question = build_question(:rank, question, {descriptions: descriptions, levels: levels, top: top, field: field})
      stream = open_rows(question, records, name, cancel)
      return stream unless block
      stream.each(&block)
      nil
    end
    # Return the matched span, or an answered nil when no span matches. A collection belongs in find_each.
    def find(question, record, threshold: UNSET, field: UNSET, name: nil, cancel: nil)
      question = build_question(:find, question, {threshold: threshold, field: field})
      stream = open_rows(question, single_record(record), name, cancel)
      begin
        single_result(stream.next)
      ensure
        stream.close
      end
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
    def recognize(question_or_record, record = UNSET, kinds: UNSET, threshold: UNSET, field: UNSET, name: nil, cancel: nil)
      input = single_record(record.equal?(UNSET) ? question_or_record : record)
      question = record.equal?(UNSET) ? nil : question_or_record
      question = build_question(:recognize, question, {kinds: kinds, threshold: threshold, field: field})
      stream = open_rows(question, input, name, cancel)
      begin
        single_result(stream.next)
      ensure
        stream.close
      end
    end
    # Yield one recognize row per input. Without a block, return a closable Rows enumerator.
    def recognize_each(question_or_records, records = UNSET, kinds: UNSET, threshold: UNSET, field: UNSET, name: nil, cancel: nil, &block)
      question = UNSET.equal?(records) ? nil : question_or_records
      records = question_or_records if UNSET.equal?(records)
      question = build_question(:recognize, question, {kinds: kinds, threshold: threshold, field: field})
      stream = open_rows(question, records, name, cancel)
      return stream unless block
      stream.each(&block)
      nil
    end
    # Return relations with character offsets and probabilities. A collection belongs in relate_each.
    def relate(question_or_record, record = UNSET, rules: UNSET, threshold: UNSET, field: UNSET, name: nil, cancel: nil)
      input = single_record(record.equal?(UNSET) ? question_or_record : record)
      question = record.equal?(UNSET) ? nil : question_or_record
      question = build_question(:relate, question, {rules: rules, threshold: threshold, field: field})
      stream = open_rows(question, input, name, cancel)
      begin
        single_result(stream.next)
      ensure
        stream.close
      end
    end
    # Yield one relate row per input. Without a block, return a closable Rows enumerator.
    def relate_each(question_or_records, records = UNSET, rules: UNSET, threshold: UNSET, field: UNSET, name: nil, cancel: nil, &block)
      question = UNSET.equal?(records) ? nil : question_or_records
      records = question_or_records if UNSET.equal?(records)
      question = build_question(:relate, question, {rules: rules, threshold: threshold, field: field})
      stream = open_rows(question, records, name, cancel)
      return stream unless block
      stream.each(&block)
      nil
    end
    # Return named child answer rows for the supplied questions. A collection belongs in annotate_each.
    def annotate(question, record, field: UNSET, name: nil, cancel: nil)
      question = build_question(:annotate, question, {field: field})
      stream = open_rows(question, single_record(record), name, cancel)
      begin
        single_result(stream.next)
      ensure
        stream.close
      end
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
    def check_failures(failures)
      raise UsageError.new("failures must be :raise or :row", code: :"usage.argument") unless [:raise, :row].include?(failures)
    end
    # Pass non-failed rows to the block, then raise for or yield the failed rows.
    def deliver(stream, failures, &handle)
      Enumerator.new do |output|
        # @type var failed: Array[untyped]
        failed = []
        begin
          stream.each { |entry| entry.failed? ? failed << entry : handle.call(entry, output) }
          if failures == :row
            failed.each { |entry| output << entry }
          elsif !failed.empty?
            error = failed.first.row.error
            raise RecordError.new(error.message, code: error.code, http_status: error.http_status, rows: failed)
          end
        ensure
          stream.close
        end
      end
    end
    def single_result(row)
      if row.failed? && row.error.code == :"record.empty"
        raise RecordError.new(row.error.message, code: row.error.code, http_status: row.error.http_status, rows: [row])
      end
      row
    end
    def single_record(record)
      raise UsageError.new("a single call does not accept a collection; use the _each method", code: :"usage.argument") if record.respond_to?(:each) && !record.is_a?(Hash)
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
      elsif [:recognize, :relate].include?(function)
        klass.new(**options)
      else
        klass.new(question, **options)
      end
    end
    def open_rows(question, records, name, cancel)
      raise UsageError.new("records must be an Enumerable; use the single-record method for text", code: :"usage.argument") unless records.respond_to?(:each) && !records.is_a?(Hash)
      raise UsageError.new("name must be a String", code: :"usage.name") unless name.nil? || name.is_a?(String)
      raise UsageError.new("cancel must be a CancelToken", code: :"usage.argument") unless cancel.nil? || cancel.is_a?(CancelToken)
      request = Native.context_dump({question: question.value, **(name.nil? ? {} : {name: name})}, :question)
      stream = Rows.new(self, request, records, cancel, [:filter, :rank].find { |function| function.to_s == question.value.fetch("function") })
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
    def admit(request, records = nil, invalid = {})
      @mutex.synchronize do
        raise UsageError.new("client is closed", code: :"usage.closed") if @closed || !@pointer
        Native.bytes(request) do |bytes, length|
          output = FFI::MemoryPointer.new(:pointer)
          if records
            Native.records(records, invalid) do |input, count|
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
