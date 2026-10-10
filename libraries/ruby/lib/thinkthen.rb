# frozen_string_literal: true

# The Ruby surface over the thinkthen public Rust API.
#
# The asking verbs are module methods on ThinkThen, beside question, set,
# usage, and with_tick. The same methods live on a
# ThinkThen::Engine value, which carries the engine settings. The module
# methods call one lazy engine built from the environment.
#
# A question is built with ThinkThen.question(decide: "...", threshold:
# 0.2..0.8), and a verb that takes a question also takes the bare question
# text. A Range threshold is the band. nil is unsure.
#
# Every failure raises a ThinkThen::Error. The six kind classes inherit it,
# and the binding's own refusals raise UsageError. A record that is not a
# String crosses as its JSON text, never as Ruby's to_s form.

require "json"
require "timeout"
require_relative "thinkthen/version"

module ThinkThen
  # The one base class. `kind` is the kind's word and `retryable` the
  # engine's retry signal.
  class Error < StandardError
    # Each kind's code in the C door, 1 through 6.
    CODES = { "usage" => 1, "backend" => 2, "deadline" => 3, "local" => 4, "cancelled" => 5, "defect" => 6 }.freeze

    attr_reader :kind, :retryable, :facts, :details, :completion

    def initialize(message = nil, kind = nil, retryable = false)
      super(message)
      @kind = kind
      @retryable = retryable
    end

    def code = CODES.fetch(kind, 6)
  end

  # A decide answer's code in the C door. ThinkThen.outcome maps an answer to it.
  YES = 1
  NO = 0
  UNSURE = 2

  class UsageError < Error; end
  class BackendError < Error; end
  class LocalError < Error; end
  class CancelledError < Error; end
  class DeadlineError < Error; end
  class DefectError < Error; end

  # The old Ruby value plus one completed call's immutable account.
  class Call
    attr_reader :value, :facts, :details

    def initialize(value, facts, details)
      @value = value
      @facts = ThinkThen.__send__(:deep_freeze, facts)
      @details = ThinkThen.__send__(:deep_freeze, details)
      freeze
    end

    def map
      self.class.new(yield(value), facts, details)
    rescue Exception => error
      ThinkThen.__send__(:attach_account, error, facts, details)
      raise
    end

    def inspect
      "#<ThinkThen::Call value=<withheld> records=#{facts[:records]}>"
    end
  end

  # A prompt stop retains the original worker's eventual final account.
  class Completion
    def initialize(native)
      @native = native
    end

    def done? = @native.done?
    alias done done?

    def result(timeout: nil)
      raise UsageError.new("completion timeout takes a nonnegative number", "usage") unless timeout.nil? ||
        (timeout.is_a?(Numeric) && timeout.real? && timeout.finite? && timeout >= 0)

      raw = @native.result(timeout)
      raise Timeout::Error, "the call has not completed" if raw.nil?

      @result ||= ThinkThen.__send__(:deep_freeze, raw)
    end
  end
end

require_relative "thinkthen/thinkthen"

module ThinkThen
  # The print form of every result value. A field holding the caller's text
  # prints as its byte count, as the Rust Debug and Python repr forms do. A
  # list prints as its length. Places, probabilities, and rule names print
  # in clear.
  module Withheld
    TEXT = %i[name kind text record unit].freeze

    def inspect
      fields = each_pair.map do |name, value|
        shown = if value.nil? then "nil"
                elsif TEXT.include?(name) then "<#{value.to_s.bytesize} bytes withheld>"
                elsif value.is_a?(Array) then value.size.to_s
                else value.inspect
                end
        "#{name}=#{shown}"
      end
      "#<struct #{self.class.name} #{fields.join(", ")}>"
    end

    alias to_s inspect

    def pretty_print(printer)
      printer.text(inspect)
    end
  end

  # A name and its kind, as relate reads and returns them.
  Entity = Struct.new(:name, :kind) { include Withheld }

  # One name recognize found. `start`, `end`, and `length` count characters,
  # so `text[start...end]` is the name. `kind` is "ENTITY" when the call
  # named no kind.
  RecognizedEntity = Struct.new(:text, :start, :end, :length, :kind, :strength) { include Withheld }

  # One relation between two recognized names.
  # `either` is true when the relation holds both ways.
  Relation = Struct.new(:relation, :source, :target, :probability, :either) { include Withheld }

  # What recognize returned. `relations` is nil when no rule was given.
  Recognized = Struct.new(:entities, :relations) { include Withheld }

  # One edge relate found, with Entity ends. `either` is true when the edge
  # holds both ways; its ends are then in input order.
  Edge = Struct.new(:relation, :source, :target, :probability, :either) { include Withheld }

  # The record's place in the input, the record, and its probability.
  # Most likely yes first. Ties keep input order. `to_s` is the record.
  Ranked = Struct.new(:index, :record, :probability) do
    include Withheld

    def to_s
      record.to_s
    end
  end

  # The selected unit's place, the unit, and its probability. Every field
  # is nil when nothing is selected.
  Found = Struct.new(:index, :unit, :probability) { include Withheld }

  # The watchdog's cadence and one in-flight call's row.
  WATCHDOG_INTERVAL = 0.1
  Row = Struct.new(:tick, :token, :error)
  private_constant :WATCHDOG_INTERVAL, :Row, :Native, :Withheld

  @rows = {}
  @rows_mutex = Mutex.new
  @default_mutex = Mutex.new

  # An engine and its settings. Omitted settings come from the environment,
  # as the module methods' engine does. The throttle is the most requests
  # in flight at once, per loaded copy of the library.
  class Engine
    def initialize(backend: nil, base_url: nil, model: nil, throttle: nil, max_requests: nil, max_request_bytes: nil, cache: nil,
                   timeout: nil, max_retries: nil, record: nil, replay: nil, profile: nil, batch: nil, max_requests_total: nil, refresh_cache: false, **unknown)
      raise UsageError.new("unsupported engine setting", "usage") unless unknown.empty?
      ThinkThen.__send__(:text_setting, :backend, backend)
      ThinkThen.__send__(:text_setting, :base_url, base_url)
      ThinkThen.__send__(:text_setting, :model, model)
      ThinkThen.__send__(:whole_setting, :throttle, throttle)
      ThinkThen.__send__(:whole_setting, :max_requests, max_requests)
      ThinkThen.__send__(:whole_setting, :max_request_bytes, max_request_bytes)
      ThinkThen.__send__(:whole_setting, :timeout, timeout)
      ThinkThen.__send__(:whole_setting, :max_retries, max_retries)
      ThinkThen.__send__(:whole_setting, :max_requests_total, max_requests_total)
      { record: record, replay: replay, profile: profile }.each do |name, value|
        ThinkThen.__send__(:text_setting, name, value)
      end
      unless cache.nil? || cache == false || cache.is_a?(String)
        raise UsageError.new("cache is a folder path, false for none, or nil for the default", "usage")
      end
      batch = ThinkThen.__send__(:batch_of, batch)
      @native = Native.engine({ backend: backend, base_url: base_url, model: model, throttle: throttle,
        max_requests: max_requests, max_request_bytes: max_request_bytes,
        cache_at: cache || nil, no_cache: cache == false,
        timeout: timeout, max_retries: max_retries, record: record, replay: replay, profile: profile, batch: batch,
        max_requests_total: max_requests_total, refresh_cache:refresh_cache })
    end

    def self.from_native(native)
      engine = allocate
      engine.instance_variable_set(:@native, native)
      engine
    end
    # Explicit native reader; question is the existing JSON grammar for any verb.
    def files(question, paths, unit: "line", window: nil, cancel: nil, deadline_ms: nil)
      source = {paths: paths.is_a?(String) ? [paths] : paths, unit: unit}
      source[:window] = window unless window.nil?
      crossing("files", JSON.generate(question), JSON.generate(source), cancel, deadline_ms)
        .map { |text| JSON.parse(text) }
    end

    private_class_method :from_native

    def inspect
      "#<ThinkThen::Engine>"
    end

    def decide(question, evidence, cancel: nil, deadline_ms: nil, batch: nil, context: nil)
      crossing("decide", ThinkThen.__send__(:built, question), ThinkThen.__send__(:text_of, evidence), cancel, deadline_ms,
               batch: batch, context: context)
    end

    def decide_many(question, records, cancel: nil, deadline_ms: nil, batch: nil, context: nil)
      decide_many_with_probabilities(question, records, cancel: cancel, deadline_ms: deadline_ms, batch: batch,
                                    context: context).map { |rows| rows.map { |row| row[:answer] } }
    end

    # Each answer with its probability, from the same requests.
    def decide_many_with_probabilities(question, records, cancel: nil, deadline_ms: nil, batch: nil, context: nil)
      crossing("decide_many", ThinkThen.__send__(:built, question), ThinkThen.__send__(:texts, records.to_a), cancel, deadline_ms,
               batch: batch, context: context, batch_ok: true, context_ok: true)
        .map { |rows| rows.map { |answer, probability| { answer: answer, probability: probability } } }
    end

    def filter(question, records, cancel: nil, deadline_ms: nil, batch: nil, context: nil)
      list = records.to_a
      crossing("filter", ThinkThen.__send__(:built, question), ThinkThen.__send__(:texts, list), cancel, deadline_ms,
               batch: batch, context: context, batch_ok: true, context_ok: true)
        .map { |places| places.map { |place| list[place] } }
    end

    def rank(question, records, top: nil, cancel: nil, deadline_ms: nil, batch: nil, context: nil)
      list = records.to_a
      crossing("rank", ThinkThen.__send__(:question_text, "rank", question), ThinkThen.__send__(:texts, list), cancel, deadline_ms,
               batch: batch, context: context, batch_ok: true, context_ok: true)
        .map do |placed|
          ranked = placed.map { |place, probability| Ranked.new(place, list[place], probability) }
          top ? ranked.first(top) : ranked
        end
    end

    # none: true offers a none candidate, as find --none does, so nothing may fit.
    def find(question, units, none: false, cancel: nil, deadline_ms: nil, batch: nil, context: nil)
      raise UsageError.new("none is true or false", "usage") unless [true, false].include?(none)

      list = units.to_a
      crossing(none ? "find_none" : "find", ThinkThen.__send__(:question_text, "find", question), ThinkThen.__send__(:texts, list), cancel, deadline_ms,
               batch: batch, context: context).map do |place, probability|
        Found.new(place, place.nil? ? nil : list[place], probability)
      end
    end

    def choose(question, evidence, options: nil, cancel: nil, deadline_ms: nil, batch: nil, context: nil)
      question = ThinkThen.__send__(:keyed, "choose", question, :options, options)
      crossing("details", question, ThinkThen.__send__(:text_of, evidence), cancel, deadline_ms,
               batch: batch, context: context).map { |json| JSON.parse(json)["value"] }
    end

    def score(question, evidence, levels: nil, cancel: nil, deadline_ms: nil, batch: nil, context: nil)
      question = ThinkThen.__send__(:keyed, "score", question, :levels, levels)
      crossing("score", question, ThinkThen.__send__(:text_of, evidence), cancel, deadline_ms,
               batch: batch, context: context)
    end

    # The position and its nearest level, from one call.
    def score_with_level(question, evidence, levels: nil, cancel: nil, deadline_ms: nil, batch: nil, context: nil)
      question = ThinkThen.__send__(:keyed, "score", question, :levels, levels)
      crossing("details", question, ThinkThen.__send__(:text_of, evidence), cancel, deadline_ms,
               batch: batch, context: context).map { |json| JSON.parse(json).then { |doc| [doc["value"], doc.dig("answer", "level")] } }
    end

    def tag(question, evidence, labels: nil, cancel: nil, deadline_ms: nil, batch: nil, context: nil)
      question = ThinkThen.__send__(:keyed, "tag", question, :labels, labels)
      crossing("details", question, ThinkThen.__send__(:text_of, evidence), cancel, deadline_ms,
               batch: batch, context: context).map { |json| JSON.parse(json)["value"] }
    end

    # The command's --details document for one text.
    def details(question, evidence, cancel: nil, deadline_ms: nil, batch: nil, context: nil)
      crossing("details", ThinkThen.__send__(:built, question), ThinkThen.__send__(:text_of, evidence), cancel, deadline_ms,
               batch: batch, context: context).map { |json| JSON.parse(json) }
    end

    def choose_many(question, records, options: nil, cancel: nil, deadline_ms: nil, batch: nil, context: nil)
      many("choose", question, :options, options, records, cancel, deadline_ms, batch, context)
    end

    def score_many(question, records, levels: nil, cancel: nil, deadline_ms: nil, batch: nil, context: nil)
      many("score", question, :levels, levels, records, cancel, deadline_ms, batch, context)
    end

    def tag_many(question, records, labels: nil, cancel: nil, deadline_ms: nil, batch: nil, context: nil)
      many("tag", question, :labels, labels, records, cancel, deadline_ms, batch, context)
    end

    # One Hash per record, named by the set's questions. A set member whose
    # `on` names a part reads it from each record as JSON text. With `on:`, each
    # record is a Hash, its `on` value is the evidence, and the answers
    # join its own keys. A question landing on any record's key refuses
    # before any request.
    def annotate(set, records, on: nil, cancel: nil, deadline_ms: nil, batch: nil, context: nil)
      set = ThinkThen.set(set) if set.is_a?(String)
      list = records.to_a
      if on
        ThinkThen.__send__(:refuse_clashes, set, list)
        evidence = list.each_with_index.map { |record, place| ThinkThen.__send__(:text_of, record[on], "record #{place}") }
      else
        evidence = ThinkThen.__send__(:texts, list)
      end
      crossing("annotate", set, evidence, cancel, deadline_ms, batch: batch, context: context, batch_ok: true)
        .map do |rows|
          rows.each_with_index.map do |json, place|
            answers = JSON.parse(json).transform_keys(&:to_sym)
            on ? list[place].transform_keys(&:to_sym).merge(answers) : answers
          end
        end
      end

    # Find every name in a text and say what kind it is.
    #
    #   found = ThinkThen.recognize(text, kinds: %w[person organization place],
    #                               relations: { works_for: %w[person organization] })
    #   found.entities.first.kind  # "person"
    def recognize(text, kinds: nil, relations: nil, threshold: nil, relation_threshold: nil, instructions: nil, entity_definition: nil, file: nil, cancel: nil, deadline_ms: nil,
                  batch: nil, context: nil)
      unless file.nil?
        ThinkThen.__send__(:refuse, "recognize file takes no inline plan options") unless [kinds, relations, threshold, relation_threshold, instructions, entity_definition].all?(&:nil?)
        spec = Native.plan_file(ThinkThen.__send__(:file_path, file, "plan"), "recognize")
      else
        spec = JSON.generate(ThinkThen.__send__(:recognize_spec, kinds, relations, threshold, relation_threshold, instructions, entity_definition))
      end
      crossing("recognize", spec, ThinkThen.__send__(:text_of, text), cancel, deadline_ms,
               batch: batch, context: context).map { |json| ThinkThen.__send__(:recognized, JSON.parse(json)) }
    end

    # Say how named entities relate. An entity is a [name, kind] pair, a
    # Hash with name and kind, an Entity, or a RecognizedEntity. A found name,
    # or a Hash with text and no name, is named by its text.
    #
    #   edges = ThinkThen.relate([["Ana", "person"], ["Acme", "organization"]],
    #                            relations: { works_for: %w[person organization] })
    def relate(entities, relations: nil, either: nil, threshold: nil, file: nil, cancel: nil, deadline_ms: nil, batch: nil, context: nil)
      unless file.nil?
        ThinkThen.__send__(:refuse, "relate file takes no inline plan options") unless [relations, either, threshold].all?(&:nil?)
        spec = Native.plan_file(ThinkThen.__send__(:file_path, file, "plan"), "relate")
      else
        ThinkThen.__send__(:refuse, "relate needs at least one relation rule") if relations.nil?
        spec = JSON.generate(ThinkThen.__send__(:relate_spec, relations, either, threshold))
      end
      pairs = entities.to_a.each_with_index.map { |entity, place| ThinkThen.__send__(:pair_of, entity, place) }
      crossing("relate", spec, pairs, cancel, deadline_ms, batch: batch, context: context)
        .map do |rows|
          rows.map do |json|
            edge = JSON.parse(json)
            Edge.new(edge["relation"], ThinkThen.__send__(:entity, edge["source"]), ThinkThen.__send__(:entity, edge["target"]), edge["probability"], edge["either"] == true)
          end
        end
      end

    # Preview a decide, choose, score, or tag call without asking it: the
    # records, the requests, the prepared bytes, the input-token band, and the
    # first request body, as the result schema's plan object. It reads no key
    # and no cache and sends nothing.
    def plan(question, records, batch: nil, context: nil)
      @native.plan(ThinkThen.__send__(:built, question), ThinkThen.__send__(:texts, records.to_a),
                   ThinkThen.__send__(:batch_of, batch), ThinkThen.__send__(:context_of, context))
    end

    # The engine's counters since the process started.
    def usage
      @native.usage
    end

    # Set this thread's tick. The watchdog runs it about ten times a second
    # while a call from this thread is in flight. A raise inside it stops the
    # call at once and surfaces from the call. A positional tick with a block
    # scopes the tick to the block's calls.
    def with_tick(tick = nil, &block)
      held = tick || block
      Thread.current[:thinkthen_tick] = held
      return self unless block && tick

      begin
        yield
      ensure
        Thread.current[:thinkthen_tick] = nil
      end
    end

    private

    def many(verb, question, key, members, records, cancel, deadline_ms, batch, context)
      question = ThinkThen.__send__(:keyed, verb, question, key, members)
      crossing("many", question, ThinkThen.__send__(:texts, records.to_a), cancel, deadline_ms,
               batch: batch, context: context, batch_ok: true, context_ok: true)
        .map { |rows| rows.map { |json| JSON.parse(json)["value"] } }
    end

    def crossing(verb, subject, input, cancel, deadline_ms, batch: nil, context: nil, batch_ok: false, context_ok: false)
      unless cancel.nil? || cancel.is_a?(Cancel)
        raise UsageError.new("cancel is a ThinkThen::Cancel or nil", "usage")
      end

      raise UsageError.new("#{verb} does not take batch", "usage") if !batch.nil? && !batch_ok
      raise UsageError.new("#{verb} does not take context", "usage") if !context.nil? && !context_ok
      batch = ThinkThen.__send__(:batch_of, batch)
      context = ThinkThen.__send__(:context_of, context)

      deadline_ms = ThinkThen.__send__(:deadline_ms_of, deadline_ms)
      own = Cancel.new
      tick = Thread.current[:thinkthen_tick]
      row = tick && ThinkThen.__send__(:watch, Row.new(tick, own, nil))
      pending = nil
      begin
        raw, facts, details = @native.call(verb, subject, input, own, cancel, deadline_ms, batch, context)
        Call.new(raw, facts, details)
      rescue Interrupt => error
        pending = error
        stopped = CancelledError.new("the call was cancelled", "cancelled")
        ThinkThen.__send__(:attach_receipt, stopped, error)
        raise stopped
      rescue Exception => error
        pending = error
        ThinkThen.__send__(:attach_receipt, error, error)
        raise
      ensure
        if row
          ThinkThen.__send__(:unwatch, row)
          if row.error
            ThinkThen.__send__(:attach_receipt, row.error, pending) if pending
            raise row.error
          end
        end
      end
    end
  end

  class << self
    # Build a question from the question file's keys: one verb key
    # (decide, choose, score, or tag) and its text, and threshold, options,
    # levels, labels, or meanings. A Range threshold is the band.
    def question(**keywords)
      if keywords.key?(:file)
        raise UsageError.new("question file takes no other question keys", "usage") unless keywords.size == 1

        return Native.question_file(file_path(keywords[:file], "question"))
      end

      body = keywords.transform_keys(&:to_s)
      threshold = body["threshold"]
      body["threshold"] = "#{threshold.begin}:#{threshold.end}" if threshold.is_a?(Range)
      Native.question(JSON.generate(body))
    end

    # A question set: a path to a set file, or questions by name.
    def set(path = nil, **questions)
      return Native.set_file(path.to_s) if path

      body = { "version" => 1, "questions" => {} }
      questions.each do |name, spec|
        body["questions"][name.to_s] = spec.is_a?(Question) ? JSON.parse(spec.json) : spec
      end
      Native.set_json(JSON.generate(body))
    end

    %i[decide decide_many decide_many_with_probabilities filter rank find choose choose_many score score_many score_with_level
       tag tag_many details annotate recognize relate plan usage files].each do |name|
      define_method(name) { |*args, **keywords, &block| default_engine.public_send(name, *args, **keywords, &block) }
    end

    def with_tick(tick = nil, &block)
      default_engine.with_tick(tick, &block)
    end

    # YES, NO, or UNSURE for a decide answer: true, false, or nil.
    def outcome(answer)
      { true => YES, false => NO, nil => UNSURE }.fetch(answer) { refuse("an outcome reads true, false, or nil") }
    end

    # The failure of one annotate answer, `{ "kind" => ..., "cause" => ... }`,
    # or nil for any value. nil is unresolved, never a failure.
    def failed(member)
      return unless member.is_a?(Hash) && member.size == 1

      held = member["failed"] || member[:failed]
      held if held.is_a?(Hash)
    end

    private

    def deep_freeze(value)
      case value
      when Hash
        value.each { |key, held| deep_freeze(key); deep_freeze(held) }
      when Array
        value.each { |held| deep_freeze(held) }
      end
      value.freeze
    end

    def batch_of(value)
      return nil if value.nil?
      return value if value == "max" || (value.is_a?(Integer) && value.positive?)

      refuse("batch takes max or a whole number of at least 1")
    end

    def context_of(value)
      return nil if value.nil?
      refuse("context is nonblank text") unless value.is_a?(String)
      text = text_of(value, "context")
      refuse("context is nonblank text") if text.strip.empty?

      text
    end

    def attach_receipt(error, source)
      native = source.instance_variable_get(:@thinkthen_completion)
      receipt = native ? Completion.new(native) : source.respond_to?(:completion) ? source.completion : nil
      error.instance_variable_set(:@completion, receipt) if receipt
      unless error.respond_to?(:completion)
        error.define_singleton_method(:completion) { @completion }
      end
      if error.is_a?(Error)
        deep_freeze(error.facts) if error.facts
        deep_freeze(error.details) if error.details
      end
    end

    def attach_account(error, facts, details)
      error.instance_variable_set(:@facts, facts)
      error.instance_variable_set(:@details, details)
      error.define_singleton_method(:facts) { @facts } unless error.respond_to?(:facts)
      error.define_singleton_method(:details) { @details } unless error.respond_to?(:details)
    end

    def default_engine
      @default_mutex.synchronize do
        @default ||= Engine.__send__(:from_native, Native.default_engine)
      end
    end

    def refuse(message)
      raise UsageError.new(message, "usage")
    end

    def file_path(path, role)
      refuse("#{role} file path is valid text") unless path.is_a?(String)
      path = path.dup.force_encoding(Encoding::UTF_8) unless path.encoding == Encoding::UTF_8
      refuse("#{role} file path is valid text") unless path.valid_encoding? && !path.include?("\0") && !path.empty?

      path
    end

    def text_setting(name, value)
      refuse("#{name} is text or nil") unless value.nil? || value.is_a?(String)
    end

    def whole_setting(name, value)
      refuse("#{name} is a whole number or nil") unless value.nil? || value.is_a?(Integer)
    end

    # A deadline is whole milliseconds: nil or -1 for none, 0 for spent. The
    # engine rules on the value; this only keeps it a 64-bit whole number.
    def deadline_ms_of(value)
      return nil if value.nil?
      unless value.is_a?(Integer) && value.bit_length < 64
        refuse("deadline_ms is a whole number of milliseconds, or -1 or nil for no deadline")
      end

      value
    end

    # The text one record crosses as: a String unchanged, anything else its
    # JSON text. nil, invalid UTF-8, and a NUL byte refuse.
    def text_of(record, where = "the evidence")
      refuse("#{where} is nil; a record is text or a JSON value") if record.nil?
      text = record.is_a?(String) ? record : JSON.generate(record)
      text = text.dup.force_encoding(Encoding::UTF_8) unless text.encoding == Encoding::UTF_8
      refuse("#{where} is not valid UTF-8") unless text.valid_encoding?
      refuse("#{where} holds a NUL byte") if text.include?("\0")
      text
    end

    def texts(list)
      list.each_with_index.map { |record, place| text_of(record, "record #{place}") }
    end

    # A built question, or a question's own text with its default cut.
    def built(value)
      return value if value.is_a?(Question)
      return Native.question(JSON.generate({ "decide" => value })) if value.is_a?(String)

      refuse("a question is a built question or its text")
    end

    # rank and find read one question text and no rule.
    def question_text(verb, value)
      return value if value.is_a?(String)
      if value.is_a?(Question)
        spec = JSON.parse(value.json)
        refuse("#{verb} takes a decide question") unless spec.key?("decide")
        extra = spec.keys.find { |key| key != "decide" }
        refuse("#{verb} takes a decide question with no #{extra}") if extra
        return spec.fetch("decide")
      end

      refuse("a question is a built question or its text")
    end

    # A choose, score, or tag question from its text and members, or a
    # built question that carries its own.
    def keyed(verb, question, key, members)
      if question.is_a?(Question)
        refuse("#{verb}: a built question carries its own #{key}; pass them on the question") if members

        return question
      end
      body = { verb => question }
      body[key.to_s] = members if members
      Native.question(JSON.generate(body))
    end

    def refuse_clashes(set, list)
      existing = list.flat_map { |record| record.keys.map(&:to_sym) }.uniq
      clash = (set.names.map(&:to_sym) & existing).first
      return unless clash

      refuse("annotate cannot add a question named '#{clash}': the input already has a key by that name; rename one")
    end

    def recognize_spec(kinds, relations, threshold, relation_threshold, instructions = nil, entity_definition = nil)
      kinds = [] if kinds.nil?
      kinds = kinds.to_h { |name| [name.to_s, nil] } if kinds.is_a?(Array)
      body = { "kinds" => kinds.to_h { |name, description| [name.to_s, description] } }
      body["relations"] = relation_rules(relations) if relations
      body["instructions"] = instructions unless instructions.nil?
      body["entity_definition"] = entity_definition unless entity_definition.nil?
      spec = { "version" => 1, "recognize" => body }
      spec["threshold"] = threshold unless threshold.nil?
      spec["relation_threshold"] = relation_threshold unless relation_threshold.nil?
      spec
    end

    # relations: names (any kind to any kind), a Hash of name to [source,
    # target], or the file's rule Hashes. either: the names that read the
    # same both ways.
    def relate_spec(relations, either, threshold)
      rules = relation_rules(relations)
      both = Array(either).map(&:to_s)
      rules.each { |rule| rule["either"] = true if both.include?(rule["name"]) }
      spec = { "version" => 1, "relate" => { "relations" => rules } }
      spec["threshold"] = threshold unless threshold.nil?
      spec
    end

    def relation_rules(relations)
      relations.map do |name, ends|
        next name.transform_keys(&:to_s) if name.is_a?(Hash)
        next { "name" => name.to_s, "source" => "*", "target" => "*" } if ends.nil?

        source, target = ends
        { "name" => name.to_s, "source" => source.to_s, "target" => target.to_s }
      end
    end

    def pair_of(entity, place)
      name, kind = case entity
                   when Entity then [entity.name, entity.kind]
                   when RecognizedEntity then [entity.text, entity.kind]
                   when Hash then [first_of(entity, :name) || first_of(entity, :text), first_of(entity, :kind)]
                   when Array then entity
                   else refuse("entity #{place} is a [name, kind] pair, a Hash, or an Entity")
                   end
      refuse("entity #{place} needs a name and a kind as text") unless name.is_a?(String) && kind.is_a?(String)

      [text_of(name, "entity #{place}"), text_of(kind, "entity #{place}")]
    end

    def first_of(hash, key)
      hash.key?(key) ? hash[key] : hash[key.to_s]
    end

    def entity(held)
      Entity.new(held["name"], held["kind"])
    end

    def recognized_entity(held)
      RecognizedEntity.new(*held.values_at("text", "start", "end", "length", "kind", "strength"))
    end

    def recognized(held)
      relations = held["relations"]&.map do |one|
        Relation.new(one["relation"], recognized_entity(one["source"]), recognized_entity(one["target"]), one["probability"], one["either"] == true)
      end
      Recognized.new(held.fetch("entities").map { |one| recognized_entity(one) }, relations)
    end

    def watch(row)
      @rows_mutex.synchronize { @rows[row.object_id] = row }
      watchdog
      row
    end

    def unwatch(row)
      @rows_mutex.synchronize { @rows.delete(row.object_id) }
    end

    # One thread runs every in-flight call's tick. A tick that raises keeps
    # its error in the row and fires the call's own token, and the call's
    # next wait slice stops it. The thread starts again after a fork.
    def watchdog
      return if @watchdog&.alive?

      @watchdog = Thread.new do
        Thread.current.name = "thinkthen-watchdog"
        loop do
          sleep WATCHDOG_INTERVAL
          @rows_mutex.synchronize { @rows.values }.each do |row|
            row.tick.call
          rescue Exception => e # rubocop:disable Lint/RescueException
            row.error ||= e
            row.token.cancel
          end
        end
      end
    end
  end
end

require_relative "thinkthen/complete"
require_relative "thinkthen/native_complete"

require_relative "thinkthen/session"
